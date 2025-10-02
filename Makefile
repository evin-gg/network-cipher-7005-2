all: build

build:
	cargo clean
	cargo build

server:
	cargo clean
	cargo build
	cargo run --bin server
