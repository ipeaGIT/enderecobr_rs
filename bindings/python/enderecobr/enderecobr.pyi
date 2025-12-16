from collections.abc import Iterable

def padronizar_logradouros(valor: str) -> str:
    """
    Padroniza uma string representando logradouros de municípios brasileiros.

    Realiza uma série de transformações para normalizar nomes de ruas, avenidas e
    outros tipos de logradouros segundo convenções comuns em bases de endereços no Brasil.

    Parameters
    ----------
    valor : str
        Texto bruto representando um logradouro (ex: 'r. gen.. glicério').

    Returns
    -------
    str
        Texto padronizado em caixa alta, sem acentos, com abreviações expandidas
        e formatação consistente.

    Examples
    --------
    >>> import enderecobr
    >>> enderecobr.padronizar_logradouros("r. gen.. glicério")
    'RUA GENERAL GLICERIO'

    Notes
    -----
    As seguintes operações são aplicadas sequencialmente:

    - Remoção de espaços extras no início, fim e entre palavras;
    - Conversão para maiúsculas;
    - Remoção de acentos e conversão de caracteres não-ASCII;
    - Normalização de pontos em abreviações (ex: 'gen..' → 'GEN');
    - Inserção de espaços após abreviações com pontos (ex: 'R.JOSE' → 'R. JOSE');
    - Expansão de abreviações comuns (ex: 'R.' → 'RUA', 'AV.' → 'AVENIDA');
    - Correção de erros ortográficos frequentes (ex: 'GLICERIO' em vez de 'GLICÉRIO').

    As expressões regulares são compiladas na primeira chamada, portanto a primeira
    execução pode ser mais lenta. Chamadas subsequentes reutilizam as regexes compiladas.
    """
    ...

def padronizar_numeros(valor: str) -> str:
    """
    Padroniza uma string representando números de logradouros.

    Normaliza números de endereços, removendo formatações inconsistentes e
    tratando casos especiais como ausência de número (ex: "SN", "S/N", etc).

    Parameters
    ----------
    valor : str
        Texto bruto representando o número de um logradouro (ex: '0210', 'S. N. ').

    Returns
    -------
    str
        Número padronizado. Números comuns têm zeros à esquerda removidos;
        valores nulos ou variações de "SN" são convertidos para "S/N".

    Examples
    --------
    >>> import enderecobr
    >>> enderecobr.padronizar_numeros("0210")
    '210'
    >>> enderecobr.padronizar_numeros("  S. N.  ")
    'S/N'

    Notes
    -----
    As seguintes operações são aplicadas:

    - Remoção de espaços extras no início, fim e entre caracteres;
    - Remoção de zeros à esquerda em números;
    - Detecção e substituição de variações de "sem número" (SN, S N, S./N., etc) por "S/N";
    - Retorno de string vazia se a entrada for completamente inválida ou nula.

    As expressões regulares são compiladas na primeira chamada, portanto a primeira
    execução pode ser mais lenta. Chamadas subsequentes reutilizam as regexes compiladas.
    """
    ...

def padronizar_complementos(valor: str) -> str:
    """
    Padroniza uma string representando complementos de logradouros.

    Parameters
    ----------
    valor : str
        Texto bruto representando um complemento (ex: 'APTO. 405', 'QD1 LT2 CS3').

    Returns
    -------
    str
        Texto padronizado em caixa alta, sem acentos, com abreviações expandidas
        e formatação consistente.

    Examples
    --------
    >>> import enderecobr
    >>> enderecobr.padronizar_complementos("QD1 LT2 CS3")
    'QUADRA 1 LOTE 2 CASA 3'
    >>> enderecobr.padronizar_complementos("APTO. 405")
    'APARTAMENTO 405'

    Notes
    -----
    As seguintes operações são aplicadas:

    - Remoção de espaços extras no início, fim e entre palavras;
    - Conversão para maiúsculas;
    - Remoção de acentos e caracteres não ASCII;
    - Normalização de pontos em abreviações (ex: 'APTO.' → 'APARTAMENTO');
    - Expansão de abreviações comuns (ex: 'QD' → 'QUADRA', 'LT' → 'LOTE', 'CS' → 'CASA');
    - Correção de erros ortográficos ou variações comuns.

    As expressões regulares são compiladas na primeira chamada, portanto a primeira
    execução pode ser mais lenta. Chamadas subsequentes reutilizam as regexes compiladas.
    """
    ...

def padronizar_bairros(valor: str) -> str:
    """
    Padroniza uma string representando bairros de municípios brasileiros.

    Examples
    --------
    >>> import enderecobr
    >>> enderecobr.padronizar_bairros("PRQ IND")
    'PARQUE INDUSTRIAL'
    >>> enderecobr.padronizar_bairros("NSA SEN DE FATIMA")
    'NOSSA SENHORA DE FATIMA'
    >>> enderecobr.padronizar_bairros("ILHA DO GOV")
    'ILHA DO GOVERNADOR'

    Notes
    -----
    Operações realizadas durante a padronização:

    - Remoção de espaços em branco antes, depois e excesso entre palavras;
    - Conversão para caixa alta;
    - Remoção de acentos e caracteres não ASCII;
    - Adição de espaços após abreviações com pontos;
    - Expansão de abreviações comuns usando expressões regulares;
    - Correção de pequenos erros ortográficos.

    As expressões regulares são compiladas na primeira chamada, portanto a primeira
    execução pode ser mais lenta. Chamadas subsequentes reutilizam as regexes compiladas.
    """
    ...

def padronizar_municipios(valor: str) -> str:
    """
    Padroniza uma string representando municípios brasileiros.

    Parameters
    ----------
    valor : str
        String contendo o nome ou código de um município brasileiro.
        Pode ser nome (com variações de caixa, acentos, espaçamento),
        código IBGE (7 ou 8 dígitos, com ou sem zeros à esquerda), ou string vazia.

    Returns
    -------
    str
        Nome padronizado do município em caixa alta, sem acentos,
        com correções ortográficas e atualizações conforme IBGE 2022.
        Retorna string vazia se entrada for vazia.

    Examples
    --------
    >>> import enderecobr
    >>> enderecobr.padronizar_municipios("3304557")
    'RIO DE JANEIRO'
    >>> enderecobr.padronizar_municipios("003304557")
    'RIO DE JANEIRO'
    >>> enderecobr.padronizar_municipios("  3304557  ")
    'RIO DE JANEIRO'
    >>> enderecobr.padronizar_municipios("RIO DE JANEIRO")
    'RIO DE JANEIRO'
    >>> enderecobr.padronizar_municipios("rio de janeiro")
    'RIO DE JANEIRO'
    >>> enderecobr.padronizar_municipios("SÃO PAULO")
    'SAO PAULO'
    >>> enderecobr.padronizar_municipios("PARATI")
    'PARATY'
    >>> enderecobr.padronizar_municipios("AUGUSTO SEVERO")
    'CAMPO GRANDE'
    >>> enderecobr.padronizar_municipios("SAO VALERIO DA NATIVIDADE")
    'SAO VALERIO'
    >>> enderecobr.padronizar_municipios("")
    ''

    Notes
    -----
    As seguintes operações são realizadas:

    - Remoção de espaços em branco no início/fim e excesso entre palavras.
    - Conversão para caixa alta.
    - Remoção de zeros à esquerda em códigos numéricos.
    - Busca do nome completo do município a partir do código IBGE.
    - Remoção de acentos e caracteres não ASCII.
    - Correção de erros ortográficos comuns e nomes desatualizados,
      com base na lista oficial de municípios do IBGE (2022).

    As expressões regulares são compiladas na primeira chamada, portanto a primeira
    execução pode ser mais lenta. Chamadas subsequentes reutilizam as regexes compiladas.
    """
    ...

def padronizar_estados_para_nome(valor: str) -> str:
    """
    Padroniza uma string representando estados brasileiros para seu nome por extenso,
    porém sem diacríticos.

    Parameters
    ----------
    valor : str
        String contendo o código numérico (ex: '21', '021'), sigla (ex: 'MA', 'ma') ou nome
        de um estado brasileiro. Pode conter espaços extras ou formatação irregular.

    Returns
    -------
    str
        Nome por extenso do estado em caixa alta e sem diacríticos (ex: 'MARANHAO').
        Retorna string vazia se o valor for inválido, vazio ou não corresponder a um estado.

    Notes
    -----
    Operações realizadas durante a padronização:

    - Remoção de espaços em branco no início e fim, e espaços extras internos;
    - Conversão para caixa alta;
    - Remoção de zeros à esquerda (em códigos numéricos);
    - Mapeamento partir do código numérico ou da abreviação da UF, do nome completo de cada estado.

    Examples
    --------
    >>> import enderecobr
    >>> enderecobr.padronizar_estados_para_nome("21")
    'MARANHAO'
    >>> enderecobr.padronizar_estados_para_nome("021")
    'MARANHAO'
    >>> enderecobr.padronizar_estados_para_nome("MA")
    'MARANHAO'
    >>> enderecobr.padronizar_estados_para_nome(" 21")
    'MARANHAO'
    >>> enderecobr.padronizar_estados_para_nome(" MA ")
    'MARANHAO'
    >>> enderecobr.padronizar_estados_para_nome("ma")
    'MARANHAO'
    >>> enderecobr.padronizar_estados_para_nome("")
    ''
    >>> enderecobr.padronizar_estados_para_nome("me")
    ''
    >>> enderecobr.padronizar_estados_para_nome("maranhao")
    'MARANHAO'

    """
    ...

def padronizar_tipo_logradouro(valor: str) -> str:
    """
    Padroniza uma string representando complementos de logradouros.

    Examples
    --------
    >>> import enderecobr
    >>> enderecobr.padronizar_tipo_logradouro("R")
    'RUA'
    >>> enderecobr.padronizar_tipo_logradouro("AVE")
    'AVENIDA'
    >>> enderecobr.padronizar_tipo_logradouro("QDRA")
    'QUADRA'

    Notes
    -----
    Operações realizadas durante a padronização:

    - Remoção de espaços em branco antes e depois das strings e remoção de espaços em excesso entre palavras.
    - Conversão de caracteres para caixa alta.
    - Remoção de acentos e caracteres não ASCII.
    - Adição de espaços após abreviações sinalizadas por pontos.
    - Expansão de abreviações frequentemente utilizadas através de expressões regulares.
    - Correção de pequenos erros ortográficos.

    A primeira chamada pode ser mais lenta devido à compilação inicial das expressões regulares.
    """

def padronizar_cep_leniente(valor: str) -> str:
    """
    Padroniza CEPs em formato textual para uma string formatada, tentando corrigir possíveis erros.

    Esta função ignora quaisquer caracteres não numéricos, além de remover números
    extras e completar com zeros à esquerda quando necessário.

    Parameters
    ----------
    valor : str
        CEP em formato textual, que pode conter caracteres não numéricos e formatação irregular.

    Returns
    -------
    str
        CEP padronizado no formato XXXXX-XXX (com 8 dígitos, separados por hífen, completado com zeros à esquerda se necessário).

    Examples
    --------
    >>> import enderecobr
    >>> enderecobr.padronizar_cep_leniente("a123b45  6")
    '00123-456'

    Notes
    -----
    - São extraídos apenas os dígitos numéricos da entrada.
    - Se mais de 8 dígitos forem fornecidos, apenas os 8 primeiros são considerados.
    - Se menos de 8 dígitos forem fornecidos, zeros são adicionados à esquerda.
    """
    ...

class Padronizador:
    """
    Estrutura para padronização condicional de textos de endereços usando expressões regulares.

    Permite definir regras de substituição com condições de exclusão (``regex_ignorar``).
    Usa um conjunto de regex compilado para acelerar a detecção de padrões.

    Use `$1`, `$2`... para referenciar um grupo de captura na string de substituição.

    Por usar o motor de expressões regulares do Rust, esta classe **NÃO** aceita o uso de:

    - look-arounds -> ex: `^RUA(?!\\.)` (começa com "RUA", mas não deve ter um ponto após);
    - backreferences -> ex: `(\\w+) \\1` (duas palavras iguais repetidas após um espaço);


    Examples
    --------
    >>> from enderecobr import Padronizador
    >>> pad = Padronizador()
    >>> pad.adicionar_substituicoes([
    ...     ["AV ", "AVENIDA "],
    ...     ["^R ", "RUA ", "R APT"],
    ... ])
    >>> pad.padronizar("AV AZUL")
    'AVENIDA AZUL'
    >>> pad.padronizar("R APT AMARELA")
    'R APT AMARELA'

    """

    def __init__(self) -> None:
        """
        Inicializa um novo padronizador com nenhuma regra de substituição.
        """

    def adicionar_substituicoes(self, pares: Iterable[Iterable[None | str]]) -> None:
        """
        Adiciona múltiplas regras de substituição a partir de uma lista de listas.

        Cada sublista representa uma regra no formato:
        [regex, substituição, regex_ignorar] (valores None são ignorados).

        Regras:

        - 1 item: equivalente a [regex, '']
        - 2 itens: equivalente a [regex, substituição]
        - 3 ou mais itens: equivalente a [regex, substituição, regex_ignorar]

        Parameters
        ----------
        pares : list of list of (str or None)
            Lista de regras de substituição. Cada regra é uma lista com até três elementos.

        Examples
        --------
        >>> pad = Padronizador()
        >>> pad.adicionar_substituicoes([
        ...     ["R ", "RUA ", "APT R "],
        ...     ["AV ", "AVENIDA ", None],
        ...     ["NO ", "Nº"]
        ... ])

        """
        ...

    def padronizar(self, valor: str) -> str:
        """
        Aplica todas as regras de substituição ao texto, com normalização prévia.

        O texto é convertido para maiúsculas, acentos são removidos, e espaços
        extras são reduzidos. As regras são aplicadas em ordem, com condição de
        exclusão verificada antes de cada substituição.

        Parameters
        ----------
        valor : str
            Texto de entrada a ser padronizado.

        Returns
        -------
        str
            Texto padronizado.

        Examples
        --------
        >>> pad = Padronizador()
        >>> pad.adicionar_substituicoes([[r"\bR\b", "RUA"]])
        >>> pad.padronizar("  r amarela ")
        'RUA AMARELA'

        """
        ...

    def obter_substituicoes(self) -> list[tuple[str, str, None | str]]:
        """
        Retorna as regras de substituição atuais.

        Returns
        -------
        list of tuple of (str, str, str or None)
            Lista de triplas: (regex, substituição, regex_ignorar).

        Examples
        --------
        >>> pad = Padronizador()
        >>> pad.adicionar_substituicoes([["R ", "RUA "]])
        >>> pad.obter_substituicoes()
        [('R ', 'RUA ', None)]

        """
        ...

def obter_padronizador_logradouros() -> Padronizador:
    """
    Obtém o padronizador utilizado internamente pela função `padronizar_logradouros`.
    Útil para adicionar padrões de substituição não incluídos originalmente.
    """
    ...

def obter_padronizador_numeros() -> Padronizador:
    """
    Obtém o padronizador utilizado internamente pela função `padronizar_numeros`.
    Útil para adicionar padrões de substituição não incluídos originalmente.
    """
    ...

def obter_padronizador_bairros() -> Padronizador:
    """
    Obtém o padronizador utilizado internamente pela função `padronizar_bairros`.
    Útil para adicionar padrões de substituição não incluídos originalmente.
    """
    ...

def obter_padronizador_complementos() -> Padronizador:
    """
    Obtém o padronizador utilizado internamente pela função `padronizar_complementos`.
    Útil para adicionar padrões de substituição não incluídos originalmente.
    """
    ...

def obter_padronizador_tipos_logradouros() -> Padronizador:
    """
    Obtém o padronizador utilizado internamente pela função `padronizar_tipo_logradouro`.
    Útil para adicionar padrões de substituição não incluídos originalmente.
    """
    ...

def metaphone(valor: str) -> str:
    """
    Gera um código fonético (Metaphone-BR adaptado) para nomes em português.

    Aplica transformações fonéticas a um nome visando representar sua pronúncia
    aproximada em português brasileiro. Útil para agrupar nomes com sonoridade
    similar, mesmo com grafias diferentes.

    Parameters
    ----------
    valor : str
        Nome ou texto a ser convertido em código fonético. Pode conter acentos,
        caracteres especiais, espaços e letras minúsculas.

    Returns
    -------
    str
        Código fonético em maiúsculas, com transformações aplicadas segundo regras
        adaptadas do Metaphone para o português brasileiro.

    Notes
    -----
    O processo inclui:

        - Remoção de acentos, números e conversão para maiúsculas;
        - Eliminação de letras silenciosas (ex: 'H' inicial);
        - Simplificação de dígrafos (ex: 'LH' → 'L', 'CH' → 'X');
        - Agrupamento de consoantes com sonoridade similar (ex: C/K/S, G/J);
        - Tratamento de sons nasais e vogais duplicadas;
        - Compactação de espaços e letras repetidas.

    Esta é uma adaptação que não segue rigorosamente nenhum algoritmo Metaphone
    publicado, mas foi inspirada neles, considerando o contexto do português brasileiro.

    Examples
    --------
    >>> enderecobr.metaphone("João Silva")
    'JOAO SILVA'
    >>> enderecobr.metaphone("Marya")
    'MARIA'
    >>> enderecobr.metaphone("Helena")
    'ELENA'
    >>> enderecobr.metaphone("Philippe")
    'FILIPE'
    >>> enderecobr.metaphone("Chavier")
    'XAVIER'
    >>> enderecobr.metaphone("Maçã")
    'MASA'

    """
    ...

def padronizar_numeros_por_extenso(valor: str) -> str:
    """
    Converte sequências de dígitos em uma string para seus equivalentes por extenso em português.

    A função percorre a string de entrada e, ao encontrar números inteiros (em formato ASCII),
    os substitui pelo nome completo do número (ex: "2" → "dois"), utilizando a função `numero_por_extenso`.

    Parameters
    ----------
    valor : str
        String de entrada que pode conter dígitos a serem convertidos.

    Returns
    -------
    str
        Nova string com dígitos convertidos por extenso. Retorna a string original se não houver dígitos.

    Notes
    -----
    - Números muito grandes ou inválidos (ex: overflow no parse para `i32`) são deixados inalterados.
    - Não trata números negativos ou decimais.
    - Se a string de entrada não contém nenhum dígito ASCII, retorna a string original.

    Examples
    --------
    >>> enderecobr.padronizar_numeros_por_extenso("RUA 2")
    'RUA DOIS'
    >>> enderecobr.padronizar_numeros_por_extenso("RUA -2")
    'RUA -DOIS'
    >>> enderecobr.padronizar_numeros_por_extenso("RUA -2.2")
    'RUA -DOIS.DOIS'
    >>> enderecobr.padronizar_numeros_por_extenso("Sem números")
    'Sem números'

    """
    ...

def padronizar_numero_romano_por_extenso(valor: str) -> str:
    """
    Substitui números romanos em um texto por suas representações por extenso (em palavras).
    Apenas sequências que formam números romanos válidos (1–3999) são convertidas.

    Parameters
    ----------
    valor : str
        Texto contendo números romanos a serem convertidos.

    Returns
    -------
    str
        Texto com números romanos substituídos por suas formas por extenso (maiúsculas).
        Retorna o texto original se nenhuma substituição for necessária.

    Examples
    --------
    >>> enderecobr.padronizar_numero_romano_por_extenso("Capítulo IX")
    'Capítulo NOVE'

    >>> enderecobr.padronizar_numero_romano_por_extenso("Séculos XV e XX")
    'Séculos QUINZE e VINTE'

    >>> enderecobr.padronizar_numero_romano_por_extenso("Rei João VI e Papa Bento XVI")
    'Rei João SEIS e Papa Bento DEZESSEIS'

    """
    ...

def numero_por_extenso(n: int) -> str:
    """
    Converte um número inteiro para sua representação por extenso em português.

    Retorna uma string com o número por extenso em letras maiúsculas.

    Parameters
    ----------
    n : int
        Número inteiro a ser convertido.

    Returns
    -------
    str
        Representação por extenso do número em português.

    Examples
    --------
    >>> enderecobr.numero_por_extenso(0)
    'ZERO'
    >>> enderecobr.numero_por_extenso(42)
    'QUARENTA E DOIS'
    >>> enderecobr.numero_por_extenso(-1500)
    'MENOS MIL E QUINHENTOS'
    >>> enderecobr.numero_por_extenso(2_001_000)
    'DOIS MILHOES E MIL'

    """
    ...

def romano_para_inteiro(valor: str) -> int:
    """
    Converte um número romano em sua representação por extenso (número inteiro).

    Aceita entradas em maiúsculas ou minúsculas. A conversão segue a regra padrão de números romanos,
    onde símbolos menores à esquerda de maiores são subtraídos. Suporta valores de 1 a 3999.

    Parameters
    ----------
    valor : str
        String contendo a representação de um número romano (ex: "IX", "MCMXC").

    Returns
    -------
    int
        Valor inteiro correspondente ao número romano. Retorna resultados inesperados se a string
        contiver caracteres inválidos (não tratados como erro).

    Examples
    --------
    >>> enderecobr.romano_para_inteiro("IX")
    9
    >>> enderecobr.romano_para_inteiro("xlII")
    42
    >>> enderecobr.romano_para_inteiro("MCMXC")
    1990
    >>> enderecobr.romano_para_inteiro("mmmcmxcix")
    3999

    Notes
    -----
    - Caracteres inválidos são tratados como 0 e podem gerar resultados inesperados.
    - A função não valida a correção gramatical da sequência romana (ex: "IIII" retorna 4).
    """
    ...

def padronizar_cep(valor: str) -> str:
    """
    Padroniza CEPs em formato textual para uma string formatada.

    Esta função valida e formata CEPs brasileiros (Código de Endereçamento Postal),
    garantindo o formato correto XXXXX-XXX. Realiza validação rigorosa dos dígitos.

    Parameters
    ----------
    valor : str
        CEP em formato textual, que deve conter apenas dígitos numéricos (0-9),
        podendo conter formatação como espaços, pontos ou hífens.

    Returns
    -------
    str
        CEP padronizado no formato XXXXX-XXX.

    Raises
    ------
    ValueError
        Se o CEP contiver caracteres não numéricos ou tiver mais de 8 dígitos.

    Examples
    --------
    >>> import enderecobr
    >>> enderecobr.padronizar_cep("22290-140")
    '22290-140'
    >>> enderecobr.padronizar_cep("22290 140")
    '22290-140'
    >>> enderecobr.padronizar_cep("1000000")
    '01000-000'

    Notes
    -----
    - Remove espaços, pontos e outros caracteres não numéricos
    - Completa com zeros à esquerda se necessário
    - Valida o número de dígitos (máximo 8)
    - Retorna erro para caracteres inválidos
    """
    ...

def padronizar_cep_numerico(valor: int) -> str:
    """
    Padroniza CEPs em formato numérico para uma string formatada.

    Converte um valor numérico inteiro para o formato padrão de CEP brasileiro XXXXX-XXX.

    Parameters
    ----------
    valor : int
        CEP em formato numérico inteiro (ex: 22290140).

    Returns
    -------
    str
        CEP padronizado no formato XXXXX-XXX.

    Raises
    ------
    ValueError
        Se o valor tiver mais de 8 dígitos.

    Examples
    --------
    >>> import enderecobr
    >>> enderecobr.padronizar_cep_numerico(22290140)
    '22290-140'
    >>> enderecobr.padronizar_cep_numerico(1000000)
    '01000-000'

    Notes
    -----
    - Completa com zeros à esquerda se necessário
    - Valida o número de dígitos (máximo 8)
    - Retorna erro para valores com mais de 8 dígitos
    """
    ...

def padronizar_estados_para_codigo(valor: str) -> str:
    """
    Padroniza uma string representando estados brasileiros para seu código numérico.

    Parameters
    ----------
    valor : str
        String contendo a sigla (ex: 'MA', 'ma') ou nome de um estado brasileiro.
        Pode conter espaços extras ou formatação irregular.

    Returns
    -------
    str
        Código numérico do estado (2 dígitos). Retorna string vazia se o valor
        for inválido, vazio ou não corresponder a um estado.

    Examples
    --------
    >>> import enderecobr
    >>> enderecobr.padronizar_estados_para_codigo("MA")
    '21'
    >>> enderecobr.padronizar_estados_para_codigo("maranhao")
    '21'
    >>> enderecobr.padronizar_estados_para_codigo("21")
    '21'

    Notes
    -----
    Operações realizadas durante a padronização:
    - Remoção de espaços em branco no início e fim
    - Conversão para caixa alta
    - Mapeamento para o código numérico de cada estado
    """
    ...

def padronizar_estados_para_sigla(valor: str) -> str:
    """
    Padroniza uma string representando estados brasileiros para sua sigla.

    Parameters
    ----------
    valor : str
        String contendo o código numérico (ex: '21') ou nome de um estado brasileiro.
        Pode conter espaços extras ou formatação irregular.

    Returns
    -------
    str
        Sigla do estado em caixa alta (ex: 'MA'). Retorna string vazia se o valor
        for inválido, vazio ou não corresponder a um estado.

    Examples
    --------
    >>> import enderecobr
    >>> enderecobr.padronizar_estados_para_sigla("21")
    'MA'
    >>> enderecobr.padronizar_estados_para_sigla("maranhao")
    'MA'
    >>> enderecobr.padronizar_estados_para_sigla("MA")
    'MA'

    Notes
    -----
    Operações realizadas durante a padronização:
    - Remoção de espaços em branco no início e fim
    - Conversão para caixa alta
    - Mapeamento para a sigla de cada estado
    """
    ...

def padronizar_numeros_para_int(valor: str) -> int:
    """
    Converte uma string representando um número de endereço para seu valor inteiro.

    Converte strings de números de endereços para valores inteiros, tratando
    casos especiais como "S/N" (sem número).

    Parameters
    ----------
    valor : str
        String contendo um número de endereço, que pode estar em formato
        textual com zeros à esquerda, espaços ou variações de "S/N".

    Returns
    -------
    int
        Valor inteiro do número. Retorna 0 para casos de "S/N".

    Raises
    ------
    ValueError
        Se a string não puder ser convertida para um número válido.

    Examples
    --------
    >>> import enderecobr
    >>> enderecobr.padronizar_numeros_para_int("210")
    210
    >>> enderecobr.padronizar_numeros_para_int("0210")
    210
    >>> enderecobr.padronizar_numeros_para_int("S/N")
    0
    >>> enderecobr.padronizar_numeros_para_int("SN")
    0

    Notes
    -----
    - Remove zeros à esquerda
    - Converte variações de "S/N" para 0
    - Valida o formato numérico
    """
    ...

def padronizar_numeros_para_string(valor: str) -> str:
    """
    Converte uma string representando um número de endereço para formato padronizado.

    Converte strings de números de endereços para um formato padronizado,
    removendo zeros à esquerda e tratando casos especiais.

    Parameters
    ----------
    valor : str
        String contendo um número de endereço, que pode estar em formato
        textual com zeros à esquerda, espaços ou variações de "S/N".

    Returns
    -------
    str
        String padronizada do número. Retorna "S/N" para casos de sem número.

    Raises
    ------
    ValueError
        Se a string não puder ser convertida para um número válido.

    Examples
    --------
    >>> import enderecobr
    >>> enderecobr.padronizar_numeros_para_string("210")
    '210'
    >>> enderecobr.padronizar_numeros_para_string("0210")
    '210'
    >>> enderecobr.padronizar_numeros_para_string("S/N")
    'S/N'
    >>> enderecobr.padronizar_numeros_para_string("sn")
    'S/N'

    Notes
    -----
    - Remove zeros à esquerda
    - Padroniza variações de "S/N" para "S/N"
    - Valida o formato numérico
    """
    ...

def normalizar(valor: str) -> str:
    """
    Normaliza uma string removendo acentos e convertendo para maiúsculas.

    Função utilitária que realiza a normalização básica de textos, removendo
    diacríticos e convertendo para caixa alta, preparando para processamento
    posterior de padronização.

    Parameters
    ----------
    valor : str
        String a ser normalizada.

    Returns
    -------
    str
        String normalizada sem acentos e em maiúsculas.

    Examples
    --------
    >>> import enderecobr
    >>> enderecobr.normalizar("Olá, mundo")
    'OLA, MUNDO'
    >>> enderecobr.normalizar("R. DO AÇAÍ 15º")
    'R. DO ACAI 15O'

    Notes
    -----
    Operações realizadas:
    - Remoção de espaços extras no início e fim
    - Conversão para maiúsculas
    - Remoção de acentos e caracteres não ASCII
    - Preserva apenas caracteres ASCII
    """
    ...
