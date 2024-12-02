
.PHONY: start start-debug install-deps test up build build-mac build-win

CARGO_WATCH = cargo watch -w src -x run
INSTALL_DEPS = rustup target add wasm32-unknown-unknown && cargo install --locked trunk
BUILD_FRONT = cd front && trunk build --release && cd ..

start:
	$(CARGO_WATCH)

start-debug:
	RUST_LOG=actix_web=debug $(CARGO_WATCH)

install-deps:
	$(INSTALL_DEPS)

test: install-deps
	$(BUILD_FRONT)
	cargo tarpaulin

up: install-deps
	$(BUILD_FRONT)
	cargo run

build: install-deps
	$(BUILD_FRONT)
	cargo build --release

build-mac: install-deps
	$(BUILD_FRONT)
	cargo bundle --release

build-win:
	$(BUILD_FRONT)
	cargo build --release
      

