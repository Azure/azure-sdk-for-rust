# Cosmos Documentation Instructions

- Start with [README.md](README.md), then read [Project.md](Project.md) and
  [Architecture.md](Architecture.md) for project context.
- Inventory the available specs, ADRs, and reports through README, then read
  only the documents relevant to the task. Do not load the entire documentation
  set.
- Keep durable Cosmos design documentation under this directory.
- Keep only `Project.md`, `Architecture.md`, `README.md`, and `AGENTS.md` at
  the top level.
- Put mutable feature designs in `specs/` using the next available
  `NNNN-kebab-case-name.md` number.
- Put only finalized, cross-cutting architecture decisions in `adrs/` using the
  next available `NNNN-kebab-case-name.md` number. An ADR must capture a core
  constraint with meaningful alternatives that agents and maintainers must
  respect across features.
- Keep feature-specific choices, detailed mechanics, and evolving design
  rationale in the relevant spec, even when the choice is important.
- Treat accepted ADRs as immutable. Add a new ADR that explicitly supersedes an
  old decision instead of rewriting the old ADR.
- The 2026 core-ADR rewrite was a one-time, human-approved cleanup of the
  pre-existing feature ADRs. It does not create a general exception to ADR
  immutability.
- Put historical investigations and measurement results in `reports/` without
  sequence numbers.
- Update [README.md](README.md) and affected links whenever adding, moving, or
  superseding a document.
- Link to detailed specs and ADRs instead of duplicating their content in
  `Project.md` or `Architecture.md`.
- Leave crate READMEs, changelogs, contributor guidance, and source-consumed
  Rustdoc fragments with their crates.
