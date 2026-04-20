#!/bin/sh
cargo check && cargo test || exit 1

(cd target-tuple-pieces && cargo publish) || exit 1
(cd target-tuples-macro && cargo publish) || exit 1
cargo publish || exit 1