#!/usr/bin/env bash
# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# Updater script for the Cosmos DB perf CronJob.
# Checks a remote branch for new commits, rebuilds the Docker image,
# and restarts the perf Job with the new commit SHA.
#
# Expected environment variables (set by the CronJob):
#   POLL_BRANCH         - Remote branch to poll (e.g., release/azure_data_cosmos-previews)
#   REPO_URL            - Git repo URL
#   ACR_NAME            - Azure Container Registry name
#   IMAGE_TAG           - Docker image tag
#   RESOURCE_GROUP      - Azure resource group
#   PERF_NAMESPACE      - K8s namespace for perf pods
#   COSMOS_ENDPOINT     - Cosmos DB workload endpoint
#   RESULTS_ENDPOINT    - Cosmos DB results endpoint
#   RESULTS_AUTH        - Auth method for results
#   APPLICATION_REGION  - Application region for routing
#   CONCURRENCY         - Concurrent operations per pod
#   PARALLELISM         - Number of parallel pods
#   IDENTITY_CLIENT_ID  - Managed identity client ID

set -euo pipefail

echo "=== Cosmos Perf Updater — $(date -u '+%Y-%m-%dT%H:%M:%SZ') ==="

# Get the current commit SHA from the running job (if any)
CURRENT_SHA=$(kubectl get job cosmos-perf -n "$PERF_NAMESPACE" \
    -o jsonpath='{.spec.template.spec.containers[0].args}' 2>/dev/null \
    | grep -oP '(?<=--commit-sha",")[^"]+' || echo "none")

echo "  Current commit: $CURRENT_SHA"

# Clone the repo (shallow) and check the branch
WORK_DIR=$(mktemp -d)
git clone --depth 1 --branch "$POLL_BRANCH" "$REPO_URL" "$WORK_DIR/repo" 2>/dev/null
REMOTE_SHA=$(git -C "$WORK_DIR/repo" rev-parse --short HEAD)

echo "  Remote commit:  $REMOTE_SHA"

if [[ "$REMOTE_SHA" == "$CURRENT_SHA" ]]; then
    echo "  No new commits. Nothing to do."
    rm -rf "$WORK_DIR"
    exit 0
fi

echo "  New commit detected: $CURRENT_SHA -> $REMOTE_SHA"

# Build and push new Docker image
echo "  Building Docker image..."

# Create minimal build context (exclude target/, .git/)
BUILD_CONTEXT="$WORK_DIR/context"
rsync -a \
    --exclude='target' \
    --exclude='.git' \
    --exclude='.github' \
    --exclude='.devcontainer' \
    --exclude='.vscode' \
    --exclude='doc' \
    --exclude='eng' \
    "$WORK_DIR/repo/" "$BUILD_CONTEXT/"

az acr build \
    --registry "$ACR_NAME" \
    --image "cosmos-perf:$IMAGE_TAG" \
    --file "$BUILD_CONTEXT/sdk/cosmos/azure_data_cosmos_perf/Dockerfile" \
    "$BUILD_CONTEXT" \
    --output none

echo "  Image built and pushed."

# Restart the perf Job with the new commit SHA
echo "  Restarting perf job..."
kubectl delete job cosmos-perf -n "$PERF_NAMESPACE" 2>/dev/null || true

# Apply the job manifest from the repo template, substituting env vars
export COMMIT_SHA="$REMOTE_SHA"
export ACR_NAME IMAGE_TAG PERF_NAMESPACE COSMOS_ENDPOINT RESULTS_ENDPOINT RESULTS_AUTH
export APPLICATION_REGION CONCURRENCY PARALLELISM IDENTITY_CLIENT_ID
TEMPLATE="$WORK_DIR/repo/sdk/cosmos/azure_data_cosmos_perf/deploy/perf-job.yaml"
envsubst < "$TEMPLATE" | kubectl apply -f -

echo "  Job restarted with commit $REMOTE_SHA"
rm -rf "$WORK_DIR"
echo "=== Update complete ==="
