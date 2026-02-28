# AGENTS.md

This document provides guidance for AI agents (e.g., GitHub Copilot, MCP servers, or LLM-based assistants) interacting with the Azure SDK for Rust repository.

## Repository Overview

The Azure SDK for Rust provides Rust language bindings and client libraries for Azure services, following the [Azure SDK Design Guidelines for Rust](https://azure.github.io/azure-sdk/rust_introduction.html).

⚠️ Under active development. Large breaking changes may occur before 1.0 release.

- **Primary Language**: Rust (MSRV: 1.88)
- **Key Technologies**: Cargo, TypeSpec, OpenTelemetry, Test Proxy

## Repository Structure

```text
.
├── sdk/                      # Service-specific crates organized by service
│   └── <service>/            # Service directory (e.g., "keyvault", "storage")
│       ├── <crate>/          # Service crate (e.g., "azure_security_keyvault_secrets")
│       ├── assets.json       # Pointer to test recordings (may be under <crate>/)
│       ├── test-resources.bicep # Test resource definitions (may be under <crate>/)
│       └── tsp-location.yaml # Pointer to TypeSpec in azure-rest-api-specs (may be under <crate>/)
├── eng/                      # Engineering system scripts and common tooling
├── doc/                      # Additional documentation
├── .github/
│   ├── instructions/         # Agent instruction files for specific tasks
│   ├── prompts/              # Reusable Copilot prompts
│   └── skills/               # Copilot skills (e.g., check-spelling, lint-markdown)
├── CONTRIBUTING.md           # Contribution guidelines (see for detailed workflows)
└── README.md                 # Repository overview
```

## Agent Capabilities

Always check if there is an MCP tool or skill available before performing operations manually, including listing Azure subscriptions, deploying resources, setting up a new crate, generating code, and other common workflows.

All new crates must be generated from TypeSpec specifications in [Azure/azure-rest-api-specs]. TypeSpec specifications are located under `specification/<service>/`. Use the `create-crate` skill to set up a new crate.

### Recommended Actions

AI agents can assist with:

1. **Code Generation**
   - Writing new Rust code following the coding conventions below
   - Generating unit tests using `#[cfg(test)]` modules
   - Creating integration tests with `#[recorded::test]` attributes (see `CONTRIBUTING.md` for details)
   - Generating documentation tests in `.rs` files (avoid `no_run` when tests can be run)
   - Running `rustfmt` on all generated code to ensure proper formatting

2. **Code Review**
   - Identifying potential bugs or safety issues
   - Suggesting improvements for idiomatic Rust patterns
   - Checking adherence to Azure SDK design guidelines
   - Reviewing error handling using `azure_core::Result<T>`

3. **Documentation**
   - Improving inline documentation (using `///` doc comments)
   - Updating README files (use ` ```rust no_run` for examples with placeholders)
   - Creating or updating CHANGELOG entries (see `.github/instructions/changelog.instructions.md`)
   - Writing hero scenario examples in doc comments (avoid examples in `examples/` directories unless demonstrating primary use cases)

4. **Issue Triage**
   - Labeling issues with appropriate tags
   - Identifying duplicate issues
   - Suggesting relevant code owners based on `CODEOWNERS`
   - Summarizing issue discussions

5. **Refactoring**
   - Applying clippy suggestions
   - Improving code organization and modularity
   - Updating dependencies in `Cargo.toml`
   - Consolidating imports (e.g., `use std::{borrow::Cow, marker::PhantomData};` instead of separate lines)

### Restricted Actions

AI agents **should not**:

1. **Modify Generated Code**
   - Never edit files in `generated/` subdirectories
   - These are produced by TypeSpec code generators and will be overwritten
   - Instead, propose changes to TypeSpec specifications in [Azure/azure-rest-api-specs]

2. **Break API Compatibility**
   - Avoid introducing breaking changes without explicit approval
   - Check if changes affect public APIs before proceeding
   - Consider the deprecation process (see `doc/deprecation-process.md`)

3. **Bypass CI/CD Checks**
   - Do not suggest skipping or disabling CI checks
   - All code must pass `cargo build`, `cargo test`, and `cargo clippy`

4. **Commit Secrets**
   - Never include credentials, keys, or tokens in code
   - Use environment variables for sensitive data
   - Sanitize test recordings to remove secrets

5. **Modify Security or License Files**
   - Do not alter `SECURITY.md`, `LICENSE.txt`, or `CODE_OF_CONDUCT.md` without maintainer approval

6. **Hand-Write Generated Clients**
   - Do not hand-write client, model, or operation code when a TypeSpec specification exists in [Azure/azure-rest-api-specs]
   - Use `tsp-client update` or the `azsdk_package_generate_code` MCP tool to generate client code from TypeSpec
   - Hand-written wrapper code (e.g., custom client constructors in `clients.rs`) on top of generated code is acceptable. See `sdk/keyvault/azure_security_keyvault_secrets` for an example. Check how `src/clients.rs` imports generated clients and how those are exported to avoid duplicate type exports.

## Persona

You are an expert Rust programmer. You write safe, efficient, maintainable, and well-tested code.

- Use an informal tone.
- Do not be overly apologetic and focus on clear guidance.
- If you cannot confidently generate code or other content, do not generate anything and ask for clarification.

## Prerequisites

- To use Azure SDK MCP tool calls, the user must have PowerShell installed. Provide [PowerShell installation instructions](https://learn.microsoft.com/powershell/scripting/install/installing-powershell) if not installed, and recommend restarting the IDE to start the MCP server.

## Coding Conventions

### Naming

- Type names and variants are PascalCase.
- Constants and statics are UPPER_SNAKE_CASE.
- Field, function, parameter, and variable names are snake_case.
- Crate and module names are snake_case.
- Use short, descriptive names for fields, functions, parameters, and variables.

### Imports

- Keep `use` directives at the top of the module in which they are used, and avoid placing them inside functions or blocks unless absolutely necessary.
- Prefer using `crate` in `use` directives to refer to types anywhere in the current crate instead of using its name, or relative paths like `super` or `self`.
- Prefer merging new `use` directives into existing ones rather than creating new `use` blocks.
- All imported types, constants, functions, modules, and macros should be imported explicitly. Never import `*`.

### Error Handling

- Handle errors using Rust's `Result` type with the `?` operator when the parent function returns a `Result`.
- Use `azure_core::Result<T>` for public APIs.

### Documentation

- Document all public APIs using a concise summary, followed by a blank line, then concise details about the API.
- Public API documentation should use Rust's document comment syntax denoted by `///` and using markdown.
- In README markdown files, use ` ```rust no_run` for examples with placeholders.

### Dependencies

- Dependencies should be defined in the root workspace's `Cargo.toml` file.
- Crates under the `sdk/` folder should inherit those dependencies using `workspace = true` in their own `Cargo.toml` files.

### General

- Prioritize safety, efficiency, and correctness.
- Respect Rust's ownership and borrowing rules.
- Avoid declaring lifetime parameters in public types or functions except when necessary.
- If you have trouble generating safe, efficient, maintainable, and lint-free code, insert a `TODO` comment describing what should happen.
- Do not modify generated code, found in `generated` subdirectories. These files are generated by external tools and should not be edited manually.
- When searching for function call chains in Rust code, be aware that rustfmt often formats method chains across multiple lines like `obj\n    .foo()\n    .bar()`. Use multi-line search patterns (e.g., `rg` with `-U` flag) or search for individual method names rather than complete call chains.

## Building

```bash
# Build a specific crate
cargo build -p <crate-name>

# Build entire workspace (not recommended unless necessary)
cargo build --workspace
```

### Code Generation

For crates with TypeSpec specifications:

```bash
cd sdk/<service>/<crate-name>
tsp-client update
```

### Running Examples

```bash
cargo run --package <crate-name> --example <example-name>
```

## Testing

When running `cargo test`, use `--all-features` to ensure no tests are missed.

```bash
# Run tests for a specific crate
cargo test -p <crate-name> --all-features

# Run integration tests with recordings
cargo test -p <crate-name> --test <test-name>

# Provision test resources (see CONTRIBUTING.md for details)
eng/common/TestResources/New-TestResources.ps1 -ServiceDirectory <service>

# Record new test sessions (requires provisioned resources)
AZURE_TEST_MODE=record cargo test -p <crate-name> --test <test-name>
```

See `CONTRIBUTING.md` for comprehensive testing guidance including debugging, Test Proxy usage, and trace logging.

### Test Generation

- Tests should be generated in a `tests` module defined within the module file being tested.
- The `tests` module should be defined at the bottom after all the existing code to test.
  It may already be a separate file named `tests.rs` next to `lib.rs` or `mod.rs` files,
  or as `foo/test.rs` for a module named `foo.rs`.
- If the `tests` module already exists, only add test functions and merge imports as needed.
- The `tests` module should be conditioned on `#[cfg(test)]`.
- The `tests` module should always import APIs from `super`.
- Do not begin test function names with "test" unless necessary to disambiguate from the function being tested.
- Test functions do not need to be public.

## Linting and Formatting

```bash
# Check for common issues
cargo clippy -p <crate-name>

# Auto-fix some issues
cargo clippy --fix -p <crate-name>

# Format code
cargo fmt -p <crate-name>
```

Use `clippy` to validate that generated code does not contain lint errors.

## CI/CD Integration

All pull requests trigger:

- `cargo build` - Compilation check
- `cargo test` - Unit and integration tests
- `cargo clippy` - Lint checks
- `cargo fmt --check` - Format validation
- License/CLA verification
- Code coverage analysis

Integration tests use the Azure SDK Test Proxy for recording/playback. See `CONTRIBUTING.md` for Test Proxy setup and usage.

## Safety and Security

1. **Code Review**: All changes require review and approval from code owners
2. **Static Analysis**: Must pass `cargo clippy` without warnings
3. **Secret Scanning**: Automated checks prevent committing credentials
4. **Dependencies**: Managed through workspace `Cargo.toml`, vetted for security
5. **Vulnerability Reporting**: Via MSRC at <secure@microsoft.com>

## Agent-Specific Instructions

Additional specialized instructions for specific workflows can be found in:

- `.github/instructions/` - Task-specific instructions (loaded when pattern-matched)
- `.github/prompts/` - Reusable Copilot prompts (use `#prompt` in Copilot)
- `.github/skills/` - Copilot skills for common tasks:
  - `check-spelling` - Check and fix spelling in project source files using cSpell
  - `create-crate` - Create a new Azure SDK crate from a TypeSpec specification
  - `lint-markdown` - Check and fix formatting in markdown files using markdownlint-cli2

## Cross-References

- **Contributing Guide**: `CONTRIBUTING.md`
- **Changelog Updates**: `.github/instructions/changelog.instructions.md`
- **Git Commit Standards**: `.github/instructions/git-commit.instructions.md`
- **GitHub Pull Request Standards**: `.github/instructions/github-pullrequest.instructions.md`
- **PowerShell Scripts**: `.github/instructions/pwsh.instructions.md`
- **Deprecation Process**: `doc/deprecation-process.md`
- **Azure SDK Design Guidelines**: <https://azure.github.io/azure-sdk/rust_introduction.html>

## Getting Help

- **Issues**: <https://github.com/Azure/azure-sdk-for-rust/issues>
- **Discussions**: Use issue comments or StackOverflow with `azure` + `rust` tags
- **Code Owners**: See `.github/CODEOWNERS` for service-specific contacts

---

**Last Updated**: 2026-02-28
**Version**: 2.0
**Canonical Spec**: <https://agents.md>

[Azure/azure-rest-api-specs]: https://github.com/Azure/azure-rest-api-specs
