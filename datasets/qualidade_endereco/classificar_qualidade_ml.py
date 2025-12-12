#!/usr/bin/env -S uv run
# /// script
# requires-python = ">=3.10"
# dependencies = [
#   "scikit-learn",
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
from sklearn import linear_model

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


def obter_embeddings_treinamento():
    with duckdb.connect(DUCKDB_PATH, read_only=True) as con:
        res = con.execute(
            "SELECT qualidade, embeddings FROM embeddings e LEFT JOIN dataset d ON d.id = e.id WHERE d.qualidade IS NOT NULL AND d.qualidade_rotulador != 'ML';"
        ).fetchnumpy()
        embs = np.vstack(res["embeddings"])
        labels = np.array(
            [1 if l == "Processar" else 0 for l in res["qualidade"].tolist()]
        )
        return embs, labels


def treinar_regressao():
    print("Treinando...")
    embs, labels = retry_duckdb(obter_embeddings_treinamento)
    model = linear_model.LogisticRegressionCV(max_iter=1000)
    model.fit(embs, labels)
    print("Scores", model.scores_)
    return model


def obter_embeddings_pendentes():
    print("Obtendo pendentes")
    with duckdb.connect(DUCKDB_PATH, read_only=True) as con:
        res = con.execute(
            "SELECT e.id, e.embeddings FROM embeddings e LEFT JOIN dataset d ON d.id = e.id WHERE d.qualidade IS NULL;"
        ).fetchnumpy()
        return np.vstack(res["embeddings"]), np.array([i for i in res["id"].tolist()])


def salvar_resultados(probas, ids):
    print(f"Salvando resultados ({len(ids)} novos)")
    with duckdb.connect(DUCKDB_PATH) as con:
        excluir = []
        processar = []
        for prob, idx in zip(probas, ids):
            if prob < 0.5:
                excluir.append(idx)
            else:
                processar.append(idx)

        con.execute(
            "UPDATE dataset SET qualidade = ?, qualidade_rotulador = ? WHERE id IN ?;",
            ("Excluir", "ML", excluir),
        )
        con.execute(
            "UPDATE dataset SET qualidade = ?, qualidade_rotulador = ? WHERE id IN ?;",
            ("Processar", "ML", processar),
        )


def main():
    backup_duckdb()

    model = treinar_regressao()

    pendentes, ids = retry_duckdb(obter_embeddings_pendentes)
    print(f"Pendentes: {pendentes.shape}")

    probas = model.predict_proba(pendentes)[:, 1]  # classe 1
    mask = (probas >= 0.9) | (probas <= 0.1)

    retry_duckdb(lambda: salvar_resultados(probas[mask].tolist(), ids[mask].tolist()))


if __name__ == "__main__":
    main()
