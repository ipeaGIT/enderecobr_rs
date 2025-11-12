use pyo3::prelude::*;

#[pymodule]
pub mod enderecobr {

    use pyo3::prelude::*;

    /// Padroniza uma string representando logradouros de municípios brasileiros.
    ///
    /// # Exemplo
    /// ```python
    /// import enderecobr
    /// assert enderecobr.padronizar_logradouros("r. gen.. glicério") == "RUA GENERAL GLICERIO"
    /// ```
    ///
    /// # Detalhes
    /// Operações realizadas durante a padronização:
    /// - remoção de espaços em branco antes e depois das strings e remoção de espaços em excesso entre palavras;
    /// - conversão de caracteres para caixa alta;
    /// - remoção de acentos e caracteres não ASCII;
    /// - adição de espaços após abreviações sinalizadas por pontos;
    /// - expansão de abreviações frequentemente utilizadas através de diversas expressões regulares (regexes);
    /// - correção de alguns pequenos erros ortográficos.
    ///
    /// Note que existe uma etapa de compilação das expressões regulares utilizadas,
    /// logo a primeira execução desta função pode demorar um pouco a mais.
    ///
    #[pyfunction]
    fn padronizar_logradouros(valor: &str) -> String {
        enderecobr_rs::padronizar_logradouros(valor)
    }

    /// Padroniza uma string representando números de logradouros.
    ///
    /// # Exemplo
    /// ```python
    /// import enderecobr
    /// assert enderecobr.padronizar_numeros("0210") == "210"
    /// ```
    ///
    /// # Detalhes
    /// Operações realizadas durante a padronização:
    /// - remoção de espaços em branco antes e depois dos números e de espaços em branco em excesso entre números;
    /// - remoção de zeros à esquerda;
    /// - substituição de números vazios e de variações de SN (SN, S N, S.N., S./N., etc) por S/N.
    ///
    /// Note que existe uma etapa de compilação das expressões regulares utilizadas,
    /// logo a primeira execução desta função pode demorar um pouco a mais.
    ///
    #[pyfunction]
    fn padronizar_numeros(valor: &str) -> String {
        enderecobr_rs::padronizar_numeros(valor)
    }

    /// Padroniza uma string representando complementos de logradouros.
    ///
    /// # Exemplo
    /// ```python
    /// import enderecobr
    /// assert enderecobr.padronizar_complementos("QD1 LT2 CS3") == "QUADRA 1 LOTE 2 CASA 3")
    /// assert enderecobr.padronizar_complementos("APTO. 405") == "APARTAMENTO 405")
    /// ```
    ///
    /// # Detalhes
    /// Operações realizadas durante a padronização:
    /// - remoção de espaços em branco antes e depois das strings e remoção de espaços em excesso entre palavras;
    /// - conversão de caracteres para caixa alta;
    /// - remoção de acentos e caracteres não ASCII;
    /// - adição de espaços após abreviações sinalizadas por pontos;
    /// - expansão de abreviações frequentemente utilizadas através de diversas expressões regulares (regexes);
    /// - correção de alguns pequenos erros ortográficos.
    ///
    /// Note que existe uma etapa de compilação das expressões regulares utilizadas,
    /// logo a primeira execução desta função pode demorar um pouco a mais.
    ///
    #[pyfunction]
    fn padronizar_complementos(valor: &str) -> String {
        enderecobr_rs::padronizar_complementos(valor)
    }

    /// Padroniza uma string representando bairros de municípios brasileiros.
    ///
    /// # Exemplo
    /// ```python
    /// import enderecobr
    /// assert enderecobr.padronizar_bairros("PRQ IND") == "PARQUE INDUSTRIAL")
    /// assert enderecobr.padronizar_bairros("NSA SEN DE FATIMA") == "NOSSA SENHORA DE FATIMA")
    /// assert enderecobr.padronizar_bairros("ILHA DO GOV") == "ILHA DO GOVERNADOR")
    /// ```
    ///
    /// # Detalhes
    /// Operações realizadas durante a padronização:
    /// - remoção de espaços em branco antes e depois das strings e remoção de espaços em excesso entre palavras;
    /// - conversão de caracteres para caixa alta;
    /// - remoção de acentos e caracteres não ASCII;
    /// - adição de espaços após abreviações sinalizadas por pontos;
    /// - expansão de abreviações frequentemente utilizadas através de diversas expressões regulares (regexes);
    /// - correção de alguns pequenos erros ortográficos.
    ///
    /// Note que existe uma etapa de compilação das expressões regulares utilizadas,
    /// logo a primeira execução desta função pode demorar um pouco a mais.
    ///
    #[pyfunction]
    fn padronizar_bairros(valor: &str) -> String {
        enderecobr_rs::padronizar_bairros(valor)
    }

    /// Padroniza uma string representando município brasileiros.
    ///
    /// ```
    /// import enderecobr_rs
    /// assert enderecobr.padronizar_municipios("3304557") == "RIO DE JANEIRO"
    /// assert enderecobr.padronizar_municipios("003304557") == "RIO DE JANEIRO"
    /// assert enderecobr.padronizar_municipios("  3304557  ") == "RIO DE JANEIRO"
    /// assert enderecobr.padronizar_municipios("RIO DE JANEIRO") == "RIO DE JANEIRO"
    /// assert enderecobr.padronizar_municipios("rio de janeiro") == "RIO DE JANEIRO"
    /// assert enderecobr.padronizar_municipios("SÃO PAULO") == "SAO PAULO"
    /// assert enderecobr.padronizar_municipios("PARATI") == "PARATY"
    /// assert enderecobr.padronizar_municipios("AUGUSTO SEVERO") == "CAMPO GRANDE"
    /// assert enderecobr.padronizar_municipios("SAO VALERIO DA NATIVIDADE") == "SAO VALERIO"
    /// assert enderecobr.padronizar_municipios("") == ""
    /// ```
    ///
    /// # Detalhes
    /// Operações realizadas durante a padronização:
    /// - remoção de espaços em branco antes e depois das strings e remoção de espaços em excesso entre palavras;
    /// - conversão de caracteres para caixa alta;
    /// - remoção de zeros à esquerda;
    /// - busca, a partir do código numérico, do nome completo de cada município;
    /// - remoção de acentos e caracteres não ASCII, correção de erros ortográficos frequentes e atualização
    ///   de nomes conforme listagem de municípios do IBGE de 2022.
    ///
    /// Note que existe uma etapa de compilação das expressões regulares utilizadas,
    /// logo a primeira execução desta função pode demorar um pouco a mais.
    ///
    #[pyfunction]
    fn padronizar_municipios(valor: &str) -> String {
        enderecobr_rs::padronizar_municipios(valor)
    }

    /// Padroniza uma string representando estados brasileiros para seu nome por extenso,
    /// porém sem diacríticos.
    ///
    /// # Exemplo
    /// ```python
    /// use enderecobr_rs::padronizar_estados_para_nome;
    /// assert enderecobr.padronizar_estados_para_nome("21") == "MARANHAO"
    /// assert enderecobr.padronizar_estados_para_nome("021") == "MARANHAO"
    /// assert enderecobr.padronizar_estados_para_nome("MA") == "MARANHAO"
    /// assert enderecobr.padronizar_estados_para_nome(" 21") == "MARANHAO"
    /// assert enderecobr.padronizar_estados_para_nome(" MA ") == "MARANHAO"
    /// assert enderecobr.padronizar_estados_para_nome("ma") == "MARANHAO"
    /// assert enderecobr.padronizar_estados_para_nome("") == ""
    /// assert enderecobr.padronizar_estados_para_nome("me") == ""
    /// assert enderecobr.padronizar_estados_para_nome("maranhao") == "MARANHAO"
    /// ```
    ///
    /// # Detalhes
    /// Operações realizadas durante a padronização:
    /// - remoção de espaços em branco antes e depois dos valores e remoção de espaços em excesso entre palavras;
    /// - conversão de caracteres para caixa alta;
    /// - remoção de zeros à esquerda;
    /// - busca, a partir do código numérico ou da abreviação da UF, do nome completo de cada estado;
    ///
    #[pyfunction]
    fn padronizar_estados_para_nome(valor: &str) -> String {
        enderecobr_rs::padronizar_estados_para_nome(valor)
    }

    /// Padroniza uma string representando complementos de logradouros.
    ///
    /// # Exemplo
    /// ```
    /// import enderecobr
    /// assert enderecobr.padronizar_tipo_logradouro("R") == "RUA"
    /// assert enderecobr.padronizar_tipo_logradouro("AVE") == "AVENIDA"
    /// assert enderecobr.padronizar_tipo_logradouro("QDRA") == "QUADRA"
    /// ```
    ///
    /// # Detalhes
    /// Operações realizadas durante a padronização:
    /// - remoção de espaços em branco antes e depois das strings e remoção de espaços em excesso entre palavras;
    /// - conversão de caracteres para caixa alta;
    /// - remoção de acentos e caracteres não ASCII;
    /// - adição de espaços após abreviações sinalizadas por pontos;
    /// - expansão de abreviações frequentemente utilizadas através de diversas expressões regulares (regexes);
    /// - correção de alguns pequenos erros ortográficos.
    ///
    /// Note que existe uma etapa de compilação das expressões regulares utilizadas,
    /// logo a primeira execução desta função pode demorar um pouco a mais.
    ///
    #[pyfunction]
    fn padronizar_tipo_logradouro(valor: &str) -> String {
        enderecobr_rs::padronizar_tipo_logradouro(valor)
    }

    /// Padroniza CEPs em formato textual para uma string formatada, tentando corrigir possíveis erros.
    ///
    /// Esta função ignora quaisquer caracteres não numéricos, além de remover números extras e completar com zeros à
    /// esquerda quando necessário.
    ///
    /// # Exemplo
    /// ```
    /// import enderecobr
    /// assert enderecobr.padronizar_cep_leniente("a123b45  6") == "00123-456"
    /// ```
    ///
    #[pyfunction]
    fn padronizar_cep_leniente(valor: &str) -> String {
        enderecobr_rs::padronizar_cep_leniente(valor)
    }

    // TODO: terminar casos de tipos diferenciados
    //
    // pub use cep::padronizar_cep;
    // pub use cep::padronizar_cep_numerico;
    // pub use estado::padronizar_estados_para_codigo;
    // pub use estado::padronizar_estados_para_sigla;
    // pub use numero::padronizar_numeros_para_int;
    // pub use numero::padronizar_numeros_para_string;
}
