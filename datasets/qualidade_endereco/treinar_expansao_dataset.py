#!/usr/bin/env -S uv run
# /// script
# requires-python = ">=3.10"
# dependencies = [
#   "duckdb",
#   "dspy",
# ]
# ///

# pyright: basic

import datetime
from doctest import Example
import pathlib
import os
import shutil
import time
from typing import Callable, TypeVar

import dspy
import duckdb

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
    erros de digitação, omitir trechos, omitir campos inteiros, etc. Maiúsculas, minúsculas, diacríticos e acentos não serão considerado pelo CRF, logo
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

    referencias_mod: list[str] = dspy.OutputField(
        desc="Informações extras incluídas no endereço original. Não é obrigatório, varie entre incluí-lo ou não. Evite informações que podem ser compreendidas como pertencentes a outros campos."
    )

    endereco_completo_mod: str = dspy.OutputField(
        desc="Endereço formatado final, contendo alguns ou todos trechos modificados. Pode conter informações extras não presentes nos campos modificados. Varie a formatação final, por exemplo, intercalando os campos numa ordem menos usual."
    )


class AssinaturaAvaliacaoExpansorEndereco(dspy.Signature):
    """Responsável pela avaliação de uma expansão de endereços, usada para de treinar um modelo de segmentação robusto baseado em CRF. A expansão pode adicionar abreviações,
    erros de digitação, omitir trechos, omitir campos inteiros, etc. Maiúsculas, minúsculas, diacríticos e acentos não serão considerado.
    Campos podem não ser incluído no resultado final apenas retornando uma string vazia para aquele caso.
    """

    logradouro_mod: str = dspy.InputField()
    numero_mod: str = dspy.InputField()
    complemento_mod: str = dspy.InputField()
    localidade_mod: str = dspy.InputField()
    municipio_mod: str = dspy.InputField()
    uf_mod: str = dspy.InputField()

    referencias_mod: list[str] = dspy.InputField(
        desc="Refências incluídas ao endereço. Pode não existir. Evite informações que podem ser compreendidas como pertencentes a outros campos, tipicamente que se confundam com complementos ou localidades."
    )

    endereco_completo_mod: str = dspy.InputField(
        desc="Endereço formatado final que será usado como input do CRF. Se um campo anterior foi adicionado, ele deve aparecer em algum lugar aqui. A formatação final pode variar, incluindo campos numa ordem não usual. "
    )

    manter: bool = dspy.OutputField(
        desc="Diz se os endereços finais gerados podem ser utilizados no treinamento do modelo CRF. Endereços muito bem comportados não são desejados, pois já ocorrem naturalmente no dataset, e o objetivo aqui é tornar o modelo mais robusto. Todos os campos presentes devem poder ser identificados de forma não ambígua no endereço completo."
    )


def obter_lm():
    return dspy.LM(
        model="openai/Qwen/Qwen3-235B-A22B-Instruct-2507",
        api_key=os.environ["LLM_API_KEY"],
        api_base=os.environ.get("LLM_API_BASE", "https://ipeagpt.ipea.gov.br/api"),
        num_retries=3,
        temperature=2,
    )


def obter_expansor() -> dspy.Module:
    expansor = dspy.Predict(AssinaturaExpansorDatasetEndereco)
    return expansor


def obter_juiz() -> dspy.Module:
    return dspy.ChainOfThought(AssinaturaAvaliacaoExpansorEndereco)


def expansao_para_example(pred: dspy.Prediction) -> dspy.Example:
    return dspy.Example(
        logradouro_mod=pred.logradouro_mod,
        numero_mod=pred.numero_mod,
        complemento_mod=pred.complemento_mod,
        localidade_mod=pred.localidade_mod,
        municipio_mod=pred.municipio_mod,
        uf_mod=pred.uf_mod,
        referencias_mod=pred.referencias_mod,
        endereco_completo_mod=pred.endereco_completo_mod,
    ).with_inputs(
        "logradouro_mod",
        "numero_mod",
        "complemento_mod",
        "localidade_mod",
        "municipio_mod",
        "uf_mod",
        "referencias_mod",
        "endereco_completo_mod",
    )


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

    expansor = obter_expansor()
    juiz = obter_juiz()

    exemplos = retry_duckdb(obter_exemplos)
    tam_train = int(len(exemplos) * 0.8)
    train = exemplos[:tam_train]
    val = exemplos[tam_train:]

    def metrica_com_feedback(
        exemple, pred, trace=None, pred_name=None, pred_trace=None
    ):
        julgado = juiz(**expansao_para_example(pred))
        if trace is None:
            return julgado.manter
        return dspy.Prediction(score=julgado.manter, feedback=julgado.reasoning)

    evaluate = dspy.Evaluate(
        devset=val,
        metric=metrica_com_feedback,
        display_progress=True,
        display_table=5,
        num_threads=2,
        provide_traceback=True,
    )
    print("Avaliação Inicial: ", evaluate(expansor))

    for v in val:
        print(v)
        expansao = expansor(**v)
        print(expansao)
        julgado = juiz(**expansao_para_example(expansao))
        print(julgado)

    optimizer = dspy.GEPA(
        metric=metrica_com_feedback, num_threads=2, auto="light", reflection_lm=lm
    )
    expansor_otim = optimizer.compile(expansor, trainset=train, valset=val)

    for v in val:
        print(v)
        expandido = expansor_otim(**v)
        print(expandido)
        print(metrica_com_feedback(v, expandido))

    expansor_otim.save("./dados/expansor.json")


if __name__ == "__main__":
    main()
