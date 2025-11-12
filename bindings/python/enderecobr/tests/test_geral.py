import enderecobr


# Testes bem simples só para garantir que as funções estão sendo executadas.


def testa_logradouro():
    assert enderecobr.padronizar_logradouros("R") == "RUA"


def testa_numero():
    assert enderecobr.padronizar_numeros("0001") == "1"


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
