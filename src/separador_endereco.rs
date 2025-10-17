use std::{collections::HashMap, iter::zip, sync::LazyLock};

use crfsuite::{Attribute, Model};

use itertools::Itertools;
use regex::Regex;

use crate::{
    padronizar_bairros, padronizar_complemento, padronizar_logradouros, padronizar_numeros,
};

// TODO: Resolver warnings deste módulo!

fn tokenize(text: &str) -> Vec<String> {
    let re = Regex::new(r"\w+|\S").unwrap();
    re.find_iter(text)
        .map(|mat| mat.as_str().to_string())
        .collect()
}

fn token2features(sent: &[String], i: usize) -> Vec<String> {
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

fn tokens2attributes(tokens: &Vec<String>) -> Vec<Vec<Attribute>> {
    tokens
        .iter()
        .enumerate()
        .map(|(i, _)| token2features(tokens, i))
        .map(|feats| feats.iter().map(|feat| Attribute::new(feat, 1.0)).collect())
        .collect()
}

fn extrair_campos(tokens: Vec<String>, tags: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut grupos = HashMap::new();
    let mut current: Option<String> = None;

    for t in &tags {
        if t == "O" {
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

// Em Rust, a constant é criada durante a compilação, então só posso chamar funções muito restritas
// quando uso `const`. Nesse caso,  como tenho uma construção complexa da struct `Padronizador`,
// tenho que usar static com inicialização Lazy (o LazyLock aqui previne condições de corrida).
static MODEL: LazyLock<Model> = LazyLock::new(carregar_modelo);

fn carregar_modelo() -> Model {
    let modelo_bin = include_bytes!("./data/tagger.crf");
    Model::from_memory(modelo_bin).unwrap()
}

pub fn separar_endereco(texto: &str) -> HashMap<String, Vec<String>> {
    let mut tagger = MODEL.tagger().unwrap();
    let tokens = tokenize(texto);
    let atributos = tokens2attributes(&tokens);

    let tags = tagger.tag(&atributos).unwrap();
    extrair_campos(tokens, tags)
}

pub fn padronizar_endereco_bruto(texto: &str) -> String {
    let partes = separar_endereco(texto);

    let log = partes
        .get("LOG")
        .map(|l| l.first().map(|x| padronizar_logradouros(x)))
        .flatten();

    let num = partes
        .get("NUM")
        .map(|l| l.first().map(|x| padronizar_numeros(x)))
        .flatten();

    let com = partes
        .get("COM")
        .map(|l| l.iter().map(|x| padronizar_complemento(x)).join(" "));

    let loc = partes
        .get("LOC")
        .map(|l| l.first().map(|x| padronizar_bairros(x)))
        .flatten();

    [log, num, com, loc]
        .into_iter()
        .flatten()
        .map(|x| x.trim().to_string())
        .join(", ")
}
