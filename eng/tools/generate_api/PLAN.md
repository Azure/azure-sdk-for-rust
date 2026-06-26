# Generate API final design

## Goal

`eng/tools/generate_api` is a Rust CLI that generates two public API artifacts for a target crate:

1. `API.md` — a Markdown file containing exactly one fenced `rust` code block.
2. `apiview.json` — an APIView token file matching the tree-style `CodeFile` schema.

## Scope

- This tool lives entirely under `eng/tools/generate_api`.
- `eng/tools/generate_api_report` is reference-only prior art and is not part of this design.
- The CLI is intended to be run from the repository root.

## CLI

The tool exposes:

- `--manifest-path <path/to/Cargo.toml>`
- `--format <review|apiview>`
- `--output <directory>`

Behavior:

- `--format review` writes `API.md`
- `--format apiview` writes `apiview.json`
- progress messages go to stdout
- fatal errors go to stderr and exit with code `1`

## Toolchain and workspace

- The crate is a standalone bin crate in the `eng/tools` workspace.
- It uses the pinned `eng/tools/rust-toolchain.toml` toolchain: `nightly-2025-05-09`.
- The implementation currently depends on `rustdoc-types`, `serde`, `serde_json`, and `clap`.
- `rustc-dev` is included in the toolchain because the long-term direction remains closer alignment with librustdoc/HIR behavior.

## Extraction design

The current extraction pipeline is:

1. run `cargo metadata` for package and workspace context
2. run `cargo rustdoc -Z unstable-options --output-format json`
3. load rustdoc JSON
4. normalize rustdoc output into a stable, tool-owned intermediate model
5. render either Markdown or APIView from that shared model

This is intentionally structured so renderer code does not depend directly on unstable upstream `rustdoc-types` details.

## Shared intermediate model

The shared model is the compatibility boundary between extraction and rendering.

It currently models:

- package metadata
- modules
- item doc comments
- item attributes
- public items
- explicit trait impl blocks
- associated members (including trait methods, associated types, and associated consts)

Workspace crate models are cached via `Arc<T>` to avoid deep-cloning on repeated lookups during workspace-dep expansion.

Supported item kinds include:

- re-exports
- macros / proc macros
- functions
- structs
- enums
- traits
- trait aliases
- explicit trait impls
- unions
- type aliases
- constants
- statics

## Ordering rules

Ordering is deterministic and shared by both output formats.

- crate root is rendered first and is not wrapped in a module declaration
- child modules are rendered recursively in lexical order
- within each module:
  - re-exports first
  - macros / proc macros
  - free functions
  - types and other item kinds by stable item-kind order
  - ties break alphabetically by item name
- associated members are sorted alphabetically

## Module rendering

- Review output renders child modules as actual nested `pub mod name { ... }` blocks.
- APIView output uses the same logical module tree with root unwrapped.
- Module doc comments and attributes are emitted above the module declaration.
- Trait members (methods, associated types, associated consts) are extracted into `ApiItem.members`
  rather than embedded in the declaration string.  Both renderers handle the opening `{` in the
  declaration and the implied closing `}` separately, which makes each trait member navigable with
  its own `LineId` in APIView output.
- Non-derived trait impls are also extracted as items with `ApiItem.members`, so both renderers can
  emit source-shaped `impl Trait for Type { ... }` blocks and make each implemented member
  navigable in APIView.

## Re-export rules

Re-export handling is driven by public reachability and workspace membership.

### Same-crate re-exports

- If a same-crate re-export points at a source path that is already publicly reachable, keep it as `pub use ...`.
- If a same-crate re-export exposes an item from a non-public or stripped module path, lift the declaration to the public re-export site.
- This allows private implementation modules to stay hidden while public exported API appears at the correct visible surface.

### Workspace-crate re-exports

- Re-exports from crates that are also defined in this repository workspace are expanded into declarations at the re-export site.
- This applies both at crate root and inside public modules.
- Example consequence: if `azure_core::tracing` re-exports a type from `typespec_client_core::tracing`, the type is declared under `azure_core::tracing`.
- When a workspace re-export lifts a concrete type declaration, sibling explicit trait impl blocks for
  that type are lifted alongside it so public surfaces such as `azure_core::Error` still show
  `impl fmt::Debug for Error { ... }`, `impl fmt::Display for Error { ... }`, and similar
  declarations.

### External-crate re-exports

- Re-exports from crates outside this workspace remain `pub use ...`.
- When rustdoc supplies a more canonical external path, that canonical path is preferred.

## Attribute and doc normalization

Attributes are normalized once in the shared extraction layer before either renderer consumes them.

Current normalization rules include:

- fix rustdoc pretty-printed `cfg` / `cfg_attr` forms
- rewrite `pin(__private(...))` forms to `pin_project(...)` / `pin_project`
- remove spaces around `clippy::` lint paths in `allow`, `deny`, and `expect` attrs
- synthesize `#[derive(...)]` for known non-workspace derive traits discovered via impl blocks on
  structs, enums, and unions when the impl item carries `#[automatically_derived]`. Current
  recognized derives are `Clone`, `Copy`, `Debug`, `Default`, `Eq`, `Hash`, `Ord`, `PartialEq`,
  `PartialOrd`, `serde::Serialize`, and `serde::Deserialize`. `Debug` is recognized from `Debug`,
  `fmt::Debug`, `core::fmt::Debug`, and `std::fmt::Debug`. `Serialize` and `Deserialize` are
  recognized from either qualified or unqualified rustdoc paths and are normalized to their
  `serde::...` spellings. Workspace-defined derives such as `SafeDebug` are not synthesized.
- synthesized derive attributes follow lifted type declarations across same-crate and workspace-crate
  re-export expansion, so the visible public API keeps `#[derive(...)]` on the same declaration
  surface where the type itself is emitted
- non-derived trait impls are rendered as explicit `impl` blocks instead of being folded into
  synthesized `#[derive(...)]` attributes
- explicit trait impl blocks follow lifted type declarations across same-crate and workspace-crate
  re-export expansion, so the visible public API keeps source-shaped impl declarations even when the
  type itself is surfaced through `pub use`

Documentation handling:

- rustdoc item docs are stored separately from attrs in the shared model
- review output renders them as `///` comment lines
- APIView output renders them as comment/documentation tokens

Signature normalization:

- receiver parameters are rendered in source-like Rust forms when rustdoc JSON spells them as
  `self: Self`, `self: &Self`, or `self: &mut Self`; those become `self`, `&self`, and
  `&mut self`
- `Self` remains rendered normally when it appears as a non-receiver type, such as `Pin<Self>`, a
  return type, or an associated type

## Async-trait rendering

Traits whose rustdoc-expanded methods carry synthetic async-trait lifetimes are normalized to better match source intent.

- the trait gets a synthesized `#[async_trait]` attribute
- synthetic `'lifeN` and `'async_trait` lifetimes are elided from method signatures
- empty generic parameter lists are removed after elision

## APIView output design

The APIView output targets:

- TypeSpec source: <https://github.com/Azure/azure-sdk-tools/blob/main/tools/apiview/parsers/apiview-treestyle-parser-schema/codeFile.tsp>
- JSON schema: <https://github.com/Azure/azure-sdk-tools/blob/main/tools/apiview/parsers/apiview-treestyle-parser-schema/CodeFile.json>

Concise schema summary to keep with the design:

- top-level `CodeFile` fields used by this tool:
  - `PackageName`
  - `PackageVersion`
  - `ParserVersion`
  - `Language`
  - `ReviewLines`
- important nested structures:
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
- important token kinds used by the current implementation:
  - `Text = 0`
  - `Punctuation = 1`
  - `Keyword = 2`
  - `TypeName = 3`
  - `MemberName = 4`
  - `Comment = 7`

Required top-level fields:

- `PackageName`
- `PackageVersion`
- `ParserVersion`
- `Language`
- `ReviewLines`

Current APIView decisions:

- `Language` is `Rust`
- review lines use stable `LineId` generation
  - module LineIds: `module.{sanitized_path}`
  - item LineIds: `{module_line_id}.{item_name}_{index}`
  - member LineIds: `{item_line_id}.{member_name}_{index}`
- duplicate `LineId`s are rejected at validation time
- tokens include `HasPrefixSpace` and `HasSuffixSpace`
- doc comments use token kind `Comment` with `IsDocumentation = true`
- nested module structure is represented through `ReviewLine.Children`
- all declaration tokens are typed: keywords use `Keyword`, the item's own name uses `TypeName`
  (structs, enums, traits, etc.) or `MemberName` (functions), other identifiers default to
  `TypeName`, and punctuation uses `Punctuation`
- synthesized derive attributes are tokenized the same way as other attributes: `derive` is a
  `Keyword`, the derived trait names are `TypeName`, and `#`, `[`, `]`, `(`, `)`, `,`, and `::`
  use `Punctuation`
- trait members are rendered as children with individual `LineId` values, enabling navigation
- explicit trait impl blocks use the same typed tokens as source-shaped item declarations: `impl`,
  `for`, and `fn` are `Keyword`, trait and type identifiers are `TypeName`, implemented method
  names are `MemberName`, and punctuation such as `#`, `[`, `]`, `(`, `)`, `,`, `:`, `;`, `->`,
  `&`, `{`, `}`, and `::` uses `Punctuation`

## Rustdoc / librustdoc alignment

The design target remains librustdoc-like behavior rather than HTML scraping.

Important preserved assumptions:

- rustdoc operates on compiler data after HIR is available
- body type-checking is not required for this task
- public API signatures, attrs, docs, macros, and module structure are the important outputs

The current implementation still uses rustdoc JSON as the acquisition mechanism, but the architecture is intentionally set up so extraction can move closer to direct librustdoc/HIR integration later without rewriting either renderer.
