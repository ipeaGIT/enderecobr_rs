// ====== Funções Públicas =======

/// Padroniza CEPs em formato numérico para uma string formatada.
///
/// Completa com zeros à esquerda, caso necessário, e retorna erro se o valor numérico
/// for maior do que o tamanho permitido para CEPs.
///
/// # Exemplo
/// ```
/// use enderecobr_rs::padronizar_cep_numerico;
/// let cep = padronizar_cep_numerico(123456);
/// assert_eq!(cep, Ok("00123-456".to_string()));
/// ```
///
pub fn padronizar_cep_numerico(valor: i32) -> Result<String, String> {
    if valor >= 99999999 {
        return Err("CEP com muitos dígitos".to_string());
    }
    let cep = format!("{:08}", valor);
    Ok(format!("{}-{}", &cep[0..5], &cep[5..8]))
}

/// Padroniza CEPs em formato textual para uma string formatada, retornando possíveis erros.
///
/// Esta função ignora quaisquer caracteres de pontuação, além de completar com zeros à esquerda quando necessário.
///
/// # Exemplo
/// ```
/// use enderecobr_rs::padronizar_cep;
/// let cep = padronizar_cep("12345-6");
/// assert_eq!(cep, Ok("00123-456".to_string()));
///
/// let cep_grande = padronizar_cep("123456789");
/// assert_eq!(cep_grande, Err("CEP com muitos dígitos".to_string()));
///
/// let cep_invalido = padronizar_cep("123456e");
/// assert_eq!(cep_invalido, Err("CEP com caracteres inválidos".to_string()));
/// ```
///
pub fn padronizar_cep(valor: &str) -> Result<String, String> {
    if valor
        .chars()
        .any(|c| !c.is_ascii_punctuation() && !c.is_numeric())
    {
        return Err("CEP com caracteres inválidos".to_string());
    }

    if valor.is_empty() {
        return Ok("".to_string());
    }

    let valor_numerico: String = valor.chars().filter(|c| c.is_numeric()).collect();

    if valor_numerico.len() > 8 {
        return Err("CEP com muitos dígitos".to_string());
    }

    // Padding na esquerda
    let cep = format!("{:0>8}", valor_numerico);
    Ok(format!("{}-{}", &cep[0..5], &cep[5..8]))
}

/// Padroniza CEPs em formato textual para uma string formatada, tentando corrigir possíveis erros.
///
/// Esta função ignora quaisquer caracteres não numéricos, além de remover números extras e completar com zeros à
/// esquerda quando necessário.
///
/// # Exemplo
/// ```
/// use enderecobr_rs::padronizar_cep_leniente;
/// let cep = padronizar_cep_leniente("a123b45  6");
/// assert_eq!(cep, "00123-456".to_string());
/// ```
///
pub fn padronizar_cep_leniente(valor: &str) -> String {
    if valor.is_empty() {
        return "".to_string();
    }

    let valor_numerico: String = valor.chars().filter(|c| c.is_numeric()).take(8).collect();

    // Padding na esquerda
    let cep = format!("{:0>8}", valor_numerico);
    format!("{}-{}", &cep[0..5], &cep[5..8])
}
