use pyo3::prelude::*;

#[pyclass]
struct Padronizador {
    interno: enderecobr_rs::Padronizador,
}

#[pymethods]
impl Padronizador {
    #[new]
    fn novo() -> Padronizador {
        Padronizador {
            interno: enderecobr_rs::Padronizador::default(),
        }
    }
    fn adicionar_substituicoes(&mut self, pares: Vec<Vec<Option<String>>>) {
        // PS: Aparentemente preciso que seja um Vec de Vec quando não uso
        // os struct específicos do PyO3.

        // Converte Option<String> em Option<&str>
        let pares_str: Vec<Vec<Option<&str>>> = pares
            .iter()
            .map(|inner| inner.iter().map(|opt| opt.as_deref()).collect())
            .collect();

        // Converte para um vetor de slices
        let slices: Vec<&[Option<&str>]> = pares_str.iter().map(Vec::as_slice).collect();

        self.interno.adicionar_pares(&slices);
    }

    fn padronizar(&self, valor: &str) -> String {
        self.interno.padronizar(valor)
    }

    fn obter_substituicoes(&self) -> Vec<(&str, &str, Option<&str>)> {
        self.interno.obter_pares()
    }
}

#[pymodule]
pub mod enderecobr {

    use pyo3::prelude::*;

    #[pymodule_export]
    use super::Padronizador;

    #[pyfunction]
    fn padronizar_logradouros(valor: &str) -> String {
        enderecobr_rs::padronizar_logradouros(valor)
    }

    #[pyfunction]
    fn padronizar_numeros(valor: &str) -> String {
        enderecobr_rs::padronizar_numeros(valor)
    }

    #[pyfunction]
    fn padronizar_complementos(valor: &str) -> String {
        enderecobr_rs::padronizar_complementos(valor)
    }
    #[pyfunction]
    fn padronizar_bairros(valor: &str) -> String {
        enderecobr_rs::padronizar_bairros(valor)
    }

    #[pyfunction]
    fn padronizar_municipios(valor: &str) -> String {
        enderecobr_rs::padronizar_municipios(valor)
    }

    #[pyfunction]
    fn padronizar_estados_para_nome(valor: &str) -> &'static str {
        enderecobr_rs::padronizar_estados_para_nome(valor)
    }

    #[pyfunction]
    fn padronizar_tipo_logradouro(valor: &str) -> String {
        enderecobr_rs::padronizar_tipo_logradouro(valor)
    }

    #[pyfunction]
    fn padronizar_cep_leniente(valor: &str) -> String {
        enderecobr_rs::padronizar_cep_leniente(valor)
    }

    #[pyfunction]
    fn metaphone(valor: &str) -> String {
        enderecobr_rs::metaphone::metaphone(valor)
    }

    #[pyfunction]
    fn padronizar_numeros_por_extenso(valor: &str) -> String {
        enderecobr_rs::numero_extenso::padronizar_numeros_por_extenso(valor).to_string()
    }

    #[pyfunction]
    fn padronizar_numero_romano_por_extenso(valor: &str) -> String {
        enderecobr_rs::numero_extenso::padronizar_numero_romano_por_extenso(valor).to_string()
    }

    #[pyfunction]
    fn numero_por_extenso(valor: i32) -> String {
        enderecobr_rs::numero_extenso::numero_por_extenso(valor).to_string()
    }

    #[pyfunction]
    fn romano_para_inteiro(valor: &str) -> i32 {
        enderecobr_rs::numero_extenso::romano_para_inteiro(valor)
    }

    // TODO: terminar casos de tipos diferenciados
    //
    // pub use cep::padronizar_cep;
    // pub use cep::padronizar_cep_numerico;
    // pub use estado::padronizar_estados_para_codigo;
    // pub use estado::padronizar_estados_para_sigla;
    // pub use numero::padronizar_numeros_para_int;
    // pub use numero::padronizar_numeros_para_string;

    // ========= Padronizadores pré prontos ==========

    #[pyfunction]
    fn obter_padronizador_logradouros() -> Padronizador {
        Padronizador {
            interno: enderecobr_rs::logradouro::criar_padronizador_logradouros(),
        }
    }

    #[pyfunction]
    fn obter_padronizador_numeros() -> Padronizador {
        Padronizador {
            interno: enderecobr_rs::numero::criar_padronizador_numeros(),
        }
    }

    #[pyfunction]
    fn obter_padronizador_bairros() -> Padronizador {
        Padronizador {
            interno: enderecobr_rs::bairro::criar_padronizador_bairros(),
        }
    }

    #[pyfunction]
    fn obter_padronizador_complementos() -> Padronizador {
        Padronizador {
            interno: enderecobr_rs::complemento::criar_padronizador_complemento(),
        }
    }

    #[pyfunction]
    fn obter_padronizador_tipos_logradouros() -> Padronizador {
        Padronizador {
            interno: enderecobr_rs::tipo_logradouro::criar_padronizador_tipo_logradouro(),
        }
    }
}
