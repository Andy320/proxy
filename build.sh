#!/bin/bash
export CARGO_GIT="$HOME/.cargo/git"
export CARGO_REGISTRY="$HOME/.cargo/registry"

# stable rust, gcc 9.3, musl libc 1.1.19, llvm 10, full static linking
# target = "x86_64-unknown-linux-musl"
compiler=registry.cn-beijing.aliyuncs.com/andy320/dev-ubuntu18.04:v1.0.0

# stable rust, gcc 9, llvm 10, dynamic linking glibc 2.17
# target = "x86_64-unknown-linux-gnu"
#compiler=registry.cn-beijing.aliyuncs.com/andy320/dev-centos7:v1.0.0

output_dir=output
bin_dir=target/x86_64-unknown-linux-musl/release

alias builder='docker run --rm -it -v "$(pwd)":/root/rust/src -v "$CARGO_GIT":/root/.cargo/git -v "$CARGO_REGISTRY":/root/.cargo/registry "$compiler"'

shopt -s  expand_aliases

builder cargo build --release

mkdir -p $output_dir && rm -rf $output_dir/*
cp $bin_dir/router  $output_dir
cp $bin_dir/handler $output_dir
cp $bin_dir/broker  $output_dir


