use enderecobr_rs::padronizar_logradouros;
use std::{collections::HashSet, io, time::SystemTime};

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

fn process_data(vetor: &[String]) {
    let _res: Vec<String> = vetor
        .iter()
        .collect::<HashSet<&String>>()
        .into_iter()
        .map(|x| padronizar_logradouros(x.to_string()))
        .collect();
}

fn main() {
    let vetor = timeit(load_data);
    timeit(|| process_data(&vetor));
    println!(
        "Finalizado para {} registros ({} unicos)",
        vetor.len(),
        vetor.iter().collect::<HashSet<&String>>().len(),
    );
}
