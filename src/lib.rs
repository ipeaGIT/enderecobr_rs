use std::sync::LazyLock;

use diacritics::remove_diacritics;
use regex::{Regex, RegexSet};

mod bairro;
mod logradouro;
mod numero;

#[derive(Debug)]
struct ParSubstituicao {
    regexp: Regex,
    substituicao: String,
}

impl ParSubstituicao {
    fn new(regex: &str, substituicao: &str) -> Self {
        ParSubstituicao {
            regexp: Regex::new(regex).unwrap(),
            substituicao: substituicao.to_uppercase().to_string(),
        }
    }
}

#[derive(Default)]
struct Padronizador {
    substituicoes: Vec<ParSubstituicao>,
    grupo_regex: RegexSet,
}

impl Padronizador {
    fn adicionar(&mut self, regex: &str, substituicao: &str) -> &mut Self {
        self.substituicoes
            .push(ParSubstituicao::new(regex, substituicao));
        self
    }
    fn preparar(&mut self) {
        let regexes: Vec<&str> = self
            .substituicoes
            .iter()
            .map(|par| par.regexp.as_str())
            .collect();

        self.grupo_regex = RegexSet::new(regexes).unwrap();
    }
    fn padronizar(&self, valor: &str) -> String {
        let mut preproc = remove_diacritics(valor.to_uppercase().trim());
        let mut ultimo_idx: Option<usize> = None;

        while self.grupo_regex.is_match(preproc.as_str()) {
            let idx_substituicao = self
                .grupo_regex
                .matches(preproc.as_str())
                .iter()
                .find(|idx| ultimo_idx.is_none_or(|ultimo| *idx > ultimo));

            if idx_substituicao.is_none() {
                break;
            }

            ultimo_idx = Some(idx_substituicao.unwrap());
            let par = self.substituicoes.get(idx_substituicao.unwrap()).unwrap();

            preproc = par
                .regexp
                .replace_all(preproc.as_str(), par.substituicao.as_str())
                .to_string();
        }

        preproc.to_string()
    }
}

// Em Rust, a constant é criada durante a compilação, então só posso chamar funções muito restritas
// quando uso `const`. Nesse caso,  como tenho uma construção complexa da struct `Padronizador`,
// tenho que usar static com inicialização Lazy (o LazyLock aqui previne condições de corrida).
//
// TODO: acho que dá pra virar macro.

static PADRONIZADOR_LOGRADOUROS: LazyLock<Padronizador> =
    LazyLock::new(logradouro::criar_padronizador_logradouros);

static PADRONIZADOR_NUMEROS: LazyLock<Padronizador> =
    LazyLock::new(numero::criar_padronizador_numeros);

static PADRONIZADOR_BAIRROS: LazyLock<Padronizador> =
    LazyLock::new(bairro::criar_padronizador_bairros);

pub fn padronizar_logradouros(valor: &str) -> String {
    // Forma de obter a variável lazy
    let padronizador = &*PADRONIZADOR_LOGRADOUROS;
    padronizador.padronizar(valor)
}

pub fn padronizar_numeros(valor: &str) -> String {
    // Forma de obter a variável lazy
    let padronizador = &*PADRONIZADOR_NUMEROS;
    padronizador.padronizar(valor)
}

pub fn padronizar_bairros(valor: &str) -> String {
    // Forma de obter a variável lazy
    let padronizador = &*PADRONIZADOR_BAIRROS;
    padronizador.padronizar(valor)
}

pub fn obter_padronizador_por_tipo(tipo: &str) -> Result<fn(&str) -> String, &str> {
    match tipo {
        "logradouro" | "logr" => Ok(padronizar_logradouros),
        "numero" | "num" => Ok(padronizar_numeros),
        "bairro" => Ok(padronizar_bairros),
        _ => Err("Nenhum padronizador encontrado"),
    }
}
