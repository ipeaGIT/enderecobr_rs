#!/usr/bin/env -S uv run
# /// script
# requires-python = ">=3.10"
# dependencies = [
#   "dspy",
#   "duckdb",
# ]
# ///

# pyright: basic

import datetime
import json
import os
import pathlib
import shutil
import time
from typing import Callable, Literal, ParamSpec, TypeVar

import dspy
import duckdb

DUCKDB_PATH = "dados/dataset.duckdb"
DSPY_PROMPT_PATH = "dados/prompt-qualidade.json"


class AssinaturaQualidadeEndereco(dspy.Signature):
    endereco: str = dspy.InputField()
    qualidade: Literal["Processar", "Excluir"] = dspy.OutputField()


def obter_classificador() -> dspy.ChainOfThought:
    lm_llama = dspy.LM(
        model="openai/ollama-2gpu.gpt-oss:latest",
        api_key=os.environ["LLM_API_KEY"],
        api_base=os.environ.get("LLM_API_BASE", "https://ipeagpt.ipea.gov.br/api"),
        num_retries=3,
    )
    dspy.configure(lm=lm_llama)
    classify = dspy.ChainOfThought(AssinaturaQualidadeEndereco)
    return classify


####################


def criar_exemplo_inferencia(row: tuple[str, ...]):
    return dspy.Example(
        endereco=json.dumps(
            {
                "logradouro": row[0],
                "numero": row[1],
                "complemento": row[2],
                "localidade": row[3],
            }
        )
    ).with_inputs("endereco")


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


def carregar_dados() -> list[tuple]:
    with duckdb.connect(DUCKDB_PATH, read_only=True) as con:
        return con.execute("""
        SELECT logradouro, numero, complemento, localidade, id
        FROM dataset
        WHERE qualidade IS NULL
        USING SAMPLE 5;
        """).fetchall()


def salvar_dados(rows, dataset_pred):
    with duckdb.connect(DUCKDB_PATH) as con:
        for row, pred in zip(rows, dataset_pred):
            if pred is None or pred.qualidade is None:
                continue

            con.execute(
                """
                UPDATE dataset 
                SET qualidade = ?, qualidade_justificativa = ?, 
                qualidade_rotulador = 'LLM'
                WHERE id = ?;""",
                (
                    pred.qualidade,
                    pred.reasoning,
                    row[-1],  # id
                ),
            )


def main():
    classify = obter_classificador()
    classify.load(DSPY_PROMPT_PATH)
    print("Modelo carregado")

    backup_duckdb()

    for i in range(100):
        rows = retry_duckdb(carregar_dados)

        if len(rows) == 0:
            return

        dataset_pred = classify.batch([criar_exemplo_inferencia(r) for r in rows])

        retry_duckdb(lambda: salvar_dados(rows, dataset_pred))


if __name__ == "__main__":
    main()
