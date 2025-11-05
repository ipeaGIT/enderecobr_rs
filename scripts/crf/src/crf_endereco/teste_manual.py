import sklearn_crfsuite
import sys
from crf_endereco.preproc import token2features, tokenize


def extrair_campos(crf: sklearn_crfsuite.CRF, frase: str) -> dict[str, list[str]]:
    tokens = tokenize(frase)
    x = [token2features(tokens, i) for i in range(len(tokens))]
    pred = crf.predict_single(x)
    print(pred)

    grupos: dict[str, list[str]] = {
        "LOG": [],
        "NUM": [],
        "COM": [],
        "LOC": [],
        "MUN": [],
        "UF": [],
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

    return grupos


def main():
    # Por algum motivo, basta só importar isso para a função
    # input funcionar adequadamente.
    import readline

    crf = sklearn_crfsuite.CRF(model_filename="../tagger.crf")

    while True:
        try:
            line = input()
        except EOFError:
            break
        if not line:
            continue
        campos = extrair_campos(crf, line)
        print(campos)


if __name__ == "__main__":
    main()
