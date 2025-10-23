use std::num::NonZero;

use clap::Parser;
use enderecobr_rs::obter_padronizador_por_tipo;
use polars::prelude::{
    Column, CsvWriterOptions, DataType, Field, IntoColumn, LazyFrame, PlPath, ScanArgsParquet,
    SinkOptions, SinkTarget, StringChunked, col,
};

#[derive(Debug, Parser)]
#[clap(author, version, about("Comparador"), long_about = None)]
struct Args {
    #[clap(help("Arquivo de entrada"))]
    arquivo_entrada: String,
    #[clap(short, long, default_value = "./diff.csv", help("Arquivo saída"))]
    arquivo_saida: String,
    #[clap(
        short,
        long,
        default_value = "logradouro",
        help("Tipo de Padronizador")
    )]
    tipo_padronizador: String,
    #[clap(long, default_value = "logradouro", help("Campo com valor bruto"))]
    campo_bruto: String,
    #[clap(
        long,
        default_value = "logradouro_padr",
        help("Campo com valor a ser comparado")
    )]
    campo_baseline: String,
}

fn main() {
    let args = Args::parse();

    let padronizador = obter_padronizador_por_tipo(&args.tipo_padronizador).unwrap();

    let res = LazyFrame::scan_parquet(
        PlPath::from_string(args.arquivo_entrada),
        ScanArgsParquet {
            low_memory: true,
            parallel: polars::prelude::ParallelStrategy::RowGroups,
            use_statistics: false,
            rechunk: true,
            ..Default::default()
        },
    )
    .unwrap()
    .with_new_streaming(true)
    .select([col(&args.campo_bruto), col(&args.campo_baseline)])
    .with_column(
        col(&args.campo_bruto)
            .map(
                move |campo: Column| {
                    let iterador = campo.str().unwrap().iter().map(|opt| opt.map(padronizador));
                    let col = StringChunked::from_iter(iterador).into_column();
                    Ok(col)
                },
                |_, _| Ok(Field::new("campo_processado".into(), DataType::String)),
            )
            .alias("campo_processado"),
    )
    .filter(col("campo_processado").eq(col(&args.campo_baseline)).not())
    .unique(None, polars::frame::UniqueKeepStrategy::First)
    .sink_csv(
        SinkTarget::Path(PlPath::new(&args.arquivo_saida)),
        CsvWriterOptions {
            batch_size: NonZero::new(100).unwrap(),
            ..Default::default()
        },
        None,
        SinkOptions {
            ..Default::default()
        },
    )
    .unwrap()
    .collect_with_engine(polars::prelude::Engine::Streaming);

    if let Some(err) = res.err() {
        println!("{}", err);
    };
}
