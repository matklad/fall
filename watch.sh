#!/bin/sh
cargo run --bin gen --package fall_gen -- --examples $1

fswatch -o --event Updated $1 | xargs -n1 -I{} \
cargo run --bin gen --package fall_gen -- --examples $1
