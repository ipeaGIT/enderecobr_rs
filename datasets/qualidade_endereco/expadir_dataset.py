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
import json
import os
import pathlib
import shutil
import time
from typing import Callable, TypeVar

import dspy
import duckdb

DUCKDB_PATH = "dados/dataset.duckdb"
EXPANSOES_DUCKDB_PATH = "dados/expansoes.duckdb"


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


def obter_exemplos():
    with duckdb.connect(DUCKDB_PATH, read_only=True) as con:
        rows = con.execute(
            """
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


def salvar_expansoes(valores):
    with duckdb.connect(EXPANSOES_DUCKDB_PATH, read_only=False) as con:
        con.execute("""
CREATE SEQUENCE IF NOT EXISTS expansao_seq;

CREATE TABLE IF NOT EXISTS expansao (id INTEGER PRIMARY KEY DEFAULT nextval('expansao_seq'), logradouro text, numero text, complemento text, localidade text, municipio text, uf text, referencias json, endereco text); """)
        vals = [
            (
                v.logradouro_mod,
                v.numero_mod,
                v.complemento_mod,
                v.localidade_mod,
                v.municipio_mod,
                v.uf_mod,
                json.dumps(v.referencias_mod),
                v.endereco_completo_mod,
            )
            for v in valores
        ]
        _ = con.executemany(
            """
INSERT INTO expansao(logradouro, numero, complemento, localidade, municipio, uf, referencias, endereco) VALUES(?, ?, ?, ?, ?, ?, ?, ?);
        """,
            vals,
        )


def main():
    lm = obter_lm()
    dspy.configure(lm=lm)

    expansor = obter_expansor()
    expansor.load("./dados/expansor.json")

    for _ in range(1_000):
        exemplos = retry_duckdb(obter_exemplos)
        expansoes = expansor.batch(exemplos)
        retry_duckdb(lambda: salvar_expansoes(expansoes))


if __name__ == "__main__":
    main()
