import duckdb
from sklearn.metrics import make_scorer
from sklearn.model_selection import ShuffleSplit, GridSearchCV
import sklearn_crfsuite
from sklearn_crfsuite.metrics import flat_accuracy_score

from crf_endereco.endereco import (
    ABREVIACOES_COMUNS,
    Endereco,
    EnderecoGerador,
    GeradorParametrosEndereco,
)
from crf_endereco.preproc import RotuladorEnderecoBIO, tokenize, ExtratorFeature


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
from './dados/treino.parquet'
"""
    con = duckdb.connect()
    print("Coletando dados...")
    dados = con.sql(query).fetchall()
    con.close()

    gerador_parametros = GeradorParametrosEndereco.sem_ruido()
    gerador = EnderecoGerador(ABREVIACOES_COMUNS)
    rotulador = RotuladorEnderecoBIO(tokenize)
    extrator = ExtratorFeature()

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
        feats = extrator.tokens2features(toks)
        x.append(feats)
        y.append(tags)

    crf = sklearn_crfsuite.CRF(
        algorithm="lbfgs",
        max_iterations=100,
        all_possible_transitions=False,
        min_freq=5,
        verbose=False,
        # model_filename="./dados/tagger.crf",
    )

    scorer = make_scorer(flat_accuracy_score)
    dataset_splitter = ShuffleSplit(n_splits=1, test_size=0.1, random_state=42)

    # Experimental
    # from sklearn.experimental import enable_halving_search_cv
    # from sklearn.model_selection import HalvingRandomSearchCV
    # params_space = {
    #     "c1": scipy.stats.expon(scale=0.5),
    #     "c2": scipy.stats.expon(scale=0.05),
    # }
    # random_search = HalvingRandomSearchCV(
    #     crf,
    #     params_space,
    #     cv=dataset_splitter,
    #     verbose=3,
    #     n_jobs=-1,
    #     scoring=scorer,
    #     n_candidates=100,
    #     return_train_score=True,
    #     max_resources=1000,
    #     min_resources='exhaust',
    #     resource='max_iterations',
    # )
    random_search = GridSearchCV(
        crf,
        param_grid={
            "c1": [0.1, 0.25, 0.5, 0.75, 1.0],
            "c2": [0.1, 0.25, 0.5, 0.75, 1.0],
        },
        cv=dataset_splitter,
        verbose=3,
        n_jobs=13,
        scoring=scorer,
        refit=False,
    )
    _ = random_search.fit(x, y)

    print(f"Results: {random_search.cv_results_!s}")
    print("best params:", random_search.best_params_)
    print("best CV score:", random_search.best_score_)
    print("model size: {:0.2f}M".format(random_search.best_estimator_.size_ / 1000000))


if __name__ == "__main__":
    main()
