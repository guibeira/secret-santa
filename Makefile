start:
	cargo watch -w src -x  run

start-debug:
	RUST_LOG=debug cargo watch -w src -x  run

test:
	cargo watch -w src -x  tarpaulin


install-deps:
	rustup target add wasm32-unknown-unknown && cargo install --locked trunk

up:
	make install-deps && cd front && trunk build --release && cd .. && cargo run
