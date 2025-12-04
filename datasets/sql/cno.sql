install zipfs;
load zipfs;


copy (
  select
    concat_ws(' ', nullif("Tipo de logradouro", 'OUTROS'), "Logradouro") as logradouro, 
  "Número do logradouro" as numero,
  "Complemento",
  "Bairro" as localidade ,
  "CEP",
  "Nome do município" as municipio,
  "Estado" as uf,
  from read_csv('zip://dados/brutos/cno.zip/cno.csv', encoding='ISO_8859_1')
  using sample 100_000
)
to './dados/intermediarios/cno.parquet' (format parquet);

