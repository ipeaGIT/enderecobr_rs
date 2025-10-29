.PHONY: build doc test testes-comparativos

ARQ_COMPARACAO = /mnt/storage6/usuarios/CGDTI/IpeaDataLab/projetos/2025_poc_enderecos/benchmark.parquet
BIN_TESTE_COMPARATIVO = target/release/teste-comparativo

# ==== Utilitários ====

build:
	cargo build --release

doc:
	cargo doc --no-deps --lib --release

test:
	cargo test

# ==== Testes comparativos com a implementação em R ====

testes-comparativos: diff/logr.csv diff/num.csv diff/comp.csv diff/bairro.csv diff/cep.csv

diff/logr.csv: $(BIN_TESTE_COMPARATIVO) $(ARQ_COMPARACAO)
	$(BIN_TESTE_COMPARATIVO) $(ARQ_COMPARACAO) --arquivo-saida $@ --tipo-padronizador logr --campo-bruto logradouro --campo-referencia logradouro_padr

diff/num.csv: $(BIN_TESTE_COMPARATIVO) $(ARQ_COMPARACAO)
	$(BIN_TESTE_COMPARATIVO) $(ARQ_COMPARACAO) --arquivo-saida $@ --tipo-padronizador num --campo-bruto numero --campo-referencia numero_padr

diff/comp.csv: $(BIN_TESTE_COMPARATIVO) $(ARQ_COMPARACAO)
	$(BIN_TESTE_COMPARATIVO) $(ARQ_COMPARACAO) --arquivo-saida $@ --tipo-padronizador comp --campo-bruto complemento --campo-referencia complemento_padr

diff/bairro.csv: $(BIN_TESTE_COMPARATIVO) $(ARQ_COMPARACAO)
	$(BIN_TESTE_COMPARATIVO) $(ARQ_COMPARACAO) --arquivo-saida $@ --tipo-padronizador bairro --campo-bruto bairro --campo-referencia bairro_padr

diff/cep.csv: $(BIN_TESTE_COMPARATIVO) $(ARQ_COMPARACAO)
	$(BIN_TESTE_COMPARATIVO) $(ARQ_COMPARACAO) --arquivo-saida $@ --tipo-padronizador cep --campo-bruto cep --campo-referencia cep_padr
