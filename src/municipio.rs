use std::sync::LazyLock;

use crate::Padronizador;

static PADRONIZADOR: LazyLock<Padronizador> = LazyLock::new(criar_padronizador);

fn criar_padronizador() -> Padronizador {
    let mut padronizador = Padronizador::default();

    padronizador
        .adicionar(r"\b0+(\d+)\b", "$1") // Remove zeros na frente
        .adicionar(r"\s{2,}", " ") // Remove espaços extra
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

    // a include_str! embute a string no código em tempo de compilação.
    let municipios_csv: &str = include_str!("data/municipios.csv");

    for linha in municipios_csv.lines().skip(1) {
        let cols: Vec<&str> = linha.split(",").collect();
        let codigo = cols.first().unwrap();
        let nome = cols.get(1).unwrap();
        // Adiciona código do ibge no padronizador
        padronizador.adicionar(format!("^{}$", codigo).as_str(), nome);
    }

    padronizador.preparar();
    padronizador
}

// ====== Funções Públicas =======

pub fn padronizar_municipios(valor: &str) -> String {
    let padronizador = &*PADRONIZADOR;
    padronizador.padronizar(valor)
}
