---
name: create-hotfix
description: Create a hotfix (patch) release branch for an Azure SDK Rust crate.
---

# Create a Hotfix

Use the bundled script for deterministic discovery and git operations. Run all commands from the repository root.

1. List workspace crates:

   ```bash
   python3 .github/skills/create-hotfix/create_hotfix.py list-crates
   ```

   Use a single-select prompt to ask the user which crate to patch, using the
   crate names from the JSON output.

2. Prepare the hotfix:

   ```bash
   python3 .github/skills/create-hotfix/create_hotfix.py prepare <crate-name>
   ```

   The command fetches tags and `main` directly from `https://github.com/Azure/azure-sdk-for-rust`, finds the latest stable `<crate-name>@<version>` tag, calculates the next patch version, creates `hotfix/<crate-name>-<version>` from that tag unless already on a `hotfix/` branch, and returns candidate commits from `main` as JSON.

3. If candidates were returned, prompt the user with a multiselect containing
   each candidate's SHA and deterministic summary. Allow selecting none.

4. If commits are selected, preserve their displayed order and run:

   ```bash
   python3 .github/skills/create-hotfix/create_hotfix.py cherry-pick <sha>...
   ```

   If cherry-picking conflicts, stop and report the conflict without aborting or skipping it.

5. Report the selected crate, base tag, patch version, branch, and cherry-picked commits. Tell the user:

   > Make any additional hotfix changes on this branch. Do not merge this branch
   > into `main`; cherry-pick any new fixes into `main` separately. See the
   > [Azure SDK hotfix branch policy](https://azure.github.io/azure-sdk/policies_repobranching.html#hotfix-branches)
   > for more information.
