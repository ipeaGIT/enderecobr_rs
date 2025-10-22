use enderecobr_rs::padronizar_logradouros;
use polars::prelude::{
    Column, DataType, Field, IntoColumn, LazyFrame, ParquetCompression, ParquetWriteOptions,
    PlPath, ScanArgsParquet, SinkOptions, StatisticsOptions, StringChunked, col,
    sync_on_close::SyncOnCloseType,
};

fn main() {
    let mut args = std::env::args();
    let arquivo = args.next_back().unwrap();
    let path = PlPath::new(&arquivo);

    let _df = LazyFrame::scan_parquet(
        path,
        ScanArgsParquet {
            low_memory: true,
            parallel: polars::prelude::ParallelStrategy::RowGroups,
            use_statistics: true,
            rechunk: true,
            ..Default::default()
        },
    )
    .unwrap()
    .with_new_streaming(true)
    .select([col("logradouro")])
    .with_column(col("logradouro").map(
        |logr: Column| {
            let lote = StringChunked::from_iter(
                logr.str()
                    .unwrap()
                    .iter()
                    .map(|opt| opt.map(padronizar_logradouros)),
            );
            Ok(lote.into_column())
        },
        |_, _| Ok(Field::new("logr_padr".into(), DataType::String)),
    ))
    .sink_parquet(
        polars::prelude::SinkTarget::Path(PlPath::new("./aaaa.parquet")),
        ParquetWriteOptions {
            compression: ParquetCompression::Zstd(None),
            statistics: StatisticsOptions {
                min_value: true,
                max_value: true,
                distinct_count: true,
                null_count: true,
            },
            row_group_size: Some(10_000),
            data_page_size: Some(1024 * 1024),
            ..Default::default()
        },
        None,
        SinkOptions {
            sync_on_close: SyncOnCloseType::All,
            maintain_order: false,
            ..Default::default()
        },
    )
    .unwrap()
    .collect_with_engine(polars::prelude::Engine::Streaming);
}
