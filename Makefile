.PHONY: run build

run:
	cargo watch -x run -w src

build:
	cargo build
