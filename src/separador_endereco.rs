#![cfg(feature = "experimental")]
//! # Exemplo de uso
//! ```
//! use enderecobr_rs::{padronizar_endereco_bruto, separar_endereco};
//! let endereco_separado = Endereco { logradouro: Some("av n sra copacabana".to_string()), numero: Some("123".to_string()), complemento: Some("apt 301".to_string()), ..Default::default() };
//! assert_eq!(separar_endereco("av n sra copacabana 123 apt 301"), endereco_separado);
//!
//! let endereco_padronizado_esperado = Endereco { logradouro: Some("AVENIDA NOSSA SENHORA COPACABANA".to_string()), numero: Some("123".to_string()), complemento: Some("APARTAMENTO 301".to_string()), ..Default::default() };
//! assert_eq!(endereco_separado.endereco_padronizado(), endereco_padronizado_esperado);
//! ```
use std::sync::LazyLock;

use crfs::{Attribute, Model};

use regex::Regex;

use crate::Endereco;

struct SeparadorEndereco<'a> {
    regex_tokenizer: Regex,
    model: Model<'a>,
}

impl SeparadorEndereco<'_> {
    fn new() -> Self {
        let modelo_bin = include_bytes!("./data/tagger.crf");
        let model = Model::new(modelo_bin).unwrap();

        SeparadorEndereco {
            regex_tokenizer: Regex::new(r"\w+|\S").unwrap(),
            model,
        }
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        self.regex_tokenizer
            .find_iter(text)
            .map(|mat| mat.as_str().to_string())
            .collect()
    }

    fn token2features(&self, sent: &[String], i: usize) -> Vec<String> {
        let mut features = Vec::new();
        let word = &sent[i];

        features.push("bias".to_string());
        features.push(format!("0:{}", word.to_uppercase()));

        if word.chars().all(|c| c.is_ascii_digit()) {
            features.push("is_digit".to_string());
        }

        if word.chars().all(|c| c.is_alphabetic()) {
            features.push("is_alpha".to_string());
        }

        if i == 0 {
            features.push("BOS".to_string());
        }
        if i >= 1 {
            features.push(format!("-1:{}", sent[i - 1].to_uppercase()));
        }
        if i >= 2 {
            features.push(format!("-2:{}", sent[i - 2].to_uppercase()));
        }

        if i == sent.len() - 1 {
            features.push("EOS".to_string());
        }
        if i < sent.len() - 1 {
            features.push(format!("+1:{}", sent[i + 1].to_uppercase()));
        }
        if i < sent.len() - 2 && sent.len() >= 2 {
            features.push(format!("+2:{}", sent[i + 2].to_uppercase()));
        }

        features
    }

    fn criar_features(&self, texto: &str) -> Vec<Vec<String>> {
        let tokens = self.tokenize(texto);

        tokens
            .iter()
            .enumerate()
            .map(|(i, _)| self.token2features(&tokens, i))
            .collect()
    }

    fn tokens2attributes(&self, tokens: &[String]) -> Vec<Vec<Attribute>> {
        tokens
            .iter()
            .enumerate()
            .map(|(i, _)| self.token2features(tokens, i))
            .map(|feats| feats.iter().map(|feat| Attribute::new(feat, 1.0)).collect())
            .collect()
    }

    // TODO: tornar lógica mais legível: muitos níveis de indentação.
    fn extrair_campos(&self, tokens: Vec<String>, tags: Vec<&str>) -> Endereco {
        let mut logradouro = None;
        let mut numero = None;
        let mut complemento = None;
        let mut localidade = None;

        let mut tipo_tag_atual: Option<String> = None;

        for (tok, tag) in tokens.into_iter().zip(tags.into_iter()) {
            if let Some(sufixo) = tag.strip_prefix("B-") {
                tipo_tag_atual = Some(sufixo.to_string());
                match tipo_tag_atual.as_deref() {
                    Some("LOG") if logradouro.is_none() => logradouro = Some(tok),
                    Some("NUM") if numero.is_none() => numero = Some(tok),
                    Some("COM") if complemento.is_none() => complemento = Some(tok),
                    Some("LOC") if localidade.is_none() => localidade = Some(tok),
                    _ => {}
                }
            } else if tag.strip_prefix("I-").is_some() {
                if let Some(tipo_atual) = &tipo_tag_atual {
                    let destino = match tipo_atual.as_str() {
                        "LOG" => &mut logradouro,
                        "NUM" => &mut numero,
                        "COM" => &mut complemento,
                        "LOC" => &mut localidade,
                        _ => continue,
                    };
                    if let Some(last) = destino {
                        last.push(' ');
                        last.push_str(&tok);
                    }
                }
            } else {
                tipo_tag_atual = None;
            }
        }

        Endereco {
            logradouro,
            numero,
            complemento,
            localidade,
        }
    }

    fn separar_endereco(&self, texto: &str) -> Endereco {
        let mut tagger = self.model.tagger().unwrap();
        let tokens = self.tokenize(texto);
        let atributos = self.tokens2attributes(&tokens);

        let tags = tagger.tag(&atributos).unwrap();
        self.extrair_campos(tokens, tags)
    }
}

// Em Rust, a constant é criada durante a compilação, então só posso chamar funções muito restritas
// quando uso `const`. Nesse caso,  como tenho uma construção complexa da struct `Padronizador`,
// tenho que usar static com inicialização Lazy (o LazyLock aqui previne condições de corrida).
static SEPARADOR: LazyLock<SeparadorEndereco<'static>> = LazyLock::new(criar_separador);

fn criar_separador() -> SeparadorEndereco<'static> {
    SeparadorEndereco::new()
}

/// Tenta separa um endereço bruto utilizando um pequeno modelo probabilístico embutido nesta biblioteca.
///
/// # Exemplo:
/// ```
/// use enderecobr_rs::{separar_endereco, Endereco};
/// let endereco = separar_endereco("av n sra copacabana, 123, apt 302");
/// assert_eq!(Endereco {
///     logradouro: Some("av n sra copacabana".to_string()),
///     numero: Some("123".to_string()),
///     complemento: Some("apt 302".to_string()),
///     localidade: None}, endereco);
/// ```
///
pub fn separar_endereco(texto: &str) -> Endereco {
    let separador = &*SEPARADOR;
    separador.separar_endereco(texto)
}

/// Função utilitária que separa o endereço recebido, padroniza seus campos,
/// e formata eles numa nova string, separando-os por vírgula.
///
/// # Exemplo:
/// ```
/// use enderecobr_rs::padronizar_endereco_bruto;
/// let endereco = padronizar_endereco_bruto("av n sra copacabana, 123, apt 302");
/// assert_eq!(endereco, "AVENIDA NOSSA SENHORA COPACABANA, 123, APARTAMENTO 302");
/// ```
///
pub fn padronizar_endereco_bruto(texto: &str) -> String {
    separar_endereco(texto).endereco_padronizado().formatar()
}
