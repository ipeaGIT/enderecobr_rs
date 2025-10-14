SELECT DISTINCT logradouro, logradouro_padr 
FROM read_parquet('/mnt/storage6/usuarios/CGDTI/IpeaDataLab/projetos/2025_poc_enderecos/benchmark.parquet')
