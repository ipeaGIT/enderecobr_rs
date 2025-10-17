use enderecobr_rs::{
    criar_features, obter_padronizador_por_tipo, padronizar_endereco_bruto, separar_endereco,
};

fn main() {
    let mut args = std::env::args();
    let valor = args.next_back().unwrap();

    let separado = separar_endereco(&valor);
    println!("{:#?}", criar_features(valor.as_str()));
    println!("{:?}", separado);
    println!("{:?}", separado.padronizar());
}
