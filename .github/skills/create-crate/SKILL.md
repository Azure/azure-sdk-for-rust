---
name: create-crate
description: Create a new Azure SDK crate from a TypeSpec specification.
---

# Creating a New Crate

All new service crates must be generated from TypeSpec specifications in [Azure/azure-rest-api-specs](https://github.com/Azure/azure-rest-api-specs). Do not hand-write client libraries from scratch.

1. **Install tsp-client**
   - Follow `eng/common/tsp-client/README.md` to install dependencies
   - Run `npm ci` from `eng/common/tsp-client/`
   - Use `npx --prefix eng/common/tsp-client tsp-client <command>` to invoke it from the repository root

2. **Find the TypeSpec spec**
   - Look for a `tspconfig.yaml` under `specification/<service>/` in [azure-rest-api-specs](https://github.com/Azure/azure-rest-api-specs)
   - Check the `tspconfig.yaml` for `@azure-tools/typespec-rust` emitter configuration

3. **Initialize the crate**
   - Run `tsp-client init --tsp-config <url>` from the repository root
   - `<url>` is the GitHub URL to the `tspconfig.yaml`
   - Always use a specific commit SHA in the URL — never a branch or tag, which can move
   - Example: `https://github.com/Azure/azure-rest-api-specs/blob/<commit-sha>/specification/<service>/<rp>/tspconfig.yaml`

4. **Generate the client**
   - Use the `azsdk_package_generate_code` MCP tool, or
   - Run `tsp-client update` from the crate directory

5. **Add hand-written wrappers**
   - Create `clients.rs` (or similar) for custom client constructors, authentication setup, and convenience methods
   - Build on top of the generated code in `src/generated/`

6. **Register the crate**
   - Add the new crate to the workspace `members` list in the root `Cargo.toml`

7. **Add tests, examples, and documentation**
   - Write unit tests and integration tests (using `#[recorded::test]`)
   - Add examples, a `README.md`, and a `CHANGELOG.md`
