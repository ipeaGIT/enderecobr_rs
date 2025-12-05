
copy (
  SELECT
      concat_ws(' ', tipo_logradouro, logradouro) as logradouro,
      coalesce(numero_estab, '') as numero,
      coalesce(complemento, '') as complemento,
      coalesce(bairro, '') as localidade,
      coalesce(cep, '') as cep,
      m.municipio_ibge as municipio,
      m.uf as uf,
  FROM
      read_parquet('/mnt/storage6/bases/DADOS/PUBLICO/CNPJ/parquet/estabelecimentos/*.parquet') as d
  left join 'dados/intermediarios/municipios_cnpj.parquet' as m on m.cod_municipio_cnpj = d.municipio
  using sample 150_000
)
to './dados/intermediarios/cnpj.parquet' (format parquet);
