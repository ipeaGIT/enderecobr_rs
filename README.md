# enderecobr_rs <img align="right" src="./assets/logo.svg" alt="" width="180">

[![crates.io Latest Release](https://img.shields.io/crates/v/enderecobr_rs.svg)](https://crates.io/crates/enderecobr_rs)
[![docs.rs Rust](https://docs.rs/enderecobr_rs/badge.svg)](https://docs.rs/enderecobr_rs/)
[![PyPi Latest Release](https://img.shields.io/pypi/v/enderecobr.svg)](https://pypi.org/project/enderecobr)
[![CI](https://github.com/ipeaGIT/enderecobr_rs/workflows/CI/badge.svg)](https://github.com/ipeaGIT/enderecobr_rs/actions)
[![readthedocs Python](https://app.readthedocs.org/projects/enderecobr-py/badge)](https://enderecobr-py.readthedocs.io)
[![codecov](https://codecov.io/gh/ipeaGIT/enderecobr_rs/graph/badge.svg?token=ckgU5oGbxf)](https://codecov.io/gh/ipeaGIT/enderecobr_rs)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Documentações: [R](https://ipeagit.github.io/enderecobr/) [Python](https://enderecobr-py.readthedocs.io) [Rust](https://docs.rs/enderecobr_rs/)

> Versão experimental!

Esta biblioteca tem por objetivo prover de funções utilizadas para padronizar endereços brasileiros,
corrigindo erros comuns, expandindo abreviações etc, afim de facilitar processamentos posteriores.

Esta biblioteca é uma adaptação para Rust do [enderecobr](https://github.com/ipeaGIT/enderecobr)
visando ganho de eficiência e expandir seu uso para demais linguagens de programação,
utilizando esta implementação como base das demais.

Ela usa majoritariamente expressões regulares nas padronizações, com exceção do módulo
experimental de separação de endereços, que utiliza um modelo probabilístico de
[Conditional Random Field](https://en.wikipedia.org/wiki/Conditional_random_field) já embutido na bilioteca.

## Bindings

O repositório da versão em R se mantém em [enderecobr](https://github.com/ipeaGIT/enderecobr), utilizando
internamente as funções desta lib. A versão em Python é feita no subdiretório `bindings/python` deste projeto.
Algumas funções ou classes/structs da implementação em Rust podem não estar expostas em algumas linguagens.
Vide [documentação](https://docs.rs/enderecobr_rs) do Rust para identificar tais situações.
Para solicitar esse tipo de mudança ou bindings em outras linguagens de programação, abrir uma Issue ou
Pull request neste repositório.

## Instalação

A última versão pode ser baixada do crates com o comando:

```bash
cargo add enderecobr_rs
```

Ou adicionando em no seu `Cargo.toml`:

```toml
[dependencies]
enderecobr_rs = { version = "0.0.3" }
```

## Exemplo de uso

### Rust:

```rust
use enderecobr_rs::{padronizar_complementos, padronizar_logradouros};
assert_eq!(padronizar_logradouros("r. gen.. glicério"), "RUA GENERAL GLICERIO");
assert_eq!(padronizar_complementos("QD1 LT2 CS3"), "QUADRA 1 LOTE 2 CASA 3");
```

### Python:

```python
import enderecobr
assert enderecobr.padronizar_logradouros("r. gen.. glicério") == "RUA GENERAL GLICERIO"
assert enderecobr.padronizar_logradouros("QD1 LT2 CS3") == "QUADRA 1 LOTE 2 CASA 3"
```

## Nota <a href="https://www.ipea.gov.br"><img src="./assets/ipea_logo.png" alt="Ipea" align="right" width="300"/></a>

**enderecobr** é desenvolvido por uma equipe de pesquisadores do Instituto de Pesquisa Econômica Aplicada (Ipea).
