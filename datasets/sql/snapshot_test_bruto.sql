COPY (SELECT DISTINCT logradouro.upper().trim() FROM './dados/dataset.parquet' ORDER BY 1) TO 'dados/snapshot_test/logradouro_bruto.csv' (HEADER false, DELIMITER '', QUOTE '', ESCAPE '', FORMAT CSV);
COPY (SELECT DISTINCT numero.upper().trim() FROM './dados/dataset.parquet' ORDER BY 1) TO 'dados/snapshot_test/numero_bruto.csv' (HEADER false, DELIMITER '', QUOTE '', ESCAPE '', FORMAT CSV);
COPY (SELECT DISTINCT complemento.upper().trim() FROM './dados/dataset.parquet' ORDER BY 1) TO 'dados/snapshot_test/complemento_bruto.csv' (HEADER false, DELIMITER '', QUOTE '', ESCAPE '', FORMAT CSV);
COPY (SELECT DISTINCT localidade.upper().trim() FROM './dados/dataset.parquet' ORDER BY 1) TO 'dados/snapshot_test/localidade_bruto.csv' (HEADER false, DELIMITER '', QUOTE '', ESCAPE '', FORMAT CSV);
COPY (SELECT DISTINCT municipio.upper().trim() FROM './dados/dataset.parquet' ORDER BY 1) TO 'dados/snapshot_test/municipio_bruto.csv' (HEADER false, DELIMITER '', QUOTE '', ESCAPE '', FORMAT CSV);
COPY (SELECT DISTINCT uf.upper().trim() FROM './dados/dataset.parquet' ORDER BY 1) TO 'dados/snapshot_test/uf_bruto.csv' (HEADER false, DELIMITER '', QUOTE '', ESCAPE '', FORMAT CSV);
