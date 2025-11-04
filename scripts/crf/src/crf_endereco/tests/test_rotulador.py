import pytest

from crf_endereco.endereco import Endereco
from crf_endereco.preproc import RotuladorEnderecoBIO, tokenize


@pytest.mark.parametrize(
    "text,expected",
    [
        ("Rua das Flores, 123", ["Rua", "das", "Flores", ",", "123"]),
        (
            "Av. Paulista 1000 - São Paulo",
            ["Av", ".", "Paulista", "1000", "-", "São", "Paulo"],
        ),
        # Não esperava que nº iria ficar junto.
        ("R. Azul, nº 45", ["R", ".", "Azul", ",", "nº", "45"]),
        ("CEP: 01311-200", ["CEP", ":", "01311", "-", "200"]),
        ("Bloco B apto. 12", ["Bloco", "B", "apto", ".", "12"]),
    ],
)
def test_tokenize(text: str, expected: list[str]):
    assert tokenize(text) == expected


@pytest.fixture
def rotulador():
    return RotuladorEnderecoBIO(tokenize)


def test_rotulador_simples(rotulador: RotuladorEnderecoBIO):
    e = Endereco(
        logradouro="Rua das Flores",
        numero="123",
        complemento="Apto 5",
        bairro="Centro",
        municipio="São Paulo",
        uf="SP",
        cep="01000-000",
        formato="logradouro numero complemento bairro municipio",
    )

    tokens, tags = rotulador.obter_tokenizado(e)

    assert tokens == [
        "RUA",
        "DAS",
        "FLORES",
        ",",
        "123",
        ",",
        "APTO",
        "5",
        ",",
        "CENTRO",
        ",",
        "SÃO",
        "PAULO",
    ]
    assert tags == [
        "B-LOG",
        "I-LOG",
        "I-LOG",
        "O",
        "B-NUM",
        "O",
        "B-COM",
        "I-COM",
        "O",
        "B-LOC",
        "O",
        "B-MUN",
        "I-MUN",
    ]


def test_rotulador_sem_complemento(rotulador: RotuladorEnderecoBIO):
    e = Endereco(
        logradouro="Rua Azul",
        numero="45",
        complemento="",
        bairro="Bela Vista",
        municipio="Rio",
        uf="RJ",
        cep="22222-222",
        formato="logradouro numero complemento bairro municipio",
    )
    tokens, tags = rotulador.obter_tokenizado(e)

    assert tokens == ["RUA", "AZUL", ",", "45", ",", "BELA", "VISTA", ",", "RIO"]
    assert tags == ["B-LOG", "I-LOG", "O", "B-NUM", "O", "B-LOC", "I-LOC", "O", "B-MUN"]


def test_rotulador_separador(rotulador: RotuladorEnderecoBIO):
    e = Endereco(
        logradouro="Av Paulista",
        numero="1000",
        complemento="CJ 12",
        bairro="Bela Vista",
        municipio="São Paulo",
        uf="SP",
        cep="01311-200",
        formato="logradouro numero bairro municipio",
        separador=" - ",
    )
    tokens, tags = rotulador.obter_tokenizado(e)

    assert tokens == [
        "AV",
        "PAULISTA",
        "-",
        "1000",
        "-",
        "BELA",
        "VISTA",
        "-",
        "SÃO",
        "PAULO",
    ]

    assert tags == [
        "B-LOG",
        "I-LOG",
        "O",
        "B-NUM",
        "O",
        "B-LOC",
        "I-LOC",
        "O",
        "B-MUN",
        "I-MUN",
    ]


def test_rotulador_ordem_diferente(rotulador: RotuladorEnderecoBIO):
    e = Endereco(
        logradouro="Rua A",
        numero="10",
        complemento="Fundos",
        bairro="Centro",
        municipio="Curitiba",
        uf="PR",
        cep="80000-000",
        formato="municipio bairro logradouro numero",
    )

    tokens, tags = rotulador.obter_tokenizado(e)

    assert tokens == ["CURITIBA", ",", "CENTRO", ",", "RUA", "A", ",", "10"]

    assert tags == ["B-MUN", "O", "B-LOC", "O", "B-LOG", "I-LOG", "O", "B-NUM"]


def test_rotulador_apenas_um_campo(rotulador: RotuladorEnderecoBIO):
    e = Endereco(
        logradouro="Rua das Laranjeiras",
        numero="123",
        complemento="Casa 2",
        bairro="Laranjeiras",
        municipio="Rio de Janeiro",
        uf="RJ",
        cep="9999999",
        formato="logradouro",
    )
    tokens, tags = rotulador.obter_tokenizado(e)

    assert tokens == ["RUA", "DAS", "LARANJEIRAS"]
    assert tags == ["B-LOG", "I-LOG", "I-LOG"]
