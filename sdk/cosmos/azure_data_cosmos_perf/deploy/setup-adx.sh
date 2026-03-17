#!/usr/bin/env bash
# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# Creates an Azure Data Explorer cluster, database, tables, ingestion mappings,
# and Cosmos DB change feed data connections.
#
# Usage:
#   ./setup-adx.sh \
#     --cluster-name cosmosperfadx \
#     --database-name PerfMetrics \
#     --resource-group my-rg \
#     --location eastus \
#     --results-cosmos-account myresults \
#     --results-cosmos-rg my-cosmos-rg

set -euo pipefail

CLUSTER_NAME=""
DATABASE_NAME=""
RESOURCE_GROUP=""
LOCATION=""
RESULTS_COSMOS_ACCOUNT=""
RESULTS_COSMOS_RG=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --cluster-name)           CLUSTER_NAME="$2"; shift 2 ;;
        --database-name)          DATABASE_NAME="$2"; shift 2 ;;
        --resource-group)         RESOURCE_GROUP="$2"; shift 2 ;;
        --location)               LOCATION="$2"; shift 2 ;;
        --results-cosmos-account) RESULTS_COSMOS_ACCOUNT="$2"; shift 2 ;;
        --results-cosmos-rg)      RESULTS_COSMOS_RG="$2"; shift 2 ;;
        --help|-h)
            echo "Usage: $0 [options]"
            echo ""
            echo "Creates an ADX cluster, database, tables, and Cosmos DB change feed connections."
            echo ""
            echo "Required:"
            echo "  --cluster-name NAME           ADX cluster name"
            echo "  --database-name NAME          ADX database name"
            echo "  --resource-group RG           Resource group"
            echo "  --location REGION             Azure region"
            echo "  --results-cosmos-account NAME Results Cosmos DB account name"
            echo "  --results-cosmos-rg RG        Results Cosmos DB resource group"
            echo ""
            echo "Output (stdout):"
            echo "  ADX_URI=<cluster-uri>"
            exit 0
            ;;
        *) echo "Unknown option: $1" >&2; exit 1 ;;
    esac
done

if [[ -z "$CLUSTER_NAME" || -z "$DATABASE_NAME" || -z "$RESOURCE_GROUP" || -z "$LOCATION" \
   || -z "$RESULTS_COSMOS_ACCOUNT" || -z "$RESULTS_COSMOS_RG" ]]; then
    echo "Error: All options are required. Run '$0 --help' for usage." >&2
    exit 1
fi

# --- EUAP region fallback ---
resolve_adx_location() {
    case "$1" in
        eastus2euap|centraluseuap)  echo "eastus2" ;;
        eastusstg|southcentralusstg) echo "eastus" ;;
        *)                          echo "$1" ;;
    esac
}
ADX_LOCATION=$(resolve_adx_location "$LOCATION")
if [[ "$ADX_LOCATION" != "$LOCATION" ]]; then
    echo "  ADX not available in '$LOCATION', using '$ADX_LOCATION' instead." >&2
fi

# --- Create or verify ADX cluster ---
if az kusto cluster show --name "$CLUSTER_NAME" --resource-group "$RESOURCE_GROUP" &>/dev/null; then
    echo "  ADX cluster '$CLUSTER_NAME' already exists." >&2
    # Ensure cluster is running (Dev SKU auto-stops after inactivity)
    CLUSTER_STATE=$(az kusto cluster show --name "$CLUSTER_NAME" --resource-group "$RESOURCE_GROUP" --query "state" -o tsv 2>/dev/null)
    if [[ "$CLUSTER_STATE" != "Running" ]]; then
        echo "  Cluster is '$CLUSTER_STATE', starting..." >&2
        az kusto cluster start --name "$CLUSTER_NAME" --resource-group "$RESOURCE_GROUP" --no-wait --output none 2>/dev/null || true
    fi
else
    echo "  Creating ADX cluster '$CLUSTER_NAME' in '$ADX_LOCATION' (this may take 10-15 minutes)..." >&2
    az kusto cluster create \
        --name "$CLUSTER_NAME" \
        --resource-group "$RESOURCE_GROUP" \
        --location "$ADX_LOCATION" \
        --sku name="Dev(No SLA)_Standard_E2a_v4" tier="Basic" capacity=1 \
        --output none
fi

# Wait for cluster to be running
echo "  Waiting for ADX cluster to be ready..." >&2
az kusto cluster wait --name "$CLUSTER_NAME" --resource-group "$RESOURCE_GROUP" \
    --custom "state=='Running'" 2>/dev/null || true

# --- Create database ---
if az kusto database show --cluster-name "$CLUSTER_NAME" --database-name "$DATABASE_NAME" --resource-group "$RESOURCE_GROUP" &>/dev/null; then
    echo "  Database '$DATABASE_NAME' already exists." >&2
else
    echo "  Creating database '$DATABASE_NAME'..." >&2
    SUBSCRIPTION_ID=$(az account show --query id -o tsv)
    az rest --method put \
        --url "https://management.azure.com/subscriptions/${SUBSCRIPTION_ID}/resourceGroups/${RESOURCE_GROUP}/providers/Microsoft.Kusto/clusters/${CLUSTER_NAME}/databases/${DATABASE_NAME}?api-version=2023-08-15" \
        --body "{\"kind\":\"ReadWrite\",\"location\":\"${ADX_LOCATION}\",\"properties\":{\"softDeletePeriod\":\"P30D\",\"hotCachePeriod\":\"P7D\"}}" \
        --output none
    # Wait for database to be provisioned before creating tables
    echo "  Waiting for database to be ready..." >&2
    for i in $(seq 1 30); do
        if az kusto database show --cluster-name "$CLUSTER_NAME" --database-name "$DATABASE_NAME" --resource-group "$RESOURCE_GROUP" &>/dev/null; then
            echo "  Database ready." >&2
            break
        fi
        sleep 5
    done
fi

# --- Create tables and ingestion mappings via ARM Scripts API ---
SUBSCRIPTION_ID=$(az account show --query id -o tsv)
KQL_SCRIPT_URL="https://management.azure.com/subscriptions/${SUBSCRIPTION_ID}/resourceGroups/${RESOURCE_GROUP}/providers/Microsoft.Kusto/clusters/${CLUSTER_NAME}/databases/${DATABASE_NAME}/scripts/deploySetup?api-version=2023-08-15"

KQL_SCRIPT=$(cat <<'KQLEOF'
.create-merge table RawResults (id: string, partition_key: string, workload_id: string, commit_sha: string, hostname: string, TIMESTAMP: datetime, operation: string, ["count"]: long, errors: long, min_ms: real, max_ms: real, mean_ms: real, p50_ms: real, p90_ms: real, p99_ms: real, cpu_percent: real, memory_bytes: long, system_cpu_percent: real, system_total_memory_bytes: long, system_used_memory_bytes: long, error_message: string, source_message: string)

.create-merge table PerfResults (id: string, partition_key: string, workload_id: string, commit_sha: string, hostname: string, TIMESTAMP: datetime, operation: string, ["count"]: long, errors: long, min_ms: real, max_ms: real, mean_ms: real, p50_ms: real, p90_ms: real, p99_ms: real, cpu_percent: real, memory_bytes: long, system_cpu_percent: real, system_total_memory_bytes: long, system_used_memory_bytes: long)

.create-merge table ErrorResults (id: string, partition_key: string, workload_id: string, commit_sha: string, hostname: string, TIMESTAMP: datetime, operation: string, error_message: string, source_message: string)

.create-or-alter table RawResults ingestion json mapping 'RawResultsMapping' '[{"column":"id","path":"$.id"},{"column":"partition_key","path":"$.partition_key"},{"column":"workload_id","path":"$.workload_id"},{"column":"commit_sha","path":"$.commit_sha"},{"column":"hostname","path":"$.hostname"},{"column":"TIMESTAMP","path":"$.TIMESTAMP"},{"column":"operation","path":"$.operation"},{"column":"count","path":"$.count"},{"column":"errors","path":"$.errors"},{"column":"min_ms","path":"$.min_ms"},{"column":"max_ms","path":"$.max_ms"},{"column":"mean_ms","path":"$.mean_ms"},{"column":"p50_ms","path":"$.p50_ms"},{"column":"p90_ms","path":"$.p90_ms"},{"column":"p99_ms","path":"$.p99_ms"},{"column":"cpu_percent","path":"$.cpu_percent"},{"column":"memory_bytes","path":"$.memory_bytes"},{"column":"system_cpu_percent","path":"$.system_cpu_percent"},{"column":"system_total_memory_bytes","path":"$.system_total_memory_bytes"},{"column":"system_used_memory_bytes","path":"$.system_used_memory_bytes"},{"column":"error_message","path":"$.error_message"},{"column":"source_message","path":"$.source_message"}]'

.alter table PerfResults policy update @'[{"IsEnabled": true, "Source": "RawResults", "Query": "RawResults | where isempty(error_message) | project-away error_message, source_message", "IsTransactional": false}]'

.alter table ErrorResults policy update @'[{"IsEnabled": true, "Source": "RawResults", "Query": "RawResults | where isnotempty(error_message) | project id, partition_key, workload_id, commit_sha, hostname, TIMESTAMP, operation, error_message, source_message", "IsTransactional": false}]'
KQLEOF
)

FORCE_TAG="v$(date +%s)"
echo "  Creating tables and ingestion mappings via ARM Scripts API..." >&2
az rest --method put \
    --url "$KQL_SCRIPT_URL" \
    --body "$(jq -n --arg script "$KQL_SCRIPT" --arg tag "$FORCE_TAG" \
        '{properties: {scriptContent: $script, forceUpdateTag: $tag, continueOnErrors: true}}')" \
    --output none || echo "  Warning: Script execution may have partially failed. Check tables in portal." >&2

echo "  Waiting for script to complete..." >&2
az rest --method get --url "$KQL_SCRIPT_URL" \
    --query "properties.provisioningState" -o tsv 2>/dev/null || true

# --- Cosmos DB change feed data connections ---
RESULTS_COSMOS_ID=$(az cosmosdb show --name "$RESULTS_COSMOS_ACCOUNT" --resource-group "$RESULTS_COSMOS_RG" --query id -o tsv)
ADX_PRINCIPAL_ID=$(az kusto cluster show --name "$CLUSTER_NAME" --resource-group "$RESOURCE_GROUP" --query "identity.principalId" -o tsv 2>/dev/null)

# Enable system-assigned identity on ADX cluster if not already
if [[ -z "$ADX_PRINCIPAL_ID" || "$ADX_PRINCIPAL_ID" == "null" ]]; then
    echo "  Enabling system-assigned identity on ADX cluster..." >&2
    az rest --method patch \
        --url "https://management.azure.com/subscriptions/${SUBSCRIPTION_ID}/resourceGroups/${RESOURCE_GROUP}/providers/Microsoft.Kusto/clusters/${CLUSTER_NAME}?api-version=2023-08-15" \
        --body '{"identity":{"type":"SystemAssigned"}}' \
        --output none
    echo "  Waiting for identity to propagate..." >&2
    for i in $(seq 1 30); do
        ADX_PRINCIPAL_ID=$(az kusto cluster show --name "$CLUSTER_NAME" --resource-group "$RESOURCE_GROUP" --query "identity.principalId" -o tsv 2>/dev/null)
        if [[ -n "$ADX_PRINCIPAL_ID" && "$ADX_PRINCIPAL_ID" != "null" ]]; then
            break
        fi
        sleep 5
    done
    if [[ -z "$ADX_PRINCIPAL_ID" || "$ADX_PRINCIPAL_ID" == "null" ]]; then
        echo "Error: ADX cluster identity did not propagate within timeout." >&2
        exit 1
    fi
fi

# Grant ADX cluster read access to the Cosmos DB change feed
echo "  Granting ADX cluster read access to results Cosmos DB account..." >&2

# Data plane RBAC (for reading change feed data)
az cosmosdb sql role assignment create \
    --account-name "$RESULTS_COSMOS_ACCOUNT" \
    --resource-group "$RESULTS_COSMOS_RG" \
    --role-definition-id "00000000-0000-0000-0000-000000000001" \
    --principal-id "$ADX_PRINCIPAL_ID" \
    --scope "/" \
    --output none 2>/dev/null || echo "  ADX Cosmos DB data plane RBAC already assigned." >&2

# ARM control plane role (required for the data connection to access account metadata)
az role assignment create \
    --assignee "$ADX_PRINCIPAL_ID" \
    --role "Cosmos DB Account Reader Role" \
    --scope "$RESULTS_COSMOS_ID" \
    --output none 2>/dev/null || echo "  ADX Cosmos DB ARM role already assigned." >&2

# Create data connection for PerfResults via REST API
echo "  Creating PerfResults change feed connection..." >&2
ADX_RESOURCE_ID="/subscriptions/${SUBSCRIPTION_ID}/resourceGroups/${RESOURCE_GROUP}/providers/Microsoft.Kusto/clusters/${CLUSTER_NAME}"

az rest --method put \
    --url "https://management.azure.com${ADX_RESOURCE_ID}/databases/${DATABASE_NAME}/dataConnections/perf-results-feed?api-version=2023-08-15" \
    --body "{
        \"kind\": \"CosmosDb\",
        \"location\": \"${ADX_LOCATION}\",
        \"properties\": {
            \"cosmosDbAccountResourceId\": \"${RESULTS_COSMOS_ID}\",
            \"cosmosDbDatabase\": \"perfdb\",
            \"cosmosDbContainer\": \"perfresults\",
            \"tableName\": \"RawResults\",
            \"mappingRuleName\": \"RawResultsMapping\",
            \"managedIdentityResourceId\": \"${ADX_RESOURCE_ID}\"
        }
    }" \
    --output none || echo "  Warning: Failed to create data connection. You may need to create it manually in the Azure portal." >&2

ADX_URI="https://${CLUSTER_NAME}.${ADX_LOCATION}.kusto.windows.net"

# Output for the caller
echo "ADX_URI=$ADX_URI"
