---
applyTo: "sdk/cosmos/**/CHANGELOG.md"
---

- Goal: every customer-visible change gets a changelog entry so users can quickly understand the risk/benefit of upgrading.

- Never modify already released sections (anything not marked "Unreleased"). Only add entries under the current "(Unreleased)" version.

- If the PR title starts with `[Internal]`, do not change any `CHANGELOG.md` files.

- Add one entry per PR under an existing category header (for example: "Features Added", "Breaking Changes", "Bugs Fixed", "Other Changes").
	- Do not create new category headers.
	- Put the entry under the most appropriate existing category.

- Entry format:
	- Prefer the existing style used in the file.
	- When the file uses PR-linked bullets, use the same pattern:
		- `- [#12345](https://github.com/Azure/azure-sdk-for-rust/pull/12345) <one-line summary of the change>`
	- If you are working on a local branch and there is no PR yet, you may add the one-line summary without the link.
		- Once a PR exists (even before merge), update the entry to include the PR link.

- For the first unreleased version section, keep all standard category headers present even if some are empty.

- Keep entries concise, accurate, and relevant to the release.

- Reference example (style only):
	- https://github.com/Azure/azure-sdk-for-java/blob/main/sdk/cosmos/azure-cosmos/CHANGELOG.md

