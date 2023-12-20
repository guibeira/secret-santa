start:
	cargo watch -w src -x  run

start-debug:
	RUST_LOG=debug cargo watch -w src -x  run

test:
	cargo watch -w src -x  tarpaulin

