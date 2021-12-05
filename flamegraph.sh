#!/bin/sh
cargo install --path .
~/.cargo/bin/flamegraph -o my_flamegraph.svg ./target/release/advent-of-code
