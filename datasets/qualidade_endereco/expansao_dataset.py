#!/usr/bin/env -S uv run
# /// script
# requires-python = ">=3.10"
# dependencies = [
#   "duckdb",
#   "dspy",
#   "sentence_transformers",
#   "strsimpy",
#   "einops",
# ]
# ///

# pyright: basic

import datetime
import pathlib
import os
import shutil
import time
from typing import Callable, TypeVar
from strsimpy.sorensen_dice import SorensenDice
from strsimpy.overlap_coefficient import OverlapCoefficient

import dspy
import duckdb
from sentence_transformers import CrossEncoder

MODEL_NAME = "jinaai/jina-reranker-v2-base-multilingual"

DUCKDB_PATH = "dados/dataset.duckdb"


def backup_duckdb():
    print("Criando Backup")
    src = pathlib.Path(DUCKDB_PATH)
    ts = datetime.datetime.now().strftime("%Y%m%d%H%M%S")
    backup_name = f"{src.stem}_{ts}{src.suffix}"
    backup_path = src.parent / backup_name
    shutil.copy2(src, backup_path)  # mantém metadados


R = TypeVar("R")


def retry_duckdb(func: Callable[..., R], retries=5, delay=5) -> R:
    for i in range(retries):
        try:
            return func()
        except duckdb.IOException:
            print(f"Tentantiva #{i} de acessar DuckDB")
            time.sleep(delay)
    raise Exception("DESISTO!")


class AssinaturaExpansorDatasetEndereco(dspy.Signature):
    """Responsável pela expansão do dataset de endereços, afim de treinar um modelo de segmentação robusto baseado em CRF. A expansão pode adicionar abreviações,
    erros de digitação, omitir trechos, etc, porém NÃO deve descaracterizar o dado segmentado original. Maiúsculas, minúsculas, diacríticos e acentos não serão considerado, logo
    crie os campos preferencialmente em maiúsculo e sem acentos e diacríticos.
    Campos podem não ser incluído no resultado final apenas retornando uma string vazia para aquele caso.
    """

    logradouro: str = dspy.InputField()
    numero: str = dspy.InputField()
    complemento: str = dspy.InputField()
    localidade: str = dspy.InputField()
    municipio: str = dspy.InputField()
    uf: str = dspy.InputField()

    logradouro_mod: str = dspy.OutputField()
    numero_mod: str = dspy.OutputField()
    complemento_mod: str = dspy.OutputField()
    localidade_mod: str = dspy.OutputField()
    municipio_mod: str = dspy.OutputField()
    uf_mod: str = dspy.OutputField()

    info_extras_mod: list[str] = dspy.OutputField(desc="Informações extras incluídas no endereço original. Não obrigatório. Evite informações que podem ser compreendidas como pertencentes a outros campos.")
    
    endereco_completo_mod: str = dspy.OutputField(
        desc="Endereço formatado final, contendo alguns ou todos trechos modificados. Pode conter informações extras não presentes nos campos modificados. Varie a formatação final, por exemplo, intercalando os campos numa ordem não usual."
    )


def obter_lm():
    return dspy.LM(
        model="openai/Qwen/Qwen3-235B-A22B-Instruct-2507",
        api_key=os.environ["LLM_API_KEY"],
        api_base=os.environ.get("LLM_API_BASE", "https://ipeagpt.ipea.gov.br/api"),
        num_retries=3,
        temperature=2,
    )


def obter_expansor() -> dspy.ChainOfThought:
    expansor = dspy.Predict(AssinaturaExpansorDatasetEndereco)
    return expansor


metrica_similaridade = OverlapCoefficient()
model = CrossEncoder(MODEL_NAME, trust_remote_code=True)


def metrica(args, pred):
    original = f"{args["logradouro"]} {args["numero"]} {args["complemento"]} {args["localidade"]} {args["municipio"]} {args["uf"]}"
    similaridade = metrica_similaridade.similarity(
        original.upper(), pred.endereco_completo_mod.upper()
    )

    print(original)

    original_estruturado = f"Logradouro: {args["logradouro"]}\nNúmero: {args["numero"]}\nComplemento: {args["complemento"]}\nLocalidade: {args["localidade"]}\nMunicípio:{args["municipio"]} {args["uf"]}"
    sem_sim = model.predict(
        [(f"{pred.endereco_completo_mod}", original)]
    )[0]

    print(pred.endereco_completo_mod)
    print(f"sem_sim {sem_sim} - similaridade {similaridade} = {sem_sim - similaridade}")

    return sem_sim - similaridade


def metrica_com_feedback(exemple, pred, trace=None, pred_name=None, pred_trace=None):

    if pred.logradouro_mod not in pred.endereco_completo_mod:
        return 0.0 if pred_name is None else dspy.Prediction(score=0.0, feedback=f"O logradouro gerado ({pred.logradouro_mod}) deve ser incluído no endereço completo.")
        
    if pred.numero_mod not in pred.endereco_completo_mod:
        return 0.0 if pred_name is None else dspy.Prediction(score=0.0, feedback=f"O número gerado ({pred.numero_mod}) deve ser incluído no endereço completo.")

    if pred.complemento_mod not in pred.endereco_completo_mod:
        return 0.0 if pred_name is None else dspy.Prediction(score=0.0, feedback=f"O complemento gerado ({pred.complemento_mod}) deve ser incluído no endereço completo.")

    if pred.localidade_mod not in pred.endereco_completo_mod:
        return 0.0 if pred_name is None else dspy.Prediction(score=0.0, feedback=f"A localidade gerada ({pred.localidade_mod}) deve ser incluída no endereço completo.")
    
    if pred.municipio_mod not in pred.endereco_completo_mod:
        return 0.0 if pred_name is None else dspy.Prediction(score=0.0, feedback=f"O município gerado ({pred.municipio_mod}) deve ser incluído no endereço completo.")

    if pred.uf_mod not in pred.endereco_completo_mod:
        return 0.0 if pred_name is None else dspy.Prediction(score=0.0, feedback=f"O UF gerado ({pred.uf_mod}) deve ser incluído no endereço completo.")

    if any([info not in pred.endereco_completo_mod for info in pred.info_extras_mod]):
        return 0.0 if pred_name is None else dspy.Prediction(score=0.0, feedback=f"Todas as informações extras geradas devem ser incluídas no endereço completo.")
    
    original = f"{exemple["logradouro"]} {exemple["numero"]} {exemple["complemento"]} {exemple["localidade"]} {exemple["municipio"]} {exemple["uf"]}"

    originais = [
        exemple["logradouro"],
        exemple["numero"],
        exemple["complemento"],
        exemple["localidade"],
    ]

    similaridades = (
        [
            metrica_similaridade.similarity(
                o.upper(), pred.endereco_completo_mod.upper()
            )
            for o in originais if o is not None and len(o) >= 3
        ]
    )

    text_sim = sum(similaridades)/len(similaridades)
    text_sim_adequado = text_sim >= 0.3 and text_sim <= 0.9

    sem_sim = model.predict(
        [(f"{pred.endereco_completo_mod}", original)]
    )[0]
    sem_sim_adequado = sem_sim >= 0.85

    tudo_adequado = sem_sim_adequado and text_sim_adequado
    
    if pred_name is None:
        return 1.0 if tudo_adequado else 0.0

    if tudo_adequado:
        return dspy.Prediction(score=1.0, feedback=f"Você gerou um exemplo de endereço completo com similaridade textual ({text_sim}) e semântica ({sem_sim}) adequados. De qualquer forma, pense em estratégias para tornar os exemplos mais diversificados.")
        
    feedback = "Você gerou um exemplo de endereço completo não adequado. "

    if not text_sim_adequado:
        feedback += f"A similaridade textual média dos campos de logradouro, número, complemento e localidade foi de {text_sim}, quando a meta é manter entre 0.3 e 0.9. Campos vazios não são considerados. "

    if not sem_sim_adequado:
        feedback += f"A similaridade semântica do endereço gerado com o endereço original foi de {sem_sim}, quando a meta é manter acima de 0.85. "

    feedback += "Lembrando que as métricas não são sensíveis a maíusculas ou minúsculas e descartam acentos e diacríticos. Além disso, busque forma de tornar os exemplos mais diversificados, inclusive na formatação, afim de tornar o modelo mais robusto."

    return dspy.Prediction(score=0.0, feedback=feedback)
    
#     return dspy.Prediction(
#         score=score,
#         feedback=f"""
# Você gerou um exemplo de endereço completo com a similaridade textual (Soma dos Overlap Coefficient de cada campo original com o endereço final) de {similaridade} e similaridade semântica de {sem_sim}, resultando num score final (sim. semântica - textual) de {score}. As métricas NÃO são sensíveis a maiúsculas ou minúsculas e descartam diacríticos. Pense em formas de aumentar a similaridade semântica e reduzir a similaridade textual. O objetivo é criar exemplos críveis e interessantes para formar um dataset.
# """,
#     )


def obter_exemplos():
    with duckdb.connect(DUCKDB_PATH, read_only=True) as con:
        rows = con.execute(
            """
        SELECT setseed(0.555);
        WITH a AS (
            SELECT logradouro, numero, complemento, localidade, municipio, uf
            FROM dataset
        ) SELECT * FROM a
        USING SAMPLE 100;
        """
        ).fetchall()
        return [
            dspy.Example(
                logradouro=row[0],
                numero=row[1],
                complemento=row[2],
                localidade=row[3],
                municipio=row[4],
                uf=row[5],
            ).with_inputs(
                "logradouro", "numero", "complemento", "localidade", "municipio", "uf"
            )
            for row in rows
        ]


def main():
    lm = obter_lm()
    dspy.configure(lm=lm)

    # expansor = dspy.Refine(obter_expansor(), N=3, threshold=1.0, reward_fn=metrica)
    expansor = obter_expansor()
    # expansor.load("./expansor2.json")

    exemplos = retry_duckdb(obter_exemplos)
    tam_train = int(len(exemplos) * 0.8)
    train = exemplos[:tam_train]
    val = exemplos[tam_train:]

    for e in val:
        pred = expansor(**e, n=5)
        print(pred)
        print(metrica_com_feedback(e, pred, pred_name="teste"))
        break

    evaluate = dspy.Evaluate(
        devset=val,
        metric=metrica_com_feedback,
        display_progress=True,
        display_table=5,
        num_threads=2,
        provide_traceback=True,
    )
    print("Avaliação Inicial: ", evaluate(expansor))

    optimizer = dspy.GEPA(
        metric=metrica_com_feedback, num_threads=2, auto="light", reflection_lm=lm
    )
    expansor_otim = optimizer.compile(expansor, trainset=train, valset=val)
    
    for e in val:
        print(expansor_otim(**e).endereco_completo_mod)
    

    expansor_otim.save("./expansor2.json")


if __name__ == "__main__":
    main()
