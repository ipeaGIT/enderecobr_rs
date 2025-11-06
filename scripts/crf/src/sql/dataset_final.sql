
CREATE TEMP TABLE dados AS
  SELECT *, random() r, 'cneas' as origem FROM read_parquet('./dados/intermediarios/cneas.parquet')
  UNION
  SELECT *, random() r, 'cno' FROM read_parquet('./dados/intermediarios/cno.parquet')
  UNION
  SELECT *, random() r, 'cnes' FROM read_parquet('./dados/intermediarios/cnes.parquet')
  UNION
  SELECT *, random() r, 'censo_escolar' FROM read_parquet('./dados/intermediarios/censo_escolar.parquet')
  UNION
  SELECT *, random() r, 'postos_cadunico' FROM read_parquet('./dados/intermediarios/postos_cadunico.parquet')
  UNION
  SELECT *, random() r, 'cnefe' FROM read_parquet('./dados/intermediarios/cnefe.parquet')
  UNION
  SELECT *, random() r, 'cnpj' FROM read_parquet('./dados/intermediarios/cnpj.parquet');

COPY (
    SELECT * EXCLUDE (r)
    FROM dados
    WHERE r < 0.9
    ORDER BY uf, municipio, localidade, logradouro
) TO 'dados/treino.parquet' (FORMAT 'parquet');

COPY (
    SELECT * EXCLUDE (r)
    FROM dados
    WHERE r >= 0.9
    ORDER BY uf, municipio, localidade, logradouro
) TO 'dados/teste.parquet' (FORMAT 'parquet');

SUMMARIZE './dados/treino.parquet';
SUMMARIZE './dados/teste.parquet';
