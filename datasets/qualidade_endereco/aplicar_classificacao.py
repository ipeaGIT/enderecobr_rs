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
from typing import Literal

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


def main():
    classify = obter_classificador()
    classify.load(DSPY_PROMPT_PATH)
    print("Modelo carregado")

    backup_duckdb()

    for i in range(100):
        with duckdb.connect(DUCKDB_PATH, read_only=True) as con:
            rows = con.execute("""
            SELECT logradouro, numero, complemento, localidade, id
            FROM dataset
            WHERE qualidade IS NULL
            USING SAMPLE 20;
            """).fetchall()

        if len(rows) == 0:
            return

        dataset_pred = classify.batch([criar_exemplo_inferencia(r) for r in rows])

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


if __name__ == "__main__":
    main()
