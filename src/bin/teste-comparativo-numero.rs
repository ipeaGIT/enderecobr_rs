use duckdb::Connection;
use enderecobr_rs::padronizar_numeros;

#[derive(Debug)]
struct Endereco {
    pos: i32,
    numero: Option<String>,
    numero_padr: Option<String>,
}

fn main() {
    let arquivo = std::env::args().next_back().unwrap();

    let query = format!(
        "SELECT numero, numero_padr FROM read_parquet('{}');",
        arquivo
    );

    let conn = Connection::open_in_memory().unwrap();
    let mut stmt = conn.prepare(query.as_str()).unwrap();
    let mut i: i32 = 0;

    let end_iter = stmt
        .query_map([], |row| {
            i += 1;
            Ok(Endereco {
                pos: i,
                numero: row.get(0).unwrap(),
                numero_padr: row.get(1).unwrap(),
            })
        })
        .unwrap();

    let mut total = 0;
    let mut diff = 0;

    println!("Consulta realizada");
    for e in end_iter {
        total += 1;
        let registro = e.unwrap();

        let novo = registro
            .numero
            .clone()
            .map(|x| padronizar_numeros(x.as_str()));

        let baseline = registro.numero_padr;

        if novo.clone().unwrap_or(String::from("S/N"))
            != baseline.clone().unwrap_or(String::from("S/N"))
        {
            diff += 1;
            println!(
                "{}) {} => Original: {} | Novo: {}",
                registro.pos,
                registro.numero.unwrap_or("Null".to_string()),
                baseline.unwrap_or("Null".to_string()),
                novo.unwrap_or("Null".to_string())
            );
        }
    }
    println!("Diferentes => {}/{}", diff, total);
}
