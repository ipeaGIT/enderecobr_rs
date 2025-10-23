// use std::{cell::RefCell, fs::File, ops::Deref, sync::Arc};
//
// use arrow::{
//     array::{RecordBatch, StringArray},
//     datatypes::{DataType, Field, Schema},
// };
// use duckdb::{ArrowStream, Connection, Statement};
// use parquet::{arrow::ArrowWriter, file::properties::WriterProperties};
//
// use crate::Endereco;
//
// pub struct ConfiguracaoCampos {
//     pub endereco_bruto: Option<String>,
//
//     pub logradouro: Option<String>,
//     pub numero: Option<String>,
//     pub complemento: Option<String>,
//     pub localidade: Option<String>,
//     // logradouro_padr: Option<String>,
//     // numero_padr: Option<String>,
//     // complemento_padr: Option<String>,
//     // localidade_padr: Option<String>,
// }
//
// impl ConfiguracaoCampos {
//     pub fn valores_padrao() -> ConfiguracaoCampos {
//         ConfiguracaoCampos {
//             endereco_bruto: None,
//             logradouro: Some("logradouro".to_string()),
//             numero: Some("numero".to_string()),
//             complemento: Some("complemento".to_string()),
//             localidade: Some("localidade".to_string()),
//         }
//     }
// }
//
// fn criar_schema(config: &ConfiguracaoCampos) -> Arc<Schema> {
//     let campos: Vec<Field> = vec![
//         config.endereco_bruto.clone(),
//         config.logradouro.clone(),
//         config.numero.clone(),
//         config.complemento.clone(),
//         config.localidade.clone(),
//     ]
//     .into_iter()
//     .filter_map(|opt| opt.map(|nome_campo| Field::new(nome_campo, DataType::Utf8, true)))
//     .collect();
//
//     Arc::new(Schema::new(campos))
// }
//
// pub struct ParquetOutput {
//     configuracao_campos: ConfiguracaoCampos,
//     tam_batch: usize,
//
//     buffer_endereco: Vec<Endereco>,
//     buffer_endereco_bruto: Vec<Option<String>>,
//
//     schema: Arc<Schema>,
//     pos: usize,
//     writer: ArrowWriter<File>,
// }
//
// impl ParquetOutput {
//     pub fn new(configuracao_campos: ConfiguracaoCampos, tam_batch: usize, arquivo: File) -> Self {
//         let props = WriterProperties::builder()
//             .set_compression(parquet::basic::Compression::SNAPPY)
//             .build();
//         let schema = criar_schema(&configuracao_campos);
//         let writer = ArrowWriter::try_new(arquivo, schema.clone(), Some(props)).unwrap();
//
//         ParquetOutput {
//             configuracao_campos,
//             tam_batch,
//             buffer_endereco: Vec::with_capacity(tam_batch),
//             buffer_endereco_bruto: Vec::with_capacity(tam_batch),
//             schema,
//             pos: 0i32 as usize,
//             writer,
//         }
//     }
//
//     pub fn iniciar_linha(&mut self) {
//         self.pos += 1;
//
//         if !self.pos.is_multiple_of(self.tam_batch) {
//             return;
//         }
//
//         let logr: Vec<Option<&str>> = self
//             .buffer_endereco
//             .iter()
//             .map(|e| e.logradouro.as_deref())
//             .collect();
//
//         let num: Vec<Option<&str>> = self
//             .buffer_endereco
//             .iter()
//             .map(|e| e.numero.as_deref())
//             .collect();
//
//         let com: Vec<Option<&str>> = self
//             .buffer_endereco
//             .iter()
//             .map(|e| e.complemento.as_deref())
//             .collect();
//
//         let loc: Vec<Option<&str>> = self
//             .buffer_endereco
//             .iter()
//             .map(|e| e.localidade.as_deref())
//             .collect();
//
//         self.writer
//             .write(
//                 &RecordBatch::try_new(
//                     self.schema.clone(),
//                     vec![
//                         Arc::new(StringArray::from(logr)),
//                         Arc::new(StringArray::from(num)),
//                         Arc::new(StringArray::from(com)),
//                         Arc::new(StringArray::from(loc)),
//                     ],
//                 )
//                 .unwrap(),
//             )
//             .unwrap();
//
//         self.buffer_endereco.clear();
//         self.buffer_endereco_bruto.clear();
//     }
//     pub fn close(self) -> Result<(), String> {
//         self.writer.close().map_err(|e| e.to_string())?;
//         Ok(())
//     }
//     // pub fn adicionar_endereco_bruto(
//     //     &mut self,
//     //     endereco_bruto: Option<String>,
//     // ) -> Result<(), String> {
//     //     self.buffer_endereco_bruto.push(endereco_bruto);
//     //     Ok(())
//     // }
//     pub fn adicionar_endereco(&mut self, endereco: Endereco) {
//         self.buffer_endereco.push(endereco);
//     }
// }
//
// ///////////////////////
//
// pub struct DuckDBInput<'a> {
//     query: String,
//     stmt: Statement<'a>,
//     schema: Arc<Schema>,
//     stream: Option<ArrowStream<'a>>,
// }
//
// impl<'a> DuckDBInput<'a> {
//     pub fn new(conn: &'a Connection, query: String) -> Self {
//         let mut schema_stmt = conn
//             .prepare(format!("with query as ( {} ) select * from query limit 1", query).as_str())
//             .unwrap();
//         let schema = schema_stmt.query_arrow([]).unwrap().get_schema();
//
//         // 2. Cria struct com stmt temporariamente None
//         DuckDBInput {
//             query: query.clone(),
//             stmt: conn.prepare(&query).unwrap(),
//             schema: schema.clone(),
//             stream: None,
//         }
//     }
//
//     pub fn como_seq(&mut self) -> DuckDBSeq<'_> {
//         let stream = self.stmt.stream_arrow([], self.schema.clone()).unwrap();
//         DuckDBSeq {
//             stream,
//             records: None,
//             buffer: None,
//             batch_pos: 0,
//             buffer_size: 0,
//         }
//     }
// }
//
// pub struct DuckDBSeq<'a> {
//     stream: ArrowStream<'a>,
//     records: Option<RecordBatch>,
//     buffer: Option<Vec<Option<String>>>,
//     batch_pos: usize,
//     buffer_size: usize,
// }
//
// impl Iterator for DuckDBSeq<'_> {
//     type Item = Option<String>;
//     // type Item = String;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         // Se o batch atual foi esgotado ou ainda não foi carregado
//         if self.records.is_none() {
//             if let Some(batch) = self.stream.next() {
//                 // Processo
//                 let col: Vec<Option<String>> = batch
//                     .column(0)
//                     .as_any()
//                     .downcast_ref::<StringArray>()
//                     .unwrap()
//                     .iter()
//                     .map(|x| x.map(|y| y.to_string()))
//                     .collect();
//
//                 self.buffer = Some(col);
//                 self.batch_pos = 0;
//                 self.buffer_size = batch.num_rows();
//                 // println!("{}", self.buffer_size);
//             } else {
//                 return None; // Fim do stream
//             }
//         }
//
//         // Verificamos se ainda há linhas no batch
//         if self.batch_pos >= self.buffer_size {
//             // Batch esgotado, resetamos para carregar o próximo
//             self.records = None;
//             return self.next(); // Recursão para próximo batch
//         }
//
//         self.batch_pos += 1;
//         self.buffer.as_ref().unwrap().get(self.batch_pos).cloned()
//     }
// }
//
// // fn aaaa() {
// //     LazyFrame::scan_parquet("....", Default::default()).unwrap().select(logradouro).
// // }
//
// //
// // struct DuckDBStream {
// //     _conn: Connection,
// //     stream: ArrowStream<'static>,
// // }
// //
// // impl DuckDBInput {
// //     fn new(query: &str) -> Result<Self, duckdb::Error> {
// //         let conn = Connection::open_in_memory()?;
// //
// //         // Obtém o schema
// //         let mut schema_stmt = conn.prepare(&format!(
// //             "with query as ( {} ) select * from query limit 1",
// //             query
// //         ))?;
// //         let schema = schema_stmt.query_arrow([])?.get_schema();
// //
// //         // Executa a query principal
// //         let mut stmt = conn.prepare(&format!("{} limit 1000;", query))?;
// //         let stream = stmt.stream_arrow([], schema)?;
// //
// //         Ok(Self {
// //             inner: Box::new(DuckDBStream {
// //                 _conn: conn,
// //                 stream,
// //             }),
// //         })
// //     }
// // }
//
// // pub struct DuckDBInput<'a> {
// //     query: String,
// //     stmt: Statement<'a>,
// //     conn: &'a Connection,
// //     schema: Arc<Schema>,
// //     stream: ArrowStream<'a>
// // }
// // impl<'a> DuckDBInput<'a> {
// //     pub fn new(conn: &'a Connection, query: String) -> Self {
// //         let mut stmt = conn.prepare(&query).unwrap();
// //         let mut schema_stmt = conn
// //             .prepare(format!("with query as ( {} ) select * from query limit 1", query).as_str())
// //             .unwrap();
// //         let schema = schema_stmt.query_arrow([]).unwrap().get_schema();
// //         let stream = stmt.stream_arrow([], schema.clone()).unwrap();
// //
// //         DuckDBInput {
// //             query,
// //             stmt,
// //             conn,
// //             schema,
// //             stream
// //         }
// //     }
// //
// //     pub fn stream(&mut self) -> ArrowStream<'_> {
// //         self.stmt.stream_arrow([], self.schema.clone()).unwrap()
// //     }
// // }
//
// // impl Iterator for DuckDBInput {
// //     type Item = Vec<String>;
// //
// //     fn next(&mut self) -> Option<Self::Item> {
// //         for batch in iterador {
// //             let colunas: Vec<&GenericByteArray<GenericStringType<i32>>> = batch
// //                 .columns()
// //                 .iter()
// //                 .map(|x| x.as_any().downcast_ref::<StringArray>().unwrap())
// //                 .collect();
// //
// //             //     let logradouros = batch
// //             //         .column(1)
// //             //         .as_any()
// //             //         .downcast_ref::<StringArray>()
// //             //         .unwrap();
// //             //
// //             // let colunas =;
// //         }
// //     }
// // }
//
// // let query = read_to_string(arq_consulta).unwrap();
// // let conn = Connection::open_in_memory().unwrap();
// //
// // let mut schema_stmt = conn
// //     .prepare(format!("with query as ( {} ) select * from query limit 1", query).as_str())
// //     .unwrap();
// // let schema = schema_stmt.query_arrow([]).unwrap().get_schema();
// //
// // let mut stmt = conn
// //     .prepare(format!("{} limit 1000;", query).as_str())
// //     .unwrap();
// //
// // println!("Realizando Consulta");
// // let inicio_consulta = SystemTime::now();
// //
// // let end_iter = stmt.stream_arrow([], schema).unwrap();
// //
// // let fim_consulta = SystemTime::now();
// //
// // for batch in end_iter {
// //     let originais = batch
// //         .column(0)
// //         .as_any()
// //         .downcast_ref::<StringArray>()
// //         .unwrap();
// //
// //     let logradouros = batch
// //         .column(1)
// //         .as_any()
// //         .downcast_ref::<StringArray>()
// //         .unwrap();
// //     let numeros = batch
// //         .column(2)
// //         .as_any()
// //         .downcast_ref::<StringArray>()
// //         .unwrap();
// //     let complementos = batch
// //         .column(3)
// //         .as_any()
// //         .downcast_ref::<StringArray>()
// //         .unwrap();
//
// // let localidades = batch
// //     .column(4)
// //     .as_any()
// //     .downcast_ref::<StringArray>()
// //     .unwrap();
//
// // for i in 0..batch.num_rows() {
