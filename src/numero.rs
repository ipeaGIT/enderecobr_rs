use std::sync::LazyLock;

use crate::Padronizador;

fn criar_padronizador_numeros() -> Padronizador {
    let mut padronizador = Padronizador::default();
    padronizador
        // Regexp adicional: remove espaços em branco repetidos
        .adicionar(r"\s{2,}", " ")
        // Regexp Original: (?<!\.)\b0+(\d+)\b
        // 015 -> 15, 00001 -> 1, 0180 0181 -> 180 181, mas não 1.028 -> 1.28
        // A ideia da regexp original é tirar zeros à esquerda que não sejam separadores de milhar.
        // Como Rust não aceita look around, tentei adaptar.
        .adicionar(r"(^|[^.])\b0+(\d+)\b", "$1$2")
        // separador de milhar
        .adicionar(r"(\d+)\.(\d{3})", "$1$2")
        // SN ou S.N. ou S N ou .... -> S/N
        .adicionar(r"S\.?( |\/)?N(O|º)?\.?", "S/N")
        .adicionar(r"SEM NUMERO", "S/N")
        .adicionar(r"^(X|0|-)+$", "S/N")
        // Regexp adicional: string vazia => S/N
        .adicionar("^$", "S/N");

    padronizador.preparar();
    padronizador
}

// Em Rust, a constant é criada durante a compilação, então só posso chamar funções muito restritas
// quando uso `const`. Nesse caso,  como tenho uma construção complexa da struct `Padronizador`,
// tenho que usar static com inicialização Lazy (o LazyLock aqui previne condições de corrida).
static PADRONIZADOR_NUMEROS: LazyLock<Padronizador> = LazyLock::new(criar_padronizador_numeros);

pub fn padronizar_numeros(valor: &str) -> String {
    // Forma de obter a variável lazy
    let padronizador = &*PADRONIZADOR_NUMEROS;
    padronizador.padronizar(valor)
}
