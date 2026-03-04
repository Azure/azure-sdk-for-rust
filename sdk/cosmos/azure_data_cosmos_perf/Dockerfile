# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# Multi-stage build for the Cosmos DB performance testing tool.
# Build from the repo root: docker build -f sdk/cosmos/azure_data_cosmos_perf/Dockerfile .

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
