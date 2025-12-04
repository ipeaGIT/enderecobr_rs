.PHONY: build doc test testes-comparativos

DIR_DATASETS = ./datasets
DIR_SNAPSHOT = ./datasets/dados/snapshot_test

# ==== Utilitários ====

build:
	cargo build --release

build-lto:
	cargo build --profile release-lto --all-features

doc:
	cargo doc --no-deps --lib --release --all-features

test:
	cargo test

snapshot-test: snapshot-download
	cargo run --release -p snapshot -- $(DIR_SNAPSHOT)

snapshot-criar:
	cargo run --release -p snapshot -- -s $(DIR_SNAPSHOT)

snapshot-upload:
	$(MAKE) -C $(DIR_DATASETS) hf-publicar-snapshot-test

snapshot-download:
	$(MAKE) -C $(DIR_DATASETS) snapshot-download

snapshot-download-force:
	$(MAKE) -C $(DIR_DATASETS) -B snapshot-download
