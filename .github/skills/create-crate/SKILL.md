---
name: create-crate
description: Create a new Azure SDK crate from a TypeSpec specification.
---

# Creating a New Crate

All new service crates must be generated from TypeSpec specifications in [Azure/azure-rest-api-specs](https://github.com/Azure/azure-rest-api-specs). Do not hand-write client libraries from scratch.

1. **Install tsp-client**
   - Use the repository-pinned CLI; do not invoke a globally installed binary:

     ```bash
     _TspClientDir="$(pwd)/eng/common/tsp-client"
     npm ci --prefix "$_TspClientDir"
     ```

   - Run commands with `npm exec --prefix "$_TspClientDir" --no -- tsp-client`
   - See [`eng/common/tsp-client/README.md`](../../../eng/common/tsp-client/README.md) for prerequisites and more context

2. **Find the TypeSpec spec**
   - Look for a `tspconfig.yaml` under `specification/{service}/` in [azure-rest-api-specs](https://github.com/Azure/azure-rest-api-specs)
   - Check the `tspconfig.yaml` for `@azure-tools/typespec-rust` emitter configuration
   - If no emitter configuration exists, stop and report an error indicating that the TypeSpec specification in [Azure/azure-rest-api-specs](https://github.com/Azure/azure-rest-api-specs) must be updated before generating the client

3. **Initialize the crate**
   - From the repository root, run `npm exec --prefix "$_TspClientDir" --no -- tsp-client init --tsp-config {url}`
   - `{url}` is the GitHub URL to the `tspconfig.yaml`
   - Always use a specific commit SHA in the URL — never a branch or tag, which can move
   - Example: `https://github.com/Azure/azure-rest-api-specs/blob/{commit-sha}/specification/{service}/{rp}/tspconfig.yaml`

4. **Generate the client**
   - Use the `azsdk_package_generate_code` MCP tool, or
   - From the crate directory, run `npm exec --prefix "$_TspClientDir" --no -- tsp-client update`

5. **Add hand-written wrappers**
   - Create `clients.rs` (or similar) for custom client constructors, authentication setup, and convenience methods
   - Build on top of the generated code in `src/generated/`

6. **Register the crate**
   - Add the new crate to the workspace `members` list in the root `Cargo.toml`

7. **Add tests, examples, and documentation**
   - Write unit tests and integration tests (using `#[recorded::test]`)
   - Add examples, a `README.md`, and a `CHANGELOG.md`
