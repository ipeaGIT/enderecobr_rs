from collections import Counter
from pprint import pprint
import random

import duckdb
import sklearn_crfsuite
from sklearn_crfsuite import metrics

from crf_endereco.endereco import (
    Endereco,
    EnderecoGerador,
    EnderecoParametros,
)
from crf_endereco.preproc import RotuladorEnderecoBIO, tokenize, tokens2features


def erros_mais_comuns_feature(x, y_true, y_pred, n=20):
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


def amostra_erros(tokens, y_true, y_pred, n=10, classes: list[str] = []):
    erros = []
    for toks, true, pred in zip(tokens, y_true, y_pred):
        for t, p in zip(true, pred):
            if t != p and (len(classes) == 0 or t in classes or p in classes):
                erros.append((toks, true, pred))
                break

    random.shuffle(erros)
    return erros[:n]


def acuracia_labels(y_true, y_pred, labels):
    n = 0
    acertos = 0
    for true, pred in zip(y_true, y_pred):
        for t, p in zip(true, pred):
            if t not in labels and p not in labels:
                continue
            n += 1
            if t == p:
                acertos += 1

    return acertos, n


def acuracia_proporcional(y_true, y_pred):
    proporcoes = []
    for true, pred in zip(y_true, y_pred):
        acertos_sent = 0
        for t, p in zip(true, pred):
            if t == p:
                acertos_sent += 1
        prop_sent = acertos_sent / len(true)
        proporcoes.append(prop_sent)

    return sum(proporcoes) / len(proporcoes)


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


def show(sent: list[str], y_true: list[str], y_pred: list[str]):
    if len(sent) == 0:
        return

    max_len = max([len(t) for t in sent]) + 2
    print(f"{'String':<{max_len}} {'Target':<10} {'Predito':<10}")
    print("----------------------------------")
    for s, t, p in zip(sent, y_true, y_pred):
        print(f"{s:<{max_len}} {t:<10} {p:<10}")
    print("################################")


def avaliar(enderecos: list[Endereco]):
    rotulador = RotuladorEnderecoBIO(tokenize)

    x: list[list[dict[str, float]]] = []
    y: list[list[str]] = []
    categorias: list[list[str]] = []
    tokens: list[list[str]] = []

    parametros: list[EnderecoParametros] = []
    for f in (
        # "logradouro numero complemento",
        # "logradouro numero complemento bairro",
        "logradouro numero complemento bairro municipio",
        # "logradouro numero complemento bairro municipio cep",
        # "logradouro numero complemento cep bairro municipio",
        # "logradouro numero complemento bairro cep municipio",
        # "municipio bairro logradouro numero complemento",
        # "logradouro bairro numero complemento municipio",
        # "logradouro numero bairro complemento",
    ):
        for sep in (" ",):
            parametros.append(EnderecoParametros(formato=f, separador=sep))

    print("Processando dados...")
    gerador = EnderecoGerador()
    for endereco in enderecos:
        for p in parametros:
            novo_endereco = gerador.gerar_endereco(endereco, p)
            toks, tags = rotulador.obter_tokenizado(novo_endereco)
            feats = tokens2features(toks)
            x.append(feats)
            y.append(tags)
            tokens.append(toks)

            cats = [p.formato, p.separador]
            categorias.append(cats)

    print("Executando modelo...")
    crf = sklearn_crfsuite.CRF(model_filename="./dados/tagger.crf")

    y_pred = crf.predict(x)
    print(metrics.flat_classification_report(y, y_pred, zero_division=0.0))

    print(f"Acurácia no trecho completo: {metrics.sequence_accuracy_score(y, y_pred)}")

    print(f"Acurácia proporcional: {acuracia_proporcional(y, y_pred)}")

    print()

    for l in (("B-LOG", "I-LOG"), ("B-NUM", "I-NUM"), ("B-COM", "I-COM")):
        acertos, n = acuracia_labels(y, y_pred, l)
        print(f"Acurácia de {l}: {acertos}/{n} = {acertos / n:.2}")

    print()

    print("Erros Token:")
    pprint(erros_mais_comuns_token(x, y, y_pred))

    print()
    print("Erros Features:")
    pprint(erros_mais_comuns_feature(x, y, y_pred))

    print()
    print("Erros Categorias:")
    pprint(erros_mais_comuns_por_categoria(x, y, y_pred, categorias))

    # print("Amostra erros:")
    # amostra = amostra_erros(tokens, y, y_pred, n=7)
    # for a in amostra:
    #     show(*a)

    print()
    for classes in (("B-LOG", "I-LOG"), ("B-NUM", "I-NUM"), ("B-COM", "I-COM")):
        amostra = amostra_erros(tokens, y, y_pred, n=2, classes=list(classes))
        print(f"Amostra de Erros de {classes!s}")
        for a in amostra:
            show(*a)


def consulta_duckdb(query: str, batch_size: int = 1000):
    con = duckdb.connect()
    consulta = con.sql(query)

    while batch := consulta.fetchmany(batch_size):
        for b in batch:
            yield b

    con.close()


def main():
    iter = consulta_duckdb("""
select 
    coalesce(logradouro, ''), 
    coalesce(numero, ''),
    coalesce(complemento, ''),
    coalesce(localidade, ''),
    coalesce(cep, ''),
    coalesce(municipio, ''),
    coalesce(uf, ''),
    coalesce(origem, ''),
from './dados/teste.parquet'
""")
    avaliar(
        [
            Endereco(
                logradouro=str(d[0]),
                numero=str(d[1]),
                complemento=str(d[2]),
                bairro=str(d[3]),
                cep=str(d[4]),
                municipio=str(d[5]),
                uf=str(d[6]),
            )
            for d in iter
        ]
    )


if __name__ == "__main__":
    main()
