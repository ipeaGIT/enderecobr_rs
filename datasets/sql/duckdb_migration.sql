CREATE SEQUENCE IF NOT EXISTS id_incremental;

ALTER TABLE dataset ADD COLUMN IF NOT EXISTS id INTEGER DEFAULT nextval('id_incremental');


ALTER TABLE dataset ADD COLUMN IF NOT EXISTS qualidade string;
ALTER TABLE dataset ADD COLUMN IF NOT EXISTS qualidade_justificativa string;
ALTER TABLE dataset ADD COLUMN IF NOT EXISTS qualidade_rotulador string;
