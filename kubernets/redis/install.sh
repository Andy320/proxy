#!/bin/bash
kube_dir="$HOME/rust_projects/proxy/kubernets"
redis_dir="$kube_dir/redis"

if [ -d "$redis_dir" ]
then
  echo "installing a single redis instance"
  kubectl apply -f "$redis_dir"/redis.yaml
else
  echo "redis directory does not exist"
fi

echo "=== Done ==="