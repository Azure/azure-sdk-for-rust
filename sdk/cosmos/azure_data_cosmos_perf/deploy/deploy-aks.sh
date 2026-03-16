#!/usr/bin/env bash
# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# Deploys the Cosmos DB perf testing tool to AKS with Workload Identity.
#
# This is a thin orchestrator that calls individual setup scripts:
#   setup-cosmos-db.sh, setup-aks.sh, setup-identity-rbac.sh,
#   setup-k8s-identity.sh, deploy-k8s-job.sh, setup-adx.sh
#
# Usage:
#   ./deploy-aks.sh \
#     --cosmos-endpoint https://myaccount.documents.azure.com \
#     --cosmos-rg my-cosmos-rg \
#     --cosmos-account myaccount \
#     [--results-endpoint https://results.documents.azure.com] \
#     [--results-rg my-results-rg] \
#     [--results-account resultsaccount] \
#     [--results-auth aad] \
#     [--location eastus] \
#     [--parallelism 4] \
#     [--concurrency 200]
#
# Prerequisites:
#   - Azure CLI (az) logged in with appropriate permissions
#   - kubectl
#   - Docker (or ACR build)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Defaults
LOCATION="eastus"
RG="cosmos-perf-rg"
AKS_NAME="cosmos-perf-aks"
ACR_NAME="cosmosperfacr"
IDENTITY_NAME="cosmos-perf-identity"
NODE_SIZE="Standard_D4s_v5"
NODE_COUNT=2
PARALLELISM=4
CONCURRENCY=200
IMAGE_TAG="latest"
RESULTS_AUTH="aad"
CREATE_RESULTS_ACCOUNT=false
CREATE_WORKLOAD_ACCOUNT=false
RESULTS_ACCOUNT_NAME="cosmos-perf-results"
WORKLOAD_ACCOUNT_NAME="cosmos-perf-workload"
ADX_CLUSTER_NAME="cosmosperfadx"
ADX_DATABASE_NAME="PerfMetrics"
MULTI_WRITE=false
COSMOS_REGIONS=""
APPLICATION_REGION=""
POLL_BRANCH=""

# Required (unless --create-workload-account is used)
COSMOS_ENDPOINT=""
COSMOS_RG=""
COSMOS_ACCOUNT=""
RESULTS_ENDPOINT=""
RESULTS_RG=""
RESULTS_ACCOUNT=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --cosmos-endpoint)    COSMOS_ENDPOINT="$2"; shift 2 ;;
        --cosmos-rg)          COSMOS_RG="$2"; shift 2 ;;
        --cosmos-account)     COSMOS_ACCOUNT="$2"; shift 2 ;;
        --results-endpoint)   RESULTS_ENDPOINT="$2"; shift 2 ;;
        --results-rg)         RESULTS_RG="$2"; shift 2 ;;
        --results-account)    RESULTS_ACCOUNT="$2"; shift 2 ;;
        --results-auth)       RESULTS_AUTH="$2"; shift 2 ;;
        --create-results-account) CREATE_RESULTS_ACCOUNT=true; shift ;;
        --results-account-name)   RESULTS_ACCOUNT_NAME="$2"; shift 2 ;;
        --create-workload-account) CREATE_WORKLOAD_ACCOUNT=true; shift ;;
        --workload-account-name)   WORKLOAD_ACCOUNT_NAME="$2"; shift 2 ;;
        --adx-cluster-name)       ADX_CLUSTER_NAME="$2"; shift 2 ;;
        --adx-database-name)      ADX_DATABASE_NAME="$2"; shift 2 ;;
        --multi-write)            MULTI_WRITE=true; shift ;;
        --regions)                COSMOS_REGIONS="$2"; shift 2 ;;
        --regions=*)              COSMOS_REGIONS="${1#*=}"; shift ;;
        --application-region)     APPLICATION_REGION="$2"; shift 2 ;;
        --poll-branch)            POLL_BRANCH="$2"; shift 2 ;;
        --location)           LOCATION="$2"; shift 2 ;;
        --rg)                 RG="$2"; shift 2 ;;
        --aks-name)           AKS_NAME="$2"; shift 2 ;;
        --acr-name)           ACR_NAME="$2"; shift 2 ;;
        --node-size)          NODE_SIZE="$2"; shift 2 ;;
        --node-count)         NODE_COUNT="$2"; shift 2 ;;
        --parallelism)        PARALLELISM="$2"; shift 2 ;;
        --concurrency)        CONCURRENCY="$2"; shift 2 ;;
        --image-tag)          IMAGE_TAG="$2"; shift 2 ;;
        --help|-h)
            echo "Usage: $0 [options]"
            echo ""
            echo "Workload Cosmos DB account (provide existing OR create new):"
            echo "  --cosmos-endpoint URL          Cosmos DB workload endpoint (existing account)"
            echo "  --cosmos-rg RG                 Resource group of the Cosmos DB account"
            echo "  --cosmos-account NAME          Cosmos DB account name"
            echo "  --create-workload-account      Create a new Cosmos DB account for the workload"
            echo "  --workload-account-name NAME   Name for the new workload account (default: cosmos-perf-workload)"
            echo ""
            echo "Results account (optional, defaults to same as workload):"
            echo "  --results-endpoint URL         Cosmos DB results endpoint (use with existing account)"
            echo "  --results-rg RG                Resource group of the results account"
            echo "  --results-account NAME         Results Cosmos DB account name"
            echo "  --results-auth METHOD          Auth for results: aad (default) or key"
            echo "  --create-results-account       Create a new Cosmos DB account for results"
            echo "  --results-account-name NAME    Name for the new results account (default: cosmos-perf-results)"
            echo ""
            echo "Cosmos DB account options (apply to created accounts):"
            echo "  --multi-write                  Enable multi-region writes on created accounts"
            echo "  --regions R1,R2,...             Comma-separated Azure regions (default: --location value)"
            echo "                                 First region is primary. Example: \"East US,West US,North Europe\""
            echo ""
            echo "Infrastructure (optional):"
            echo "  --location REGION        Azure region (default: eastus)"
            echo "  --rg RG                  Resource group for AKS/ACR (default: cosmos-perf-rg)"
            echo "  --aks-name NAME          AKS cluster name (default: cosmos-perf-aks)"
            echo "  --acr-name NAME          ACR name (default: cosmosperfacr)"
            echo "  --node-size SKU          VM size (default: Standard_D4s_v5)"
            echo "  --node-count N           Node count (default: 2)"
            echo ""
            echo "Workload (optional):"
            echo "  --parallelism N          Number of parallel pods (default: 4)"
            echo "  --concurrency N          Concurrent operations per pod (default: 200)"
            echo "  --image-tag TAG          Docker image tag (default: latest)"
            echo "  --poll-branch BRANCH     Deploy a CronJob that polls this branch every 12h,"
            echo "                           rebuilds and restarts pods on new commits"
            echo ""
            echo "Kusto / Azure Data Explorer (optional):"
            echo "  --adx-cluster-name NAME  ADX cluster name (default: cosmosperfadx)"
            echo "  --adx-database-name NAME ADX database name (default: PerfMetrics)"
            exit 0
            ;;
        *) echo "Unknown option: $1" >&2; exit 1 ;;
    esac
done

# Default application region to location if not specified
if [[ -z "$APPLICATION_REGION" ]]; then
    APPLICATION_REGION="$LOCATION"
fi

# Handle workload account: create or validate existing
if [[ "$CREATE_WORKLOAD_ACCOUNT" == true ]]; then
    COSMOS_ACCOUNT="$WORKLOAD_ACCOUNT_NAME"
    COSMOS_RG="$RG"
    COSMOS_ENDPOINT="https://${COSMOS_ACCOUNT}.documents.azure.com:443/"
elif [[ -z "$COSMOS_ENDPOINT" || -z "$COSMOS_RG" || -z "$COSMOS_ACCOUNT" ]]; then
    echo "Error: Either use --create-workload-account, or provide --cosmos-endpoint, --cosmos-rg, and --cosmos-account." >&2
    echo "Run '$0 --help' for usage." >&2
    exit 1
fi

# Default results to workload account if not specified and not creating a new one
if [[ "$CREATE_RESULTS_ACCOUNT" == true ]]; then
    RESULTS_ACCOUNT="$RESULTS_ACCOUNT_NAME"
    RESULTS_RG="$RG"
    RESULTS_ENDPOINT="https://${RESULTS_ACCOUNT}.documents.azure.com:443/"
elif [[ -z "$RESULTS_ENDPOINT" ]]; then
    RESULTS_ENDPOINT="$COSMOS_ENDPOINT"
    RESULTS_RG="$COSMOS_RG"
    RESULTS_ACCOUNT="$COSMOS_ACCOUNT"
fi

echo "=== Cosmos DB Perf Tool — AKS Deployment ==="
echo "  Location:         $LOCATION"
echo "  Resource group:   $RG"
echo "  AKS cluster:      $AKS_NAME"
echo "  ACR:              $ACR_NAME"
echo "  Cosmos endpoint:  $COSMOS_ENDPOINT"
echo "  Results endpoint: $RESULTS_ENDPOINT"
echo "  Parallelism:      $PARALLELISM pods"
echo "  Concurrency:      $CONCURRENCY ops/pod"
echo ""

# Build common Cosmos DB args for account creation
COSMOS_COMMON_ARGS=()
if [[ "$MULTI_WRITE" == true ]]; then
    COSMOS_COMMON_ARGS+=(--multi-write)
fi
if [[ -n "$COSMOS_REGIONS" ]]; then
    COSMOS_COMMON_ARGS+=(--regions "$COSMOS_REGIONS")
fi

# --- Step 1/10: Resource Group ---
echo "--- Step 1/10: Creating resource group ---"
az group create --name "$RG" --location "$LOCATION" --output none

# --- Step 2/10: Create Cosmos DB Accounts (if requested) ---
if [[ "$CREATE_WORKLOAD_ACCOUNT" == true ]]; then
    echo "--- Step 2/10: Creating workload Cosmos DB account '$COSMOS_ACCOUNT' ---"
    "$SCRIPT_DIR/setup-cosmos-db.sh" \
        --account-name "$COSMOS_ACCOUNT" \
        --resource-group "$RG" \
        --location "$LOCATION" \
        --database-name perfdb \
        --container-name perfcontainer \
        --partition-key-path "/partition_key" \
        --throughput 100000 \
        "${COSMOS_COMMON_ARGS[@]+"${COSMOS_COMMON_ARGS[@]}"}"
    COSMOS_ENDPOINT="https://${COSMOS_ACCOUNT}.documents.azure.com:443/"
    COSMOS_RG="$RG"
    echo "  Workload endpoint: $COSMOS_ENDPOINT"
fi

if [[ "$CREATE_RESULTS_ACCOUNT" == true ]]; then
    echo "--- Step 2/10: Creating results Cosmos DB account '$RESULTS_ACCOUNT' ---"
    "$SCRIPT_DIR/setup-cosmos-db.sh" \
        --account-name "$RESULTS_ACCOUNT" \
        --resource-group "$RG" \
        --location "$LOCATION" \
        --database-name perfdb \
        --container-name perfresults \
        --partition-key-path "/partition_key" \
        --throughput 10000 \
        --ttl 86400 \
        "${COSMOS_COMMON_ARGS[@]+"${COSMOS_COMMON_ARGS[@]}"}"
    RESULTS_ENDPOINT="https://${RESULTS_ACCOUNT}.documents.azure.com:443/"
    echo "  Results endpoint: $RESULTS_ENDPOINT"
else
    echo "--- Step 2/10: Skipping results account creation (using existing) ---"
fi

# --- Steps 3-4/10: ACR + AKS ---
echo "--- Step 3/10: Creating container registry ---"
echo "--- Step 4/10: Creating AKS cluster (this may take several minutes) ---"
"$SCRIPT_DIR/setup-aks.sh" \
    --aks-name "$AKS_NAME" \
    --acr-name "$ACR_NAME" \
    --resource-group "$RG" \
    --location "$LOCATION" \
    --node-size "$NODE_SIZE" \
    --node-count "$NODE_COUNT"

# --- Step 5/10: Managed Identity + RBAC ---
echo "--- Step 5/10: Setting up managed identity and RBAC ---"
COSMOS_ACCOUNTS_PAIRS="${COSMOS_ACCOUNT}:${COSMOS_RG}"
if [[ "$RESULTS_ACCOUNT" != "$COSMOS_ACCOUNT" ]]; then
    COSMOS_ACCOUNTS_PAIRS="${COSMOS_ACCOUNTS_PAIRS},${RESULTS_ACCOUNT}:${RESULTS_RG}"
fi

IDENTITY_OUTPUT=$("$SCRIPT_DIR/setup-identity-rbac.sh" \
    --identity-name "$IDENTITY_NAME" \
    --resource-group "$RG" \
    --cosmos-accounts "$COSMOS_ACCOUNTS_PAIRS")

# Parse stdout output from setup-identity-rbac.sh
IDENTITY_CLIENT_ID=$(echo "$IDENTITY_OUTPUT" | grep "^CLIENT_ID=" | cut -d= -f2)

# --- Step 6/10: Federated Credential ---
echo "--- Step 6/10: Setting up workload identity federation ---"
"$SCRIPT_DIR/setup-k8s-identity.sh" \
    --identity-name "$IDENTITY_NAME" \
    --identity-client-id "$IDENTITY_CLIENT_ID" \
    --resource-group "$RG" \
    --aks-name "$AKS_NAME"

# --- Steps 7-8/10: Build + Deploy ---
echo "--- Step 7/10: Building and pushing Docker image ---"
echo "--- Step 8/10: Deploying perf job to AKS ---"

DEPLOY_JOB_ARGS=(
    --acr-name "$ACR_NAME"
    --image-tag "$IMAGE_TAG"
    --cosmos-endpoint "$COSMOS_ENDPOINT"
    --results-endpoint "$RESULTS_ENDPOINT"
    --results-auth "$RESULTS_AUTH"
    --application-region "$APPLICATION_REGION"
    --parallelism "$PARALLELISM"
    --concurrency "$CONCURRENCY"
    --identity-client-id "$IDENTITY_CLIENT_ID"
    --resource-group "$RG"
)
if [[ -n "$POLL_BRANCH" ]]; then
    DEPLOY_JOB_ARGS+=(--poll-branch "$POLL_BRANCH")
fi
"$SCRIPT_DIR/deploy-k8s-job.sh" "${DEPLOY_JOB_ARGS[@]}"

# --- Steps 9-10/10: ADX ---
echo "--- Step 9/10: Creating Azure Data Explorer cluster and tables ---"
echo "--- Step 10/10: Creating Cosmos DB change feed connections to ADX ---"
ADX_OUTPUT=$("$SCRIPT_DIR/setup-adx.sh" \
    --cluster-name "$ADX_CLUSTER_NAME" \
    --database-name "$ADX_DATABASE_NAME" \
    --resource-group "$RG" \
    --location "$LOCATION" \
    --results-cosmos-account "$RESULTS_ACCOUNT" \
    --results-cosmos-rg "$RESULTS_RG")

ADX_URI=$(echo "$ADX_OUTPUT" | grep "^ADX_URI=" | cut -d= -f2)

echo ""
echo "=== Deployment complete ==="
echo ""
echo "Monitor with:"
echo "  kubectl get pods -n cosmos-perf -w"
echo "  kubectl logs -n cosmos-perf -l app=cosmos-perf -f"
echo ""
echo "Kusto:"
echo "  Cluster: $ADX_URI"
echo "  Database: $ADX_DATABASE_NAME"
echo "  Tables: PerfResults, ErrorResults"
echo ""
echo "Cleanup with:"
echo "  kubectl delete job cosmos-perf -n cosmos-perf"
echo "  az group delete --name $RG --yes --no-wait"
