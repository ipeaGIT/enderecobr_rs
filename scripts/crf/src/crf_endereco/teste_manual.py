import sklearn_crfsuite
from crf_endereco.preproc import ExtratorFeature, tokenize
from pprint import pprint


def extrair_campos(
    crf: sklearn_crfsuite.CRF, extrator: ExtratorFeature, frase: str
) -> dict[str, list[str]]:
    tokens = tokenize(frase)
    x = extrator.tokens2features(tokens)
    pprint(x)
    pred = crf.predict_single(x)

    grupos: dict[str, list[str]] = {
        "LOG": [],
        "NUM": [],
        "COM": [],
        "LOC": [],
        "MUN": [],
        "CEP": [],
    }
    current = None

    for tok, tag in zip(tokens, pred):
        if tag.startswith("B-"):
            current = tag[2:]
            grupos[current].append(tok)
        elif tag.startswith("I-") and current:
            grupos[current][-1] += " " + tok
        else:
            current = None

    # for tag, tokens in grupos.items():
    #     grupos[tag] = [t.strip() for t in tokens]

    return grupos


def main():
    # Por algum motivo, basta só importar isso para a função
    # input funcionar adequadamente.
    import readline as _

    crf = sklearn_crfsuite.CRF(model_filename="./dados/tagger.crf")
    extrator = ExtratorFeature()

    while True:
        try:
            line = input()
        except EOFError:
            break
        if not line:
            continue
        campos = extrair_campos(crf, extrator, line)
        print(campos)


if __name__ == "__main__":
    main()
