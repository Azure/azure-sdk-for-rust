#!/usr/bin/env bash
# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# Creates a Cosmos DB account with optional database and container.
#
# Usage:
#   ./setup-cosmos-db.sh \
#     --account-name myaccount \
#     --resource-group my-rg \
#     --location eastus \
#     [--database-name perfdb] \
#     [--container-name mycontainer --partition-key-path /pk --throughput 10000 --ttl 86400] \
#     [--multi-write] \
#     [--regions "East US,West US"]

set -euo pipefail

# Defaults
ACCOUNT_NAME=""
RESOURCE_GROUP=""
LOCATION=""
DATABASE_NAME="perfdb"
CONTAINER_NAME=""
PARTITION_KEY_PATH=""
THROUGHPUT=""
TTL=""
MULTI_WRITE=false
REGIONS=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --account-name)       ACCOUNT_NAME="$2"; shift 2 ;;
        --resource-group)     RESOURCE_GROUP="$2"; shift 2 ;;
        --location)           LOCATION="$2"; shift 2 ;;
        --database-name)      DATABASE_NAME="$2"; shift 2 ;;
        --container-name)     CONTAINER_NAME="$2"; shift 2 ;;
        --partition-key-path) PARTITION_KEY_PATH="$2"; shift 2 ;;
        --throughput)         THROUGHPUT="$2"; shift 2 ;;
        --ttl)                TTL="$2"; shift 2 ;;
        --multi-write)        MULTI_WRITE=true; shift ;;
        --regions)            REGIONS="$2"; shift 2 ;;
        --regions=*)          REGIONS="${1#*=}"; shift ;;
        --help|-h)
            echo "Usage: $0 [options]"
            echo ""
            echo "Creates a Cosmos DB account with optional database and container."
            echo ""
            echo "Required:"
            echo "  --account-name NAME       Cosmos DB account name"
            echo "  --resource-group RG       Resource group"
            echo "  --location REGION         Azure region"
            echo ""
            echo "Optional:"
            echo "  --database-name NAME      Database name (default: perfdb)"
            echo "  --container-name NAME     Container name (skipped if not set)"
            echo "  --partition-key-path PATH Partition key path (required with --container-name)"
            echo "  --throughput N            Container throughput in RU/s"
            echo "  --ttl SECONDS             Container default TTL"
            echo "  --multi-write             Enable multi-region writes"
            echo "  --regions R1,R2,...        Comma-separated Azure regions"
            exit 0
            ;;
        *) echo "Unknown option: $1" >&2; exit 1 ;;
    esac
done

if [[ -z "$ACCOUNT_NAME" || -z "$RESOURCE_GROUP" || -z "$LOCATION" ]]; then
    echo "Error: --account-name, --resource-group, and --location are required." >&2
    exit 1
fi

if [[ -n "$CONTAINER_NAME" && -z "$PARTITION_KEY_PATH" ]]; then
    echo "Error: --partition-key-path is required when --container-name is set." >&2
    exit 1
fi

# --- Create Cosmos DB account ---
if az cosmosdb show --name "$ACCOUNT_NAME" --resource-group "$RESOURCE_GROUP" &>/dev/null; then
    echo "  Cosmos DB account '$ACCOUNT_NAME' already exists."
else
    # Build the locations JSON array
    local_locations_json=""
    if [[ -n "$REGIONS" ]]; then
        priority=0
        IFS=',' read -ra REGION_ARRAY <<< "$REGIONS"
        for region in "${REGION_ARRAY[@]}"; do
            region=$(echo "$region" | xargs) # trim whitespace
            if [[ -n "$local_locations_json" ]]; then
                local_locations_json="${local_locations_json},"
            fi
            local_locations_json="${local_locations_json}{\"locationName\":\"${region}\",\"failoverPriority\":${priority}}"
            priority=$((priority + 1))
        done
    else
        local_locations_json="{\"locationName\":\"${LOCATION}\",\"failoverPriority\":0}"
    fi

    multi_write_json="false"
    if [[ "$MULTI_WRITE" == true ]]; then
        multi_write_json="true"
    fi

    echo "  Creating Cosmos DB account '$ACCOUNT_NAME' (this may take a few minutes)..."
    if [[ "$MULTI_WRITE" == true ]]; then
        echo "  Multi-region writes: enabled"
    fi
    if [[ -n "$REGIONS" ]]; then
        echo "  Regions: $REGIONS"
    fi

    TEMPLATE_FILE=$(mktemp /tmp/cosmos-arm-XXXXXX.json)
    cat > "$TEMPLATE_FILE" <<ARMEOF
{
  "\$schema": "https://schema.management.azure.com/schemas/2019-04-01/deploymentTemplate.json#",
  "contentVersion": "1.0.0.0",
  "resources": [{
    "type": "Microsoft.DocumentDB/databaseAccounts",
    "apiVersion": "2024-05-15",
    "name": "${ACCOUNT_NAME}",
    "location": "${LOCATION}",
    "kind": "GlobalDocumentDB",
    "properties": {
      "databaseAccountOfferType": "Standard",
      "disableLocalAuth": true,
      "enableMultipleWriteLocations": ${multi_write_json},
      "consistencyPolicy": { "defaultConsistencyLevel": "Session" },
      "locations": [${local_locations_json}]
    }
  }]
}
ARMEOF
    az deployment group create \
        --resource-group "$RESOURCE_GROUP" \
        --template-file "$TEMPLATE_FILE" \
        --output none
    rm -f "$TEMPLATE_FILE"
fi

# --- Create database ---
echo "  Creating database '$DATABASE_NAME'..."
az cosmosdb sql database create --account-name "$ACCOUNT_NAME" --resource-group "$RESOURCE_GROUP" \
    --name "$DATABASE_NAME" --output none 2>/dev/null || echo "  Database '$DATABASE_NAME' already exists."

# --- Create container (optional) ---
if [[ -n "$CONTAINER_NAME" ]]; then
    echo "  Creating container '$CONTAINER_NAME'..."
    CONTAINER_ARGS=(
        --account-name "$ACCOUNT_NAME"
        --resource-group "$RESOURCE_GROUP"
        --database-name "$DATABASE_NAME"
        --name "$CONTAINER_NAME"
        --partition-key-path "$PARTITION_KEY_PATH"
    )
    if [[ -n "$THROUGHPUT" ]]; then
        CONTAINER_ARGS+=(--throughput "$THROUGHPUT")
    fi
    if [[ -n "$TTL" ]]; then
        CONTAINER_ARGS+=(--ttl "$TTL")
    fi
    az cosmosdb sql container create "${CONTAINER_ARGS[@]}" --output none 2>/dev/null \
        || echo "  Container '$CONTAINER_NAME' already exists."
fi

echo "  Cosmos DB setup complete for account '$ACCOUNT_NAME'."
