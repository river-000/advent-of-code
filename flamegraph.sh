#!/bin/sh
cargo install --path .
~/.cargo/bin/flamegraph -F 10000 -o my_flamegraph.svg ./target/release/advent-of-code
