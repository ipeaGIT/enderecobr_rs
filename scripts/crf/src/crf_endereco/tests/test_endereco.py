from crf_endereco.endereco import (
    Endereco,
    EnderecoParametros,
    EnderecoGerador,
    GeradorParametrosEndereco,
)


def testa_formato_aleatorio():
    gerador = EnderecoGerador()
    gerador_parametro = GeradorParametrosEndereco(42)
    endereco = Endereco(
        "Rua A",
        "123",
        "apt 101",
        "Centro",
        "Rio de Janeiro",
        "RJ",
        "12345678",
        None,
    )
    assert (
        gerador.gerar_endereco(
            endereco, gerador_parametro.gerar_parametro()
        ).endereco_formatado
        is not None
    )


def testa_formato_padrao():
    gerador = EnderecoGerador()
    endereco = Endereco(
        "Rua A",
        "123",
        "apt 101",
        "Centro",
        "Rio de Janeiro",
        "RJ",
        "12345678",
        None,
    )
    assert (
        gerador.gerar_endereco(endereco, EnderecoParametros()).endereco_formatado
        == "RUA A, 123, APT 101, CENTRO, RIO DE JANEIRO"
    )
