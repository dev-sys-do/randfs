.PHONY: randfs
randfs:
	cargo build

.PHONY: lint
lint:
	cargo clippy -- -D warnings

.PHONY: format
format:
	cargo fmt -- --check --config format_code_in_doc_comments=true

.PHONY: ci
ci: randfs lint format

.PHONY: clean
clean:
	cargo clean
