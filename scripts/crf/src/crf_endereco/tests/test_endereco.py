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
    )
    assert (
        gerador.gerar_endereco(endereco, gerador_parametro.gerar_parametro())
        is not None
    )


def testa_abreviacao():
    gerador = EnderecoGerador({"RUA": ["R."]})
    endereco = Endereco(
        "Rua das Flores", "10", "", "Centro", "São Paulo", "SP", "01000000"
    )
    params = EnderecoParametros(abreviar_campos={"logradouro"})
    e = gerador.gerar_endereco(endereco, params)
    assert e.logradouro == "R. DAS FLORES"


def testa_excluir_palavra():
    gerador = EnderecoGerador()
    endereco = Endereco(
        "Rua das Flores", "10", "", "Centro da Cidade", "São Paulo", "SP", "01000000"
    )
    params = EnderecoParametros(excluir_palavra_campos={"logradouro", "bairro"})
    e = gerador.gerar_endereco(endereco, params)
    # Remove a segunda palavra
    assert e.logradouro == "RUA FLORES"
    assert e.bairro == "CENTRO CIDADE"


def testa_misturar_municipio_uf():
    gerador = EnderecoGerador()
    endereco = Endereco("Rua A", "1", "", "Centro", "Campinas", "SP", "13000000")
    params = EnderecoParametros(misturar_municipio_uf=True, separador_uf="-")
    e = gerador.gerar_endereco(endereco, params)
    assert e.municipio == "CAMPINAS-SP"


def testa_numero_inexistente():
    gerador = EnderecoGerador()
    endereco = Endereco("Rua B", "", "", "Bairro X", "Cidade Y", "MG", "98765432")
    params = EnderecoParametros(padrao_numero_inexistente="SEM NUMERO")
    e = gerador.gerar_endereco(endereco, params)
    assert e.numero == "SEM NUMERO"


def testa_numero_separador_milhar_e_prefixo():
    gerador = EnderecoGerador()
    endereco = Endereco(
        "Av. Brasil", "12345", "", "Centro", "Curitiba", "PR", "80000000"
    )
    params = EnderecoParametros(numero_separado_milhar=True, prefixo_numero="Nº")
    e = gerador.gerar_endereco(endereco, params)
    assert e.numero == "Nº 12,345"


def testa_formatos_cep():
    gerador = EnderecoGerador()
    endereco = Endereco("Rua X", "10", "", "Centro", "Cidade Z", "RS", "1234567")

    params = EnderecoParametros(formato_cep="99999999")
    e = gerador.gerar_endereco(endereco, params)
    assert e.cep == "01234567"

    params = EnderecoParametros(formato_cep="99.999-999")
    e = gerador.gerar_endereco(endereco, params)
    assert e.cep == "01.234-567"

    params = EnderecoParametros(formato_cep="99999-999")
    e = gerador.gerar_endereco(endereco, params)
    assert e.cep == "01234-567"
