format:
	cargo fmt --quite

lint:
	cargo clippy --quite

test:
	cargo test

build:
	cargo build

run:
	cargo run

release:
	cargo build --release
