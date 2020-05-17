#!/bin/bash
export CARGO_GIT="$HOME/.cargo/git"
export CARGO_REGISTRY="$HOME/.cargo/registry"

base_image=registry.cn-beijing.aliyuncs.com/andy320/dev-ubuntu18.04:v1.0.0
output_dir=dockerfiles/build
bin_dir=target/x86_64-unknown-linux-musl/release

alias builder='docker run --rm -it -v "$(pwd)":/root/rust/src -v "$CARGO_GIT":/root/.cargo/git -v "$CARGO_REGISTRY":/root/.cargo/registry "$base_image"'

shopt -s  expand_aliases

builder cargo build --release

mkdir -p $output_dir && rm -rf $output_dir/* && cp $bin_dir/router $output_dir


