use enderecobr_rs::padronizar_logradouros;

fn main() {
    let vetor = [
        " av 122.123",
        "rod bbbb, 123",
        "mal.. 123.123123 JAn",
        "11111 aaaa",
    ];
    for a in vetor {
        println!("{}", padronizar_logradouros(a.to_string()));
    }
}
