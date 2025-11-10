//! Esta biblioteca tem por objetivo prover de funções utilizadas para padronizar endereços brasileiros,
//! corrigindo erros comuns, expandindo abreviações etc, afim de facilitar processamentos posteriores.
//!
//! Esta biblioteca é uma adaptação para Rust do [enderecobr](https://github.com/ipeaGIT/enderecobr)
//! visando ganho de eficiência e expandir seu uso para demais linguagens de programação,
//! utilizando esta implementação como base das demais.
//!
//! Ela usa majoritariamente expressões regulares nas padronizações, com exceção do módulo
//! experimental de separação de endereços, que utiliza um modelo probabilístico de
//! [Conditional Random Field](https://en.wikipedia.org/wiki/Conditional_random_field) já embutido na bilioteca.
//!
//! # Exemplo de uso
//!
//! ```
//! use enderecobr_rs::{padronizar_complementos, padronizar_logradouros};
//! assert_eq!(padronizar_logradouros("r. gen.. glicério"), "RUA GENERAL GLICERIO");
//! assert_eq!(padronizar_complementos("QD1 LT2 CS3"), "QUADRA 1 LOTE 2 CASA 3");
//! ```
//!
use diacritics::remove_diacritics;
use itertools::Itertools;
use regex::{Regex, RegexSet};

pub mod bairro;
pub mod cep;
pub mod complemento;
pub mod estado;
pub mod logradouro;
pub mod municipio;
pub mod numero;
pub mod separador_endereco;
pub mod tipo_logradouro;

/// Representa um endereço separado em seus atributos constituintes.
#[derive(Debug, PartialEq, Default)]
pub struct Endereco {
    pub logradouro: Option<String>,
    pub numero: Option<String>,
    pub complemento: Option<String>,
    pub localidade: Option<String>,
}

impl Endereco {
    /// Obtém o logradouro padronizado, utilizando a função [padronizar_logradouros].
    pub fn logradouro_padronizado(&self) -> Option<String> {
        self.logradouro
            .as_ref()
            .map(|x| padronizar_logradouros(x.as_str()))
    }

    /// Obtém o número padronizado, utilizando a função [padronizar_numeros].
    pub fn numero_padronizado(&self) -> Option<String> {
        self.numero.as_ref().map(|x| padronizar_numeros(x.as_str()))
    }

    /// Obtém o complemento padronizado, utilizando a função [padronizar_complementos].
    pub fn complemento_padronizado(&self) -> Option<String> {
        self.complemento
            .as_ref()
            .map(|x| padronizar_complementos(x.as_str()))
    }

    /// Obtém a localidade padronizada, utilizando a função [padronizar_bairros].
    pub fn localidade_padronizada(&self) -> Option<String> {
        self.localidade
            .as_ref()
            .map(|x| padronizar_bairros(x.as_str()))
    }

    /// Obtém uma nova struct [Endereco] com todos os campos padronizados,
    /// utilizando os métodos anteriores.
    pub fn endereco_padronizado(&self) -> Endereco {
        Endereco {
            logradouro: self.logradouro_padronizado(),
            numero: self.numero_padronizado(),
            complemento: self.complemento_padronizado(),
            localidade: self.localidade_padronizada(),
        }
    }

    /// Obtém uma representação textual dos atributos desta struct,
    /// separados por vírgula, caso existam.
    pub fn formatar(&self) -> String {
        [
            &self.logradouro,
            &self.numero,
            &self.complemento,
            &self.localidade,
        ]
        .iter()
        .filter_map(|opt| opt.as_deref())
        .map(|x| x.trim())
        .join(", ")
    }
}

/// Representa um par de "regexp replace". Usado internamente no [Padronizador].
#[derive(Debug)]
pub struct ParSubstituicao {
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

/// Struct utilitária utilizada internamente para realizar padronizações dos diversos tipos.
///
/// Guarda os pares de regexps e suas substituições, além do RegexSet, responsável por otimizar a
/// localização das regexp relevantes.
#[derive(Default)]
pub struct Padronizador {
    substituicoes: Vec<ParSubstituicao>,
    grupo_regex: RegexSet,
}

impl Padronizador {
    /// Adiciona uma regexp e sua substituição no padronizador. Compila a regexp imediatamente.
    pub fn adicionar(&mut self, regex: &str, substituicao: &str) -> &mut Self {
        self.substituicoes
            .push(ParSubstituicao::new(regex, substituicao, None));
        self
    }

    /// Adiciona no padronizador uma regexp, sua substituição e uma regexp adicional
    /// utilizada para condicionar a substituição. Compila ambas regexps imediatamente.
    pub fn adicionar_com_ignorar(
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
    /// Compila o Regexp Set, utilizado para agilizar a localização das regexp relevantes.
    pub fn preparar(&mut self) {
        let regexes: Vec<&str> = self
            .substituicoes
            .iter()
            .map(|par| par.regexp.as_str())
            .collect();

        self.grupo_regex = RegexSet::new(regexes).unwrap();
    }
    /// Realiza a padronização de fato.
    pub fn padronizar(&self, valor: &str) -> String {
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

/// Função utilitária usada internamente para normalizar uma string para processamento posterior,
/// removendo seus diacríticos e caracteres especiais.
///
/// # Exemplo
/// ```
/// use enderecobr_rs::normalizar;
/// assert_eq!(normalizar("Olá, mundo"), "Ola, mundo");
/// ```
///
pub fn normalizar(valor: &str) -> String {
    // Remove mais casos problemático, mas dificulta a comparação com a implementação em R.
    // use unicode_normalization::UnicodeNormalization;
    // valor.nfkd().filter(|c| c.is_ascii()).collect::<String>()
    remove_diacritics(valor)
}

pub use bairro::padronizar_bairros;
pub use cep::padronizar_cep;
pub use cep::padronizar_cep_leniente;
pub use cep::padronizar_cep_numerico;
pub use complemento::padronizar_complementos;
pub use estado::padronizar_estados_para_codigo;
pub use estado::padronizar_estados_para_nome;
pub use estado::padronizar_estados_para_sigla;
pub use logradouro::padronizar_logradouros;
pub use municipio::padronizar_municipios;
pub use numero::padronizar_numeros;
pub use numero::padronizar_numeros_para_int;
pub use numero::padronizar_numeros_para_string;
pub use tipo_logradouro::padronizar_tipo_logradouro;

#[cfg(feature = "experimental")]
pub use separador_endereco::padronizar_endereco_bruto;

#[cfg(feature = "experimental")]
pub use separador_endereco::separar_endereco;

/// Função utilitária utilizada nas ferramentas de CLI para selecionar um padronizador facilmente
/// via uma string descritiva.
pub fn obter_padronizador_por_tipo(tipo: &str) -> Result<fn(&str) -> String, &str> {
    match tipo {
        "logradouro" | "logr" => Ok(padronizar_logradouros),
        "tipo_logradouro" | "tipo_logr" => Ok(padronizar_tipo_logradouro),
        "numero" | "num" => Ok(padronizar_numeros),
        "bairro" => Ok(padronizar_bairros),
        "complemento" | "comp" => Ok(padronizar_complementos),
        "estado" => Ok(padronizar_estados_para_sigla),
        "estado_nome" => Ok(padronizar_estados_para_nome),
        "estado_codigo" => Ok(padronizar_estados_para_codigo),
        "municipio" | "mun" => Ok(padronizar_municipios),
        "cep" => Ok(|cep| padronizar_cep(cep).unwrap_or("".to_string())),
        "cep_leniente" => Ok(padronizar_cep_leniente),

        #[cfg(feature = "experimental")]
        "completo" => Ok(padronizar_endereco_bruto),

        #[cfg(feature = "experimental")]
        "separar" => Ok(|val| format!("{:?}", separar_endereco(val))),

        #[cfg(feature = "experimental")]
        "separar_padronizar" => {
            Ok(|val| format!("{:?}", separar_endereco(val).endereco_padronizado()))
        }

        _ => Err("Nenhum padronizador encontrado"),
    }
}
