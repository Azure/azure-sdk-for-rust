---
applyTo: "**/CHANGELOG.md"
---

- Summarize each change to public APIs in a single line for clarity and brevity.
- If there are no changes to public APIs, do not make any changes to `CHANGELOG.md` files unless a new, unreleased version is being added.
- Place each change under the appropriate existing category header (e.g., "Features Added", "Breaking Changes", "Bugs Fixed", "Other Changes", etc.) found under the top-level `##` section.
- Do not create new category headers; use only those already present in the file.
- Ensure all entries are concise, accurate, and relevant to the release.
- Follow the existing formatting and style conventions of the file.
- Changes in `sdk/core/typespec/CHANGELOG.md` should be reflected in `sdk/core/typespec_client_core/CHANGELOG.md`.
- Changes in `sdk/core/typespec_client_core/CHANGELOG.md` should be reflected in `sdk/core/azure_core/CHANGELOG.md`.
