use std::{fs::File, sync::Arc};

use arrow::{
    array::{RecordBatch, StringArray},
    datatypes::{DataType, Field, Schema},
};
use parquet::{arrow::ArrowWriter, file::properties::WriterProperties};

use crate::Endereco;

pub struct ConfiguracaoCampos {
    pub endereco_bruto: Option<String>,

    pub logradouro: Option<String>,
    pub numero: Option<String>,
    pub complemento: Option<String>,
    pub localidade: Option<String>,
    // logradouro_padr: Option<String>,
    // numero_padr: Option<String>,
    // complemento_padr: Option<String>,
    // localidade_padr: Option<String>,
}

impl ConfiguracaoCampos {
    pub fn valores_padrao() -> ConfiguracaoCampos {
        ConfiguracaoCampos {
            endereco_bruto: None,
            logradouro: Some("logradouro".to_string()),
            numero: Some("numero".to_string()),
            complemento: Some("complemento".to_string()),
            localidade: Some("localidade".to_string()),
        }
    }
}

fn criar_schema(config: &ConfiguracaoCampos) -> Arc<Schema> {
    let campos: Vec<Field> = vec![
        config.endereco_bruto.clone(),
        config.logradouro.clone(),
        config.numero.clone(),
        config.complemento.clone(),
        config.localidade.clone(),
    ]
    .into_iter()
    .filter_map(|opt| opt.map(|nome_campo| Field::new(nome_campo, DataType::Utf8, true)))
    .collect();

    Arc::new(Schema::new(campos))
}

pub struct ParquetOutput {
    configuracao_campos: ConfiguracaoCampos,
    tam_batch: usize,

    buffer_endereco: Vec<Endereco>,
    buffer_endereco_bruto: Vec<Option<String>>,

    schema: Arc<Schema>,
    pos: usize,
    writer: ArrowWriter<File>,
}

impl ParquetOutput {
    pub fn new(configuracao_campos: ConfiguracaoCampos, tam_batch: usize, arquivo: File) -> Self {
        let props = WriterProperties::builder()
            .set_compression(parquet::basic::Compression::SNAPPY)
            .build();
        let schema = criar_schema(&configuracao_campos);
        let writer = ArrowWriter::try_new(arquivo, schema.clone(), Some(props)).unwrap();

        ParquetOutput {
            configuracao_campos,
            tam_batch,
            buffer_endereco: Vec::with_capacity(tam_batch),
            buffer_endereco_bruto: Vec::with_capacity(tam_batch),
            schema,
            pos: 0i32 as usize,
            writer,
        }
    }

    pub fn iniciar_linha(&mut self) {
        self.pos += 1;

        if !self.pos.is_multiple_of(self.tam_batch) {
            return;
        }

        let logr: Vec<Option<&str>> = self
            .buffer_endereco
            .iter()
            .map(|e| e.logradouro.as_deref())
            .collect();

        let num: Vec<Option<&str>> = self
            .buffer_endereco
            .iter()
            .map(|e| e.numero.as_deref())
            .collect();

        let com: Vec<Option<&str>> = self
            .buffer_endereco
            .iter()
            .map(|e| e.complemento.as_deref())
            .collect();

        let loc: Vec<Option<&str>> = self
            .buffer_endereco
            .iter()
            .map(|e| e.localidade.as_deref())
            .collect();

        self.writer
            .write(
                &RecordBatch::try_new(
                    self.schema.clone(),
                    vec![
                        Arc::new(StringArray::from(logr)),
                        Arc::new(StringArray::from(num)),
                        Arc::new(StringArray::from(com)),
                        Arc::new(StringArray::from(loc)),
                    ],
                )
                .unwrap(),
            )
            .unwrap();

        self.buffer_endereco.clear();
        self.buffer_endereco_bruto.clear();
    }
    pub fn close(self) -> Result<(), String> {
        self.writer.close().map_err(|e| e.to_string())?;
        Ok(())
    }
    // pub fn adicionar_endereco_bruto(
    //     &mut self,
    //     endereco_bruto: Option<String>,
    // ) -> Result<(), String> {
    //     self.buffer_endereco_bruto.push(endereco_bruto);
    //     Ok(())
    // }
    pub fn adicionar_endereco(&mut self, endereco: Endereco) {
        self.buffer_endereco.push(endereco);
    }
}
