#!/usr/bin/env bash
# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# Creates a managed identity and grants Cosmos DB data plane RBAC on one or more accounts.
#
# Usage:
#   ./setup-identity-rbac.sh \
#     --identity-name cosmos-perf-identity \
#     --resource-group my-rg \
#     --cosmos-accounts "account1:rg1,account2:rg2"
#
# Outputs (to stdout, one per line):
#   CLIENT_ID=<client-id>
#   PRINCIPAL_ID=<principal-id>

set -euo pipefail

IDENTITY_NAME=""
RESOURCE_GROUP=""
COSMOS_ACCOUNTS=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --identity-name)    IDENTITY_NAME="$2"; shift 2 ;;
        --resource-group)   RESOURCE_GROUP="$2"; shift 2 ;;
        --cosmos-accounts)  COSMOS_ACCOUNTS="$2"; shift 2 ;;
        --help|-h)
            echo "Usage: $0 [options]"
            echo ""
            echo "Creates a managed identity and grants Cosmos DB RBAC on specified accounts."
            echo ""
            echo "Required:"
            echo "  --identity-name NAME          Managed identity name"
            echo "  --resource-group RG           Resource group for the identity"
            echo "  --cosmos-accounts PAIRS       Comma-separated account_name:account_rg pairs"
            echo ""
            echo "Output (stdout):"
            echo "  CLIENT_ID=<value>"
            echo "  PRINCIPAL_ID=<value>"
            exit 0
            ;;
        *) echo "Unknown option: $1" >&2; exit 1 ;;
    esac
done

if [[ -z "$IDENTITY_NAME" || -z "$RESOURCE_GROUP" || -z "$COSMOS_ACCOUNTS" ]]; then
    echo "Error: --identity-name, --resource-group, and --cosmos-accounts are required." >&2
    exit 1
fi

# --- Create managed identity ---
az identity create --name "$IDENTITY_NAME" --resource-group "$RESOURCE_GROUP" --output none 2>/dev/null || \
    echo "  Identity '$IDENTITY_NAME' already exists." >&2

IDENTITY_CLIENT_ID=$(az identity show --name "$IDENTITY_NAME" --resource-group "$RESOURCE_GROUP" --query clientId -o tsv)
IDENTITY_PRINCIPAL_ID=$(az identity show --name "$IDENTITY_NAME" --resource-group "$RESOURCE_GROUP" --query principalId -o tsv)

# --- Grant Cosmos DB data plane RBAC ---
# Role 00000000-0000-0000-0000-000000000002 = Cosmos DB Built-in Data Contributor
COSMOS_DATA_CONTRIBUTOR_ROLE="00000000-0000-0000-0000-000000000002"

IFS=',' read -ra ACCOUNT_PAIRS <<< "$COSMOS_ACCOUNTS"
SEEN_ACCOUNTS=()
for pair in "${ACCOUNT_PAIRS[@]}"; do
    account_name="${pair%%:*}"
    account_rg="${pair##*:}"

    # Skip duplicate accounts
    for seen in "${SEEN_ACCOUNTS[@]+"${SEEN_ACCOUNTS[@]}"}"; do
        if [[ "$seen" == "$account_name" ]]; then
            continue 2
        fi
    done
    SEEN_ACCOUNTS+=("$account_name")

    echo "  Granting Cosmos DB data plane RBAC on '$account_name'..." >&2
    az cosmosdb sql role assignment create \
        --account-name "$account_name" \
        --resource-group "$account_rg" \
        --role-definition-id "$COSMOS_DATA_CONTRIBUTOR_ROLE" \
        --principal-id "$IDENTITY_PRINCIPAL_ID" \
        --scope "/" \
        --output none 2>/dev/null || echo "  RBAC on '$account_name' already assigned." >&2
done

# Output identity info to stdout for the caller
echo "CLIENT_ID=$IDENTITY_CLIENT_ID"
echo "PRINCIPAL_ID=$IDENTITY_PRINCIPAL_ID"
