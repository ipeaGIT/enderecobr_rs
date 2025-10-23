use std::num::NonZero;

use enderecobr_rs::obter_padronizador_por_tipo;
use polars::prelude::{
    Column, CsvWriterOptions, DataType, Field, IntoColumn, LazyFrame, PlPath, ScanArgsParquet,
    SinkOptions, SinkTarget, StringChunked, col,
};

fn main() {
    let mut args = std::env::args();
    let arquivo = args.next_back().unwrap();
    let path = PlPath::new(&arquivo);

    let campo_bruto = "logradouro";
    let campo_baseline = "logradouro_padr";

    let tipo_padronizador = "logradouro";

    let padronizador = obter_padronizador_por_tipo(tipo_padronizador).unwrap();

    let res = LazyFrame::scan_parquet(
        path,
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
    .select([col(campo_bruto), col(campo_baseline)])
    .with_column(
        col(campo_bruto)
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
    .filter(col("campo_processado").eq(col(campo_baseline)).not())
    .unique(None, polars::frame::UniqueKeepStrategy::First)
    .sink_csv(
        SinkTarget::Path(PlPath::new("/dev/stdout")),
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
