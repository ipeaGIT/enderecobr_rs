#!/usr/bin/env -S uv run
# /// script
# requires-python = ">=3.10"
# dependencies = [
#   "sentence_transformers",
#   "numpy",
#   "duckdb",
# ]
# ///

# pyright: basic

import datetime
import pathlib
import shutil
import time
from typing import Callable, TypeVar

import duckdb
import numpy as np
from sentence_transformers import SentenceTransformer

MODEL_NAME = "Qwen/Qwen3-Embedding-0.6B"

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


def ensure_tables():
    """Cria a tabela destino se não existir."""
    with duckdb.connect(DUCKDB_PATH) as con:
        con.execute("""
        CREATE TABLE IF NOT EXISTS embeddings (
            id int,
            embeddings float[1024]
        );
        """)


def load_missing_embeddings():
    """Carrega textos sem embedding."""
    with duckdb.connect(DUCKDB_PATH, read_only=True) as con:
        rows = con.execute(
            """
            WITH a AS (
                SELECT id, logradouro, numero, complemento, localidade, municipio, uf  
                FROM dataset 
                WHERE id NOT IN (SELECT id FROM embeddings)
            ) SELECT * FROM a USING SAMPLE 100;"""
        ).fetchall()

        ids: list[int] = []
        texts: list[str] = []
        for row in rows:
            text = f"""Endereço Estruturado:
Logradouro: {row[1]}
Número: {row[2]}
Complemento: {row[3]}
Localidade: {row[4]}
Município: {row[5]}
UF: {row[6]}
"""
            texts.append(text)
            ids.append(row[0])

    return ids, texts


def salvar_embeddings(ids, embeddings):
    with duckdb.connect(DUCKDB_PATH) as con:
        for idx, emb in zip(ids, embeddings):
            con.execute(
                "INSERT INTO embeddings(id, embeddings) VALUES (?, ?);", (idx, emb)
            )


def main():
    backup_duckdb()
    retry_duckdb(ensure_tables)
    model = SentenceTransformer(MODEL_NAME)

    while True:
        print("Carregando valores faltantes")
        ids, texts = retry_duckdb(load_missing_embeddings)
        if len(ids) == 0:
            return

        # Cria embeddings
        print("Computando embeddings")
        embeddings = model.encode(
            texts,
            convert_to_numpy=True,
            normalize_embeddings=True,
            show_progress_bar=True,
            device=["cpu"],
        ).astype(np.float32)

        # Insere/atualiza na DB destino
        print("Salvando embeddings")
        retry_duckdb(lambda: salvar_embeddings(ids, embeddings))


if __name__ == "__main__":
    main()
