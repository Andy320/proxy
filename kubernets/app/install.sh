#!/bin/bash
kube_dir="$HOME/rust_projects/proxy/kubernets"
app_dir="$kube_dir/app"

if [ -d "$app_dir" ]
then
  echo "deleting old apps..."
  "$app_dir"/cleanup.sh
  sleep 10

  echo "installing apps..."
  kubectl apply -f "$app_dir"/configmap-common.yaml
  kubectl apply -f "$app_dir"/configmap-router.yaml
  kubectl apply -f "$app_dir"/configmap-handler.yaml
  kubectl apply -f "$app_dir"/configmap-broker.yaml
  kubectl apply -f "$app_dir"/applications.yaml
  kubectl apply -f "$app_dir"/destinations.yaml
  kubectl apply -f "$app_dir"/proxy-gateway.yaml
  kubectl apply -f "$app_dir"/virtual-service-normal.yaml
else
  echo "app directory does not exist"
fi

echo "=== Done ==="