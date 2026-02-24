#!/bin/bash

set -e

REPO_URL="https://github.com/apache/datafusion-ballista.git"
TARGET_DIR="deployment/datafusion-ballista"
DOCKER_REGISTRY="ballista-local"

echo "[1/8] Repo is cloning: $REPO_URL"
if [ ! -d "$TARGET_DIR" ]; then
    git clone "$REPO_URL" "$TARGET_DIR"
else
    echo "Directory already exists, skipping clone"
fi

echo "[2/8] Action: Building Ballista Docker Images"
cd "$TARGET_DIR"
./dev/build-ballista-docker.sh
cd - > /dev/null

echo "[3/8] Action: Building Custom Rust Tools (Data-Gen & Query-Runner)"
docker tag apache/datafusion-ballista-scheduler:latest "$DOCKER_REGISTRY/ballista-scheduler:latest"
docker tag apache/datafusion-ballista-executor:latest "$DOCKER_REGISTRY/ballista-executor:latest"

echo "[4/8] Building Rust Bins: $TAG"
docker build -t ballista-tools:v1 .

echo "[5/8] Action: Deploying Ballista Cluster via Helm"
helm upgrade --install ballista-cluster deployment/ballista

echo "[6/8] Status: Waiting for Scheduler to be Ready"
kubectl wait --for=condition=ready pod -l app=ballista-scheduler --timeout=60s

echo "[7/8] Action: Creating Test Data"
kubectl delete job ballista-data-generator --ignore-not-found
kubectl apply -f deployment/data-gen-job.yaml

echo "[...] Status: Waiting for data-generator to complete (Timeout: 10m)"
if ! kubectl wait --for=condition=complete job/ballista-data-generator --timeout=600s; then
    echo "[ERROR] Data generation failed or timed out!"
    kubectl logs job/ballista-data-generator
    exit 1
fi

echo "[8/8] Action: Launching Query Runner"
kubectl delete pod query-runner --ignore-not-found
kubectl apply -f deployment/query-runner.yaml

echo "----------------------------------------------------------"
echo "[SUCCESS] Pipeline Finished Successfully."
echo "Check Query Runner logs: kubectl logs -f query-runner"