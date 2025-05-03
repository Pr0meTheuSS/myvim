run:
	cargo run -- $(ARGS)

test:
	cargo test

build:
	cargo build

fmt:
	cargo fmt

coverage:
	cargo tarpaulin

coverage-html:
	cargo tarpaulin --out Html
