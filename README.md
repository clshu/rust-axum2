# Note
Use Rust Axum Full Course code as base to work on rust-web-app

# Rust Axum Full Course Source Code

This repository contains the source code for the Rust Axum Full Course, which is available on [YouTube](https://youtube.com/watch?v=XZtlD_m59sM&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q). The code is available under either the MIT or Apache license and is free to use.

Here is a [Per Chapter Fork](https://github.com/FloWi/rust-axum-course) by [@FloWi](https://github.com/FloWi). Big thanks!

## Dev (REPL)

```sh
# Terminal 1 - To run the server.
cargo watch -q -c -w src/ -x "run"

# Terminal 2 - To run the tests.
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```

## Dev

```sh
# Terminal 1 - To run the server.
cargo run

# Terminal 2 - To run the tests.
cargo test quick_dev -- --nocapture
```

## Notes

### Note 1

The `tests/quick_dev.rs` file has been moved to `examples/quick_dev.rs` (with the same code) as it is more appropriate and seems to resolve a Windows limitation when running `test` and `run` simultaneously.
