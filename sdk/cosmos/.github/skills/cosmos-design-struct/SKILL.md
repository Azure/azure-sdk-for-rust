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
    - azure_data_cosmos_driver
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

1. **Truly public** — The struct is `pub` and **all** ancestor modules up to the crate root are also `pub` (the struct is reachable from outside the crate). These structs get the full set of rules: getter coverage for externally-readable fields, explicit field-visibility decisions based on invariants/validation needs, and a construction pattern that matches the required/optional field mix:
  - It must expose `Default` when there are no logically required fields.
  - It must expose `new(required...)` when the struct has both required and optional fields.
  - For required-only structs, neither `Default` nor `new(required...)` is required by this skill.
  - It must expose fluent `with_*` methods **only for optional fields** so callers can conveniently adjust a small subset of options.
  - If a struct has exactly one field, do not force `with_*` methods.
  - If a struct has only required fields (no optional overrides), do not force `with_*` methods.
  - A separate builder type (`builder()`/`build()` + `with_*`) is optional, but if not used, the target struct itself must provide fluent `with_*` methods for optional fields when they exist.

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

#### a) `#[non_exhaustive]` required only for all-public-field truly public structs

For truly public structs, require `#[non_exhaustive]` **when all named fields are public**. This prevents external code from constructing the struct with literal syntax, ensuring forward compatibility when fields are added in future versions.

If one or more fields are non-public, `#[non_exhaustive]` is optional and typically redundant for construction control.

#### b) Prefer type-system enforcement for validation and invariants

Before making a field non-public solely to validate it in a `with_*` setter or constructor, evaluate whether the **type system** can enforce the invariant instead. Newtypes, enums, and other constrained wrapper types make invalid states unrepresentable at compile time, which is stronger and more ergonomic than runtime checks.

**Decision order** (prefer earlier options):

1. **Enum** — When the domain is a closed set of named values (e.g., `ConsistencyLevel`, `IndexingMode`), use an enum. Invalid variants simply cannot be expressed.
2. **Newtype with construction-time validation** — When values must satisfy a constraint (range, format, normalization), introduce a newtype that validates or normalizes in `new()` / `From` and guarantees the invariant internally (e.g., `RegionName` normalizes casing on construction; `SubStatusCode` wraps a raw number with parse validation).
3. **Newtype for semantic clarity** — Even without a hard constraint, a newtype can prevent accidental misuse of stringly-typed or primitive fields (e.g., wrapping `String` as `DatabaseId` to avoid mixing up identifiers).
4. **Runtime validation in `with_*`/constructor** — Use this as a **last resort**, only when type-level enforcement is impractical (e.g., cross-field invariants that span multiple values, constraints that depend on external state, or cases where introducing a new type would create excessive API friction for negligible safety gain).

When a newtype is introduced, the field holding it can often remain `pub` because the invariant is encoded in the type itself — external code cannot construct an invalid value regardless of field visibility. See subsection (g) for newtype struct conventions.

> **Rule of thumb**: If a `with_*` setter contains a `.clamp()`, range check, format validation, or normalization, that logic almost certainly belongs in a newtype's constructor instead.

#### c) Field visibility on truly public structs — choose based on validation/invariant needs

Fields on truly public structs may be `pub` **or** non-public. Choose visibility by checking (1) whether validation/invariant enforcement is needed and (2) whether crate-internal code in other modules needs direct non-getter access:

| Scenario | Field visibility |
|---|---|
| Field has no validation/invariant constraints and direct access is acceptable for external consumers | **`pub`** is allowed. `with_*` setters and direct field modification may coexist. |
| Field requires validation/invariant enforcement (e.g., value constraints, normalization, coupled-field invariants) | **Non-public** (private by default; `pub(crate)` only when justified by crate-internal usage). Route external mutation through `with_*`/constructor APIs. |
| Field is non-public and crate-internal code in other modules only reads it | Prefer **private** plus getter usage at call sites. |
| Field is non-public and crate-internal code outside the defining module requires non-getter semantics (mutation, move/consume, mutable references, nested writes) | **`pub(crate)`** (or `pub(super)` when parent-only). |

**Rationale**: Public fields maximize ergonomics and avoid forced ownership extractors for simple data. Non-public fields keep API evolution and validation logic controllable where invariants matter.

**How to determine the correct visibility**:

1. Decide first whether the field requires invariant/validation enforcement.
2. If yes, check whether a newtype or enum can encode the invariant in the type (see subsection (b) above). If the type itself enforces the invariant, `pub` is acceptable.
3. If the invariant cannot be encoded in a type, keep the field non-public and evaluate crate-internal access needs to choose private vs `pub(crate)`/`pub(super)`.
4. If no invariant is needed, `pub` is acceptable.

#### d) Getter methods for readable fields

Every field that external consumers need to **read** must have a getter method:

- Named after the field (e.g., `fn session_token(&self) -> &str`)
- Returns `&T` for non-`Copy` types, or `T` for `Copy` types (e.g., `bool`, `u32`, `f64`)

#### e) Construction APIs: required fluent pattern

For truly public structs, construction APIs must be ergonomic without over-engineering. The baseline constructor and optional setters must follow this contract:

- Baseline constructor:
  - No logically required fields: implement or derive `Default`.
  - Mixed required + optional fields: provide `new(required...)` and do **not** implement `Default`.
  - Required-only fields: no constructor requirement from this skill (`new` is optional; `Default` is not required).
- Optional-field ergonomics:
  - Provide `with_*` fluent setters for optional fields **when optional fields exist and the struct has more than one field**.
  - Setters use signature `fn with_xxx(mut self, value: T) -> Self`.
  - If there are no optional fields, `with_*` setters are not required.
  - If there is exactly one field on the struct, `with_*` setters are not required.

This can be realized in either of the following styles:

1. **Direct fluent construction on the target struct**
  - For all-optional/simple types: `Default` + `with_*`.
  - For mixed required+optional fields: `new(required...)` + `with_*`.
  - For required-only types: direct struct construction or `new(required...)` (both acceptable).

2. **Separate builder type**
   - Provide `Type::builder()` returning `<Type>Builder`.
   - Place optional `with_*` setters on the builder.
  - Finalize with `build(...)` (required parameters on `build()` when applicable).
  - If required fields exist, do not expose `Default` on the target struct.

For `with_*` setters in **either** style (on the target type or on a builder), use the same fluent signature: take `mut self` and return `Self`.

Separate builder types are still optional. Fluent `with_*` support is required only when optional fields are present and the struct has more than one field.

#### f) Required fields and setter placement

A **required field** is one that must be set for the struct to be semantically valid.

- If using the **direct** pattern, required fields belong in `new(required...)`, and `with_*` setters are for optional overrides.
- If using the **builder** pattern, required fields belong in `build(required...)`, and `with_*` setters are for optional overrides.
- If a struct has both required and optional fields, do **not** implement `Default`; use `new(required...)` (or `build(required...)`) plus optional `with_*`.
- For required-only structs, this skill does not require adding `new(required...)`; keep the simplest API that fits the crate.
- Do **not** model required fields as optional just to satisfy a construction style.

When inferring required fields on existing structs, use docs, service behavior, and call-site usage patterns.

#### g) Exemptions

**Newtype structs** are exempt from the named-field construction rules. Since a newtype wraps a single value, the full named-field struct rules do not apply. Instead, newtypes should:

- Keep the inner field **private**.
- Provide construction via `new()`, `From` impls, or associated constants.
- Provide access to the inner value via a getter (e.g., `value()`) or `Into`/`AsRef` impls.
- **Omit** `#[non_exhaustive]`.

**Builder types** (`*Builder` structs), when present, are exempt from `#[non_exhaustive]` and getters.

Serde derive macros (`Serialize`, `Deserialize`) work on private fields — no `pub(crate)` is needed for serde compatibility.

#### h) Optional builder guidance (when used)

If a separate builder type is used, follow these conventions:

1. Name it `<Type>Builder`.
2. Keep builder fields private.
3. Provide `with_*` setters for optional fields.
4. Provide terminal `build(self, ...) -> <Type>` (or `azure_core::Result<Type>` when fallible).
5. Keep required fields on `build(...)`, not as optional builder state.
6. Add `<Type>::builder(... required args ...) -> <Type>Builder` to initialize the builder type.

These conventions apply only when a builder exists; they are not a requirement to introduce one.

### Step 7 — Auto-fix or report

#### If `auto-fix` is `true`

1. Adjust visibility modifiers on structs and fields according to Steps 4–6.
2. Add `#[non_exhaustive]` to truly public structs where all named fields are public and the attribute is missing.
3. Remove `#[non_exhaustive]` from non-public structs that have it unnecessarily.
4. For truly public structs, apply field visibility decisions per Step 6c (and type-system preferences per Step 6b) rather than forcing all `pub` fields to non-public:
  - If a field has no validation/invariant requirements, `pub` is allowed.
  - If invariants/validation are required, make the field non-public and provide/update constructor or `with_*` APIs as needed.
  - When non-public fields need ownership extraction without cloning, prefer `From`/`Into` trait-based conversion; add targeted `into_*` methods only when a trait-based API is not a good fit.
  - Generate getter methods for fields that external consumers need to read and that are non-public.
5. Ensure each truly public struct has the required construction API (Step 6e), prioritizing rule compliance even if changes are semver-breaking:
  a. Enforce mixed required+optional rules strictly (`new(required...)` or `build(required...)`) and remove/avoid `Default` in that case.
  b. If no ergonomic construction API exists, add the simplest valid option:
        - all-optional/simple types: `Default` + `with_*` on the target type.
    - types with required + optional fields: `new(required...)` + `with_*`, and remove/avoid `Default`.
    - required-only types: no forced `new(required...)` and no forced `with_*`.
    - single-property types: no forced `with_*`.
        - use a separate builder only when complexity justifies it.
  c. Add getter methods for externally readable fields if missing and if the fields are non-public.
  d. Update call sites as needed to keep the crate compiling after applied fixes.
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
  External code constructing via struct literal must change to non-literal construction APIs.
```

Breaking changes include:

- A `pub` field becoming private (consumers using direct field access will break)
- Adding `#[non_exhaustive]` (consumers using struct literal construction will break)
- Removing or changing an existing construction API (`Default`, `new`, `with_*`, `builder`, or `build`)
- Adding `builder()` and a builder type is typically additive and non-breaking

## Notes

- Never modify files in `generated/` subdirectories.
- `#[non_exhaustive]` is only applicable to truly public structs; effectively-scoped and internal structs must omit it.
- For truly public structs, require `#[non_exhaustive]` when all named fields are public. If a struct has non-public fields, `#[non_exhaustive]` is optional and usually redundant for construction control.
- Fields on effectively-scoped structs can remain `pub` without additional restriction — the struct-level visibility already limits access, and repeating `pub(crate)` on every field is unnecessary noise.
- Serde derive macros (`Serialize`, `Deserialize`) work on private fields — no `pub(crate)` is needed for serde compatibility.
- Builder-pattern setters use the `with_*` prefix per [Azure SDK Rust guidelines](https://azure.github.io/azure-sdk/rust_introduction.html) (not bare field names).
- For truly public structs, fluent `with_*` support is required only when optional fields exist and the struct has more than one field. Required-only and single-property structs do not need `with_*` methods.
- `new(...)`/`build(...)` is required when required and optional fields coexist. For required-only structs, this skill does not require `new(...)`.
- If a builder type is used, follow [Azure SDK Rust builder guidelines](https://azure.github.io/azure-sdk/rust_introduction.html): keep builder fields private, keep optional setters as `with_*`, and place required params on `build()`.
- For ownership extraction from non-public fields, prefer standard `From`/`Into` traits first. Use targeted `into_*` methods as an exception when extracting a specific owned field without cloning is clearer than trait conversion.
- Reference `sdk/cosmos/AGENTS.md` for canonical model, options, and builder patterns.
- Do not skip required fixes to avoid semver-breaking outcomes; apply the rules and report all breaking changes clearly in the summary.
- When generating **new** structs, apply these rules from the start — decide field visibility from invariants and API ergonomics up front, then keep construction APIs consistent with the field mix (all-optional: `Default` + `with_*`; mixed required+optional: `new` + `with_*`; required-only: simplest API, optional builder). Also evaluate whether any constrained fields warrant a newtype or enum rather than a bare primitive type (see Step 6b).
- For **new** structs, explicitly ask the developer which fields are required if not obvious from the context.
- For **existing** structs, infer required fields from: (1) doc comments mentioning "required", (2) server rejection of default values, (3) every call site always setting the field, (4) non-`Option` type with no semantically valid zero value.
- When a field requires invariant enforcement, prefer encoding the invariant in the type system (newtypes, enums, constrained wrappers) over runtime validation in `with_*` setters or constructors. Setter-level validation (clamping, range checks, format normalization) is a last resort for invariants that cannot be practically expressed in the type system (see Step 6b).
