---
name: update-emitter
description: Update the TypeSpec emitter for Rust and optionally regenerate all clients
---

# Update the TypeSpec emitter

Run `eng/scripts/Update-Emitter.ps1` to update `eng/emitter-package.json` to the latest `@azure-tools/typespec-rust` version and regenerate the lock file.

## Regenerating clients

After updating the emitter, service crates should be regenerated to pick up the new version.

Use the repository-pinned `tsp-client`; do not invoke a globally installed binary. From the repository root, install its locked dependencies:

```bash
_TspClientDir="$(pwd)/eng/common/tsp-client"
npm ci --prefix "$_TspClientDir"
```

Run `tsp-client` with `npm exec --prefix "$_TspClientDir" --no --` while the current directory is the client library root. Service owners should regenerate only their own service crates.

To regenerate a single crate:

```bash
cd sdk/{service-directory}/{crate-directory}
npm exec --prefix "$_TspClientDir" --no -- tsp-client update
```

To regenerate all crates under a service directory from the repository root:

```bash
find sdk/{service-directory} -name tsp-location.yaml -execdir \
  npm exec --prefix "$_TspClientDir" --no -- tsp-client update \;
```

See [`eng/common/tsp-client/README.md`](../../../eng/common/tsp-client/README.md) for prerequisites, additional commands, and package management details.

## After regenerating

- Build and test each affected crate to verify correctness.
- Review any changes to public APIs and update each crate's `CHANGELOG.md` accordingly.
