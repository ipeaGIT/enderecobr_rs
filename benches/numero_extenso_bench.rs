use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use enderecobr_rs::numero_extenso;

pub fn numero_extenso_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("numero_extenso");

    for &n in &[20, 1_110, 1_234_567_890] {
        group.bench_with_input(format!("numero_por_extenso/{n}"), &n, |b, &n| {
            b.iter(|| numero_extenso::numero_por_extenso(black_box(n)));
        });
    }
    for &n in &["20", "        20       "] {
        group.bench_with_input(
            format!("padronizar_numeros_por_extenso/{n}"),
            &n,
            |b, &n| {
                b.iter(|| numero_extenso::padronizar_numeros_por_extenso(black_box(n)));
            },
        );
    }
}

criterion_group!(benches, numero_extenso_bench);
criterion_main!(benches);
