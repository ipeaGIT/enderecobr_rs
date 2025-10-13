use std::sync::LazyLock;

use crate::Padronizador;

// TODO: ver se essa é a melhor forma de definir essa struct

#[derive(Debug, Clone, Copy)]
struct Municipio {
    pub codigo: &'static str,
    pub nome: &'static str,
}

// Em Rust, a constant é criada durante a compilação, então só posso chamar funções muito restritas
// quando uso `const`. Nesse caso, como tenho uma construção complexa da struct `Padronizador`,
// tenho que usar static com inicialização Lazy (o LazyLock aqui previne condições de corrida).

// fn criar_estado_map() -> HashMap<String, &'static Estado> {
//     let mut estados = HashMap::<String, &'static Estado>::with_capacity(ESTADOS.len());
//     ESTADOS.iter().for_each(|e| {
//         estados.insert(e.sigla.to_string(), e);
//         estados.insert(e.codigo.to_string(), e);
//         estados.insert(remove_diacritics(e.nome).to_string(), e);
//     });
//     estados
// }

static PADRONIZADOR: LazyLock<Padronizador> = LazyLock::new(criar_padronizador);

fn criar_padronizador() -> Padronizador {
    let mut padronizador = Padronizador::default();

    padronizador
        .adicionar(r"\b0+(\d+)\b", "$1")
        .adicionar(r"\s{2,}", " ")
        .adicionar("^MOJI MIRIM$", "MOGI MIRIM")
        .adicionar("^GRAO PARA$", "GRAO-PARA")
        .adicionar("^BIRITIBA-MIRIM$", "BIRITIBA MIRIM")
        .adicionar("^SAO LUIS DO PARAITINGA$", "SAO LUIZ DO PARAITINGA")
        .adicionar("^TRAJANO DE MORAIS$", "TRAJANO DE MORAES")
        .adicionar("^PARATI$", "PARATY")
        .adicionar("^LAGOA DO ITAENGA$", "LAGOA DE ITAENGA")
        .adicionar("^ELDORADO DOS CARAJAS$", "ELDORADO DO CARAJAS")
        .adicionar("^SANTANA DO LIVRAMENTO$", "SANT'ANA DO LIVRAMENTO")
        .adicionar("^BELEM DE SAO FRANCISCO$", "BELEM DO SAO FRANCISCO")
        .adicionar("^SANTO ANTONIO DO LEVERGER$", "SANTO ANTONIO DE LEVERGER")
        .adicionar("^POXOREO$", "POXOREU")
        .adicionar("^SAO THOME DAS LETRAS$", "SAO TOME DAS LETRAS")
        .adicionar("^OLHO-D'AGUA DO BORGES$", "OLHO D'AGUA DO BORGES")
        .adicionar("^ITAPAGE$", "ITAPAJE")
        .adicionar("^MUQUEM DE SAO FRANCISCO$", "MUQUEM DO SAO FRANCISCO")
        .adicionar("^DONA EUSEBIA$", "DONA EUZEBIA")
        .adicionar("^PASSA-VINTE$", "PASSA VINTE")
        .adicionar("^AMPARO DE SAO FRANCISCO$", "AMPARO DO SAO FRANCISCO")
        .adicionar("^BRASOPOLIS$", "BRAZOPOLIS")
        .adicionar("^SERIDO$", "SAO VICENTE DO SERIDO")
        .adicionar("^IGUARACI$", "IGUARACY")
        .adicionar("^AUGUSTO SEVERO$", "CAMPO GRANDE")
        .adicionar("^FLORINIA$", "FLORINEA")
        .adicionar("^FORTALEZA DO TABOCAO$", "TABOCAO")
        .adicionar("^SAO VALERIO DA NATIVIDADE$", "SAO VALERIO");

    let CSV_DATA: &str = include_str!("data/municipios.csv");

    for linha in CSV_DATA.lines().skip(1) {
        let cols: Vec<&str> = linha.split(",").collect();
        padronizador.adicionar(
            format!("^{}$", cols.get(0).unwrap()).as_str(),
            cols.get(1).unwrap(),
        );
    }

    padronizador.preparar();
    padronizador
}

// ====== Funções Públicas =======

pub fn padronizar_municipios(valor: &str) -> String {
    let padronizador = &*PADRONIZADOR;
    padronizador.padronizar(valor)
}

// ============ Dados Brutos ============
const MUNICIPIOS: [Municipio; 1] = [Municipio {
    codigo: "1100015",
    nome: "ALTA FLORESTA D'OESTE",
}];
