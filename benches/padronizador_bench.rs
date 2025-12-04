use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use enderecobr_rs::{
    logradouro::criar_padronizador_logradouros, normalizar, padronizar_estados_para_nome,
};

pub fn padronizador_bench(c: &mut Criterion) {
    // Uso o de logradouro por ser o mais complexo.
    let padr = criar_padronizador_logradouros();
    let mut group = c.benchmark_group("padronizador");

    for &n in &["RUA AZUL", "R AZUL", "AV PROFA NS GRACA"] {
        group.bench_with_input(n, &n, |b, &n| {
            b.iter(|| padr.padronizar(black_box(n)));
        });
    }
}

pub fn padronizador_estado_bench(c: &mut Criterion) {
    // Uso o de logradouro por ser o mais complexo.
    let mut group = c.benchmark_group("padronizador estado");

    // Só dicionário; normalização+regex+dicionário
    for &n in &["MA", "  00021  "] {
        group.bench_with_input(n, &n, |b, &n| {
            b.iter(|| padronizar_estados_para_nome(n));
        });
    }
}

pub fn normalizador_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("normalizador");

    for &n in &["Rua do acai 15o", "RUA DO ACAI 15", "R. DO AÇAÍ 15º"] {
        group.bench_with_input(n, &n, |b, &n| {
            b.iter(|| normalizar(black_box(n)));
        });
    }
}

criterion_group!(
    benches,
    padronizador_bench,
    normalizador_bench,
    padronizador_estado_bench
);
criterion_main!(benches);
