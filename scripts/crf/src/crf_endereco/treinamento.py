from collections import Counter
from dataclasses import asdict
from pprint import pprint

import duckdb
import sklearn_crfsuite
from sklearn.model_selection import train_test_split
from sklearn_crfsuite import metrics

from crf_endereco.endereco import (
    ABREVIACOES_COMUNS,
    Endereco,
    EnderecoGerador,
    GeradorParametrosEndereco,
)
from crf_endereco.preproc import RotuladorEnderecoBIO, tokenize, tokens2features


def erros_mais_comuns(x, y_true, y_pred, n=20):
    errors = Counter()
    for feats, true, pred in zip(x, y_true, y_pred):
        for i, (t, p) in enumerate(zip(true, pred)):
            if t != p:
                errors[(str(feats[i]), f"true={t}", f"pred={p}")] += 1
    return errors.most_common(n)


def erros_mais_comuns_token(x, y_true, y_pred, n=20):
    errors = Counter()
    for feats, true, pred in zip(x, y_true, y_pred):
        for i, (t, p) in enumerate(zip(true, pred)):
            if t != p:
                token_atual = [f[2:] for f in feats[i] if f.startswith("0:")]
                if len(token_atual):
                    errors[(str(token_atual[0]), f"true={t}", f"pred={p}")] += 1
    return errors.most_common(n)


def erros_mais_comuns_por_categoria(x, y_true, y_pred, categorias, n=20):
    errors = Counter()
    for feats, true, pred, cats in zip(x, y_true, y_pred, categorias):
        dif_tags = set()
        for i, (t, p) in enumerate(zip(true, pred)):
            if t != p:
                dif_tags.add((t, p))

        for dif in dif_tags:
            for c in cats:
                errors[(dif, c)] += 1

    return errors.most_common(n)


def main():
    query = """
select 
    coalesce(logradouro, ''), 
    coalesce(numero, ''),
    coalesce(complemento, ''),
    coalesce(localidade, ''),
    coalesce(cep, ''),
    coalesce(municipio, ''),
    coalesce(uf, ''),
    coalesce(origem, ''),
from './dados/dataset.parquet'
"""
    con = duckdb.connect()
    print("Coletando dados...")
    dados = con.sql(query).fetchall()
    con.close()

    gerador_parametros = GeradorParametrosEndereco.sem_ruido()
    gerador = EnderecoGerador(ABREVIACOES_COMUNS)
    rotulador = RotuladorEnderecoBIO(tokenize)

    x: list[list[dict[str, float]]] = []
    y: list[list[str]] = []
    categorias: list[list[str]] = []

    print("Preprocessando dados...")
    for d in dados:
        params = gerador_parametros.gerar_parametro()
        endereco = Endereco(
            logradouro=str(d[0]),
            numero=str(d[1]),
            complemento=str(d[2]),
            bairro=str(d[3]),
            cep=str(d[4]),
            municipio=str(d[5]),
            uf=str(d[6]),
        )
        novo_endereco = gerador.gerar_endereco(endereco, params)
        toks, tags = rotulador.obter_tokenizado(novo_endereco)
        feats = tokens2features(toks)
        x.append(feats)
        y.append(tags)

        cats = [
            f"{k} = {str(v).upper()}"
            for k, v in asdict(params).items()
            if k in ("formato", "separador")
        ]
        cats.append(str(d[7]))
        categorias.append(cats)

    x_train, x_test, y_train, y_test, categorias_train, categorias_test = (
        train_test_split(x, y, categorias, test_size=0.1)
    )

    print("Realizando treinamento...")
    crf = sklearn_crfsuite.CRF(
        algorithm="lbfgs",
        c1=0.001,
        c2=0.01,
        max_iterations=100,
        all_possible_transitions=False,
        min_freq=5,
        model_filename="../tagger.crf",
    )
    _ = crf.fit(x_train, y_train)

    print("Avaliando...")

    print("TEST")
    print("==================================")
    y_test_pred = crf.predict(x_test)
    print(metrics.flat_classification_report(y_test, y_test_pred))

    print("Sequence Accuracy:", metrics.sequence_accuracy_score(y_test, y_test_pred))

    print("Erros:")
    pprint(erros_mais_comuns_token(x_test, y_test, y_test_pred))
    print()
    pprint(erros_mais_comuns(x_test, y_test, y_test_pred))
    print()
    pprint(
        erros_mais_comuns_por_categoria(x_test, y_test, y_test_pred, categorias_test)
    )

    print()
    print("TRAIN")
    print("==================================")
    y_train_pred = crf.predict(x_train)
    print(metrics.flat_classification_report(y_train, y_train_pred))

    print("Sequence Accuracy:", metrics.sequence_accuracy_score(y_train, y_train_pred))

    print("Erros:")
    pprint(erros_mais_comuns_token(x_train, y_train, y_train_pred))
    print()
    pprint(erros_mais_comuns(x_train, y_train, y_train_pred))
    print()
    pprint(
        erros_mais_comuns_por_categoria(
            x_train, y_train, y_train_pred, categorias_train
        )
    )


if __name__ == "__main__":
    main()
