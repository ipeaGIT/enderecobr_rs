use enderecobr_rs::{criar_features, separar_endereco};

fn main() {
    let mut args = std::env::args();
    let valor = args.next_back().unwrap();

    let separado = separar_endereco(&valor);
    println!("{:#?}", criar_features(valor.as_str()));
    println!("{:?}", separado);
    println!("{:?}", separado.endereco_padronizado().formatar());
}
