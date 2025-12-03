copy (
    SELECT
            cneas_entidade_endereco_logradouro_s as logradouro,
            cneas_entidade_endereco_numero_s as numero,
            cneas_entidade_endereco_complemento_s as complemento,
            cneas_entidade_endereco_bairro_s as localidade,
            cneas_entidade_endereco_cep_s as cep,
            cneas_entidade_nome_municipio_s as municipio,
            cneas_entidade_sigla_uf_s as uf,
    FROM read_csv('./dados/brutos/cneas.csv')
) to './dados/intermediarios/cneas.parquet' (format parquet);
