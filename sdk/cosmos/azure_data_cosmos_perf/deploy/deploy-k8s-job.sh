#!/usr/bin/env bash
# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# Builds Docker image and deploys the perf K8s Job (and optional CronJob).
#
# Usage:
#   ./deploy-k8s-job.sh \
#     --acr-name cosmosperfacr \
#     --image-tag latest \
#     --cosmos-endpoint https://... \
#     --results-endpoint https://... \
#     --results-auth aad \
#     --application-region eastus \
#     --parallelism 4 \
#     --concurrency 200 \
#     --identity-client-id <id> \
#     --resource-group my-rg \
#     [--poll-branch main]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"

ACR_NAME=""
IMAGE_TAG="latest"
COSMOS_ENDPOINT=""
RESULTS_ENDPOINT=""
RESULTS_AUTH="aad"
APPLICATION_REGION=""
PARALLELISM=4
CONCURRENCY=200
IDENTITY_CLIENT_ID=""
RESOURCE_GROUP=""
POLL_BRANCH=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --acr-name)           ACR_NAME="$2"; shift 2 ;;
        --image-tag)          IMAGE_TAG="$2"; shift 2 ;;
        --cosmos-endpoint)    COSMOS_ENDPOINT="$2"; shift 2 ;;
        --results-endpoint)   RESULTS_ENDPOINT="$2"; shift 2 ;;
        --results-auth)       RESULTS_AUTH="$2"; shift 2 ;;
        --application-region) APPLICATION_REGION="$2"; shift 2 ;;
        --parallelism)        PARALLELISM="$2"; shift 2 ;;
        --concurrency)        CONCURRENCY="$2"; shift 2 ;;
        --identity-client-id) IDENTITY_CLIENT_ID="$2"; shift 2 ;;
        --resource-group)     RESOURCE_GROUP="$2"; shift 2 ;;
        --poll-branch)        POLL_BRANCH="$2"; shift 2 ;;
        --help|-h)
            echo "Usage: $0 [options]"
            echo ""
            echo "Builds Docker image, deploys perf Job, and optional polling CronJob."
            echo ""
            echo "Required:"
            echo "  --acr-name NAME           ACR name"
            echo "  --cosmos-endpoint URL     Cosmos DB workload endpoint"
            echo "  --results-endpoint URL    Cosmos DB results endpoint"
            echo "  --identity-client-id ID   Managed identity client ID"
            echo "  --resource-group RG       Resource group (for CronJob)"
            echo ""
            echo "Optional:"
            echo "  --image-tag TAG           Docker image tag (default: latest)"
            echo "  --results-auth METHOD     Auth for results: aad or key (default: aad)"
            echo "  --application-region REG  Application region"
            echo "  --parallelism N           Number of parallel pods (default: 4)"
            echo "  --concurrency N           Concurrent ops per pod (default: 200)"
            echo "  --poll-branch BRANCH      Deploy CronJob polling this branch every 12h"
            exit 0
            ;;
        *) echo "Unknown option: $1" >&2; exit 1 ;;
    esac
done

if [[ -z "$ACR_NAME" || -z "$COSMOS_ENDPOINT" || -z "$RESULTS_ENDPOINT" || -z "$IDENTITY_CLIENT_ID" || -z "$RESOURCE_GROUP" ]]; then
    echo "Error: --acr-name, --cosmos-endpoint, --results-endpoint, --identity-client-id, and --resource-group are required." >&2
    exit 1
fi

# --- Build and push Docker image ---
echo "  Creating build context..."
BUILD_CONTEXT=$(mktemp -d /tmp/cosmos-perf-context-XXXXXX)
trap 'rm -rf "$BUILD_CONTEXT"' EXIT
rsync -a \
    --exclude='target' \
    --exclude='.git' \
    --exclude='.github' \
    --exclude='.devcontainer' \
    --exclude='.vscode' \
    --exclude='doc' \
    --exclude='eng' \
    --exclude='profile.json' \
    --exclude='**/tests/' \
    --exclude='**/assets.json' \
    "$REPO_ROOT/" "$BUILD_CONTEXT/"

echo "  Context size: $(du -sh "$BUILD_CONTEXT" | cut -f1)"
az acr build \
    --registry "$ACR_NAME" \
    --image "cosmos-perf:$IMAGE_TAG" \
    --file "$BUILD_CONTEXT/sdk/cosmos/azure_data_cosmos_perf/Dockerfile" \
    "$BUILD_CONTEXT"

# --- Deploy Job ---
COMMIT_SHA=$(git -C "$REPO_ROOT" rev-parse --short HEAD 2>/dev/null || echo "unknown")
export ACR_NAME IMAGE_TAG COSMOS_ENDPOINT RESULTS_ENDPOINT RESULTS_AUTH APPLICATION_REGION COMMIT_SHA
export PARALLELISM CONCURRENCY IDENTITY_CLIENT_ID

# Delete existing job first — K8s Jobs are immutable once created
kubectl delete job cosmos-perf -n cosmos-perf 2>/dev/null || true
envsubst < "$SCRIPT_DIR/perf-job.yaml" | kubectl apply -f -

# --- Deploy CronJob for branch polling (optional) ---
if [[ -n "$POLL_BRANCH" ]]; then
    echo "  Deploying branch polling CronJob (branch: $POLL_BRANCH, every 12h)..."
    export POLL_BRANCH
    export RG="$RESOURCE_GROUP"

    # Create the updater script as a ConfigMap
    kubectl delete configmap perf-updater-script -n cosmos-perf 2>/dev/null || true
    kubectl create configmap perf-updater-script -n cosmos-perf \
        --from-file=updater.sh="$SCRIPT_DIR/updater.sh"

    # Deploy the CronJob
    kubectl delete cronjob cosmos-perf-updater -n cosmos-perf 2>/dev/null || true
    envsubst < "$SCRIPT_DIR/poll-cronjob.yaml" | grep -v "UPDATER_SCRIPT_PLACEHOLDER" | kubectl apply -f -
    echo "  CronJob deployed. Next run in ~12 hours."
fi

echo "  K8s job deployment complete."
