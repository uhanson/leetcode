#!/bin/bash 

mkdir -p $1
cd $1
cargo new --name $1 rust/
cd rust
cargo build
cd ../..
