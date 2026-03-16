#!/usr/bin/env bash
# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# Creates Kubernetes namespace, service account, and federated credential for workload identity.
#
# Usage:
#   ./setup-k8s-identity.sh \
#     --namespace cosmos-perf \
#     --service-account-name cosmos-perf-sa \
#     --identity-name cosmos-perf-identity \
#     --identity-client-id <client-id> \
#     --resource-group my-rg \
#     --aks-name cosmos-perf-aks

set -euo pipefail

NAMESPACE="cosmos-perf"
SERVICE_ACCOUNT_NAME="cosmos-perf-sa"
IDENTITY_NAME=""
IDENTITY_CLIENT_ID=""
RESOURCE_GROUP=""
AKS_NAME=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --namespace)            NAMESPACE="$2"; shift 2 ;;
        --service-account-name) SERVICE_ACCOUNT_NAME="$2"; shift 2 ;;
        --identity-name)        IDENTITY_NAME="$2"; shift 2 ;;
        --identity-client-id)   IDENTITY_CLIENT_ID="$2"; shift 2 ;;
        --resource-group)       RESOURCE_GROUP="$2"; shift 2 ;;
        --aks-name)             AKS_NAME="$2"; shift 2 ;;
        --help|-h)
            echo "Usage: $0 [options]"
            echo ""
            echo "Creates K8s namespace, service account, and federated credential."
            echo ""
            echo "Required:"
            echo "  --identity-name NAME         Managed identity name"
            echo "  --identity-client-id ID      Managed identity client ID"
            echo "  --resource-group RG          Resource group"
            echo "  --aks-name NAME              AKS cluster name"
            echo ""
            echo "Optional:"
            echo "  --namespace NAME             Kubernetes namespace (default: cosmos-perf)"
            echo "  --service-account-name NAME  Service account name (default: cosmos-perf-sa)"
            exit 0
            ;;
        *) echo "Unknown option: $1" >&2; exit 1 ;;
    esac
done

if [[ -z "$IDENTITY_NAME" || -z "$IDENTITY_CLIENT_ID" || -z "$RESOURCE_GROUP" || -z "$AKS_NAME" ]]; then
    echo "Error: --identity-name, --identity-client-id, --resource-group, and --aks-name are required." >&2
    exit 1
fi

# --- Get OIDC issuer ---
AKS_OIDC_ISSUER=$(az aks show --name "$AKS_NAME" --resource-group "$RESOURCE_GROUP" --query "oidcIssuerProfile.issuerUrl" -o tsv)

# --- Create federated credential ---
az identity federated-credential create \
    --name cosmos-perf-fed \
    --identity-name "$IDENTITY_NAME" \
    --resource-group "$RESOURCE_GROUP" \
    --issuer "$AKS_OIDC_ISSUER" \
    --subject "system:serviceaccount:${NAMESPACE}:${SERVICE_ACCOUNT_NAME}" \
    --audience "api://AzureADTokenExchange" \
    --output none 2>/dev/null || echo "  Federated credential already exists."

echo "  K8s identity federation complete."
