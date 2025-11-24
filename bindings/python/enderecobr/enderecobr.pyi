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
