#!/usr/bin/env bash
# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# Creates an AKS cluster and ACR, configures kubeconfig and ACR pull secret.
#
# Usage:
#   ./setup-aks.sh \
#     --aks-name cosmos-perf-aks \
#     --acr-name cosmosperfacr \
#     --resource-group my-rg \
#     --location eastus \
#     [--node-size Standard_D4s_v5] \
#     [--node-count 2]

set -euo pipefail

AKS_NAME=""
ACR_NAME=""
RESOURCE_GROUP=""
LOCATION=""
NODE_SIZE="Standard_D4s_v5"
NODE_COUNT=2

while [[ $# -gt 0 ]]; do
    case "$1" in
        --aks-name)       AKS_NAME="$2"; shift 2 ;;
        --acr-name)       ACR_NAME="$2"; shift 2 ;;
        --resource-group) RESOURCE_GROUP="$2"; shift 2 ;;
        --location)       LOCATION="$2"; shift 2 ;;
        --node-size)      NODE_SIZE="$2"; shift 2 ;;
        --node-count)     NODE_COUNT="$2"; shift 2 ;;
        --help|-h)
            echo "Usage: $0 [options]"
            echo ""
            echo "Creates an AKS cluster and ACR with pull secret."
            echo ""
            echo "Required:"
            echo "  --aks-name NAME       AKS cluster name"
            echo "  --acr-name NAME       ACR name"
            echo "  --resource-group RG   Resource group"
            echo "  --location REGION     Azure region"
            echo ""
            echo "Optional:"
            echo "  --node-size SKU       VM size (default: Standard_D4s_v5)"
            echo "  --node-count N        Node count (default: 2)"
            exit 0
            ;;
        *) echo "Unknown option: $1" >&2; exit 1 ;;
    esac
done

if [[ -z "$AKS_NAME" || -z "$ACR_NAME" || -z "$RESOURCE_GROUP" || -z "$LOCATION" ]]; then
    echo "Error: --aks-name, --acr-name, --resource-group, and --location are required." >&2
    exit 1
fi

# --- Create ACR ---
az acr create --name "$ACR_NAME" --resource-group "$RESOURCE_GROUP" --sku Basic --admin-enabled true --output none 2>/dev/null || \
    echo "  ACR '$ACR_NAME' already exists."

# --- Create AKS cluster ---
if az aks show --name "$AKS_NAME" --resource-group "$RESOURCE_GROUP" &>/dev/null; then
    echo "  AKS cluster '$AKS_NAME' already exists."
else
    echo "  Creating AKS cluster '$AKS_NAME' (this may take several minutes)..."
    az aks create \
        --resource-group "$RESOURCE_GROUP" \
        --name "$AKS_NAME" \
        --node-count "$NODE_COUNT" \
        --node-vm-size "$NODE_SIZE" \
        --enable-oidc-issuer \
        --enable-workload-identity \
        --generate-ssh-keys \
        --output none
fi

az aks get-credentials --resource-group "$RESOURCE_GROUP" --name "$AKS_NAME" --overwrite-existing

# --- Create ACR pull secret ---
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

echo "  AKS + ACR setup complete."
