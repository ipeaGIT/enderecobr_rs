copy (
  SELECT
    "CÓDIGO DO MUNICÍPIO - TOM" as cod_municipio_cnpj,
    "CÓDIGO DO MUNICÍPIO - IBGE" as cod_ibge,
    "MUNICÍPIO - TOM" as municipio_cnpj,
    "MUNICÍPIO - IBGE" as municipio_ibge,
    "UF" as uf
  FROM read_csv('dados/brutos/municipios_cnpj.csv', encoding='ISO_8859_1')
)
to './dados/intermediarios/municipios_cnpj.parquet' (format parquet);

