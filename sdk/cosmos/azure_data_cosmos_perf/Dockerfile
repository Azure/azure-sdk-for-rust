# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# Single-stage build for Cosmos DB perf tool with optional Valgrind support.

FROM rust:1.88-bookworm

RUN apt-get update && \
    apt-get install -y --no-install-recommends valgrind && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /src
COPY . .

RUN cargo build --release -p azure_data_cosmos_perf --features tokio-metrics && \
    cp target/release/azure_data_cosmos_perf /usr/local/bin/ && \
    rm -rf target

COPY sdk/cosmos/azure_data_cosmos_perf/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

ENTRYPOINT ["entrypoint.sh"]
