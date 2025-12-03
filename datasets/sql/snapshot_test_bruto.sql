COPY (SELECT DISTINCT logradouro FROM './dados/dataset.parquet') TO 'dados/snapshot_test/logradouro_bruto.csv' (HEADER false, DELIMITER '', QUOTE '', ESCAPE '', FORMAT CSV);
COPY (SELECT DISTINCT numero FROM './dados/dataset.parquet') TO 'dados/snapshot_test/numero_bruto.csv' (HEADER false, DELIMITER '', QUOTE '', ESCAPE '', FORMAT CSV);
COPY (SELECT DISTINCT complemento FROM './dados/dataset.parquet') TO 'dados/snapshot_test/complemento_bruto.csv' (HEADER false, DELIMITER '', QUOTE '', ESCAPE '', FORMAT CSV);
COPY (SELECT DISTINCT localidade FROM './dados/dataset.parquet') TO 'dados/snapshot_test/localidade_bruto.csv' (HEADER false, DELIMITER '', QUOTE '', ESCAPE '', FORMAT CSV);
COPY (SELECT DISTINCT municipio FROM './dados/dataset.parquet') TO 'dados/snapshot_test/municipio_bruto.csv' (HEADER false, DELIMITER '', QUOTE '', ESCAPE '', FORMAT CSV);
COPY (SELECT DISTINCT uf FROM './dados/dataset.parquet') TO 'dados/snapshot_test/uf_bruto.csv' (HEADER false, DELIMITER '', QUOTE '', ESCAPE '', FORMAT CSV);
