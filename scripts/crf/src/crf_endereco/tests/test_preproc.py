from crf_endereco.preproc import (
    ExtratorFeature,
    normalize,
)


# ----------------------------
# normalize
# ----------------------------
def test_normalize_remove_acentos():
    assert normalize("ação") == "acao"


def test_normalize_maiusculas_e_minusculas():
    assert normalize("ÁbÇ") == "AbC"


def test_normalize_sem_acentos_retorna_igual():
    assert normalize("Rua") == "Rua"


# ----------------------------
# token2features
# ----------------------------


def token2features(sent: list[str], i: int):
    return ExtratorFeature().tokens2features(sent)[i]


def tokens2features(sent: list[str]):
    return ExtratorFeature().tokens2features(sent)


def test_token2features_primeiro_token_basico():
    sent = ["Rua", "das", "Flores"]
    feats = token2features(sent, 0)
    assert "bias" in feats
    assert "0_pos" in feats
    assert "0:RUA" in feats
    assert "0:is_alpha" in feats
    assert "BOS" in feats
    assert "+1:DAS" in feats
    assert "+1:is_alpha" in feats


def test_token2features_token_final():
    sent = ["Rua", "das", "Flores"]
    feats = token2features(sent, 2)
    assert "bias" in feats
    assert "EOS" in feats
    assert "-2:RUA" in feats
    assert "-2:is_alpha" in feats
    assert "-1:DAS" in feats
    assert "-1:is_alpha" in feats
    assert "0:FLORES" in feats
    assert "0:is_alpha" in feats


def test_token2features_token_meio_com_digito():
    sent = ["Rua", "123", "Centro"]
    feats = token2features(sent, 1)
    assert "0:is_digit" in feats
    assert "0:digit_len:3" in feats
    assert "-1:RUA" in feats
    assert "+1:CENTRO" in feats


def test_token2features_token_pontuacao():
    sent = ["Rua", ",", "Centro"]
    feats = token2features(sent, 1)
    assert "0:," in feats
    assert "0:is_punct" in feats
    assert "-1:RUA" in feats
    assert "+1:CENTRO" in feats


def test_token2features_com_token_alfanumerico():
    sent = ["Rua", "A1", "Centro"]
    feats = token2features(sent, 1)
    assert "0:A1" in feats
    assert "0:is_alpha" in feats
    assert "0:has_digit" in feats
    assert "-1:RUA" in feats
    assert "+1:CENTRO" in feats


def test_token2features_com_pontuacao_entre_palavras():
    sent = ["Rua", ",", "das", "Flores"]
    feats = token2features(sent, 0)
    assert "0:RUA" in feats
    assert "+1:DAS" in feats
    assert "+2:FLORES" in feats


def test_token2features_ignora_pontuacoes_sucessivas():
    sent = ["Rua", ",", ".", "Flores"]
    feats = token2features(sent, 0)
    assert "0:RUA" in feats
    assert "+1:FLORES" in feats
    assert "+1:is_alpha" in feats


def test_token2features_palavra_longa():
    sent = ["Inconstitucionalissimamente"]
    feats = token2features(sent, 0)
    assert "0:INCONSTITUCIONALISSIMAMENTE" in feats
    assert "BOS" in feats and "EOS" in feats


def test_token2features_todos_pontuacao():
    sent = [",", ".", ";"]
    feats = token2features(sent, 1)
    assert "0:is_punct" in feats
    assert "BOS" not in feats and "EOS" not in feats


# ----------------------------
# tokens2features
# ----------------------------
def test_tokens2features_lista_basica():
    toks = ["Rua", "das"]
    feats = tokens2features(toks)
    assert isinstance(feats, list)
    assert isinstance(feats[0], dict)
    assert "0:RUA" in feats[0]
    assert "0:DAS" in feats[1]


def test_tokens2features_contem_bias_em_todos():
    toks = ["A", "B", "C"]
    feats = tokens2features(toks)
    assert all("bias" in f for f in feats)
