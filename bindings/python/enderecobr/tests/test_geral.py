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


def testa_padronizar_cep():
    # Testes básicos de funcionamento
    assert enderecobr.padronizar_cep("22290-140") == "22290-140"
    assert enderecobr.padronizar_cep("22290 140") == "22290-140"
    assert enderecobr.padronizar_cep("1000000") == "01000-000"
    
    # Testes de erro
    try:
        enderecobr.padronizar_cep("botafogo")
        assert False, "Deveria lançar ValueError para CEP com letras"
    except ValueError:
        pass
    
    try:
        enderecobr.padronizar_cep("1234567890")
        assert False, "Deveria lançar ValueError para CEP com muitos dígitos"
    except ValueError:
        pass


def testa_padronizar_cep_numerico():
    # Testes básicos de funcionamento
    assert enderecobr.padronizar_cep_numerico(22290140) == "22290-140"
    assert enderecobr.padronizar_cep_numerico(1000000) == "01000-000"
    
    # Testes de erro
    try:
        enderecobr.padronizar_cep_numerico(100000000)
        assert False, "Deveria lançar ValueError para CEP com muitos dígitos"
    except ValueError:
        pass


def testa_padronizar_estados_para_codigo():
    # Testes básicos de funcionamento
    assert enderecobr.padronizar_estados_para_codigo("MA") == "21"
    assert enderecobr.padronizar_estados_para_codigo("maranhao") == "21"
    assert enderecobr.padronizar_estados_para_codigo("21") == "21"
    assert enderecobr.padronizar_estados_para_codigo("") == ""


def testa_padronizar_estados_para_sigla():
    # Testes básicos de funcionamento
    assert enderecobr.padronizar_estados_para_sigla("21") == "MA"
    assert enderecobr.padronizar_estados_para_sigla("maranhao") == "MA"
    assert enderecobr.padronizar_estados_para_sigla("MA") == "MA"
    assert enderecobr.padronizar_estados_para_sigla("") == ""


def testa_padronizar_numeros_para_int():
    # Testes básicos de funcionamento
    assert enderecobr.padronizar_numeros_para_int("210") == 210
    assert enderecobr.padronizar_numeros_para_int("0210") == 210
    assert enderecobr.padronizar_numeros_para_int("S/N") == 0
    assert enderecobr.padronizar_numeros_para_int("SN") == 0
    
    # Testes de erro
    try:
        enderecobr.padronizar_numeros_para_int("abc")
        assert False, "Deveria lançar ValueError para entrada inválida"
    except ValueError:
        pass


def testa_padronizar_numeros_para_string():
    # Testes básicos de funcionamento
    assert enderecobr.padronizar_numeros_para_string("210") == "210"
    assert enderecobr.padronizar_numeros_para_string("0210") == "210"
    assert enderecobr.padronizar_numeros_para_string("S/N") == "S/N"
    assert enderecobr.padronizar_numeros_para_string("sn") == "S/N"
    
    # Testes de erro
    try:
        enderecobr.padronizar_numeros_para_string("abc")
        assert False, "Deveria lançar ValueError para entrada inválida"
    except ValueError:
        pass


def testa_normalizar():
    # Testes básicos de funcionamento
    assert enderecobr.normalizar("Olá, mundo") == "OLA, MUNDO"
    assert enderecobr.normalizar("R. DO AÇAÍ 15º") == "R. DO ACAI 15O"
    assert enderecobr.normalizar("  espaços  ") == "ESPAÇOS"
    assert enderecobr.normalizar("") == ""
