use std::{fs::read_to_string, time::SystemTime};

use duckdb::Connection;
use enderecobr_rs::obter_padronizador_por_tipo;

#[derive(Debug)]
struct Endereco {
    pos: i32,
    original: Option<String>,
    padronizado_r: Option<String>,
}

fn main() {
    let mut args = std::env::args();
    let arq_consulta = args.next_back().unwrap();
    let tipo = args.next_back().unwrap();

    let padronizador = obter_padronizador_por_tipo(tipo.as_str()).unwrap();

    let query = read_to_string(arq_consulta).unwrap();
    let conn = Connection::open_in_memory().unwrap();
    let mut stmt = conn.prepare(query.as_str()).unwrap();
    let mut i: i32 = 0;

    println!("Realizando Consulta");
    let inicio_consulta = SystemTime::now();

    let end_iter = stmt
        .query_map([], |row| {
            i += 1;
            Ok(Endereco {
                pos: i,
                original: row.get(0).unwrap(),
                padronizado_r: row.get(1).unwrap(),
            })
        })
        .unwrap();

    let fim_consulta = SystemTime::now();
    println!(
        "Consulta realizada em {}s",
        fim_consulta
            .duration_since(inicio_consulta)
            .unwrap()
            .as_secs()
    );

    let mut total = 0;
    let mut diff = 0;

    let inicio_proc = SystemTime::now();
    for e in end_iter {
        total += 1;
        let registro = e.unwrap();

        let novo = registro
            .original
            .clone()
            .map(|x| padronizador(x.as_str()))
            .filter(|x| !x.is_empty())
            .unwrap_or("Null".to_string());

        let baseline = registro.padronizado_r.unwrap_or("Null".to_string());

        if novo.clone() != baseline.clone() {
            diff += 1;
            println!(
                "{}) {} => Baseline: {} | Novo: {}",
                registro.pos,
                registro.original.unwrap_or("Null".to_string()),
                baseline,
                novo,
            );
        }
    }
    let fim_proc = SystemTime::now();

    let tempo_proc = fim_proc.duration_since(inicio_proc).unwrap().as_millis();
    println!(
        "Diferentes => {}/{} ({:.3}%)",
        diff,
        total,
        diff as f64 / total as f64
    );
    println!(
        "Processado em {:.2}s ({:.1} registros/s)",
        tempo_proc as f64 / 1000.0,
        total as f64 / (tempo_proc as f64 / 1000.0)
    );
}
