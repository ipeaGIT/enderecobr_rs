import duckdb
import sklearn_crfsuite

from crf_endereco.endereco import (
    ABREVIACOES_COMUNS,
    Endereco,
    EnderecoGerador,
    GeradorParametrosEndereco,
)
from crf_endereco.preproc import (
    ExtratorFeature,
    RotuladorEnderecoBIO,
    tokenize,
)


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
from dataset
where qualidade = 'Processar';
"""
    con = duckdb.connect("datasets/dados/dataset.duckdb")
    print("Coletando dados...")
    dados = con.sql(query).fetchall()
    con.close()

    gerador_parametros = GeradorParametrosEndereco.sem_ruido()
    gerador = EnderecoGerador(ABREVIACOES_COMUNS)
    rotulador = RotuladorEnderecoBIO(tokenize)
    extrator_features = ExtratorFeature()

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
        feats = extrator_features.tokens2features(toks)
        x.append(feats)
        y.append(tags)

    print("Realizando treinamento...")
    crf = sklearn_crfsuite.CRF(
        algorithm="lbfgs",
        c1=0.75,
        c2=0.1,
        max_iterations=50,
        all_possible_transitions=False,
        min_freq=5,
        model_filename="./datasets/dados/tagger.crf",
    )
    _ = crf.fit(x, y)


if __name__ == "__main__":
    main()
