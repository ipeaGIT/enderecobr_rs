use std::{fs::read_to_string, time::SystemTime};

use arrow::array::StringArray;
use duckdb::Connection;
use enderecobr_rs::separar_endereco;

fn main() {
    let mut args = std::env::args();
    let arq_consulta = args.next_back().unwrap();

    let query = read_to_string(arq_consulta).unwrap();
    let conn = Connection::open_in_memory().unwrap();

    let mut schema_stmt = conn
        .prepare(format!("with query as ( {} ) select * from query limit 1", query).as_str())
        .unwrap();
    let schema = schema_stmt.query_arrow([]).unwrap().get_schema();

    let mut stmt = conn.prepare(query.as_str()).unwrap();

    println!("Realizando Consulta");
    let inicio_consulta = SystemTime::now();

    let end_iter = stmt.stream_arrow([], schema).unwrap();

    let fim_consulta = SystemTime::now();
    println!(
        "Consulta realizada em {}s",
        fim_consulta
            .duration_since(inicio_consulta)
            .unwrap()
            .as_secs(),
    );

    let mut total = 0;
    let mut diff = 0;

    let inicio_proc = SystemTime::now();

    for batch in end_iter {
        let originais = batch
            .column(0)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();

        let logradouros = batch
            .column(1)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        let numeros = batch
            .column(2)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        let complementos = batch
            .column(3)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();

        let localidades = batch
            .column(4)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();

        for i in 0..batch.num_rows() {
            let modelo = separar_endereco(originais.value(i)).endereco_padronizado();

            // let original = originais.value(i);
            let logr = logradouros.value(i);
            let num = numeros.value(i);
            let comp = complementos.value(i);
            let loc = localidades.value(i);

            if modelo.logradouro.as_deref().unwrap_or("") != logr
                || modelo.numero.as_deref().unwrap_or("") != num
                || modelo.complemento.as_deref().unwrap_or("") != comp
                || modelo.localidade.as_deref().unwrap_or("") != loc
            {
                // println!(
                //     "- Original: {} | Modelo: {:?} |  Padronizado: {} | {} | {} | {}",
                //     original, modelo, logr, num, comp, loc
                // );
                diff += 1;
            }

            total += 1;
        }
    }
    let fim_proc = SystemTime::now();

    println!(
        "Diferentes => {}/{} ({:.3}%)",
        diff,
        total,
        100f64 * diff as f64 / total as f64
    );

    let tempo_proc = fim_proc.duration_since(inicio_proc).unwrap().as_millis();
    println!(
        "Processado em {:.2}s ({:.1} registros/s)",
        tempo_proc as f64 / 1000.0,
        total as f64 / (tempo_proc as f64 / 1000.0)
    );
}
