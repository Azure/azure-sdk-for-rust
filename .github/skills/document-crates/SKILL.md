---
name: document-crates
description: Plan, create, or update README.md and CONTRIBUTING.md files, especially crate documentation. Use before planning or editing either filename.
---

# Document crates

- Read the target and applicable instructions. For crate docs, use
  [Key Vault secrets](../../../sdk/keyvault/azure_security_keyvault_secrets/README.md)
  as the published template; read only relevant sections.
- Apply the crate template below to crate docs, not repository/tool guides.
  Preserve existing purpose and layout for non-crate files.
- Keep consumer guidance in README; contributor workflows in a sibling
  CONTRIBUTING. Avoid duplication; link shared repository guidance.

## README order

```text
# Azure <service> client library for Rust
<consumer overview>
[Source code] | [Package (crates.io)] | [API reference documentation] | [Product documentation]
## Getting started
### Install the package
### Prerequisites
### Install dependencies
### Authenticate the client
### Instantiate a client
## Key concepts
## Examples
### <primary scenario>
## Troubleshooting
### General
## Contributing
<standard repository contribution link, CLA, Code of Conduct prose>
<!-- LINKS -->
<reference-style definitions>
```

- Keep this order for service clients; add service-specific subsections as needed.
  Omit inapplicable setup steps for non-service crates; do not invent requirements.
- Copy Contributing prose verbatim from the published template. Link its
  `CONTRIBUTING.md` reference to the repository-root GitHub URL, not the sibling.
  Retain the CLA and Code of Conduct links.
- Use actual crate/API/product URLs; Rustdoc links target the published crate.
  Document supported authentication, endpoints, features, and limitations accurately.
- Keep examples consumer-focused; preserve README-test fence labels and bindings.
  Use `rust no_run` for placeholder examples; match existing tested snippet fences.

## CONTRIBUTING content

Start with a repository contribution-guide link. Include only applicable
crate-specific details, in this order:

1. Prerequisites and local setup: tools, permissions, environment-variable names.
2. Generation: pinned inputs/tooling, commands, customization ownership and caveats.
3. Build and offline checks: package-scoped commands, `--all-features` for tests.
4. Integration tests: provisioning, record/playback commands, fixtures, ignored-case
   reasons, resource counts/cost implications, cleanup, and recording publication.
5. Contributor troubleshooting.

Do not include credentials, private payloads, session logs, or transient run status.
Link shared workflows instead of copying them. Do not run live tests or provision
resources merely to validate documentation.

## Finish

Check links, heading order, API accuracy, and preserved example labels. At the end,
use `lint-markdown` and `check-spelling` for changed files; reuse unchanged results.
Run README tests if executable snippets changed.
