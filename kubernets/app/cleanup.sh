#!/bin/bash
kube_dir="$HOME/rust_projects/proxy/kubernets"
app_dir="$kube_dir/app"

cd "$app_dir" || exit
echo "deleting..."
kubectl delete -f virtual-service-normal.yaml
kubectl delete -f destinations.yaml
kubectl delete -f proxy-gateway.yaml
kubectl delete -f applications.yaml
kubectl delete -f configmap-router.yaml
kubectl delete -f configmap-handler.yaml
kubectl delete -f configmap-broker.yaml
kubectl delete -f configmap-common.yaml

echo '===Done!==='


