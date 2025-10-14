library(arrow)
library(enderecobr)
library(dplyr)
library(duckdb)
library(dbplyr)

options(scipen = 9999)

con <- dbConnect(duckdb())
dados <- dbGetQuery(con, "
INSTALL httpfs;

SELECT
	cneas_entidade_endereco_logradouro_s as logradouro,
	cneas_entidade_endereco_numero_s as numero,
	cneas_entidade_endereco_complemento_s as complemento,
	cneas_entidade_endereco_bairro_s as bairro,
	cneas_entidade_endereco_cep_s as cep,
	codigo_ibge as cod_ibge,
	cneas_entidade_nome_municipio_s as municipio,
	cneas_entidade_sigla_uf_s as uf,
FROM
	read_csv('https://aplicacoes.mds.gov.br/sagi/servicos/misocial_mes_equipamento/?fl=codigo_ibge%2Canomes_s%20cneas_entidade_nome_municipio_s%20cneas_entidade_sigla_uf_s%20cneas_cod_entidade_s%20cneas_entidade_numero_cnpj_s%20cneas_entidade_razao_social_s%20cneas_entidade_nome_fantasia_s%20cneas_entidade_endereco_logradouro_s%20cneas_entidade_endereco_numero_s%20cneas_entidade_endereco_complemento_s%20cneas_entidade_endereco_bairro_s%20cneas_entidade_endereco_cep_s%20cneas_entidade_situacao_cadastro_s%20cneas_entidade_dt_extracao_s&fq=cneas_cod_entidade_s%3A*&q=*%3A*&rows=100000&sort=anomes_s%20desc%2C%20codigo_ibge%20asc&wt=csv&fq=anomes_s:2025*')
UNION
SELECT
	concat_ws(no_tip_logradouro_fam, no_tit_logradouro_fam, no_logradouro_fam) AS logradouro,
	nu_logradouro_fam AS numero,
	ds_complemento_fam AS complemento,
	no_localidade_fam AS bairro,
	nu_cep_logradouro_fam AS cep,
	cd_ibge_cadastro AS cod_ibge,
	null as municipio,
	null as uf,
FROM
	read_parquet('//storage6/bases/DADOS/RESTRITO/CADASTRO_UNICO/parquet/cad_familia_072024.parquet')
UNION
SELECT
	concat_ws(tipo_logradouro, logradouro) as logradouro,
	numero_estab as numero,
	complemento,
	bairro,
	cep,
	municipio as cod_ibge,
	null as municipio,
	null as uf,
FROM
	read_parquet('//storage6/bases/DADOS/PUBLICO/CNPJ/parquet/estabelecimentos/*.parquet');
")

campos <- correspondencia_campos(
  logradouro = "logradouro",
  numero = "numero",
  complemento = "complemento",
  cep = "cep",
  bairro = "bairro",
  municipio = "cod_ibge",
  estado = "uf"
)

resultado <- padronizar_enderecos(dados, campos)

write_parquet(resultado, "//storage6/usuarios/CGDTI/IpeaDataLab/projetos/2025_poc_enderecos/benchmark.parquet")
##
