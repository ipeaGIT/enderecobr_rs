use duckdb::Connection;
use enderecobr_rs::padronizar_logradouros;

#[derive(Debug)]
struct Endereco {
    pos: i32,
    logradouro: Option<String>,
    logradouro_padr: Option<String>,
}

fn main() {
    let query = "SELECT logradouro, nullif(logradouro_padr, '')
FROM read_parquet('/mnt/storage6/usuarios/cnpj-resultado-r.parquet');";
    let conn = Connection::open_in_memory().unwrap();
    let mut stmt = conn.prepare(query).unwrap();
    let mut i: i32 = 0;

    let end_iter = stmt
        .query_map([], |row| {
            i += 1;
            Ok(Endereco {
                pos: i,
                logradouro: row.get(0).unwrap(),
                logradouro_padr: row.get(1).unwrap(),
            })
        })
        .unwrap();

    let mut total = 0;
    let mut diff = 0;

    for e in end_iter {
        total += 1;
        let registro = e.unwrap();

        let novo = registro
            .logradouro
            .clone()
            .map(|x| padronizar_logradouros(x.as_str()));

        let baseline = registro.logradouro_padr;

        if novo.clone() != baseline.clone() && !(novo.clone().unwrap() == "" && baseline.is_none())
        {
            diff += 1;
            println!(
                "{}) {} => Original: {} | Novo: {}",
                registro.pos,
                registro.logradouro.unwrap_or("Null".to_string()),
                baseline.unwrap_or("Null".to_string()),
                novo.unwrap_or("Null".to_string())
            );
        }
    }
    println!("Diferentes => {}/{}", diff, total);
}
