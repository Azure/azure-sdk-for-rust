FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

USER vscode

# Install pinned stable from rust-toolchain.toml. Pinned nightly setup is done in .devcontainer/oncreate.
RUN rustup install

# Install pre-requisite tools
RUN cargo install \
    cargo-watch \
    http-server
