import re
from dataclasses import asdict
from typing import Callable

from crf_endereco.endereco import Endereco

import unicodedata


def tokenize(text: str):
    return re.findall(r"\d+|\w+|[^\s\w]", text)


class RotuladorEnderecoBIO:
    map_tags: dict[str, str] = {
        "logradouro": "LOG",
        "numero": "NUM",
        "complemento": "COM",
        "bairro": "LOC",
        "municipio": "MUN",
        "cep": "CEP",
    }

    def __init__(self, tokenizer: Callable[[str], list[str]]):
        self.tokenizer: Callable[[str], list[str]] = tokenizer

    def obter_tokenizado(self, endereco: Endereco) -> tuple[list[str], list[str]]:
        tokens: list[str] = []
        targets: list[str] = []

        ordem_campos = endereco.formato.split(" ")
        separador_toks = self.tokenizer(endereco.separador)
        tam_separador = len(separador_toks)

        # Transforma num dicionario para facilitar manipulação
        reg = {
            k: v.upper() if isinstance(v, str) else v
            for k, v in asdict(endereco).items()
        }

        for campo in ordem_campos:
            valor_campo = reg.get(campo)
            tag_atual = self.map_tags.get(campo)

            if not valor_campo or not tag_atual:
                continue

            # Adiciona separador no fim antes de adicionar os novos tokens,
            # senão estiver no início e se o separador "existir"
            if len(tokens) and tam_separador:
                tokens += separador_toks
                targets += ["O"] * tam_separador

            tokens_atuais = self.tokenizer(valor_campo)
            targets_atuais = [f"I-{tag_atual}"] * len(tokens_atuais)
            targets_atuais[0] = f"B-{tag_atual}"

            tokens += tokens_atuais
            targets += targets_atuais

        return tokens, targets


def normalize(text: str):
    normalized = unicodedata.normalize("NFKD", text)
    return "".join([c for c in normalized if c.isascii()])


def is_pontuacao(word: str) -> bool:
    if len(word) == 0:
        return False
    return word[0] in ",.;/:?!@#$%¨&*()-+[]{}\"'\\|"


# Refatorar!!
def token2features(sent: list[str], i: int) -> list[str]:
    def feat_pos(j: int, prefix: str):
        feats: list[str] = []
        word = normalize(sent[j]).upper().strip()
        feats.append(word)

        word_len = len(word)
        if word_len >= 7:
            word_len = "7+"
        feats.append(f"tam:{word_len}")

        if len(word) == 0:
            feats.append("only_is_not_ascii")
        if is_pontuacao(word):
            feats.append("is_punct")
            punct_len = len(word)
            if punct_len > 2:
                punct_len = "3+"
            feats.append(f"punct_len:{punct_len!s}")
            feats.remove(word)
            feats.append(word[0])
        elif word.isdigit():
            feats.remove(word)
            word = re.sub("^0+", "", word)
            feats.append("is_digit")
            word_len = len(word)
            if word_len >= 7:
                word_len = "7+"
            feats.append(f"digit_len:{word_len!s}")
        elif word.isalnum():
            feats.append("is_alpha")
            if any(char.isdigit() for char in word):
                feats.append("has_digit")
        else:
            feats.append("is_unknown")

        return [f"{prefix}:{f}" for f in feats]

    def get_next_word_pos(begin_i: int):
        j = begin_i + 1
        while j < len(sent):
            word = sent[j]
            if word.isalnum():
                return j
            j += 1

    def get_prev_word_pos(begin_i: int):
        j = begin_i - 1
        while j >= 0:
            word = sent[j]
            if word.isalnum():
                return j
            j -= 1

    features: list[str] = []
    features.append("bias")
    features.append(f"{int(i / len(sent) * 4)}_quarto")
    features += feat_pos(i, "0")

    if i == 0:
        features.append("BOS")

    prev_word_pos = get_prev_word_pos(i)
    if prev_word_pos is not None:
        features += feat_pos(prev_word_pos, "-1")

        if prev_word_pos < i - 1 and any(
            [is_pontuacao(tok) for tok in sent[prev_word_pos:i]]
        ):
            features.append("tem_pontuacao:-1")

        prev_prev_word_pos = get_prev_word_pos(prev_word_pos)
        if prev_prev_word_pos is not None:
            features += feat_pos(prev_prev_word_pos, "-2")

            if prev_prev_word_pos < prev_word_pos - 1 and any(
                [is_pontuacao(tok) for tok in sent[prev_prev_word_pos:prev_word_pos]]
            ):
                features.append("tem_pontuacao:-2")

    if i == len(sent) - 1:
        features.append("EOS")

    next_word_pos = get_next_word_pos(i)
    if next_word_pos is not None:
        features += feat_pos(next_word_pos, "+1")

        if next_word_pos > i + 1 and any(
            [is_pontuacao(tok) for tok in sent[i:next_word_pos]]
        ):
            features.append("tem_pontuacao:+1")

        next_next_word_pos = get_next_word_pos(next_word_pos)
        if next_next_word_pos is not None:
            features += feat_pos(next_next_word_pos, "+2")

            if next_word_pos > next_word_pos + 1 and any(
                [is_pontuacao(tok) for tok in sent[next_word_pos:next_next_word_pos]]
            ):
                features.append("tem_pontuacao:+2")

    return features


# Tokenizando antes
def sent2features(text: str) -> list[dict[str, float]]:
    toks = tokenize(text)
    return tokens2features(toks)


# Uso direto já tokenizado
def tokens2features(toks: list[str]) -> list[dict[str, float]]:
    return [{feat: 1.0 for feat in token2features(toks, i)} for i in range(len(toks))]
