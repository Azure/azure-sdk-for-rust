---
name: prepare-release
description: Prepare one or more crates for a stable or beta (preview) release
---

# Prepare a release

Use the bundled PowerShell helper for deterministic discovery and edits, then
orchestrate the existing build, pack, and release scripts. Run from the repository
root. Do not publish, create tags, commit, push, or switch branches.

**Manifest package versions already represent the next release.** Versions are
bumped after every release. Never automatically increase a version, strip a beta
suffix, or calculate a new release version. Only lower dependency requirements
for substantively unchanged crates to their last applicable released versions;
leave their own package versions unchanged.

**Stable releases must never depend on preview releases.** When preparing a
stable release, assess dependencies for substantive changes since their last
stable release, not their latest preview. A dependency unchanged since a preview
may still contain changes that have never shipped in a stable release.

## Analyze

1. Collect one or more service directories (such as `core` and `identity`), or
   explicit package names across services. Ask whether this is a stable or beta
   release if unspecified. Use the supplied release date, otherwise today's
   local date in `yyyy-MM-dd` format. Normalize service paths such as `sdk/core`
   to their directory names before passing them to the helper.
2. Use the current agent session's artifact directory for `-SessionDirectory`.
   Keep that directory consistent across repeated calls, including one-crate
   calls. Run the helper through PowerShell:

   ```powershell
   pwsh -File ./eng/scripts/Prepare-Release.Helpers.ps1 -FromSkill `
     -ServiceDirectory core,identity -ReleaseType Stable `
     -SessionDirectory <session-artifact-directory>
   ```

   Alternatively, pass `-PackageName azure_core,azure_identity` instead of
   `-ServiceDirectory`. Pass `-ReleaseDate yyyy-MM-dd` if supplied. Optionally use
   `-PlanPath <session-artifact-directory>/release-core.json` to keep multiple
   analyses. The helper is deliberately non-executable, has no shebang, and
   refuses to run without `-FromSkill`: this is an accidental-execution guard.
3. Read the resulting decision file. The helper enumerates
   `sdk/<service>/*/Cargo.toml` or finds package names under `sdk/*/*/Cargo.toml`,
   excludes `publish = false`, and checks package names against directory names.
   Cargo metadata is isolated behind `Get-ReleaseInventory` so manifest discovery
   can replace it later.

The helper fetches tags directly from
`https://github.com/Azure/azure-sdk-for-rust.git` once per successful session and
repository combination, regardless of remote names. Repeated calls reuse a
session marker; failures do not mark the fetch complete. Never force-overwrite
conflicting tags or fall back to stale tags after a failed fetch.

Stable analysis uses the latest stable tag; beta analysis uses the latest stable
or prerelease tag. Versions are ordered semantically across all major versions.
An unlabeled `0.x` tag is a stable SemVer baseline even though Azure release
policy treats `0.x` as preview. No applicable tag means a first release.

## Decide and apply

Compare each baseline to HEAD. Ignore only the crate's own version and
empty/re-dated changelog scaffolding. Substantive changelog entries, dependency
changes (including inherited workspace dependencies), and other directory
changes count. The helper supplies changed paths and dependency relationships;
read focused diffs when their significance is ambiguous:

```powershell
git diff '<baseline-tag>..HEAD' -- sdk/<service>/<crate>
```

Resolve decision-affecting local edits before applying a HEAD-based analysis;
never discard user changes. Preserve unrelated edits.

- Set `Release` to `false` for substantively unchanged crates. Their dependents
  should use the released baseline, not the next local version. First releases
  cannot be skipped this way.
- Keep changed crates' existing `path + version` dependency wiring. Do not
  upgrade unrelated registry consumers. Preserve features, default-feature
  opt-outs, optional flags, and workspace inheritance.
- Retain dev-only local wiring. When the same package is also a normal/build
  dependency in a manifest, keep all entries on the same source.
- Review local dependency relationships before excluding a crate. An otherwise
  unchanged crate may need to ship a changed dependency. Record a concise
  `DecisionReason` for overrides. For core, follow `sdk/core/AGENTS.md` semver and
  changelog propagation rules without automatically increasing versions.
- For a stable release, analyze required dependency crates with
  `-ReleaseType Stable` too. Use their last stable versions only when
  substantively unchanged since those stable tags; otherwise they need a stable
  release as well. Check all shipped normal/build dependencies, including
  registry and target-specific entries, for preview requirements. Dev-only
  dependencies do not affect the published dependency graph.
- Ask before expanding the release set to required unreleased dependencies
  outside the requested scope. Re-analyze the expanded selection only after
  agreement. Do not let packing silently choose additional release candidates.
- A manifest version inconsistent with the release channel, already released,
  or not newer than existing releases is a blocker for the user to resolve.
  Do not fix it by increasing the version.

The agent makes these judgments; do not build a release-decision engine or
scripted dialogue. Update only `Release` and `DecisionReason` in the session
decision file, then apply:

```powershell
pwsh -File ./eng/scripts/Prepare-Release.Helpers.ps1 -FromSkill `
  -Operation Apply -PlanPath <session-artifact-directory>/prepare-release.json
```

The helper rejects stale analysis, validates dependency scope, and uses the
existing Rust TOML editor to make formatting-preserving edits. It reports
`ReleaseOrder`, `WorkspaceBuild`, `ChangedManifests`, and `BuildPackages`. If
dependency decisions need revision, repeat analysis rather than bypassing a
blocker. Refresh `Cargo.lock` if needed using targeted Cargo resolution; do not
perform a blanket dependency upgrade.

## Build and pack

If any selected crate is in `sdk/core` or `sdk/identity`, build the entire
workspace, even when that selected crate is unchanged:

```powershell
cargo build --workspace --all-features --keep-going
cargo build --workspace --all-features --examples --tests --keep-going
```

Otherwise use `BuildPackages` to build release candidates and dependent crates
whose effective dependency declarations changed, including inherited changes:

```powershell
./eng/scripts/Build-Crates.ps1 -ManifestDir @('sdk/<service>/<crate>', 'sdk/<service>/<dependent>')
cargo build --package <crate> --package <dependent> --all-features --examples --tests --keep-going
```

Compile examples and test targets for the same build scope, but do not run SDK
tests. Then pack all release candidates together:

```powershell
./eng/scripts/Pack-Crates.ps1 -PackageName @('<crate>', '<dependency>')
```

Use normal verified packing, without `-NoVerify` or `-Release`. This validates
co-released unpublished dependencies using `cargo package`; `-Release` instead
uses `cargo publish --dry-run` against registry availability. Confirm the
expanded packed set contains no unapproved unpublished dependencies. An empty
release set skips packing and preparation, but dependency edits must still build.
Stop on build/pack failure, report its cause, and resolve it before preparation.

## Prepare and confirm

Only after builds and packing succeed, prepare each candidate in `ReleaseOrder`
(dependencies first), invoking the common script once per crate:

```powershell
./eng/common/scripts/Prepare-Release.ps1 -ServiceDirectory <service> `
  -PackageName <crate> -ReleaseDate yyyy-MM-dd
```

Use `-ServiceDirectory`, not `-ServiceName`. Always pass the date explicitly so
the common script does not choose a scheduled shipping date.

Accept the existing manifest version at the version prompt. Answer predictable
prompts only when the evidence settles them. Ask the user before replacing an
already-dated changelog entry or answering uncertain prompts, and relay their
answer. Never feed an endless stream of defaults or implement a prompt adapter.
If the execution environment cannot expose interactive input, let the user
run/confirm this invocation rather than hanging or guessing. Track completed
crates in the conversation to avoid repeating preparation.

The common script sanitizes changelogs but can merely warn on invalid content.
Explicitly check prepared versions, dates, and changelogs:

```powershell
pwsh -File ./eng/scripts/Prepare-Release.Helpers.ps1 -FromSkill `
  -Operation Validate -PlanPath <session-artifact-directory>/prepare-release.json
```

Inspect the final diff to confirm dependency edits survived, no package version
increased, and no unrelated files changed. If preparation altered manifest or
dependency inputs, repeat the affected build and pack checks; otherwise reuse
successful results. Report candidates, skipped crates and baselines, dependency
changes, release date, and unresolved blockers. Preparation does not publish.
