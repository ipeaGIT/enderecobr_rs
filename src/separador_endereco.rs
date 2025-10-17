use std::{collections::HashMap, iter::zip, sync::LazyLock};

use crfs::{Attribute, Model};

use itertools::Itertools;
use regex::Regex;

use crate::{
    padronizar_bairros, padronizar_complemento, padronizar_logradouros, padronizar_numeros,
};

// TODO: Resolver warnings deste módulo!

pub struct SeparadorEndereco<'a> {
    regex_tokenizer: Regex,
    model: Model<'a>,
}

pub struct Endereco {
    logradouro: Vec<String>,
    // numero: String,
    // complemento: String,
    // localidade: String,
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
        } else if i >= 1 {
            features.push(format!("-1:{}", sent[i - 1].to_uppercase()));
        } else if i >= 2 {
            features.push(format!("-2:{}", sent[i - 2].to_uppercase()));
        }

        if i == sent.len() {
            features.push("EOS".to_string());
        } else if i < sent.len() - 1 {
            features.push(format!("+1:{}", sent[i + 1].to_uppercase()));
        } else if i < sent.len() - 2 {
            features.push(format!("+2:{}", sent[i + 2].to_uppercase()));
        }

        features
    }

    fn tokens2attributes(&self, tokens: &Vec<String>) -> Vec<Vec<Attribute>> {
        tokens
            .iter()
            .enumerate()
            .map(|(i, _)| self.token2features(tokens, i))
            .map(|feats| feats.iter().map(|feat| Attribute::new(feat, 1.0)).collect())
            .collect()
    }

    fn extrair_campos(&self, tokens: Vec<String>, tags: Vec<&str>) -> HashMap<String, Vec<String>> {
        let mut grupos = HashMap::new();
        let mut current: Option<String> = None;

        for t in &tags {
            if t == &"O" {
                continue;
            }
            let (_, tipo) = t.split_at(2);
            grupos.insert(tipo.to_string(), Vec::new());
        }

        for (tok, tag) in zip(&tokens, &tags) {
            if tag.starts_with("B-") {
                let tipo = &tag[2..].to_string();
                grupos.entry(tipo.clone()).or_default().push(tok.clone());
                current = Some(tipo.clone());
            } else if tag.starts_with("I-") && current.is_some() {
                let lista = grupos.get_mut(current.as_ref().unwrap());
                let ultima = lista.unwrap().last_mut().unwrap();
                ultima.push(' ');
                ultima.push_str(tok);
            } else {
                current = None;
            }
        }

        grupos
    }
    fn separar_endereco(&self, texto: &str) -> Endereco {
        let mut tagger = self.model.tagger().unwrap();
        let tokens = self.tokenize(texto);
        let atributos = self.tokens2attributes(&tokens);

        let tags = tagger.tag(&atributos).unwrap();
        Endereco {
            logradouro: tags.iter().map(|x| x.to_string()).collect(),
        }
    }
}

// Em Rust, a constant é criada durante a compilação, então só posso chamar funções muito restritas
// quando uso `const`. Nesse caso,  como tenho uma construção complexa da struct `Padronizador`,
// tenho que usar static com inicialização Lazy (o LazyLock aqui previne condições de corrida).
static SEPARADOR: LazyLock<SeparadorEndereco<'static>> = LazyLock::new(criar_separador);

pub fn criar_separador() -> SeparadorEndereco<'static> {
    SeparadorEndereco::new()
}

pub fn separar_endereco(texto: &str) -> Endereco {
    let separador = &*SEPARADOR;
    separador.separar_endereco(texto)
}

pub fn padronizar_endereco_bruto(texto: &str) -> String {
    return "".to_string();
    // let partes = separar_endereco(texto);
    //
    // let log = partes
    //     .get("LOG")
    //     .map(|l| l.first().map(|x| padronizar_logradouros(x)))
    //     .flatten();
    //
    // let num = partes
    //     .get("NUM")
    //     .map(|l| l.first().map(|x| padronizar_numeros(x)))
    //     .flatten();
    //
    // let com = partes
    //     .get("COM")
    //     .map(|l| l.iter().map(|x| padronizar_complemento(x)).join(" "));
    //
    // let loc = partes
    //     .get("LOC")
    //     .map(|l| l.first().map(|x| padronizar_bairros(x)))
    //     .flatten();
    //
    // [log, num, com, loc]
    //     .into_iter()
    //     .flatten()
    //     .map(|x| x.trim().to_string())
    // .join(", ")
}
