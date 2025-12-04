copy (
SELECT
        endereco as logradouro,
        numero,
        complemento,
        bairro as localidade,
        cep,
        cidade as municipio,
        uf
    FROM read_csv('./dados/brutos/postos_cadunico.csv')
) to './dados/intermediarios/postos_cadunico.parquet' (format parquet);

