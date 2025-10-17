use std::{collections::HashMap, iter::zip, sync::LazyLock};

use crfsuite::{Attribute, Model};

use itertools::Itertools;
use regex::Regex;

use crate::{
    padronizar_bairros, padronizar_complemento, padronizar_logradouros, padronizar_numeros,
};

// TODO: Resolver warnings deste módulo!

pub struct SeparadorEndereco {
    regex_tokenizer: Regex,
    model: Model,
}

#[derive(Debug)]
pub struct Endereco {
    logradouro: Vec<String>,
    numero: Vec<String>,
    complemento: Vec<String>,
    localidade: Vec<String>,
}

impl SeparadorEndereco {
    fn new() -> Self {
        let modelo_bin = include_bytes!("./data/tagger.crf");
        let model = Model::from_memory(modelo_bin).unwrap();

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
        if i < sent.len() - 2 {
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

    fn tokens2attributes(&self, tokens: &Vec<String>) -> Vec<Vec<Attribute>> {
        tokens
            .iter()
            .enumerate()
            .map(|(i, _)| self.token2features(tokens, i))
            .map(|feats| feats.iter().map(|feat| Attribute::new(feat, 1.0)).collect())
            .collect()
    }

    pub fn extrair_campos(&self, tokens: Vec<String>, tags: Vec<String>) -> Endereco {
        let mut logradouro = Vec::new();
        let mut numero = Vec::new();
        let mut complemento = Vec::new();
        let mut localidade = Vec::new();
        let mut current: Option<String> = None;

        for (tok, tag) in tokens.into_iter().zip(tags.into_iter()) {
            if let Some(sufixo) = tag.strip_prefix("B-") {
                current = Some(sufixo.to_string());
                match current.as_deref() {
                    Some("LOG") => logradouro.push(tok),
                    Some("NUM") => numero.push(tok),
                    Some("COM") => complemento.push(tok),
                    Some("LOC") => localidade.push(tok),
                    _ => {}
                }
            } else if let Some(sufixo) = tag.strip_prefix("I-") {
                if let Some(curr) = &current {
                    let destino = match curr.as_str() {
                        "LOG" => &mut logradouro,
                        "NUM" => &mut numero,
                        "COM" => &mut complemento,
                        "LOC" => &mut localidade,
                        _ => continue,
                    };
                    if let Some(last) = destino.last_mut() {
                        last.push(' ');
                        last.push_str(&tok);
                    }
                }
            } else {
                current = None;
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

impl Endereco {
    pub fn padronizar(self) -> String {
        let log = self
            .logradouro
            .first()
            .map(|x| padronizar_logradouros(x))
            .unwrap_or("".to_string());

        let num = self
            .numero
            .first()
            .map(|x| padronizar_numeros(x))
            .unwrap_or("".to_string());

        let com = self
            .complemento
            .iter()
            .map(|x| padronizar_complemento(x))
            .join(" ");

        let loc = self
            .localidade
            .first()
            .map(|x| padronizar_bairros(x))
            .unwrap_or("".to_string());

        [log, num, com, loc]
            .iter()
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .join(", ")
    }
}
// Em Rust, a constant é criada durante a compilação, então só posso chamar funções muito restritas
// quando uso `const`. Nesse caso,  como tenho uma construção complexa da struct `Padronizador`,
// tenho que usar static com inicialização Lazy (o LazyLock aqui previne condições de corrida).
static SEPARADOR: LazyLock<SeparadorEndereco> = LazyLock::new(criar_separador);

pub fn criar_separador() -> SeparadorEndereco {
    SeparadorEndereco::new()
}

pub fn criar_features(texto: &str) -> Vec<Vec<String>> {
    let separador = &*SEPARADOR;
    separador.criar_features(texto)
}

pub fn separar_endereco(texto: &str) -> Endereco {
    let separador = &*SEPARADOR;
    separador.separar_endereco(texto)
}

pub fn padronizar_endereco_bruto(texto: &str) -> String {
    separar_endereco(texto).padronizar()
}
