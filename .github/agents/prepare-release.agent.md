---
name: "Release Preparer"
description: "Prepare a package release PR"
---

You will prepare a release PR for a package the user must specify

## Prerequisites

Before starting, verify the user is logged in to the Azure CLI:

```bash
az account show --query name -o tsv
```

If this command fails (exit code non-zero), the user is not logged in. **Do not log in for them.** Stop and ask the user what they want to do. They may want to:
- Log in themselves using `az login`
- Have you initiate the login flow (but they will complete the interactive steps)
- Cancel the release preparation

## Steps

1. **Create a worktree from main**

   Create a new git worktree based off `main` in a temporary directory.
   If an `upstream` remote exists (indicating the user is working in a fork), fetch from `upstream`; otherwise fetch from `origin`:

   ```bash
   WORKTREE_DIR=$(mktemp -d)
   if git remote get-url upstream &>/dev/null; then
     MAIN_REMOTE=upstream
   else
     MAIN_REMOTE=origin
   fi
   git fetch "$MAIN_REMOTE" main
   git worktree add "$WORKTREE_DIR" "$MAIN_REMOTE/main"
   ```

2. **Run the Prepare-Release script**

   Change to the worktree directory and run the release preparation script:

   ```bash
   cd "$WORKTREE_DIR"
   pwsh eng/common/scripts/Prepare-Release.ps1 -PackageName ${input:crateName} -ReleaseVersion <version>
   ```

   **Important:** You must set `-ReleaseVersion` to prevent interactive prompts. The version should be the current version from the package's `Cargo.toml`.

   If the user specifies a release date, also pass `-ReleaseDate <date>` (format: `YYYY-MM-DD`). If not specified, the script defaults to today's date.

   This script will:
   - Read the current version from the package
   - Update DevOps release tracking items
   - Validate and update the CHANGELOG.md with a release date

3. **Determine the version and create a branch**

   After the script runs, determine the version from the updated CHANGELOG.md or Cargo.toml.
   The service directory can be found from the package's location under `sdk/`.

   Create a branch named `{service-directory}-release-{version}` where:
   - `{service-directory}` is the directory under `sdk/` (e.g., `core`, `cosmos`, `keyvault`)
   - `{version}` is the release version (e.g., `0.30.0`)

   For example, if releasing `azure_cosmos` version `0.30.0`, the branch would be `cosmos-release-0.30.0`.

   ```bash
   cd "$WORKTREE_DIR"
   # Extract service directory from package location
   SERVICE_DIR=$(find sdk -name "Cargo.toml" -exec grep -l "name = \"${input:crateName}\"" {} \; | head -1 | cut -d'/' -f2)
   # Extract version from Cargo.toml
   VERSION=$(grep -A5 "^\[package\]" $(find sdk -name "Cargo.toml" -exec grep -l "name = \"${input:crateName}\"" {} \; | head -1) | grep "^version" | sed 's/.*"\(.*\)"/\1/')
   BRANCH_NAME="${SERVICE_DIR}-release-${VERSION}"
   git checkout -b "$BRANCH_NAME"
   ```

4. **Commit the CHANGELOG.md changes**

   Stage and commit only the CHANGELOG.md changes:

   ```bash
   cd "$WORKTREE_DIR"
   git add "**/CHANGELOG.md"
   git commit -m "Prepare ${input:crateName} $VERSION for release"
   ```

5. **Push the branch to origin**

   Push the release branch to the remote:

   ```bash
   cd "$WORKTREE_DIR"
   git push -u origin "$BRANCH_NAME"
   ```

6. **Clean up the worktree**

   After pushing, clean up the worktree:

   ```bash
   cd -
   git worktree remove "$WORKTREE_DIR"
   ```

7. **Create a Pull Request**

   Use the GitHub CLI to create a pull request for the release branch:

   ```bash
   gh pr create \
     --repo Azure/azure-sdk-for-rust \
     --base main \
     --head <your-fork>:<branch-name> \
     --title "Prepare <package-name> <version> for release" \
     --body "This PR prepares <package-name> version <version> for release."
   ```

   Replace `<your-fork>` with the owner of the fork (e.g., your GitHub username) if working from a fork. If working directly in the Azure/azure-sdk-for-rust repository, omit the `<your-fork>:` prefix from `--head`.

## Notes

- The `-ReleaseVersion` parameter is required to prevent interactive prompts.
- If the script modifies files beyond CHANGELOG.md (like Cargo.toml for version bumps), include those in the commit as well.
