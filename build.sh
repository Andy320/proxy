#!/bin/bash
export CARGO_GIT="$HOME/.cargo/git"
export CARGO_REGISTRY="$HOME/.cargo/registry"

# stable rust, gcc 9.3, musl libc 1.1.19, llvm 10, full static linking
# target = "x86_64-unknown-linux-musl"
compiler=registry.cn-beijing.aliyuncs.com/andy320/dev-ubuntu18.04:v1.0.0

# stable rust, gcc 9, llvm 10, dynamic linking glibc 2.17
# target = "x86_64-unknown-linux-gnu"
#compiler=registry.cn-beijing.aliyuncs.com/andy320/dev-centos7:v1.0.0

mode=release
triples=x86_64-unknown-linux-musl
project_dir="$HOME/rust_projects/proxy"
output_dir="$project_dir/output"
build_dir="$project_dir/target/$triples/$mode"

echo "=== Building ==="

alias builder='docker run --rm -it -v "$project_dir":/root/rust/src -v "$CARGO_GIT":/root/.cargo/git -v "$CARGO_REGISTRY":/root/.cargo/registry "$compiler"'
shopt -s  expand_aliases
builder cargo build --"$mode"

if [ ! -d "$build_dir" ]
then
  echo "target build directory does NOT exist"
  exit
fi

if [ -d "$output_dir" ]
then
  rm -rf "${output_dir:?}/"*
else
  mkdir -p "$output_dir"
fi

cp "$build_dir"/router  "$output_dir"
cp "$build_dir"/handler "$output_dir"
cp "$build_dir"/broker  "$output_dir"

echo "=== Make docker images ==="
echo "delete local containers"
docker ps -a | grep -e 'Exited' -e 'router' -e 'handler' -e 'broker' | awk '{print $1}' | xargs docker stop | xargs docker rm

echo "delete local images"
docker images | grep -e 'router' -e 'handler' -e 'broker' | awk '{print $3}' | xargs docker rmi

echo "build local images"
docker build -f Dockerfile-router  -t registry.cn-beijing.aliyuncs.com/andy320/router:v1.0.0 .
docker build -f Dockerfile-handler -t registry.cn-beijing.aliyuncs.com/andy320/handler:v1.0.0 .
docker build -f Dockerfile-broker  -t registry.cn-beijing.aliyuncs.com/andy320/broker:v1.0.0 .

echo "push remote images"
docker push registry.cn-beijing.aliyuncs.com/andy320/router:v1.0.0
docker push registry.cn-beijing.aliyuncs.com/andy320/handler:v1.0.0
docker push registry.cn-beijing.aliyuncs.com/andy320/broker:v1.0.0
