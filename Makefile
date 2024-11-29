start:
	cargo watch -w src -x  run

start-debug:
	RUST_LOG=actix_web=debug cargo watch -w src -x  run


install-deps:
	rustup target add wasm32-unknown-unknown && cargo install --locked trunk

test:
	make install-deps && cd front && trunk build --release && cd .. && cargo tarpaulin

up:
	make install-deps && cd front && trunk build --release && cd .. && cargo run

build:
	make install-deps && cd front && trunk build --release && cd .. && cargo build --release

build-mac:
	make install-deps && cd front && trunk build --release && cd .. && cargo bundle --release

build-win:
	 cd front && trunk build --release && cd .. && cargo build --release
