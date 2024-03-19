master:
	cargo build --release --color never --quiet

run:
	./target/release/cdn

lint:
	cargo clippy --fix