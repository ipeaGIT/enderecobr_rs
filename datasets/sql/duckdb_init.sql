DROP TABLE IF EXISTS dataset;

CREATE TABLE dataset AS SELECT * FROM read_parquet('dados/dataset.parquet');
