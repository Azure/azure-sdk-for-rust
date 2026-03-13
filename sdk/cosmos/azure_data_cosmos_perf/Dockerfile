# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# Multi-stage build for the Cosmos DB performance testing tool.
#
# The deploy-aks.sh script creates a minimal build context via rsync
# (excluding target/, .git/, tests/, etc.), so COPY . . is safe and
# avoids fragile per-crate COPY/stub logic.

FROM rust:1.88-bookworm AS builder
WORKDIR /src
COPY . .
RUN cargo build --release -p azure_data_cosmos_perf

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates git \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /src/target/release/azure_data_cosmos_perf /usr/local/bin/
ENTRYPOINT ["azure_data_cosmos_perf"]
