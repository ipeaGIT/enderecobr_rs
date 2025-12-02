use std::borrow::Cow;

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

/// Converte sequências de dígitos em uma string para seus equivalentes por extenso em português.
///
/// A função percorre a string de entrada e, ao encontrar números inteiros (em formato ASCII),
/// os substitui pelo nome completo do número (ex: "2" → "dois"), utilizando a função `numero_por_extenso`.
///
/// # Notas
/// - Números muito grandes ou inválidos (ex: overflow no parse para `i32`) são deixados inalterados.
/// - Não trata número negativos ou decimais.
/// - Se a string de entrada não contém nenhum dígito ASCII, a função retorna imediatamente uma referência
///   emprestada (`Cow::Borrowed`) para evitar alocação.
///
/// # Exemplos
/// ```rust
/// use enderecobr_rs::numero_extenso::padronizar_numeros_por_extenso;
/// assert_eq!(padronizar_numeros_por_extenso("RUA 2"), "RUA DOIS");
/// assert_eq!(padronizar_numeros_por_extenso("RUA -2"), "RUA -DOIS");
/// assert_eq!(padronizar_numeros_por_extenso("RUA -2.2"), "RUA -DOIS.DOIS");
/// assert_eq!(padronizar_numeros_por_extenso("Sem números"), "Sem números");
/// ```
///
pub fn padronizar_numeros_por_extenso(texto: &str) -> Cow<'_, str> {
    // Retorna imediatamente a mesma referência de string
    // caso não existam números na string
    if !texto.as_bytes().iter().any(|c| c.is_ascii_digit()) {
        return Cow::Borrowed(texto);
    }

    let mut numero_atual = String::new();
    let mut resultado = String::with_capacity(texto.len() + 5);

    for caracter in texto.chars() {
        if caracter.is_ascii_digit() {
            // Achei um número, adiciono ele na minha string de números.
            numero_atual.push(caracter);
        } else {
            // Quando não é um número
            if !numero_atual.is_empty() {
                // Se existia um número na string, devo salvar ele por extenso...
                match numero_atual.parse::<i32>() {
                    Ok(n) => resultado.push_str(&numero_por_extenso(n)),
                    Err(_) => resultado.push_str(&numero_atual),
                }

                // ... E limpar número atual.
                numero_atual.clear();
            }
            // Salvo o carácter atual (não número) no resultado final
            resultado.push(caracter);
        }
    }

    // Finalizo salvando o número pendente.
    if !numero_atual.is_empty() {
        match numero_atual.parse::<i32>() {
            Ok(n) => resultado.push_str(&numero_por_extenso(n)),
            Err(_) => resultado.push_str(&numero_atual),
        }
    }

    // Retorno um resultado owned (borrow seria quando é só uma referência)
    Cow::Owned(resultado)
}

/// Converte um número inteiro para sua representação por extenso em português.
///
/// Retorna uma referência estática (`Cow::Borrowed`) quando possível (números até 100),
/// ou uma string alocada dinamicamente (`Cow::Owned`) para casos compostos (negativos, grandes números).
///
/// # Exemplos
///
/// ```
/// use enderecobr_rs::numero_extenso::numero_por_extenso;
/// assert_eq!(numero_por_extenso(0), "ZERO");
/// assert_eq!(numero_por_extenso(42), "QUARENTA DOIS");
/// assert_eq!(numero_por_extenso(-1500), "MENOS MIL QUINHENTOS");
/// assert_eq!(numero_por_extenso(2_001_000), "DOIS MILHOES MIL");
/// ```
pub fn numero_por_extenso(n: i32) -> Cow<'static, str> {
    // Função auxiliar: converte números de 0 a 999
    // Retorna a referência estática para os casos até 100
    fn resolver_centenas(n: u32) -> Cow<'static, str> {
        // Se for menor que 100, usa a tabela ATE_CEM
        if n < ATE_CEM.len() as u32 {
            return Cow::Borrowed(ATE_CEM[n as usize]);
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

        Cow::Owned(resultado)
    }

    let num_abs = n.unsigned_abs();

    if (0..1000).contains(&num_abs) {
        // Caso base: número entre 0 e 999
        // Curiosamente esta é a forma "rustônica" de escrever isso,
        // segundo o clippy.
        return if n >= 0 {
            resolver_centenas(num_abs)
        } else {
            Cow::Owned(format!("MENOS {}", resolver_centenas(num_abs)))
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

    Cow::Owned(resultado)
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

    #[test]
    fn teste_padronizacao_string_por_extenso() {
        // Caso de borda
        assert_eq!(padronizar_numeros_por_extenso(""), "");

        // Caso sem número
        assert_eq!(padronizar_numeros_por_extenso("RUA AZUL"), "RUA AZUL");

        // Caso simples
        assert_eq!(
            padronizar_numeros_por_extenso("RUA 222"),
            "RUA DUZENTOS VINTE DOIS"
        );

        assert_eq!(
            padronizar_numeros_por_extenso("RUA 2 LOTE B"),
            "RUA DOIS LOTE B"
        );

        // Só número
        assert_eq!(padronizar_numeros_por_extenso("1001"), "MIL UM");

        // Vários números na string
        assert_eq!(
            padronizar_numeros_por_extenso("RUA 222 NUMERO 14 APT 101"),
            "RUA DUZENTOS VINTE DOIS NUMERO QUATORZE APT CENTO UM"
        );
    }
}
