#!/bin/bash

cd $HOME/rust_projects/proxy/kubernets/

kubectl delete -f virtual-service-normal.yaml
kubectl delete -f destinations.yaml
kubectl delete -f proxy-gateway.yaml
kubectl delete -f applications.yaml
kubectl delete -f configmap-router.yaml
kubectl delete -f configmap-handler.yaml
kubectl delete -f configmap-broker.yaml
kubectl delete -f configmap-common.yaml

echo '===Done!==='


