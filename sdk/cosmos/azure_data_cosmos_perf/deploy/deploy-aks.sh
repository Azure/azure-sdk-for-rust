#!/usr/bin/env bash
# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# Deploys the Cosmos DB perf testing tool to AKS with Workload Identity.
#
# This script creates all required Azure infrastructure from scratch:
#   - Resource group, ACR, AKS cluster (with OIDC + workload identity)
#   - Managed identity with Cosmos DB RBAC roles
#   - K8s namespace, service account, federated credential
#   - Builds and pushes the Docker image
#   - Deploys the K8s Job
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
REPO_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"

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
            echo ""
            echo "Kusto / Azure Data Explorer (optional):"
            echo "  --adx-cluster-name NAME  ADX cluster name (default: cosmosperfadx)"
            echo "  --adx-database-name NAME ADX database name (default: PerfMetrics)"
            exit 0
            ;;
        *) echo "Unknown option: $1" >&2; exit 1 ;;
    esac
done

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

# --- Step 1/10: Resource Group ---
echo "--- Step 1/10: Creating resource group ---"
az group create --name "$RG" --location "$LOCATION" --output none

# --- Step 2/10: Create Cosmos DB Accounts (if requested) ---

# Shared function to create a Cosmos DB account with local auth disabled
create_cosmos_account() {
    local account_name="$1"
    local account_rg="$2"
    local account_location="$3"

    if az cosmosdb show --name "$account_name" --resource-group "$account_rg" &>/dev/null; then
        echo "  Cosmos DB account '$account_name' already exists."
        return 0
    fi

    # Build the locations JSON array
    local locations_json=""
    if [[ -n "$COSMOS_REGIONS" ]]; then
        local priority=0
        IFS=',' read -ra REGION_ARRAY <<< "$COSMOS_REGIONS"
        for region in "${REGION_ARRAY[@]}"; do
            region=$(echo "$region" | xargs) # trim whitespace
            if [[ -n "$locations_json" ]]; then
                locations_json="${locations_json},"
            fi
            locations_json="${locations_json}{\"locationName\":\"${region}\",\"failoverPriority\":${priority}}"
            priority=$((priority + 1))
        done
    else
        locations_json="{\"locationName\":\"${account_location}\",\"failoverPriority\":0}"
    fi

    local multi_write_json="false"
    if [[ "$MULTI_WRITE" == true ]]; then
        multi_write_json="true"
    fi

    echo "  Creating Cosmos DB account '$account_name' (this may take a few minutes)..."
    if [[ "$MULTI_WRITE" == true ]]; then
        echo "  Multi-region writes: enabled"
    fi
    if [[ -n "$COSMOS_REGIONS" ]]; then
        echo "  Regions: $COSMOS_REGIONS"
    fi

    TEMPLATE_FILE=$(mktemp /tmp/cosmos-arm-XXXXXX.json)
    cat > "$TEMPLATE_FILE" <<ARMEOF
{
  "\$schema": "https://schema.management.azure.com/schemas/2019-04-01/deploymentTemplate.json#",
  "contentVersion": "1.0.0.0",
  "resources": [{
    "type": "Microsoft.DocumentDB/databaseAccounts",
    "apiVersion": "2024-05-15",
    "name": "${account_name}",
    "location": "${account_location}",
    "kind": "GlobalDocumentDB",
    "properties": {
      "databaseAccountOfferType": "Standard",
      "disableLocalAuth": true,
      "enableMultipleWriteLocations": ${multi_write_json},
      "consistencyPolicy": { "defaultConsistencyLevel": "Session" },
      "locations": [${locations_json}]
    }
  }]
}
ARMEOF
    az deployment group create \
        --resource-group "$account_rg" \
        --template-file "$TEMPLATE_FILE" \
        --output none
    rm -f "$TEMPLATE_FILE"
}

if [[ "$CREATE_WORKLOAD_ACCOUNT" == true ]]; then
    echo "--- Step 2/10: Creating workload Cosmos DB account '$COSMOS_ACCOUNT' ---"
    create_cosmos_account "$COSMOS_ACCOUNT" "$RG" "$LOCATION"
    COSMOS_ENDPOINT="https://${COSMOS_ACCOUNT}.documents.azure.com:443/"
    COSMOS_RG="$RG"
    echo "  Workload endpoint: $COSMOS_ENDPOINT"
    echo "  Creating workload database 'perfdb'..."
    az cosmosdb sql database create --account-name "$COSMOS_ACCOUNT" --resource-group "$RG" \
        --name perfdb --output none 2>/dev/null || echo "  Database 'perfdb' already exists."
    echo "  Creating workload container 'perfcontainer'..."
    az cosmosdb sql container create --account-name "$COSMOS_ACCOUNT" --resource-group "$RG" \
        --database-name perfdb --name perfcontainer --partition-key-path "/partition_key" \
        --throughput 100000 --output none 2>/dev/null || echo "  Container 'perfcontainer' already exists."
fi

if [[ "$CREATE_RESULTS_ACCOUNT" == true ]]; then
    echo "--- Step 2/10: Creating results Cosmos DB account '$RESULTS_ACCOUNT' ---"
    create_cosmos_account "$RESULTS_ACCOUNT" "$RG" "$LOCATION"
    RESULTS_ENDPOINT="https://${RESULTS_ACCOUNT}.documents.azure.com:443/"
    echo "  Results endpoint: $RESULTS_ENDPOINT"
    echo "  Creating results database 'perfdb'..."
    az cosmosdb sql database create --account-name "$RESULTS_ACCOUNT" --resource-group "$RG" \
        --name perfdb --output none 2>/dev/null || echo "  Database 'perfdb' already exists."
    echo "  Creating results container 'perfresults'..."
    az cosmosdb sql container create --account-name "$RESULTS_ACCOUNT" --resource-group "$RG" \
        --database-name perfdb --name perfresults --partition-key-path "/partition_key" \
        --throughput 10000 --ttl 86400 --output none 2>/dev/null || echo "  Container 'perfresults' already exists."
else
    echo "--- Step 2/10: Skipping results account creation (using existing) ---"
fi

# --- Step 3/10: Container Registry ---
echo "--- Step 3/10: Creating container registry ---"
az acr create --name "$ACR_NAME" --resource-group "$RG" --sku Basic --admin-enabled true --output none 2>/dev/null || \
    echo "  ACR '$ACR_NAME' already exists."

# --- Step 4/10: AKS Cluster ---
echo "--- Step 4/10: Creating AKS cluster (this may take several minutes) ---"
if az aks show --name "$AKS_NAME" --resource-group "$RG" &>/dev/null; then
    echo "  AKS cluster '$AKS_NAME' already exists."
else
    az aks create \
        --resource-group "$RG" \
        --name "$AKS_NAME" \
        --node-count "$NODE_COUNT" \
        --node-vm-size "$NODE_SIZE" \
        --enable-oidc-issuer \
        --enable-workload-identity \
        --generate-ssh-keys \
        --output none
fi

az aks get-credentials --resource-group "$RG" --name "$AKS_NAME" --overwrite-existing

# Create ACR pull secret (doesn't require Owner role, unlike --attach-acr)
echo "  Setting up ACR pull secret..."
ACR_USER=$(az acr credential show --name "$ACR_NAME" --query username -o tsv)
ACR_PASS=$(az acr credential show --name "$ACR_NAME" --query "passwords[0].value" -o tsv)
kubectl create namespace cosmos-perf 2>/dev/null || true
kubectl delete secret acr-secret -n cosmos-perf 2>/dev/null || true
kubectl create secret docker-registry acr-secret \
    --namespace cosmos-perf \
    --docker-server="${ACR_NAME}.azurecr.io" \
    --docker-username="$ACR_USER" \
    --docker-password="$ACR_PASS"

# --- Step 5/10: Managed Identity + RBAC ---
echo "--- Step 5/10: Setting up managed identity and RBAC ---"
az identity create --name "$IDENTITY_NAME" --resource-group "$RG" --output none 2>/dev/null || \
    echo "  Identity '$IDENTITY_NAME' already exists."

IDENTITY_CLIENT_ID=$(az identity show --name "$IDENTITY_NAME" --resource-group "$RG" --query clientId -o tsv)
IDENTITY_OBJECT_ID=$(az identity show --name "$IDENTITY_NAME" --resource-group "$RG" --query principalId -o tsv)

# Grant Cosmos DB data plane RBAC (native RBAC, required when local auth is disabled)
# Role 00000000-0000-0000-0000-000000000002 = Cosmos DB Built-in Data Contributor
COSMOS_DATA_CONTRIBUTOR_ROLE="00000000-0000-0000-0000-000000000002"

echo "  Granting Cosmos DB data plane RBAC on workload account..."
az cosmosdb sql role assignment create \
    --account-name "$COSMOS_ACCOUNT" \
    --resource-group "$COSMOS_RG" \
    --role-definition-id "$COSMOS_DATA_CONTRIBUTOR_ROLE" \
    --principal-id "$IDENTITY_OBJECT_ID" \
    --scope "/" \
    --output none 2>/dev/null || echo "  Workload RBAC already assigned."

# Grant on results account if different
if [[ "$RESULTS_ACCOUNT" != "$COSMOS_ACCOUNT" ]]; then
    echo "  Granting Cosmos DB data plane RBAC on results account..."
    az cosmosdb sql role assignment create \
        --account-name "$RESULTS_ACCOUNT" \
        --resource-group "$RESULTS_RG" \
        --role-definition-id "$COSMOS_DATA_CONTRIBUTOR_ROLE" \
        --principal-id "$IDENTITY_OBJECT_ID" \
        --scope "/" \
        --output none 2>/dev/null || echo "  Results RBAC already assigned."
fi

# --- Step 6/10: Federated Credential ---
echo "--- Step 6/10: Setting up workload identity federation ---"
AKS_OIDC_ISSUER=$(az aks show --name "$AKS_NAME" --resource-group "$RG" --query "oidcIssuerProfile.issuerUrl" -o tsv)

az identity federated-credential create \
    --name cosmos-perf-fed \
    --identity-name "$IDENTITY_NAME" \
    --resource-group "$RG" \
    --issuer "$AKS_OIDC_ISSUER" \
    --subject "system:serviceaccount:cosmos-perf:cosmos-perf-sa" \
    --audience "api://AzureADTokenExchange" \
    --output none 2>/dev/null || echo "  Federated credential already exists."

# --- Step 7/10: Build and Push Docker Image ---
echo "--- Step 7/10: Building and pushing Docker image ---"

# Create a minimal copy of the repo for the build context.
# az acr build doesn't always respect .dockerignore, so we exclude
# target/ (47GB+) and other large directories manually.
echo "  Creating build context..."
BUILD_CONTEXT=$(mktemp -d /tmp/cosmos-perf-context-XXXXXX)
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
rm -rf "$BUILD_CONTEXT"

# --- Step 8/10: Deploy to AKS ---
echo "--- Step 8/10: Deploying perf job to AKS ---"
export ACR_NAME IMAGE_TAG COSMOS_ENDPOINT RESULTS_ENDPOINT RESULTS_AUTH
export PARALLELISM CONCURRENCY IDENTITY_CLIENT_ID

# Delete existing job first — K8s Jobs are immutable once created
kubectl delete job cosmos-perf -n cosmos-perf 2>/dev/null || true
envsubst < "$SCRIPT_DIR/perf-job.yaml" | kubectl apply -f -

# --- Step 9/10: Create Azure Data Explorer cluster + database + tables ---
echo "--- Step 9/10: Creating Azure Data Explorer cluster and tables ---"

# ADX isn't available in all regions (e.g., EUAP/canary). Fall back to the
# closest supported region if the primary location fails.
ADX_LOCATION="$LOCATION"
resolve_adx_location() {
    # Map common unsupported regions to nearby supported ones
    case "$1" in
        eastus2euap|centraluseuap)  echo "eastus2" ;;
        eastusstg|southcentralusstg) echo "eastus" ;;
        *)                          echo "$1" ;;
    esac
}
ADX_LOCATION=$(resolve_adx_location "$LOCATION")
if [[ "$ADX_LOCATION" != "$LOCATION" ]]; then
    echo "  ADX not available in '$LOCATION', using '$ADX_LOCATION' instead."
fi

if az kusto cluster show --name "$ADX_CLUSTER_NAME" --resource-group "$RG" &>/dev/null; then
    echo "  ADX cluster '$ADX_CLUSTER_NAME' already exists."
    # Ensure cluster is running (Dev SKU auto-stops after inactivity)
    CLUSTER_STATE=$(az kusto cluster show --name "$ADX_CLUSTER_NAME" --resource-group "$RG" --query "state" -o tsv 2>/dev/null)
    if [[ "$CLUSTER_STATE" != "Running" ]]; then
        echo "  Cluster is '$CLUSTER_STATE', starting..."
        az kusto cluster start --name "$ADX_CLUSTER_NAME" --resource-group "$RG" --no-wait --output none 2>/dev/null || true
    fi
else
    echo "  Creating ADX cluster '$ADX_CLUSTER_NAME' in '$ADX_LOCATION' (this may take 10-15 minutes)..."
    az kusto cluster create \
        --name "$ADX_CLUSTER_NAME" \
        --resource-group "$RG" \
        --location "$ADX_LOCATION" \
        --sku name="Dev(No SLA)_Standard_E2a_v4" tier="Basic" capacity=1 \
        --output none
fi

# Wait for cluster to be running
echo "  Waiting for ADX cluster to be ready..."
az kusto cluster wait --name "$ADX_CLUSTER_NAME" --resource-group "$RG" \
    --custom "state=='Running'" 2>/dev/null || true

# Create database — use az rest since az kusto database create has inconsistent --location behavior
if az kusto database show --cluster-name "$ADX_CLUSTER_NAME" --database-name "$ADX_DATABASE_NAME" --resource-group "$RG" &>/dev/null; then
    echo "  Database '$ADX_DATABASE_NAME' already exists."
else
    echo "  Creating database '$ADX_DATABASE_NAME'..."
    SUBSCRIPTION_ID=$(az account show --query id -o tsv)
    az rest --method put \
        --url "https://management.azure.com/subscriptions/${SUBSCRIPTION_ID}/resourceGroups/${RG}/providers/Microsoft.Kusto/clusters/${ADX_CLUSTER_NAME}/databases/${ADX_DATABASE_NAME}?api-version=2023-08-15" \
        --body "{\"kind\":\"ReadWrite\",\"location\":\"${ADX_LOCATION}\",\"properties\":{\"softDeletePeriod\":\"P30D\",\"hotCachePeriod\":\"P7D\"}}" \
        --output none
fi

ADX_URI="https://${ADX_CLUSTER_NAME}.${ADX_LOCATION}.kusto.windows.net"

# Run KQL via the ARM Scripts API (works from any network, unlike direct Kusto endpoint).
SUBSCRIPTION_ID=$(az account show --query id -o tsv)
KQL_SCRIPT_URL="https://management.azure.com/subscriptions/${SUBSCRIPTION_ID}/resourceGroups/${RG}/providers/Microsoft.Kusto/clusters/${ADX_CLUSTER_NAME}/databases/${ADX_DATABASE_NAME}/scripts/deploySetup?api-version=2023-08-15"

# Build the KQL script with tables and ingestion mappings
KQL_SCRIPT=$(cat <<'KQLEOF'
.create-merge table PerfResults (id: string, partition_key: string, workload_id: string, commit_sha: string, TIMESTAMP: datetime, operation: string, ["count"]: long, errors: long, min_ms: real, max_ms: real, mean_ms: real, p50_ms: real, p90_ms: real, p99_ms: real, cpu_percent: real, memory_bytes: long, system_cpu_percent: real, system_total_memory_bytes: long, system_used_memory_bytes: long)

.create-merge table ErrorResults (id: string, partition_key: string, workload_id: string, commit_sha: string, TIMESTAMP: datetime, operation: string, error_message: string, source_message: string)

.create-or-alter table PerfResults ingestion json mapping 'PerfResultsMapping' '[{"column":"id","path":"$.id"},{"column":"partition_key","path":"$.partition_key"},{"column":"workload_id","path":"$.workload_id"},{"column":"commit_sha","path":"$.commit_sha"},{"column":"TIMESTAMP","path":"$.TIMESTAMP"},{"column":"operation","path":"$.operation"},{"column":"count","path":"$.count"},{"column":"errors","path":"$.errors"},{"column":"min_ms","path":"$.min_ms"},{"column":"max_ms","path":"$.max_ms"},{"column":"mean_ms","path":"$.mean_ms"},{"column":"p50_ms","path":"$.p50_ms"},{"column":"p90_ms","path":"$.p90_ms"},{"column":"p99_ms","path":"$.p99_ms"},{"column":"cpu_percent","path":"$.cpu_percent"},{"column":"memory_bytes","path":"$.memory_bytes"},{"column":"system_cpu_percent","path":"$.system_cpu_percent"},{"column":"system_total_memory_bytes","path":"$.system_total_memory_bytes"},{"column":"system_used_memory_bytes","path":"$.system_used_memory_bytes"}]'

.create-or-alter table ErrorResults ingestion json mapping 'ErrorResultsMapping' '[{"column":"id","path":"$.id"},{"column":"partition_key","path":"$.partition_key"},{"column":"workload_id","path":"$.workload_id"},{"column":"commit_sha","path":"$.commit_sha"},{"column":"TIMESTAMP","path":"$.TIMESTAMP"},{"column":"operation","path":"$.operation"},{"column":"error_message","path":"$.error_message"},{"column":"source_message","path":"$.source_message"}]'
KQLEOF
)

# Use a unique tag so re-runs update the script
FORCE_TAG="v$(date +%s)"
echo "  Creating tables and ingestion mappings via ARM Scripts API..."
az rest --method put \
    --url "$KQL_SCRIPT_URL" \
    --body "$(jq -n --arg script "$KQL_SCRIPT" --arg tag "$FORCE_TAG" \
        '{properties: {scriptContent: $script, forceUpdateTag: $tag, continueOnErrors: true}}')" \
    --output none || echo "  Warning: Script execution may have partially failed. Check tables in portal."

echo "  Waiting for script to complete..."
az rest --method get --url "$KQL_SCRIPT_URL" \
    --query "properties.provisioningState" -o tsv 2>/dev/null || true

# --- Step 10/10: Create Cosmos DB Change Feed data connections ---
echo "--- Step 10/10: Creating Cosmos DB change feed connections to ADX ---"

# Get the results Cosmos DB account resource ID
RESULTS_COSMOS_ID=$(az cosmosdb show --name "$RESULTS_ACCOUNT" --resource-group "$RESULTS_RG" --query id -o tsv)

# The change feed data connection needs the Cosmos DB account to grant read access to the ADX cluster's managed identity
ADX_PRINCIPAL_ID=$(az kusto cluster show --name "$ADX_CLUSTER_NAME" --resource-group "$RG" --query "identity.principalId" -o tsv 2>/dev/null)

# Enable system-assigned identity on ADX cluster if not already
if [[ -z "$ADX_PRINCIPAL_ID" || "$ADX_PRINCIPAL_ID" == "null" ]]; then
    echo "  Enabling system-assigned identity on ADX cluster..."
    SUBSCRIPTION_ID=$(az account show --query id -o tsv)
    az rest --method patch \
        --url "https://management.azure.com/subscriptions/${SUBSCRIPTION_ID}/resourceGroups/${RG}/providers/Microsoft.Kusto/clusters/${ADX_CLUSTER_NAME}?api-version=2023-08-15" \
        --body '{"identity":{"type":"SystemAssigned"}}' \
        --output none
    # Wait for the update to propagate
    sleep 30
    ADX_PRINCIPAL_ID=$(az kusto cluster show --name "$ADX_CLUSTER_NAME" --resource-group "$RG" --query "identity.principalId" -o tsv)
fi

# Grant ADX cluster read access to the Cosmos DB change feed
echo "  Granting ADX cluster read access to results Cosmos DB account..."
az role assignment create \
    --assignee "$ADX_PRINCIPAL_ID" \
    --role "Cosmos DB Built-in Data Reader" \
    --scope "$RESULTS_COSMOS_ID" \
    --output none 2>/dev/null || echo "  ADX Cosmos DB RBAC already assigned."

# Create data connection for PerfResults via REST API
# (az kusto data-connection cosmos-db is not available in all CLI versions)
echo "  Creating PerfResults change feed connection..."
SUBSCRIPTION_ID=$(az account show --query id -o tsv)
ADX_RESOURCE_ID="/subscriptions/${SUBSCRIPTION_ID}/resourceGroups/${RG}/providers/Microsoft.Kusto/clusters/${ADX_CLUSTER_NAME}"

az rest --method put \
    --url "https://management.azure.com${ADX_RESOURCE_ID}/databases/${ADX_DATABASE_NAME}/dataConnections/perf-results-feed?api-version=2023-08-15" \
    --body "{
        \"kind\": \"CosmosDb\",
        \"location\": \"${ADX_LOCATION}\",
        \"properties\": {
            \"cosmosDbAccountResourceId\": \"${RESULTS_COSMOS_ID}\",
            \"cosmosDbDatabase\": \"perfdb\",
            \"cosmosDbContainer\": \"perfresults\",
            \"tableName\": \"PerfResults\",
            \"mappingRuleName\": \"PerfResultsMapping\",
            \"managedIdentityResourceId\": \"${ADX_RESOURCE_ID}\"
        }
    }" \
    --output none || echo "  Warning: Failed to create data connection. You may need to create it manually in the Azure portal."

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
