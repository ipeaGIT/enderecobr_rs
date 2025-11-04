from collections import Counter
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


def main():
    query = """
select
    concat_ws(' ', nullif(nom_tipo_seglogr, 'EDF'), nullif(nom_titulo_seglogr, ''), nom_seglogr) as logradouro, 
    concat_ws(' ', nullif(num_adress, 0)) as numero,
    concat_ws(' ', nullif(dsc_modificador, 'SN'), nom_comp_elem1, val_comp_elem1, nom_comp_elem2, val_comp_elem2, nom_comp_elem3, val_comp_elem3, nom_comp_elem4)
        .regexp_replace('[ ]+', ' ', 'g').trim() as complemento,
    desc_localidade as localidade,
    cep,
    m.municipio,
    m.uf,
from '/mnt/storage6/bases/DADOS/PUBLICO/CNEFE/parquet/2022/arquivos/*.parquet'
inner join '/home/gabriel/ipea/enderecobr-rs/src/data/municipios.csv' as m on cod_ibge = code_muni
using sample 400_000
"""
    con = duckdb.connect()
    print("Coletando dados...")
    dados = con.sql(query).fetchall()
    con.close()

    gerador_parametros = GeradorParametrosEndereco(seed=42)
    gerador = EnderecoGerador(ABREVIACOES_COMUNS)
    rotulador = RotuladorEnderecoBIO(tokenize)

    x: list[list[dict[str, float]]] = []
    y: list[list[str]] = []

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

    x_train, x_test, y_train, y_test = train_test_split(x, y, test_size=0.2)

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

    print("Erros:")
    pprint(erros_mais_comuns(x_test, y_test, y_test_pred))

    print()
    print("TRAIN")
    print("==================================")
    y_train_pred = crf.predict(x_train)
    print(metrics.flat_classification_report(y_train, y_train_pred))

    print("Erros:")
    pprint(erros_mais_comuns(x_train, y_train, y_train_pred))


if __name__ == "__main__":
    main()
