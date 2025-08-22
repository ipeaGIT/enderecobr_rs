use enderecobr_rs::padronizar_logradouros;
use std::{
    collections::{HashMap, HashSet},
    io,
    time::SystemTime,
};

// Roubei da internet
fn timeit<F: Fn() -> T, T>(f: F) -> T {
    let start = SystemTime::now();
    let result = f();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("Função demorou {} milis", duration.as_millis());
    result
}

fn load_data() -> Vec<String> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut vetor = Vec::<String>::new();

    for result in rdr.records() {
        let record = result.unwrap();
        let logr = record.get(1).unwrap();
        vetor.push(logr.to_string());
    }
    vetor
}

fn save_data(path: &str, data: Vec<String>) {
    let mut writer = csv::Writer::from_path(path).unwrap();
    writer.write_record(["", "logradouros"]).unwrap();
    for (i, l) in data.iter().enumerate() {
        writer.write_record([&i.to_string(), l]).unwrap();
    }
    writer.flush().unwrap();
}

fn process_data(vetor: &[String]) -> Vec<String> {
    let mut cache = HashMap::<&String, String>::new();

    vetor
        .iter()
        .map(|x| {
            if let Some(valor_cacheado) = cache.get(&x) {
                return valor_cacheado.clone();
            }

            let resultado = padronizar_logradouros(x);
            cache.insert(x, resultado.clone());
            resultado
        })
        .collect()
}

fn main() {
    let vetor = timeit(load_data);
    let resultado = timeit(|| process_data(&vetor));
    save_data("resultado.csv", resultado);
    println!(
        "Finalizado para {} registros ({} unicos)",
        vetor.len(),
        vetor.iter().collect::<HashSet<&String>>().len(),
    );
}
