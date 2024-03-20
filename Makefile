build:
	cargo build --release --verbose

run:
	./target/release/cdn

lint:
	cargo clippy --fix
