all: build

build:
	cargo clean
	cargo build
	cargo run --bin server
