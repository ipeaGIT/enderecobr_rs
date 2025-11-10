# EnderecoBR_rs

> Versão experimental!

Esta biblioteca tem por objetivo prover de funções utilizadas para padronizar endereços brasileiros,
corrigindo erros comuns, expandindo abreviações etc, afim de facilitar processamentos posteriores.

Esta biblioteca é uma adaptação para Rust do [enderecobr](https://github.com/ipeaGIT/enderecobr)
visando ganho de eficiência e expandir seu uso para demais linguagens de programação,
utilizando esta implementação como base das demais.

Ela usa majoritariamente expressões regulares nas padronizações, com exceção do módulo
experimental de separação de endereços, que utiliza um modelo probabilístico de
[Conditional Random Field](https://en.wikipedia.org/wiki/Conditional_random_field) já embutido na bilioteca.
