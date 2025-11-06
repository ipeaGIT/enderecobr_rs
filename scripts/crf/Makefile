.PHONY: test avaliacao-manual eval

DIR_DADOS = ./dados
DIR_BRUTOS = ./dados/brutos
DIR_INTERMEDIARIOS = ./dados/intermediarios
DIR_SQLS = ./src/sql
DIR_PYTHON = ./src/crf_endereco

### Utilitários

test:
	uv run pytest

avaliacao-manual:
	uv run src/crf_endereco/teste_manual.py

### Download Brutos

# Variável '$@' indica o nome do target.

$(DIR_BRUTOS)/cno.zip:
	curl -o $@ 'https://arquivos.receitafederal.gov.br/dados/cno/cno.zip'

$(DIR_BRUTOS)/cneas.csv:
	curl -o $@ 'https://aplicacoes.mds.gov.br/sagi/servicos/misocial_mes_equipamento/?fl=codigo_ibge%2Canomes_s%20cneas_entidade_nome_municipio_s%20cneas_entidade_sigla_uf_s%20cneas_cod_entidade_s%20cneas_entidade_numero_cnpj_s%20cneas_entidade_razao_social_s%20cneas_entidade_nome_fantasia_s%20cneas_entidade_endereco_logradouro_s%20cneas_entidade_endereco_numero_s%20cneas_entidade_endereco_complemento_s%20cneas_entidade_endereco_bairro_s%20cneas_entidade_endereco_cep_s%20cneas_entidade_situacao_cadastro_s%20cneas_entidade_dt_extracao_s&fq=cneas_cod_entidade_s%3A*&q=*%3A*&rows=100000&sort=anomes_s%20desc%2C%20codigo_ibge%20asc&wt=csv&fq=anomes_s:2025*'

$(DIR_BRUTOS)/cnes.zip:
	curl -o $@ 'https://s3.sa-east-1.amazonaws.com/ckan.saude.gov.br/CNES/cnes_estabelecimentos_csv.zip'

$(DIR_BRUTOS)/censo_escolar.zip:
	curl -o $@ 'https://download.inep.gov.br/dados_abertos/microdados_censo_escolar_2024.zip'

$(DIR_BRUTOS)/postos_cadunico.csv:
	curl -o $@ 'https://aplicacoes.mds.gov.br/sagi/servicos/equipamentos?fq=tipo_equipamento:POSTO_CADASTRAMENTO&wt=csv&fl=id_equipamento,ibge,uf,cidade,nome,responsavel,telefone,endereco,numero,complemento,referencia,bairro,cep,georef_location,data_atualizacao&rows=999999999&q=*:*'

$(DIR_BRUTOS)/municipios_cnpj.csv:
	curl -o $@ 'https://www.gov.br/receitafederal/dados/municipios.csv'

### Parquets intermediários

# Variável '$<' indica a primeira dependência.

$(DIR_INTERMEDIARIOS)/cneas.parquet: $(DIR_SQLS)/cneas.sql $(DIR_BRUTOS)/cneas.csv
	duckdb < $<

$(DIR_INTERMEDIARIOS)/cno.parquet: $(DIR_SQLS)/cno.sql $(DIR_BRUTOS)/cno.zip
	duckdb < $<

$(DIR_INTERMEDIARIOS)/cnes.parquet: $(DIR_SQLS)/cnes.sql $(DIR_BRUTOS)/cnes.zip
	duckdb < $<

$(DIR_INTERMEDIARIOS)/censo_escolar.parquet: $(DIR_SQLS)/censo_escolar.sql $(DIR_BRUTOS)/censo_escolar.zip
	duckdb < $<

$(DIR_INTERMEDIARIOS)/postos_cadunico.parquet: $(DIR_SQLS)/postos_cadunico.sql $(DIR_BRUTOS)/postos_cadunico.csv
	duckdb < $<

$(DIR_INTERMEDIARIOS)/municipios_cnpj.parquet: $(DIR_SQLS)/municipios_cnpj.sql $(DIR_BRUTOS)/municipios_cnpj.csv
	duckdb < $<

$(DIR_INTERMEDIARIOS)/cnefe.parquet: $(DIR_SQLS)/cnefe.sql
	duckdb < $<

# TODO: baixar dados do CNPJ em vez de usar os locais

$(DIR_INTERMEDIARIOS)/cnpj.parquet: $(DIR_SQLS)/cnpj.sql $(DIR_INTERMEDIARIOS)/municipios_cnpj.parquet
	duckdb < $<

### Dataset final

$(DIR_DADOS)/treino.parquet $(DIR_DADOS)/teste.parquet: $(DIR_SQLS)/dataset_final.sql $(DIR_INTERMEDIARIOS)/cneas.parquet $(DIR_INTERMEDIARIOS)/cno.parquet $(DIR_INTERMEDIARIOS)/cnes.parquet $(DIR_INTERMEDIARIOS)/censo_escolar.parquet $(DIR_INTERMEDIARIOS)/postos_cadunico.parquet $(DIR_INTERMEDIARIOS)/cnefe.parquet $(DIR_INTERMEDIARIOS)/cnpj.parquet
	duckdb < $<

### Treinamento efetivo do modelo

# Preferi não colocar o script de treinamento aqui para não precisar executar toda ver que uma virgula nele mudar.
$(DIR_DADOS)/tagger.crf: $(DIR_DADOS)/treino.parquet 
	uv run $(DIR_PYTHON)/treinamento.py

eval: $(DIR_DADOS)/teste.parquet $(DIR_PYTHON)/eval.py $(DIR_DADOS)/tagger.crf
	uv run $(DIR_PYTHON)/eval.py
