FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

USER vscode

# Install nightly toolchain to give access to nightly features for docs-site build.
RUN rustup toolchain install nightly

# Install pre-requisite tools
RUN cargo install \
    cargo-watch \
    http-server
