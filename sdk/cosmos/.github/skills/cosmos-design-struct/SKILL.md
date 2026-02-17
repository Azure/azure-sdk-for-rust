---
name: cosmos-design-struct
description: >
  Enforce consistent struct design conventions across sdk/cosmos crates. Validates visibility modifiers,
  field privacy, #[non_exhaustive] usage, and construction API patterns (`Default`/`new` with `with_*` setters,
  or optional separate builders with `builder()`/`build()`),
  and construction correctness on public structs. Can auto-fix violations or report them as errors.
disable-model-invocation: false
arguments:
  scope:
    type: string
    required: false
    default: all
    description: >
      Crate to validate struct design rules against. `all` means all crates under sdk/cosmos.
  auto-fix:
    type: boolean
    required: false
    default: false
    description: >
      If true, automatically fix violations (adjust visibility, add #[non_exhaustive], generate/accessors,
      align construction APIs with allowed patterns). If false, only report violations with proposed changes.
  changed-only:
    type: boolean
    required: false
    default: true
    description: >
      If true, only scan `.rs` files that differ between the current local branch and `main`
      (i.e., `git diff --name-only main -- <target path>`). This limits work when the skill
      is triggered automatically. If false, scan all `.rs` files under the target path.
argument-hints:
  scope:
    - azure_data_cosmos
    - azure_data_cosmos_native

  auto-fix:
    - true
    - false

  changed-only:
    - true
    - false
---
# Cosmos SDK Struct Design Rules

## When to use this skill

Use this skill when:

- Reviewing or validating struct design in the Cosmos SDK
- Generating new structs or modifying existing ones under `sdk/cosmos/**`
- Preparing a PR that introduces or changes public types
- Refactoring struct visibility or field access patterns
- Auditing the public surface area for breaking changes

## Behavior

Follow these steps strictly:

### Step 1 — Determine target path

- If the `scope` argument is specified and is not equal (case-insensitive) to `all` or `*`, set the target path to `sdk/cosmos/<scope>` (for example, if `scope` is `azure_data_cosmos`, use `sdk/cosmos/azure_data_cosmos` as the target path).
- Otherwise, set the target path to `sdk/cosmos`.
- Always include per-crate `tests/` directories in the validation scope (e.g., `sdk/cosmos/azure_data_cosmos/tests/`).

### Step 2 — Determine file scope

- If `changed-only` is `true` (the default), restrict scanning to `.rs` files that differ between the current local branch and `main`. Use `git diff --name-only main -- <target path>` (and include per-crate `tests/` directories) to obtain the list. Only `.rs` files in the result set are scanned; all other files are skipped.
- If `changed-only` is `false`, scan **all** `.rs` files under the target path(s).
- In both modes, **skip** files in `generated/` subdirectories — these are produced by external tools and must never be modified.

### Step 3 — Scan struct declarations

- Find all `struct` declarations in the `.rs` files identified in Step 2.

### Step 4 — Classify each struct

Classify every struct into exactly one of these categories:

1. **Truly public** — The struct is `pub` and **all** ancestor modules up to the crate root are also `pub` (the struct is reachable from outside the crate). These structs get the full set of rules: `#[non_exhaustive]`, private/non-`pub` fields, getters, and a consistent construction API. Construction may use either direct fluent methods on the struct (`Default` and/or `new(...)` + `with_*`) or a separate builder type (`builder()`/`build()` + `with_*`).

2. **Effectively scoped** — The struct has a `pub` visibility modifier but lives inside a module that restricts visibility (e.g., `pub(crate) mod`, `pub(super) mod`). The struct is **not** reachable from outside the crate. These structs should:
   - Have the **effective visibility** annotated explicitly on the struct (e.g., `pub(crate) struct Foo` not `pub struct Foo` inside a `pub(crate) mod`).
   - **Omit** `#[non_exhaustive]` — it is unnecessary since external code cannot reach the struct.
   - Fields **can** use `pub` without further restriction — the struct-level visibility already limits access, and repeating `pub(crate)` or `pub(super)` on every field adds noise without benefit.

3. **Internal** — The struct is used only within its declaring module or submodule. These structs should:
   - Use the most restrictive visibility that still compiles (no modifier for module-private, `pub(super)` for parent module access, `pub(crate)` for crate-wide access).
   - **Omit** `#[non_exhaustive]`.

### Step 5 — Apply visibility rules

For **all** structs regardless of category:

| Usage scope | Struct visibility |
|---|---|
| Only within the declaring module | No visibility modifier (private) |
| Within the parent module | `pub(super)` |
| Within the crate | `pub(crate)` |
| Outside the crate | `pub` |

Additional rules:

- If a struct is marked `pub` but lives inside a non-public module (e.g., `pub struct Foo` inside `pub(crate) mod internal`), change the struct to use the **effective** visibility: `pub(crate) struct Foo`. This makes the actual visibility obvious at the struct declaration site without requiring the reader to trace module ancestry.
- Fields on effectively-scoped or internal structs **can** use `pub` — the struct-level visibility already constrains access. This is an intentional choice to reduce repetitive `pub(crate)` or `pub(super)` annotations on fields while still making the effective visibility clear and easy to review from the struct declaration alone.

### Step 6 — Apply truly-public struct rules

These rules apply **only** to structs classified as **truly public** in Step 3:

#### a) `#[non_exhaustive]` required

Every truly public struct must be annotated with `#[non_exhaustive]`. This prevents external code from constructing the struct with literal syntax, ensuring forward compatibility when fields are added in future versions.

#### b) No `pub` fields — use the most restrictive visibility

Fields on truly public structs must **never** be `pub`. Choose the visibility by checking both (1) whether a public accessor exists **and** (2) whether crate-internal code in other modules accesses the field directly:

| Scenario | Field visibility |
|---|---|
| Field has a public getter and/or `with_*` setter **and** no crate-internal code outside the defining module accesses the field directly | **Private** (no modifier). All access — including crate-internal — goes through the accessor methods. |
| Field has a public getter and/or `with_*` setter **but** crate-internal code in other modules also accesses the field directly (e.g., for mutation, move/consume, or nested field traversal) | **`pub(crate)`**. The getter serves external consumers; `pub(crate)` permits efficient internal access without forcing all crate-internal code through the public API. |
| Field has no public accessor but is read/written inside the crate | **`pub(crate)`** (or `pub(super)` if only the parent module needs access) |
| Field has no accessor and is only used in the declaring module | **Private** (no modifier) |

**Rationale**: Making a field `pub` is a semver commitment — removing it later is a breaking change. Private fields with accessors keep the API surface controllable.

**How to determine the correct visibility**: Before deciding between private and `pub(crate)`, search the crate for direct field access outside the defining module (e.g., `grep` for `.field_name` in other `.rs` files). If any crate-internal code accesses the field directly — not through a getter or setter — the field must be `pub(crate)`. If all access goes through accessors, the field should be private (no modifier).

#### c) Getter methods for readable fields

Every field that external consumers need to **read** must have a getter method:

- Named after the field (e.g., `fn session_token(&self) -> &str`)
- Returns `&T` for non-`Copy` types, or `T` for `Copy` types (e.g., `bool`, `u32`, `f64`)

#### d) Construction APIs: allowed patterns

For truly public structs, both of the following construction styles are valid:

1. **Direct fluent construction on the target struct**
   - For all-optional/simple types: implement or derive `Default` as appropriate.
   - For required fields: provide `new(required...)` with required parameters.
   - Provide `with_*` setters on the target struct for optional fields.

2. **Separate builder type**
   - Provide `Type::builder()` returning `<Type>Builder`.
   - Place optional `with_*` setters on the builder.
   - Finalize with `build(...)` (required parameters on `build()` when applicable).

For `with_*` setters in **either** style (on the target type or on a builder), use the same fluent signature: take `mut self` and return `Self`.

Separate builder types are **optional**, not mandatory. Prefer the simpler direct pattern for truly all-optional options structs or simple models unless a dedicated builder materially improves ergonomics.

#### e) Required fields and setter placement

A **required field** is one that must be set for the struct to be semantically valid.

- If using the **direct** pattern, required fields belong in `new(required...)`, and `with_*` setters are for optional overrides.
- If using the **builder** pattern, required fields belong in `build(required...)`, and `with_*` setters are for optional overrides.
- Do **not** model required fields as optional just to satisfy a construction style.

When inferring required fields on existing structs, use docs, service behavior, and call-site usage patterns.

#### f) Exemptions

**Newtype structs** are exempt from the named-field construction rules. Since a newtype wraps a single value, the full named-field struct rules do not apply. Instead, newtypes should:

- Keep the inner field **private**.
- Provide construction via `new()`, `From` impls, or associated constants.
- Provide access to the inner value via a getter (e.g., `value()`) or `Into`/`AsRef` impls.
- **Omit** `#[non_exhaustive]`.

**Builder types** (`*Builder` structs), when present, are exempt from `#[non_exhaustive]` and getters.

Serde derive macros (`Serialize`, `Deserialize`) work on private fields — no `pub(crate)` is needed for serde compatibility.

#### g) Optional builder guidance (when used)

If a separate builder type is used, follow these conventions:

1. Name it `<Type>Builder`.
2. Keep builder fields private.
3. Provide `with_*` setters for optional fields.
4. Provide terminal `build(self, ...) -> <Type>` (or `azure_core::Result<Type>` when fallible).
5. Keep required fields on `build(...)`, not as optional builder state.

These conventions apply only when a builder exists; they are not a requirement to introduce one.

### Step 7 — Auto-fix or report

#### If `auto-fix` is `true`

1. Adjust visibility modifiers on structs and fields according to Steps 4–6.
2. Add `#[non_exhaustive]` to truly public structs that lack it.
3. Remove `#[non_exhaustive]` from non-public structs that have it unnecessarily.
4. Make `pub` fields private or `pub(crate)` on truly public structs. For each field that was previously `pub`:
   - Search the crate for direct field access outside the defining module.
   - If a getter exists (or is being generated) **and** no crate-internal code outside the defining module accesses the field directly, make the field **private** (no modifier).
   - If a getter exists **but** crate-internal code in other modules also accesses the field directly (mutation, move/consume, nested traversal), make the field **`pub(crate)`**.
   - If no accessor is generated and the field is used elsewhere in the crate, use the most restrictive visibility that compiles (`pub(crate)` or `pub(super)`).
   - Generate getter methods for each field that external consumers need to read.
5. Ensure each truly public struct has a consistent allowed construction API (Step 6d), preferring minimal churn:
    a. Keep existing style if already valid (`Default`/`new` + `with_*`, or `builder()`/`build()`).
    b. If no ergonomic construction API exists, add the simplest valid option:
        - all-optional/simple types: prefer `Default` + `with_*` on the target type.
        - types with required fields: prefer `new(required...)` + optional `with_*`.
        - use a separate builder only when complexity justifies it.
    c. Add getter methods for externally readable fields if missing.
    d. Update call sites only when required by other applied fixes; do not force style migrations solely to introduce builders.
6. Run `cargo fmt -p <crate>` after changes.
7. Run `cargo clippy -p <crate> --all-features --all-targets` and fix any resulting warnings.
8. Run `cargo build -p <crate>` to confirm changes compile.

#### If `auto-fix` is `false`

Emit a structured report listing every violation:

```
## Violations

### <crate_name>

#### <file_path>:<line_number> — `StructName`
- **Category**: Truly public | Effectively scoped | Internal
- **Rule violated**: <rule description>
- **Current**: <current code snippet>
- **Proposed**: <proposed fix>
```

### Step 8 — Produce summary

Regardless of the `auto-fix` setting, always produce a final summary:

#### Public surface area changes

List all changes (applied or proposed) that affect truly public structs, grouped by crate and module:

- Added `#[non_exhaustive]`
- Fields changed from `pub` to private
- New getter methods added
- New builder type generated (`<Type>Builder`) (if applied)
- New `builder()` factory method added (if applied)
- `Default`/`new`/`with_*` construction APIs added or adjusted
- Visibility modifier changed on struct

#### Breaking change warnings

Highlight any change that constitutes a **semver breaking change** with:

```
⚠️ BREAKING CHANGE: `StructName::field_name` was `pub` and is now private.
   External code using `instance.field_name` must change to `instance.field_name()` (getter).
   External code constructing via struct literal must change to builder pattern.
```

Breaking changes include:

- A `pub` field becoming private (consumers using direct field access will break)
- Adding `#[non_exhaustive]` (consumers using struct literal construction will break)
- Removing or changing an existing construction API (`Default`, `new`, `with_*`, `builder`, or `build`)
- Adding `builder()` and a builder type is typically additive and non-breaking

## Notes

- Never modify files in `generated/` subdirectories.
- `#[non_exhaustive]` is **only** for truly public structs. Effectively-scoped and internal structs must omit it — it adds no value when external code cannot reach the struct.
- Fields on effectively-scoped structs can remain `pub` without additional restriction — the struct-level visibility already limits access, and repeating `pub(crate)` on every field is unnecessary noise.
- Serde derive macros (`Serialize`, `Deserialize`) work on private fields — no `pub(crate)` is needed for serde compatibility.
- Builder-pattern setters use the `with_*` prefix per [Azure SDK Rust guidelines](https://azure.github.io/azure-sdk/rust_introduction.html) (not bare field names).
- For truly public structs, separate builder types are optional. `Default`/`new` + `with_*` on the target struct are valid and often preferred for all-optional/simple types.
- If a builder type is used, follow [Azure SDK Rust builder guidelines](https://azure.github.io/azure-sdk/rust_introduction.html): keep builder fields private, keep optional setters as `with_*`, and place required params on `build()`.
- Reference `sdk/cosmos/AGENTS.md` for canonical model, options, and builder patterns.
- Breaking changes in public surface area require explicit acknowledgment from the developer before merging.
- When generating **new** structs, apply these rules from the start — it is far easier to design with private fields, getters, and a consistent construction API than to retrofit them later.
- For **new** structs, explicitly ask the developer which fields are required if not obvious from the context.
- For **existing** structs, infer required fields from: (1) doc comments mentioning "required", (2) server rejection of default values, (3) every call site always setting the field, (4) non-`Option` type with no semantically valid zero value.
