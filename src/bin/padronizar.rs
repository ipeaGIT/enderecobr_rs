use enderecobr_rs::obter_padronizador_por_tipo;

fn main() {
    let mut args = std::env::args();
    let valor = args.next_back().unwrap();
    let tipo = args.next_back().unwrap();

    let padronizador = obter_padronizador_por_tipo(tipo.as_str()).unwrap();
    println!("{}", padronizador(valor.as_str()));
}
