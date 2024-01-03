#!/bin/bash

cd rust
cargo build
cd ..

./rust/target/debug/bayes-star
