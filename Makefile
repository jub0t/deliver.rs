build:
	cargo build --release

run:
	./target/release/cdn

lint:
	cargo clippy --fix
