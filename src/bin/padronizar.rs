use clap::Parser;
use enderecobr_rs::obter_padronizador_por_tipo;
use std::io::{self, BufRead};

#[derive(Parser)]
#[clap(author, version, about("Padronizador de endereços simplificado"), long_about = None)]
struct Args {
    /// Tipo de padronizador. Aceita: logradouro, logr, numero, num, bairro, complemento, comp, estado, estado_nome, estado_codigo, municipio, mun, cep, cep_leniente, completo, separar
    #[arg(value_enum)]
    tipo: String,

    /// Valor a ser padronizado (opcional, lê de stdin se ausente)
    valor: Option<String>,
}

fn main() {
    let args = Args::parse();
    let padronizador =
        obter_padronizador_por_tipo(&args.tipo).expect("Tipo de padronizador não localizado.");

    if let Some(v) = args.valor {
        println!("{}", padronizador(&v));
    } else {
        let stdin = io::stdin();
        for line in stdin.lock().lines().map_while(Result::ok) {
            println!("{}", padronizador(&line));
        }
    }
}
