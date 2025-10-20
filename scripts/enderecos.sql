SELECT DscEndereco,
  concat_ws(' ', tipo_de_logradouro_padr, logradouro_padr),
  numero_padr,
  complemento_padr,
  bairro_padr
FROM read_parquet('/mnt/storage6/usuarios/CGDTI/IpeaDataLab/projetos/2025_estimativas_luz_para_todos/dados/siase_202505_ends_padr.parquet')
where logradouro is not null or numero is not null or complemento is not null or localidade is not null
-- limit 10000000 
-- offset 10000000
