const ATE_CEM: [&str; 101] = [
    "ZERO",
    "UM",
    "DOIS",
    "TRES",
    "QUATRO",
    "CINCO",
    "SEIS",
    "SETE",
    "OITO",
    "NOVE",
    "DEZ",
    "ONZE",
    "DOZE",
    "TREZE",
    "QUATORZE",
    "QUINZE",
    "DEZESSEIS",
    "DEZESSETE",
    "DEZOITO",
    "DEZENOVE",
    "VINTE",
    "VINTE UM",
    "VINTE DOIS",
    "VINTE TRES",
    "VINTE QUATRO",
    "VINTE CINCO",
    "VINTE SEIS",
    "VINTE SETE",
    "VINTE OITO",
    "VINTE NOVE",
    "TRINTA",
    "TRINTA UM",
    "TRINTA DOIS",
    "TRINTA TRES",
    "TRINTA QUATRO",
    "TRINTA CINCO",
    "TRINTA SEIS",
    "TRINTA SETE",
    "TRINTA OITO",
    "TRINTA NOVE",
    "QUARENTA",
    "QUARENTA UM",
    "QUARENTA DOIS",
    "QUARENTA TRES",
    "QUARENTA QUATRO",
    "QUARENTA CINCO",
    "QUARENTA SEIS",
    "QUARENTA SETE",
    "QUARENTA OITO",
    "QUARENTA NOVE",
    "CINQUENTA",
    "CINQUENTA UM",
    "CINQUENTA DOIS",
    "CINQUENTA TRES",
    "CINQUENTA QUATRO",
    "CINQUENTA CINCO",
    "CINQUENTA SEIS",
    "CINQUENTA SETE",
    "CINQUENTA OITO",
    "CINQUENTA NOVE",
    "SESSENTA",
    "SESSENTA UM",
    "SESSENTA DOIS",
    "SESSENTA TRES",
    "SESSENTA QUATRO",
    "SESSENTA CINCO",
    "SESSENTA SEIS",
    "SESSENTA SETE",
    "SESSENTA OITO",
    "SESSENTA NOVE",
    "SETENTA",
    "SETENTA UM",
    "SETENTA DOIS",
    "SETENTA TRES",
    "SETENTA QUATRO",
    "SETENTA CINCO",
    "SETENTA SEIS",
    "SETENTA SETE",
    "SETENTA OITO",
    "SETENTA NOVE",
    "OITENTA",
    "OITENTA UM",
    "OITENTA DOIS",
    "OITENTA TRES",
    "OITENTA QUATRO",
    "OITENTA CINCO",
    "OITENTA SEIS",
    "OITENTA SETE",
    "OITENTA OITO",
    "OITENTA NOVE",
    "NOVENTA",
    "NOVENTA UM",
    "NOVENTA DOIS",
    "NOVENTA TRES",
    "NOVENTA QUATRO",
    "NOVENTA CINCO",
    "NOVENTA SEIS",
    "NOVENTA SETE",
    "NOVENTA OITO",
    "NOVENTA NOVE",
    "CEM",
];

const CENTENAS: [&str; 10] = [
    "",
    "CENTO",
    "DUZENTOS",
    "TREZENTOS",
    "QUATROCENTOS",
    "QUINHENTOS",
    "SEISCENTOS",
    "SETECENTOS",
    "OITOCENTOS",
    "NOVECENTOS",
];

const ORDENS_GRANDEZA: [(&str, &str); 7] = [
    ("MIL", "MIL"),
    ("UM MILHAO", "MILHOES"),
    ("UM BILHAO", "BILHOES"), // Só i32 vai até aqui...
    ("UM TRILHAO", "TRILHOES"),
    ("UM QUADRILHAO", "QUADRILHOES"),
    ("UM QUINTILHAO", "QUINTILHOES"),
    ("UM SEXTILHAO", "SEXTILHOES"),
];

// pub fn padronizar_numeros(texto: &str) -> String {
//     texto.char_indices().map(|x| x.1.is_ascii_digit());
//     "".to_string()
// }

pub fn numero_por_extenso(n: i32) -> String {
    // Função auxiliar: converte números de 0 a 999
    fn resolver_centenas(n: u32) -> String {
        // Se for menor que 100, usa a tabela ATE_CEM
        if n < ATE_CEM.len() as u32 {
            return ATE_CEM[n as usize].to_string();
        }

        let centena = n / 100; // extrai a casa das centenas
        let dezenas = n % 100; // resto (dezenas e unidades)

        // Estimativa da média do tamanho final
        let mut resultado = String::with_capacity(10);

        if centena > 0 {
            // Adiciona palavra da centena (ex: "DUZENTOS")
            resultado.push_str(CENTENAS[centena as usize]);
        }

        if centena > 0 && dezenas > 0 {
            // Espaço entre centena e dezenas, se ambas existirem
            resultado.push(' ');
        }

        if dezenas > 0 {
            // Adiciona parte das dezenas (ex: "VINTE E CINCO")
            resultado.push_str(ATE_CEM[dezenas as usize]);
        }

        resultado
    }

    let num_abs = n.unsigned_abs();

    if (0..1000).contains(&num_abs) {
        // Caso base: número entre 0 e 999
        // Curiosamente esta é a forma "rustônica" de escrever isso,
        // segundo o clippy.
        return if n >= 0 {
            resolver_centenas(num_abs)
        } else {
            format!("MENOS {}", resolver_centenas(num_abs))
        };
    }

    // Determina a maior ordem de grandeza
    // (ex: milhão → 6 dígitos → ilog10/3 = 2 ordens de grandeza)
    let maior_ordem_grandeza = num_abs.ilog10() / 3;

    let mut base = 10u32.pow(maior_ordem_grandeza * 3); // 1, 1_000, 1_000_000, etc.
    let mut valor_restante = num_abs;

    // Estimativa/chute do tamanho esperado da string
    let mut resultado = String::with_capacity(20);
    if n < 0 {
        // Começa com "MENOS" se o número for negativo
        resultado.push_str("MENOS");
    }

    // Processa do maior para o menor agrupamento de 3 dígitos
    for ordem_grandeza in (0..=maior_ordem_grandeza).rev() {
        // Se restar menos de 1000, processa diretamente e termina
        if valor_restante > 0 && valor_restante < 1000 {
            if !resultado.is_empty() {
                resultado.push(' ');
            }
            resultado.push_str(&resolver_centenas(valor_restante));
            break;
        }

        let mais_significativo = valor_restante / base; // grupo atual (3 dígitos)
        valor_restante %= base; // valor restante que ainda falta processar
        base /= 1000; // Prepara base para a próxima iteração

        if mais_significativo == 0 {
            // pula grupos vazios (ex: 1_000_001 não menciona "ZERO MIL")
            continue;
        }

        if !resultado.is_empty() {
            // Adiciona espaço caso já tenha algo na string
            resultado.push(' ');
        }

        // Se o grupo for maior que 1, escreve o número (ex: "DOIS MIL")
        if mais_significativo != 1 {
            resultado.push_str(&resolver_centenas(mais_significativo));
            resultado.push(' ');
        }

        // Seleciona singular ou plural baseado no valor do grupo
        let (singular, plural) = ORDENS_GRANDEZA[(ordem_grandeza - 1) as usize];
        if mais_significativo == 1 {
            resultado.push_str(singular);
        } else {
            resultado.push_str(plural);
        }
    }

    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testes_basicos() {
        assert_eq!(numero_por_extenso(0), "ZERO");
        assert_eq!(numero_por_extenso(1), "UM");
        assert_eq!(numero_por_extenso(10), "DEZ");
        assert_eq!(numero_por_extenso(15), "QUINZE");
        assert_eq!(numero_por_extenso(25), "VINTE CINCO");
        assert_eq!(numero_por_extenso(100), "CEM");
        assert_eq!(numero_por_extenso(101), "CENTO UM");
        assert_eq!(numero_por_extenso(200), "DUZENTOS");
        assert_eq!(numero_por_extenso(999), "NOVECENTOS NOVENTA NOVE");
    }

    #[test]
    fn testes_milhares() {
        assert_eq!(numero_por_extenso(1000), "MIL");
        assert_eq!(numero_por_extenso(1500), "MIL QUINHENTOS");
        assert_eq!(numero_por_extenso(2001), "DOIS MIL UM");
        assert_eq!(
            numero_por_extenso(12345),
            "DOZE MIL TREZENTOS QUARENTA CINCO"
        );
    }

    #[test]
    fn testes_milhoes() {
        // Casos básicos de milhões
        assert_eq!(numero_por_extenso(1_000_000), "UM MILHAO");
        assert_eq!(numero_por_extenso(2_000_000), "DOIS MILHOES");
        assert_eq!(numero_por_extenso(10_000_000), "DEZ MILHOES");
        assert_eq!(numero_por_extenso(11_000_000), "ONZE MILHOES");
        assert_eq!(numero_por_extenso(20_000_000), "VINTE MILHOES");
        assert_eq!(numero_por_extenso(99_000_000), "NOVENTA NOVE MILHOES");

        // Combinados com milhares e unidades
        assert_eq!(
            numero_por_extenso(1_234_567),
            "UM MILHAO DUZENTOS TRINTA QUATRO MIL QUINHENTOS SESSENTA SETE"
        );
        assert_eq!(numero_por_extenso(2_001_001), "DOIS MILHOES MIL UM");
        assert_eq!(
            numero_por_extenso(99_999_999),
            "NOVENTA NOVE MILHOES NOVECENTOS NOVENTA NOVE MIL NOVECENTOS NOVENTA NOVE"
        );

        // Casos limites
        assert_eq!(numero_por_extenso(1_000_001), "UM MILHAO UM");
        assert_eq!(numero_por_extenso(1_001_000), "UM MILHAO MIL");
        assert_eq!(numero_por_extenso(1_000_100), "UM MILHAO CEM");
    }

    #[test]
    fn testes_billhoes() {
        // Casos básicos de bilhões
        assert_eq!(numero_por_extenso(1_000_000_000), "UM BILHAO");
        assert_eq!(numero_por_extenso(2_000_000_000), "DOIS BILHOES");

        // Combinados com milhões, milhares e unidades
        assert_eq!(
        numero_por_extenso(1_234_567_890),
        "UM BILHAO DUZENTOS TRINTA QUATRO MILHOES QUINHENTOS SESSENTA SETE MIL OITOCENTOS NOVENTA"
    );
        assert_eq!(
            numero_por_extenso(2_001_001_001),
            "DOIS BILHOES UM MILHAO MIL UM"
        );

        // Casos limites
        assert_eq!(numero_por_extenso(1_000_000_001), "UM BILHAO UM");
        assert_eq!(numero_por_extenso(1_001_000_000), "UM BILHAO UM MILHAO");
        assert_eq!(numero_por_extenso(1_000_100_000), "UM BILHAO CEM MIL");
        assert_eq!(numero_por_extenso(1_000_001_000), "UM BILHAO MIL");
    }

    #[test]
    fn testes_negativos() {
        assert_eq!(numero_por_extenso(-1), "MENOS UM");
        assert_eq!(numero_por_extenso(-100), "MENOS CEM");
        assert_eq!(
            numero_por_extenso(-1234),
            "MENOS MIL DUZENTOS TRINTA QUATRO"
        );
    }

    #[test]
    fn testes_limites() {
        assert_eq!(numero_por_extenso(i32::MAX), "DOIS BILHOES CENTO QUARENTA SETE MILHOES QUATROCENTOS OITENTA TRES MIL SEISCENTOS QUARENTA SETE");
        assert_eq!(numero_por_extenso(i32::MIN), "MENOS DOIS BILHOES CENTO QUARENTA SETE MILHOES QUATROCENTOS OITENTA TRES MIL SEISCENTOS QUARENTA OITO");
    }
}
