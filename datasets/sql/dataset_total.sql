
CREATE TEMP TABLE dados AS
  SELECT *, 'cneas' as origem FROM read_parquet('./dados/intermediarios/cneas.parquet')
  UNION
  SELECT *, 'cno' FROM read_parquet('./dados/intermediarios/cno.parquet')
  UNION
  SELECT *, 'cnes' FROM read_parquet('./dados/intermediarios/cnes.parquet')
  UNION
  SELECT *, 'censo_escolar' FROM read_parquet('./dados/intermediarios/censo_escolar.parquet')
  UNION
  SELECT *, 'postos_cadunico' FROM read_parquet('./dados/intermediarios/postos_cadunico.parquet')
  UNION
  SELECT *, 'cnefe' FROM read_parquet('./dados/intermediarios/cnefe.parquet')
  UNION
  SELECT *, 'cnpj' FROM read_parquet('./dados/intermediarios/cnpj.parquet');

COPY (
    SELECT *
    FROM dados
    ORDER BY uf, municipio, localidade, logradouro
) TO 'dados/dataset.parquet' (FORMAT 'parquet');

