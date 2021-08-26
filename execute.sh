#! /bin/bash

cargo fmt
cargo build
cargo test
cargo clippy --tests -- -Dclippy::all -Dclippy::pedantic

