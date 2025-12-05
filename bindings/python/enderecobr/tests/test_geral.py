import enderecobr


# Testes bem simples só para garantir que as funções estão sendo executadas.


def testa_logradouro():
    assert enderecobr.padronizar_logradouros("R") == "RUA"


def testa_numero():
    assert enderecobr.padronizar_numeros("0001") == "1"


def testa_padronizar_complementos():
    assert enderecobr.padronizar_complementos("ap 101") == "APARTAMENTO 101"


def testa_bairro():
    assert enderecobr.padronizar_bairros("NS aparecida") == "NOSSA SENHORA APARECIDA"


def testa_municipio():
    assert enderecobr.padronizar_municipios("3304557") == "RIO DE JANEIRO"


def testa_estado_nome():
    assert enderecobr.padronizar_estados_para_nome("MA") == "MARANHAO"


def testa_padronizar_tipo_logradouro():
    assert enderecobr.padronizar_tipo_logradouro("R") == "RUA"


def testa_padronizar_cep_leniente():
    assert enderecobr.padronizar_cep_leniente("a123b45  6") == "00123-456"


def testa_padronizar_adhoc():
    pad = enderecobr.Padronizador()
    pad.adicionar_substituicoes([[r"R\.", "RUA"]])
    assert pad.padronizar("R. AZUL") == "RUA AZUL"
    assert pad.obter_substituicoes() == [(r"R\.", "RUA", None)]


def testa_metaphone():
    assert enderecobr.metaphone("casa") == "KASA"


def testa_padronizar_numeros_por_extenso():
    assert enderecobr.padronizar_numeros_por_extenso("CASA 1") == "CASA UM"


def testa_padronizar_numero_romano_por_extenso():
    assert (
        enderecobr.padronizar_numero_romano_por_extenso("PAPA PIO II")
        == "PAPA PIO DOIS"
    )


def testa_numero_por_extenso():
    assert enderecobr.numero_por_extenso(20) == "VINTE"


def testa_romano_para_inteiro():
    assert enderecobr.romano_para_inteiro("VI") == 6
