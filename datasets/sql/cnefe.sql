copy (
  select
      concat_ws(' ', nullif(nom_tipo_seglogr, 'EDF'), nullif(nom_titulo_seglogr, ''), nom_seglogr) as logradouro, 
      concat_ws(' ', nullif(num_adress, 0)) as numero,
      concat_ws(' ', nullif(dsc_modificador, 'SN'), nom_comp_elem1, val_comp_elem1, nom_comp_elem2, val_comp_elem2, nom_comp_elem3, val_comp_elem3, nom_comp_elem4)
          .regexp_replace('[ ]+', ' ', 'g').trim() as complemento,
      desc_localidade as localidade,
      cep,
      m.municipio,
      m.uf,
  from '/mnt/storage6/bases/DADOS/PUBLICO/CNEFE/parquet/2022/arquivos/*.parquet'
  inner join '../src/data/municipios.csv' as m on cod_ibge = code_muni
  using sample 100_000
)
to './dados/intermediarios/cnefe.parquet' (format parquet);
