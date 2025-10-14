use std::{collections::HashMap, sync::LazyLock};

use crate::{Padronizador, normalizar};

// TODO: ver se essa é a melhor forma de definir essa struct

#[derive(Debug, Clone, Copy)]
struct Estado {
    pub codigo: u8,
    pub nome: &'static str,
    pub sigla: &'static str,
}

// Em Rust, a constant é criada durante a compilação, então só posso chamar funções muito restritas
// quando uso `const`. Nesse caso, como tenho uma construção complexa da struct `Padronizador`,
// tenho que usar static com inicialização Lazy (o LazyLock aqui previne condições de corrida).

static ESTADOS_MAP: LazyLock<HashMap<String, &'static Estado>> = LazyLock::new(criar_estado_map);

// O trecho &'static indica que estou referenciando uma posição de memória estática,
// o que evita cópias desnecessárias.

fn criar_estado_map() -> HashMap<String, &'static Estado> {
    let mut estados = HashMap::<String, &'static Estado>::with_capacity(ESTADOS.len());
    ESTADOS.iter().for_each(|e| {
        estados.insert(e.sigla.to_string(), e);
        estados.insert(e.codigo.to_string(), e);
        estados.insert(normalizar(e.nome).to_string(), e);
    });
    estados
}

static PADRONIZADOR: LazyLock<Padronizador> = LazyLock::new(criar_padronizador);

fn criar_padronizador() -> Padronizador {
    let mut padronizador = Padronizador::default();

    padronizador.adicionar(r"0+(\d)", "$1");
    padronizador.adicionar(r"\s{2,}", " ");

    padronizador.preparar();
    padronizador
}

// ====== Funções Públicas =======

pub fn padronizar_estados_para_sigla(valor: &str) -> String {
    let padronizador = &*PADRONIZADOR;
    let valor_padr = padronizador.padronizar(valor);

    let mapa = &*ESTADOS_MAP;
    mapa.get(&valor_padr)
        .map(|e| e.sigla)
        .unwrap_or("")
        .to_string()
}
pub fn padronizar_estados_para_codigo(valor: &str) -> String {
    let padronizador = &*PADRONIZADOR;
    let valor_padr = padronizador.padronizar(valor);

    let mapa = &*ESTADOS_MAP;
    mapa.get(&valor_padr)
        .map(|e| e.codigo.to_string())
        .unwrap_or("".to_string())
}
pub fn padronizar_estados_para_nome(valor: &str) -> String {
    let padronizador = &*PADRONIZADOR;
    let valor_padr = padronizador.padronizar(valor);

    let mapa = &*ESTADOS_MAP;
    mapa.get(&valor_padr)
        .map(|e| e.nome)
        .unwrap_or("")
        .to_string()
}

// ============ Dados Brutos ============

const ESTADOS: [Estado; 27] = [
    Estado {
        codigo: 11,
        nome: "RONDÔNIA",
        sigla: "RO",
    },
    Estado {
        codigo: 12,
        nome: "ACRE",
        sigla: "AC",
    },
    Estado {
        codigo: 13,
        nome: "AMAZONAS",
        sigla: "AM",
    },
    Estado {
        codigo: 14,
        nome: "RORAIMA",
        sigla: "RR",
    },
    Estado {
        codigo: 15,
        nome: "PARÁ",
        sigla: "PA",
    },
    Estado {
        codigo: 16,
        nome: "AMAPÁ",
        sigla: "AP",
    },
    Estado {
        codigo: 17,
        nome: "TOCANTINS",
        sigla: "TO",
    },
    Estado {
        codigo: 21,
        nome: "MARANHÃO",
        sigla: "MA",
    },
    Estado {
        codigo: 22,
        nome: "PIAUÍ",
        sigla: "PI",
    },
    Estado {
        codigo: 23,
        nome: "CEARÁ",
        sigla: "CE",
    },
    Estado {
        codigo: 24,
        nome: "RIO GRANDE DO NORTE",
        sigla: "RN",
    },
    Estado {
        codigo: 25,
        nome: "PARAÍBA",
        sigla: "PB",
    },
    Estado {
        codigo: 26,
        nome: "PERNAMBUCO",
        sigla: "PE",
    },
    Estado {
        codigo: 27,
        nome: "ALAGOAS",
        sigla: "AL",
    },
    Estado {
        codigo: 28,
        nome: "SERGIPE",
        sigla: "SE",
    },
    Estado {
        codigo: 29,
        nome: "BAHIA",
        sigla: "BA",
    },
    Estado {
        codigo: 31,
        nome: "MINAS GERAIS",
        sigla: "MG",
    },
    Estado {
        codigo: 32,
        nome: "ESPÍRITO SANTO",
        sigla: "ES",
    },
    Estado {
        codigo: 33,
        nome: "RIO DE JANEIRO",
        sigla: "RJ",
    },
    Estado {
        codigo: 35,
        nome: "SÃO PAULO",
        sigla: "SP",
    },
    Estado {
        codigo: 41,
        nome: "PARANÁ",
        sigla: "PR",
    },
    Estado {
        codigo: 42,
        nome: "SANTA CATARINA",
        sigla: "SC",
    },
    Estado {
        codigo: 43,
        nome: "RIO GRANDE DO SUL",
        sigla: "RS",
    },
    Estado {
        codigo: 50,
        nome: "MATO GROSSO DO SUL",
        sigla: "MS",
    },
    Estado {
        codigo: 51,
        nome: "MATO GROSSO",
        sigla: "MT",
    },
    Estado {
        codigo: 52,
        nome: "GOIÁS",
        sigla: "GO",
    },
    Estado {
        codigo: 53,
        nome: "DISTRITO FEDERAL",
        sigla: "DF",
    },
];
