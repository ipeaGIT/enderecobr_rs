install zipfs;
load zipfs;

copy (
  SELECT
          no_logradouro as endereco,
          nu_endereco as numero,
          '' as complemento, -- Não tem complemento
          no_bairro as bairro,
          co_cep as cep,
          m.municipio,
          m.uf
  FROM read_csv('zip://dados/brutos/cnes.zip/cnes_estabelecimentos.csv') as e
  LEFT JOIN '../src/data/municipios.csv' as m on m.cod_ibge = e.co_ibge
  using sample 100_000
)
to './dados/intermediarios/cnes.parquet' (format parquet);

