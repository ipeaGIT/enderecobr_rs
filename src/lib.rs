use diacritics::remove_diacritics;
use regex::{Regex, RegexSet};

mod bairro;
mod complemento;
mod estado;
mod logradouro;
mod municipio;
mod numero;

#[derive(Debug)]
struct ParSubstituicao {
    regexp: Regex,
    substituicao: String,
    regexp_ignorar: Option<Regex>,
}

impl ParSubstituicao {
    fn new(regex: &str, substituicao: &str, regex_ignorar: Option<&str>) -> Self {
        ParSubstituicao {
            regexp: Regex::new(regex).unwrap(),
            substituicao: substituicao.to_uppercase().to_string(),
            regexp_ignorar: regex_ignorar.map(|r| Regex::new(r).unwrap()),
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
            .push(ParSubstituicao::new(regex, substituicao, None));
        self
    }
    fn adicionar_com_ignorar(
        &mut self,
        regex: &str,
        substituicao: &str,
        regexp_ignorar: &str,
    ) -> &mut Self {
        self.substituicoes.push(ParSubstituicao::new(
            regex,
            substituicao,
            Some(regexp_ignorar),
        ));
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
        let mut preproc = normalizar(valor.to_uppercase().trim());
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

            // FIXME: essa solução dá problema quando eu tenho mais de um match da regexp
            // original. Precisaria de uma heurística melhor.
            if par
                .regexp_ignorar
                .as_ref()
                .map(|r| r.is_match(preproc.as_str()))
                .unwrap_or(false)
            {
                continue;
            }

            preproc = par
                .regexp
                .replace_all(preproc.as_str(), par.substituicao.as_str())
                .to_string();
        }

        preproc.to_string()
    }
}

fn normalizar(valor: &str) -> String {
    remove_diacritics(valor)
}

pub use bairro::padronizar_bairros;
pub use complemento::padronizar_complemento;
pub use estado::padronizar_estados_para_codigo;
pub use estado::padronizar_estados_para_nome;
pub use estado::padronizar_estados_para_sigla;
pub use logradouro::padronizar_logradouros;
pub use municipio::padronizar_municipios;
pub use numero::padronizar_numeros;

pub fn obter_padronizador_por_tipo(tipo: &str) -> Result<fn(&str) -> String, &str> {
    match tipo {
        "logradouro" | "logr" => Ok(padronizar_logradouros),
        "numero" | "num" => Ok(padronizar_numeros),
        "bairro" => Ok(padronizar_bairros),
        "complemento" | "comp" => Ok(padronizar_complemento),
        "estado" => Ok(padronizar_estados_para_sigla),
        "estado_nome" => Ok(padronizar_estados_para_nome),
        "estado_codigo" => Ok(padronizar_estados_para_codigo),
        "municipio" | "mun" => Ok(padronizar_municipios),
        _ => Err("Nenhum padronizador encontrado"),
    }
}
