# AGENTS.md

This document provides guidance for AI agents (e.g., GitHub Copilot, MCP servers, or LLM-based assistants) interacting with the Azure SDK for Rust repository.

## Repository Overview

**Purpose**: The Azure SDK for Rust provides Rust language bindings and client libraries for Azure services, following the [Azure SDK Design Guidelines for Rust](https://azure.github.io/azure-sdk/rust_introduction.html).

**Status**: ⚠️ Under active development. Large breaking changes may occur before 1.0 release.

**Primary Language**: Rust (MSRV: 1.85)

**Key Technologies**:
- Rust toolchain with Cargo
- TypeSpec for API specification and code generation
- OpenTelemetry for distributed tracing
- Test Proxy for recorded integration tests

## Repository Structure

```
.
├── sdk/                    # Service-specific SDK crates (e.g., identity, keyvault, storage)
│   └── <service>/         # Each service has its own crate(s)
├── eng/                   # Engineering system scripts and common tooling
├── doc/                   # Additional documentation
├── .github/              
│   ├── copilot-instructions.md  # Copilot-specific Rust coding guidelines
│   ├── instructions/            # Agent instruction files for specific tasks
│   └── prompts/                 # Reusable Copilot prompts
├── CONTRIBUTING.md       # Contribution guidelines
└── README.md            # Repository overview
```

## Agent Capabilities

### Recommended Actions

AI agents can assist with:

1. **Code Generation**
   - Writing new Rust code following repository conventions (see `.github/copilot-instructions.md`)
   - Generating unit tests using `#[cfg(test)]` modules
   - Creating integration tests with `#[recorded::test]` attributes
   - Generating documentation tests with ` ```rust no_run` blocks

2. **Code Review Support**
   - Identifying potential bugs or safety issues
   - Suggesting improvements for idiomatic Rust patterns
   - Checking adherence to Azure SDK design guidelines
   - Reviewing error handling using `Result<T, E>` types

3. **Documentation**
   - Improving inline documentation (using `///` doc comments)
   - Updating README files
   - Creating or updating CHANGELOG entries (see `.github/instructions/changelog.instructions.md`)
   - Writing code examples in `examples/` directories

4. **Issue Triage**
   - Labeling issues with appropriate tags
   - Identifying duplicate issues
   - Suggesting relevant code owners based on `CODEOWNERS`
   - Summarizing issue discussions

5. **Refactoring**
   - Applying clippy suggestions
   - Improving code organization and modularity
   - Updating dependencies in `Cargo.toml`

### Restricted Actions

AI agents **should not**:

1. **Modify Generated Code**
   - Never edit files in `generated/` subdirectories
   - These are produced by TypeSpec code generators and will be overwritten
   - Instead, propose changes to TypeSpec specifications in [Azure/azure-rest-api-specs](https://github.com/Azure/azure-rest-api-specs)

2. **Break API Compatibility**
   - Avoid introducing breaking changes without explicit approval
   - Check if changes affect public APIs before proceeding
   - Consider deprecation process (see `doc/deprecation-process.md`)

3. **Bypass CI/CD Checks**
   - Do not suggest skipping or disabling CI checks
   - All code must pass `cargo build`, `cargo test`, and `cargo clippy`

4. **Commit Secrets**
   - Never include credentials, keys, or tokens in code
   - Use environment variables for sensitive data
   - Sanitize test recordings to remove secrets

5. **Modify Security or License Files**
   - Do not alter `SECURITY.md`, `LICENSE.txt`, or `CODE_OF_CONDUCT.md` without maintainer approval

## Key Workflows

### Building

```bash
# Build a specific crate
cargo build -p <crate-name>

# Build entire workspace (not recommended unless necessary)
cargo build --workspace
```

### Testing

```bash
# Run tests for a specific crate
cargo test -p <crate-name>

# Run integration tests with recordings
cargo test -p <crate-name> --test <test-name>

# Record new test sessions (requires live resources)
AZURE_TEST_MODE=record cargo test -p <crate-name> --test <test-name>
```

### Linting

```bash
# Check for common issues
cargo clippy -p <crate-name>

# Auto-fix some issues
cargo clippy --fix -p <crate-name>
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

## Coding Standards

Agents should follow guidelines in `.github/copilot-instructions.md`, including:

- **Naming Conventions**: 
  - Types/variants: `PascalCase`
  - Functions/fields/parameters: `snake_case`
  - Constants: `UPPER_SNAKE_CASE`
  - Crates/modules: `snake_case`

- **Import Style**:
  - Explicit imports (no `use foo::*`)
  - Prefer `crate::` for internal references
  - Keep imports at module top level

- **Error Handling**:
  - Use `Result<T, E>` and `?` operator
  - Provide meaningful error messages

- **Documentation**:
  - All public APIs need `///` doc comments
  - Include examples in doc comments where appropriate
  - Use `no_run` for examples that require live resources

- **Testing**:
  - Place unit tests in `#[cfg(test)] mod tests`
  - Use `#[recorded::test]` for integration tests
  - Test names should be descriptive, not prefixed with "test"

## CI/CD Integration

### Automated Checks

All pull requests trigger:
- `cargo build` - Compilation check
- `cargo test` - Unit and integration tests
- `cargo clippy` - Lint checks
- License/CLA verification
- Code coverage analysis

### Test Proxy

Integration tests use the Azure SDK Test Proxy for recording/playback:
- Recordings stored in `<crate>/tests/recordings/`
- Automatically started during test execution
- Set `AZURE_TEST_MODE=record` to update recordings
- Set `PROXY_MANUAL_START=true` to start manually for debugging

## Safety and Security

1. **Code Review**: All changes require review and approval from code owners
2. **Static Analysis**: Must pass `cargo clippy` without warnings
3. **Secret Scanning**: Automated checks prevent committing credentials
4. **Dependencies**: Managed through workspace `Cargo.toml`, vetted for security
5. **Vulnerability Reporting**: Via MSRC at <secure@microsoft.com>

## Cross-References

- **General Rust Guidelines**: `.github/copilot-instructions.md`
- **Contribution Process**: `CONTRIBUTING.md`
- **Changelog Guidelines**: `.github/instructions/changelog.instructions.md`
- **Git Commit Standards**: `doc/git-commit-instructions.md`
- **Deprecation Process**: `doc/deprecation-process.md`
- **Azure SDK Design Guidelines**: https://azure.github.io/azure-sdk/rust_introduction.html

## Agent-Specific Instructions

Additional specialized instructions for specific workflows can be found in:
- `.github/instructions/` - Task-specific agent instructions
- `.github/prompts/` - Reusable Copilot prompts (use `#prompt` in Copilot)

## Getting Help

- **Issues**: https://github.com/Azure/azure-sdk-for-rust/issues
- **Discussions**: Use issue comments or StackOverflow with `azure` + `rust` tags
- **Code Owners**: See `.github/CODEOWNERS` for service-specific contacts

## Telemetry and Privacy

The SDK includes telemetry via `User-Agent` headers. When generating code:
- Respect user telemetry preferences in `ClientOptions`
- Do not log or transmit sensitive data
- Follow Microsoft Privacy Statement: https://go.microsoft.com/fwlink/?LinkID=824704

## License

All contributions are licensed under the MIT License. See `LICENSE.txt`.

---

**Last Updated**: 2025-01-20  
**Version**: 1.0  
**Canonical Spec**: https://agents.md
