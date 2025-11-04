from collections import Counter
from pprint import pprint

import duckdb
import sklearn_crfsuite
from sklearn_crfsuite import metrics

from crf_endereco.endereco import (
    Endereco,
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


def avaliar(enderecos: list[Endereco]):
    rotulador = RotuladorEnderecoBIO(tokenize)

    x: list[list[dict[str, float]]] = []
    y: list[list[str]] = []

    for endereco in enderecos:
        toks, tags = rotulador.obter_tokenizado(endereco)
        feats = tokens2features(toks)
        x.append(feats)
        y.append(tags)

    crf = sklearn_crfsuite.CRF(model_filename="../tagger.crf")

    print("==================================")
    y_pred = crf.predict(x)
    print(metrics.flat_classification_report(y, y_pred, zero_division=0.0))

    print("Erros Token:")
    pprint(erros_mais_comuns_token(x, y, y_pred))

    print("Erros Features:")
    pprint(erros_mais_comuns_feature(x, y, y_pred))


def consulta_duckdb(query: str, batch_size: int = 1000):
    con = duckdb.connect()
    consulta = con.sql(query)

    while batch := consulta.fetchmany(batch_size):
        for b in batch:
            yield b

    con.close()


def main():
    print("Avaliando base de CNPJs...")

    iter = consulta_duckdb("""
SELECT
    concat_ws(' ', tipo_logradouro, logradouro) as logradouro,
    coalesce(numero_estab, '') as numero,
    coalesce(complemento, ''),
    coalesce(bairro, ''),
    coalesce(cep, ''),
    -- m.municipio,
    -- m.uf,
FROM
    read_parquet('/mnt/storage6/bases/DADOS/PUBLICO/CNPJ/parquet/estabelecimentos/*.parquet') as d
-- inner join '/home/gabriel/ipea/enderecobr-rs/src/data/municipios.csv' as m on m.cod_ibge = d.municipio
using sample 10_000
""")
    avaliar(
        [
            Endereco(
                logradouro=d[0],
                numero=d[1],
                complemento=d[2],
                bairro=d[3],
                cep=d[4],
                municipio="",
                uf="",
                formato="logradouro numero complemento bairro cep",
            )
            for d in iter
        ]
    )

    print("==========================")
    print("Carregando base do CNEAS...")

    iter = consulta_duckdb("""
    SELECT
            cneas_entidade_endereco_logradouro_s as logradouro,
            cneas_entidade_endereco_numero_s as numero,
            cneas_entidade_endereco_complemento_s as complemento,
            cneas_entidade_endereco_bairro_s as bairro,
            cneas_entidade_endereco_cep_s as cep,
            codigo_ibge as cod_ibge,
            cneas_entidade_nome_municipio_s as municipio,
            cneas_entidade_sigla_uf_s as uf,
    FROM
            read_csv('./cneas.csv')
    """)
    avaliar(
        [
            Endereco(
                logradouro=d[0],
                numero=d[1],
                complemento=d[2],
                bairro=d[3],
                cep=d[4],
                municipio=d[6],
                uf=d[7],
                formato="logradouro numero complemento bairro municipio cep",
            )
            for d in iter
        ]
    )

    print("==========================")
    print("Carregando base dos Postos do CADUnico...")

    iter = consulta_duckdb("""
SELECT
        endereco,
        numero,
        complemento,
        bairro,
        cep,
        cidade,
        uf,
        -- referencia,
        -- georef_location,
FROM
        read_csv('./postos-cadunico.csv')
""")
    avaliar(
        [
            Endereco(
                logradouro=d[0],
                numero=d[1],
                complemento=d[2],
                bairro=d[3],
                cep=d[4],
                municipio=d[5],
                uf=d[6],
                formato="logradouro numero complemento bairro municipio cep",
                separador=" ",
            )
            for d in iter
        ]
    )

    print("==========================")
    print("Carregando base do CNES...")

    iter = consulta_duckdb("""
SELECT
        no_logradouro as endereco,
        nu_endereco as numero,
        -- Não tem complemento
        no_bairro as bairro,
        co_cep as cep,
        m.municipio,
        m.uf,
FROM
        './cnes_estabelecimentos.csv' as e
LEFT JOIN '../../src/data/municipios.csv' as m on m.cod_ibge = e.co_ibge
""")
    avaliar(
        [
            Endereco(
                logradouro=d[0],
                numero=d[1],
                complemento="",
                bairro=d[2],
                cep=d[3],
                municipio=d[4],
                uf=d[5],
                # formato="logradouro numero complemento bairro municipio cep",
                formato="logradouro numero complemento bairro",
                separador=", ",
            )
            for d in iter
        ]
    )

    print("==========================")
    print("Carregando base do Inep...")

    iter = consulta_duckdb("""
SELECT
        coalesce(ds_endereco, '') as endereco,
        coalesce(nu_endereco, '') as numero,
        coalesce(ds_complemento, '') as complemento,
        coalesce(no_bairro, '') as bairro,
        coalesce(co_cep, '') as cep,
        coalesce(no_municipio, '') as municipio,
        coalesce(sg_uf, '') as uf,
FROM
        './inep-utf.csv'
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
                formato="logradouro numero complemento bairro municipio cep",
                # formato="logradouro numero complemento bairro",
                separador=" ",
            )
            for d in iter
        ]
    )


if __name__ == "__main__":
    main()
