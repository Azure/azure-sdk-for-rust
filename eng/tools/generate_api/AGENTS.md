# generate_api agent guidance

## Goal

`eng/tools/generate_api` is a Rust CLI that generates two public API artifacts for a target crate:

1. `API.md` — one fenced `rust` block
2. `apiview.json` — an APIView tree-style `CodeFile`

## Scope

- This tool lives under `eng/tools/generate_api`.
- `eng/tools/generate_api_report` is prior art only and not part of this design.
- Run the CLI from the repo root.
- Keep this file current as design behavior or integration points change.
- Keep this file concise for LLMs but still easy for humans to review. Prefer short bullets and only enough detail to preserve intent.
- Keep `README.md` focused on basic intent and usage: what the tool does, how to call it, what it writes, and where it is called from under `eng/pipelines/`. Keep deeper extraction rendering and ordering rules here.

## CLI

The tool exposes:

- `--manifest-path <path/to/Cargo.toml>`
- `--format <markdown|apiview>` default `markdown`
- `--no-docs` only for `apiview`
- `--output <directory>`

Behavior:

- default `markdown` writes `API.md`
- `--format apiview` writes `apiview.json`
- `--no-docs` suppresses APIView doc comment tokens
- progress goes to stdout
- fatal errors go to stderr and exit `1`

## Toolchain and workspace

- Standalone bin crate in the `eng/tools` workspace
- Uses `eng/tools/rust-toolchain.toml` toolchain `nightly-2025-05-09`
- Current deps: `rustdoc-types`, `serde`, `serde_json`, `clap`
- `rustc-dev` stays included because long-term direction remains closer to librustdoc/HIR
- Keep implementation and tests separate when practical. Prefer sibling `tests.rs` files over nested `mod tests` blocks. Tiny local `#[test]` items may stay inline

## Extraction design

Pipeline:

1. run `cargo metadata`
2. run `cargo rustdoc -Z unstable-options --output-format json`
3. load rustdoc JSON
4. normalize into a tool-owned model
5. render Markdown or APIView from that model

Keep renderers independent from unstable `rustdoc-types` details. Prefer renderer option structs over threading booleans through helper stacks.

## Shared intermediate model

The shared model is the boundary between extraction and rendering.

It currently models:

- package metadata
- modules
- item doc comments
- item attributes
- public items
- inherent impl blocks
- explicit trait impl blocks
- associated members including trait methods associated types and associated consts

Workspace crate models are cached with `Arc<T>` to avoid repeated deep clones during workspace re-export expansion.

Supported item kinds:

- re-exports
- macros / proc macros
- functions
- structs
- enums
- traits
- trait aliases
- inherent impls
- explicit trait impls
- unions
- type aliases
- constants
- statics

## Ordering rules

Ordering is deterministic and shared by both output formats.

- crate root renders first and is not wrapped in `mod`
- child modules recurse in lexical order
- within each module:
  - re-exports first
  - macros / proc macros
  - free functions
  - types and other kinds by stable item-kind order
  - ties break alphabetically by item name
- inherent impl blocks sort immediately after their owning struct / enum / union
- inherent impl ordering is:
  - generic type parameters first
  - inferred `_` type args next
  - explicit resolved types last
  - ties break by rendered self type then declaration text
- associated members sort alphabetically within each impl or trait block

## Module rendering

- Markdown output renders child modules as nested `pub mod name { ... }`
- APIView uses the same logical module tree with root unwrapped
- Module doc comments and attributes render above the module declaration
- Trait members are extracted into `ApiItem.members` instead of being embedded in the declaration string. Renderers handle the opening `{` and implied closing `}` separately so each member gets its own APIView `LineId`
- Inherent impl blocks on structs enums and unions are first-class items with `ApiItem.members`. Do not flatten their members into the owning type. This preserves typestate surfaces such as multiple `read()` methods on different `SasBuilder` impls
- Keep separate source impl blocks separate even when their rendered headers match. Preserve each block's own attrs docs and members
- Non-derived trait impls are also first-class items with `ApiItem.members`

## Re-export rules

Re-export handling is driven by public reachability and workspace membership.

### Same-crate re-exports

- If the source path is already publicly reachable keep `pub use ...`
- If the source path is non-public or stripped lift the declaration to the public re-export site

### Workspace-crate re-exports

- Re-exports from workspace crates expand into declarations at the re-export site
- Applies at crate root and inside public modules
- When a lifted type has sibling explicit trait impls or inherent impls lift those too so the public surface keeps the visible impl blocks

### External-crate re-exports

- Re-exports from crates outside this workspace stay `pub use ...`
- Prefer the canonical external path when rustdoc provides one

## Attribute and doc normalization

Attributes are normalized once in extraction before either renderer consumes them.

Current normalization:

- fix rustdoc pretty-printed `cfg` and `cfg_attr`
- rewrite `pin(__private(...))` to `pin_project(...)` or `pin_project`
- flatten rustdoc whitespace and newlines inside attributes while preserving string literals
- remove whitespace around path separators
- remove extra spaces around `clippy::` lint paths
- synthesize `#[derive(...)]` for known non-workspace derive traits discovered from `#[automatically_derived]` impls on structs enums and unions
- do not synthesize workspace-defined derives such as `SafeDebug`
- keep synthesized derives on the same visible declaration surface after re-export lifting
- render non-derived trait impls as explicit `impl` blocks instead of folding them into derives
- keep lifted explicit trait impls on the same visible surface as the lifted type

Known synthesized derives:

- `Clone`
- `Copy`
- `Debug` including `fmt::Debug`, `core::fmt::Debug`, `std::fmt::Debug`
- `Default`
- `Eq`
- `Hash`
- `Ord`
- `PartialEq`
- `PartialOrd`
- `serde::Serialize`
- `serde::Deserialize`

Documentation handling:

- rustdoc docs stay separate from attrs in the shared model
- markdown output currently omits doc comments
- APIView renders comment tokens with documentation markers

Signature normalization:

- render receivers spelled as `self: Self`, `self: &Self`, `self: &mut Self` as `self`, `&self`, `&mut self`
- keep `Self` unchanged in non-receiver positions
- keep inherent impls in their original impl-header shape including generics and bounds

## Async-trait rendering

For traits whose rustdoc-expanded methods carry synthetic async-trait lifetimes:

- synthesize `#[async_trait]`
- elide synthetic `'lifeN` and `'async_trait` lifetimes from signatures
- remove empty generic parameter lists after elision

## APIView output design

Targets:

- TypeSpec source: <https://github.com/Azure/azure-sdk-tools/blob/main/tools/apiview/parsers/apiview-treestyle-parser-schema/codeFile.tsp>
- JSON schema: <https://github.com/Azure/azure-sdk-tools/blob/main/tools/apiview/parsers/apiview-treestyle-parser-schema/CodeFile.json>

Top-level fields used:

- `PackageName`
- `PackageVersion`
- `ParserVersion`
- `Language`
- `ReviewLines`

Important nested structures:

- `ReviewLine`
  - `LineId?`
  - `Tokens`
  - `Children?`
  - `IsContextEndLine?`
  - `RelatedToLine?`
- `ReviewToken`
  - `Kind`
  - `Value`
  - `HasPrefixSpace?`
  - `HasSuffixSpace?`
  - `IsDocumentation?`
  - `NavigationDisplayName?`
  - `RenderClasses?`

Token kinds used:

- `Text = 0`
- `Punctuation = 1`
- `Keyword = 2`
- `TypeName = 3`
- `MemberName = 4`
- `Comment = 7`

Current APIView decisions:

- `Language` is `Rust`
- stable `LineId` generation:
  - module: `module.{sanitized_path}`
  - item: `{module_line_id}.{item_name}_{index}`
  - member: `{item_line_id}.{member_name}_{index}`
- reject duplicate `LineId`s
- include `HasPrefixSpace` and `HasSuffixSpace`
- doc comments use `Comment` with `IsDocumentation = true`
- represent nested modules through `ReviewLine.Children`
- type all declaration tokens:
  - keywords use `Keyword`
  - item names use `TypeName` except functions use `MemberName`
  - other identifiers default to `TypeName`
  - punctuation uses `Punctuation`
- tokenize synthesized derives like other attrs
- render trait members as child lines with their own `LineId`
- render inherent impl members as child lines of their own impl line so duplicate method names stay distinct across impl headers
- explicit trait impls use the same typed token rules as source-shaped declarations

## Current pipeline integration

Current API review caller chain under `eng/pipelines/`:

1. `eng/pipelines/pr.yml` or `eng/pipelines/pullrequest.yml`
2. `eng/pipelines/templates/stages/archetype-sdk-client.yml`
3. `eng/pipelines/templates/jobs/ci.yml`
4. `eng/pipelines/templates/jobs/pack.yml`
5. `eng/scripts/Pack-Crates.ps1`

`pack.yml` also runs the shared `create-apireview` step. `Pack-Crates.ps1` currently generates the artifact that step consumes. If pipeline adoption changes update this caller chain instead of adding a second path.

## Rustdoc / librustdoc alignment

The design target remains librustdoc-like behavior not HTML scraping.

Preserved assumptions:

- rustdoc runs after HIR is available
- body type-checking is not required
- the important outputs are public API signatures attrs docs macros and module structure

The implementation still acquires data through rustdoc JSON but the architecture should keep future movement toward direct librustdoc/HIR possible without rewriting the renderers.
