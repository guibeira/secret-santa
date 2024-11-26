start:
	cargo watch -w src -x  run

start-debug:
	RUST_LOG=actix_web=debug cargo watch -w src -x  run


install-deps:
	rustup target add wasm32-unknown-unknown && cargo install --locked trunk

test:
	make install-deps && cd front && trunk build --release && cd .. && cargo tarpaulin --verbose --out Html

up:
	make install-deps && cd front && trunk build --release && cd .. && cargo run

build:
	make install-deps && cd front && trunk build --release && cd .. && cargo build --release

build-mac:
	make build && ./scripts/make_bundle_mac.sh

build-win:
	 cd front && trunk build --release && cd .. && cargo build --release
