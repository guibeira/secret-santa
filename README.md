# Secret Santa

This is one experiment to not use database, and only use Rust (backend and frontend)

https://github.com/guibeira/secret-santa/assets/10093193/391cd3ed-e76f-4739-a8a1-22ad3359b2f8

## Installation

This project uses Rust, make sure you have it [installed](https://rustup.rs)

First install the WebAssembly Target

```bash
rustup target add wasm32-unknown-unknown
```

Then install Trunk

```bash
cargo install --locked trunk
```

the Backend uses Watch to refresh the server,

```bash
cargo install cargo-watch
```

With all the dependencies installed, it's time to run our frontend. In the front folder, run:

```bash
trunk serve
```

Open another terminal, and in the root folder, run:

```bash
make start
```

Now in your browser access [localhost:8080](http://localhost:8080)

## Test

```bash
cargo test
```

This project I aim to use with my friends, and most of the then are Brazilian, for this reason, all the messages are in Portuguese.
If you know how to add translations to this project, feel free to open a pull request.
