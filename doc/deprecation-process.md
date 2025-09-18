# Rust Crate Deprecation Process

## Overview

This guide describes the step-by-step process for deprecating a Rust crate in the `azure-sdk-for-rust` repository. You likely need to read this if you are a crate maintainer and need to explain to your users that the crate should no longer be used.

Note that a deprecated crate is a signal to users that they are strongly encouraged to stop using it and migrate to another crate. The crate is still available to install from crates.io and is not yanked, and we still have the ability to publish critical security fixes as necessary.

The overall approach for Rust crates is:

-   Add deprecation attributes to public APIs in the code to provide compile-time warnings
-   Update the README.md file with a deprecation disclaimer
-   Update the CHANGELOG.md file to document the deprecation
-   Add deprecation metadata to Cargo.toml
-   Publish a new release to crates.io
-   Update the API reference documentation to show the deprecated status
-   Eventually archive the crate on crates.io if the service is fully retired

## Pre-deprecation: Blog Post

If applicable, consider adding a post to the Azure Blog stating that:

-   A new crate is available which replaces the old crate
-   The old crate is scheduled to be deprecated on a specific date
-   Guidance on adjusting code to use the new crate

Reach out to the Rust Azure SDK PM if you have any questions about creating a blog post.

## Step 1: Updates to the crate files

Clone the `azure-sdk-for-rust` repository and update the following files of your crate.

### Cargo.toml

Add deprecation metadata to the crate's `Cargo.toml` file:

```toml
[package]
# ... existing fields ...

[package.metadata.deprecation]
deprecated = true
deprecation_date = "2024-12-31"  # Format: YYYY-MM-DD
reason = "This crate has been deprecated. Use azure_new_crate instead."
replacement = "azure_new_crate"  # Optional: name of replacement crate
migration_guide = "https://aka.ms/azsdk/rust/migrate/new-crate"  # Optional
```

### README.md

A disclaimer should be added indicating the end-of-life date (EOLDate) of the crate and directing to a replacement crate and migration guide as necessary.

-   The EOLDate should be in the format `MM-DD-YYYY`.
    -   If there is no replacement crate, the crate EOLDate should be the service retirement date.
    -   If there is a replacement crate, the EOLDate should be the same as the deprecation release date of the old crate in the CHANGELOG.md.
    -   Service retirement dates MAY be listed in the [Azure Services Retirement Workbook](https://aka.ms/servicesretirementworkbook), where retiring feature says 'Entire service'.
-   The link to the replacement crate should be a crates.io link: `https://crates.io/crates/azure_new_crate`.
-   The link to the migration guide should be a link in the format `https://aka.ms/azsdk/rust/migrate/new-crate`. To create this aka.ms link, follow the "How to create aka.ms links" section [here](https://dev.azure.com/azure-sdk/internal/_wiki/wikis/internal.wiki/233/Azure-SDK-AKA.ms-Links?anchor=how-to-create-aka.ms-links).
    -   NOTE: You may decide to postpone or skip writing a migration guide based on download numbers (found on [crates.io](https://crates.io/), [lib.rs](https://lib.rs/), etc.) and internal knowledge of the usage of the crate.

Replace ALL existing text with a disclaimer in the following format:

**If a replacement crate and migration guide exist:**

```markdown
# Microsoft Azure SDK for Rust

> [!CAUTION]
> This crate has been deprecated and will no longer be maintained after <EOLDate>. This crate will only receive security fixes until <EOLDate>. To receive updates on new features and non-security bug fixes, upgrade to the replacement crate, [azure_new_crate](https://crates.io/crates/azure_new_crate). Refer to the [migration guide](https://aka.ms/azsdk/rust/migrate/new-crate) for guidance on upgrading.
```

**If a migration guide is not provided:**

```markdown
# Microsoft Azure SDK for Rust

> [!CAUTION]
> This crate has been deprecated and will no longer be maintained after <EOLDate>. This crate will only receive security fixes until <EOLDate>. To receive updates on new features and non-security bug fixes, upgrade to the replacement crate, [azure_new_crate](https://crates.io/crates/azure_new_crate).
```

**If a replacement crate does not exist:**

```markdown
# Microsoft Azure SDK for Rust

> [!CAUTION]
> This crate has been deprecated and will no longer be maintained after <EOLDate>. This crate will only receive security fixes until <EOLDate>.
```

**If a new service has replaced the service, and existing customers should be directed to the new service's REST API docs/repo:**

```markdown
# Microsoft Azure SDK for Rust

> [!CAUTION]
> This crate has been deprecated and will no longer be maintained after <EOLDate>. This crate will only receive security fixes until <EOLDate>. Refer to the samples in the [My New Service repo](https://github.com/microsoft/my-new-service/tree/main) instead.

For additional support, open a new issue in the [Issues](https://github.com/microsoft/my-new-service/issues) section of the My New Service repo.
```

### CHANGELOG.md and Cargo.toml version

-   Update the version in the crate's `Cargo.toml` file to the next patch version if the crate has had a stable release, or the next pre-release version if the crate has only been in pre-release. For example:

    -   If a stable version WAS NEVER RELEASED and the last released version was 1.0.0-beta.1, the new version should be 1.0.0-beta.2.
    -   If a stable version HAS BEEN RELEASED and the last released version was 1.2.3-beta.1, the new version should be 1.2.4.
    -   If the last released version was 1.2.3, the new version should be 1.2.4.

-   In `CHANGELOG.md`, add the new version with the same disclaimer as in the `README.md`, along with a release date. No other changes/features added/breaking changes should be included for this version. For example:

```markdown
## 1.2.4 (2024-03-31)

### Other Changes

-   This crate has been deprecated and will no longer be maintained after 12-31-2024. This crate will only receive security fixes until 12-31-2024. To receive updates on new features and non-security bug fixes, upgrade to the replacement crate, [azure_new_crate](https://crates.io/crates/azure_new_crate). Refer to the [migration guide](https://aka.ms/azsdk/rust/migrate/new-crate) for guidance on upgrading.
```

### Source Code Deprecation Attributes

Add `#[deprecated]` attributes to all public APIs in the crate to provide compile-time warnings to users:

```rust
#[deprecated(since = "1.2.4", note = "This crate has been deprecated. Use azure_new_crate instead.")]
pub struct MyStruct {
    // ...
}

#[deprecated(since = "1.2.4", note = "This crate has been deprecated. Use azure_new_crate instead.")]
pub fn my_function() {
    // ...
}

#[deprecated(since = "1.2.4", note = "This crate has been deprecated. Use azure_new_crate instead.")]
pub mod my_module {
    // ...
}
```

For the main crate-level documentation, add a deprecation notice:

```rust
//! # Microsoft Azure SDK for Rust
//!
//! > **Warning**: This crate has been deprecated and will no longer be maintained after 12-31-2024.
//! > To receive updates on new features and non-security bug fixes, upgrade to the replacement crate,
//! > [`azure_new_crate`](https://crates.io/crates/azure_new_crate).
//!
//! [migration guide]: https://aka.ms/azsdk/rust/migrate/new-crate

#![deprecated(since = "1.2.4", note = "This crate has been deprecated. Use azure_new_crate instead.")]
```

### CI Configuration

-   Ensure the crate is listed in the workspace CI configuration so that the artifact is generated for release.
-   Consider adding the crate to a "deprecated" group in CI configurations to reduce unnecessary checks while still allowing security updates.

## Step 2: Resolve all open issues/PRs corresponding to the crate

If there is a replacement crate, provide a link to the new crate or an existing migration guide before closing issues and PRs.

## Step 3: Create a PR

Create a PR targeting the `main` branch.

### Fix any CI issues

Wait for the CI to run. Fix any issues related to deprecation in the PR, such as CHANGELOG.md or README.md formatting.

There should not be any major test failures as deprecated crates should still function correctly.

### Post your PR in the Rust review channel

Post your PR in the review channel for Rust for verification that all requirements for deprecation have been met. If you are not the codeowner, please explicitly tag the codeowner in the post for approval.

### Merge PR

Once the PR is approved, merge.

## Step 4: Publish the release

### Pre-Release

Before publishing, ensure all changes are properly documented and the version has been incremented correctly.

### Publish the Crate

Trigger the release pipeline for the crate.

Note: This release DOES NOT need to be done during release week and can be done any time.

### Post-Release

Check to make sure that the new version of the crate has been published on crates.io and that the deprecation warnings are visible in the crate documentation.

## Step 5: Create a new PR to remove the crate from the main branch

After the deprecated version has been published and sufficient time has passed (typically 12 months), consider removing the crate from the main branch:

-   Append a note to the README.md deprecation message stating the crate has been removed from the main branch, with links to the latest release tag and crate on crates.io.

```markdown
# Microsoft Azure SDK for Rust

> [!CAUTION]
> This crate has been deprecated and will no longer be maintained after <EOLDate>. This crate will only receive security fixes until <EOLDate>.

Crate source code and samples have been removed from the `main` branch and can be found under the release tag for the latest version. See [azure_my_crate_v1.2.4](https://github.com/Azure/azure-sdk-for-rust/tree/azure_my_crate_v1.2.4/sdk/mycrate/azure_my_crate). The latest release can be found on [crates.io](https://crates.io/crates/azure_my_crate).

If you have any questions, please open a [GitHub Issue](https://github.com/Azure/azure-sdk-for-rust/issues) or contact the team.
```

-   Delete all files in the crate directory EXCEPT for the README.md at the crate directory root.
-   Remove the crate from the workspace `Cargo.toml` members list.
-   Remove the crate from any CI configuration files.
-   Create a new PR targeting the `main` branch of the repository.
-   Post the PR in the review channel for Rust.
-   Once the PR has been approved by codeowner, merge.
-   You're responsible for fixing any CI issues related to this PR.

## Step 6: Update API Documentation

### Remove the entry in the docs.rs documentation

The deprecated crate will automatically show deprecation warnings on docs.rs. No manual action is typically required, as the deprecation attributes will be reflected in the generated documentation.

### Update Azure SDK package resource

-   Create your own fork of the [azure-sdk](https://github.com/Azure/azure-sdk) repo and clone it, if you haven't already.
-   Create a branch in your local copy of the repo: `git checkout -b rust/azure_my_crate_deprecation`
-   Open the `_data/releases/latest/rust-packages.csv` file.
-   Find the entry for your crate and update the following fields:
    -   `EOLDate`: In MM/DD/YYYY format. If the SDK deprecation is due to a service retirement, this date should match the service final retirement date. If there is a replacement crate, this should match the release date of the deprecated crate.
    -   `Support`: Change the value to `deprecated`.
    -   `Replace`: If it exists, set the value to the name of the Azure SDK for Rust crate meant to replace the crate being deprecated. If not, set the value to `NA`.
    -   `ReplaceGuide`: If it exists, link to a migration guide in the following format: `aka.ms/azsdk/rust/migrate/<crate>`. If not, set the value to `NA`.
-   Note: If you are deprecating multiple crates, please wait until all deprecated crates have been published and update all entries necessary in one PR.
-   Create a PR to push these changes. Checks will run to notify the repo owners to review your commit.

### Archive the crate on crates.io

If the service is retired and users should not expect to receive any future updates, including security fixes or maintenance, your crate can be yanked from crates.io as a last resort. However, this should be done very carefully as it can break existing builds.

Instead of yanking, consider:

1. Keeping the deprecated version available but not publishing new versions
2. Using the deprecation metadata in Cargo.toml to clearly mark the crate as deprecated
3. Providing clear migration paths in documentation

To yank a version (use with extreme caution):

```bash
cargo yank --vers 1.2.4 azure_my_crate
```

Note: Yanking does not remove the crate from the index entirely, but prevents new projects from depending on it.

### Update overview/conceptual documentation that points to deprecated crates

Review and update any Azure documentation that references the deprecated crate:

-   These will be on the MS Learn page
-   Search for mentions of the deprecated crate in Azure documentation
-   Update examples and tutorials to use the replacement crate
-   Add migration guidance where appropriate

## Additional Resources

-   [Azure SDK for Rust Guidelines](https://azure.github.io/azure-sdk/rust_introduction.html)
-   [Cargo Book - Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)
-   [RFC 1270 - Deprecation](https://github.com/rust-lang/rfcs/blob/master/text/1270-deprecation.md)
