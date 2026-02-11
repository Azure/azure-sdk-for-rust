---
applyTo: "sdk/cosmos/**/CHANGELOG.md"
---
- Follow the [Azure SDK changelog instructions for Rust](https://github.com/Azure/azure-sdk-for-java/blob/main/.github/changelog.instructions.md) as the primary reference for changelogs unless they conflict with Cosmos-specific requirements outlined below or in other `cosmos.*.instructions.md` files.

- Goal: every customer-visible change gets a changelog entry so users can quickly understand the risk/benefit of upgrading.

- Never modify already released sections (anything not marked "Unreleased"). Only add entries under the current "(Unreleased)" version.

- Before modifying CHANGELOG.md:
    - Check if the current branch already has an associated PR with a changelog entry
    - If a changelog entry for the current PR already exists, DO NOT add additional entries
    - All changes within a single PR should be summarized in ONE changelog entry
    - The PR author is responsible for updating the existing entry if needed, not the AI agent

- If the PR title starts with `[Internal]`, do not change any `CHANGELOG.md` files.

- Add one entry per PR under an existing category header (for example: "Features Added", "Breaking Changes", "Bugs Fixed", "Other Changes").
	- Do not create new category headers.
	- Put the entry under the most appropriate existing category.

- Entry format:
	- Prefer the existing style used in the file.
	- When the file uses PR-linked bullets, use the same pattern:
		- Prefer placing the PR link at the end, matching existing Cosmos changelog entries:
			- `- <one-line summary of the change>. ([#12345](https://github.com/Azure/azure-sdk-for-rust/pull/12345))`
	- If you are working on a local branch and there is no PR yet, you may add the one-line summary without the link.
		- Once a PR exists (even before merge), update the entry to include the PR link.

- PR number/link discovery (do this automatically when a PR exists):
	- Determine the current branch name: `git rev-parse --abbrev-ref HEAD`.
	- Prefer using GitHub CLI if available/authenticated:
		- `gh pr view --repo Azure/azure-sdk-for-rust --json number,url`
		- or `gh pr list --repo Azure/azure-sdk-for-rust --head <forkOwner>:<branch> --json number,url --limit 1`
	- If `gh` is unavailable, search GitHub PRs by head branch (works in a browser):
		- Query: `is:pr head:<forkOwner>:<branch>`
		- URL: `https://github.com/Azure/azure-sdk-for-rust/pulls?q=is%3Apr+head%3A<forkOwner>%3A<branch>`

- For the first unreleased version section, keep all standard category headers present even if some are empty.

- Keep entries concise, accurate, and relevant to the release.

- Reference example (style only):
	- https://github.com/Azure/azure-sdk-for-java/blob/main/sdk/cosmos/azure-cosmos/CHANGELOG.md
