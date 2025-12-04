install zipfs;
load zipfs;

copy (
SELECT
        coalesce(ds_endereco, '') as logradouro,
        coalesce(nu_endereco, '') as numero,
        coalesce(ds_complemento, '') as complemento,
        coalesce(no_bairro, '') as localidade,
        coalesce(co_cep, '') as cep,
        coalesce(no_municipio, '') as municipio,
        coalesce(sg_uf, '') as uf
FROM read_csv('zip://dados/brutos/censo_escolar.zip/microdados_censo_escolar_2024/dados/microdados_ed_basica_2024.csv', encoding='ISO_8859_1')
USING SAMPLE 100_000
)
to './dados/intermediarios/censo_escolar.parquet' (format parquet);

