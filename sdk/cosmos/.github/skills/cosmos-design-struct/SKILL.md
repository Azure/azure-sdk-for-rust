---
name: cosmos-design-struct
description: >
  Enforce consistent struct design conventions across sdk/cosmos crates. Validates visibility modifiers,
  field privacy, #[non_exhaustive] usage, builder pattern (builder()/build() with with_* setters),
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
      If true, automatically fix violations (adjust visibility, add #[non_exhaustive], generate builders/getters,
      remove Default). If false, only report violations with proposed changes.
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

1. **Truly public** — The struct is `pub` and **all** ancestor modules up to the crate root are also `pub` (the struct is reachable from outside the crate). These structs get the full set of rules: `#[non_exhaustive]`, private fields, getters, builder type with `with_*` setters, and a `build()` method that takes required fields as parameters.

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

**Rationale**: Tokio and other major Rust crates universally keep fields private on public structs. Making a field `pub` is a semver commitment — removing it later is a breaking change. Private fields with accessors give full control over the API surface. When a field has a public getter but crate-internal code in other modules also needs direct access — for mutation, move/consume, or nested field traversal — use `pub(crate)`. The public getter exists for external consumers; `pub(crate)` avoids forcing crate-internal code through the public API when direct access is more ergonomic or necessary (e.g., `options.method_options.context` or `options.fault_injection_enabled = true`). Only make a field fully private when **all** access — including crate-internal — goes through the accessor methods.

**How to determine the correct visibility**: Before deciding between private and `pub(crate)`, search the crate for direct field access outside the defining module (e.g., `grep` for `.field_name` in other `.rs` files). If any crate-internal code accesses the field directly — not through a getter or setter — the field must be `pub(crate)`. If all access goes through the accessor methods, the field should be private (no modifier).

**Examples**:

- `ItemOptions::session_token` — has a getter `session_token()` and a setter `with_session_token()`. All crate-internal code accesses it through the getter or through `AsHeaders` in the same module. No direct field access from other modules. → **Private** (no modifier).

- `ItemOptions::method_options` — has a getter `method_options()`. But `container_client.rs` accesses `options.method_options.context` directly for nested field traversal. → **`pub(crate)`**.

- `CosmosClientOptions::fault_injection_enabled` — has a getter `fault_injection_enabled()`. But `client_builder.rs` writes `options.fault_injection_enabled = true` directly. → **`pub(crate)`**.

#### c) Getter methods for readable fields

Every field that external consumers need to **read** must have a getter method:

- Named after the field (e.g., `fn session_token(&self) -> &str`)
- Returns `&T` for non-`Copy` types, or `T` for `Copy` types (e.g., `bool`, `u32`, `f64`)

#### d) `with_*` setter methods (builder pattern)

`with_*` setters live **only** on the builder type (see Step 6h), never on the target struct itself. External and internal consumers set optional fields via the builder:

```rust
// On the builder type, NOT on the target struct:
pub fn with_session_token(mut self, value: impl Into<String>) -> Self {
    self.session_token = Some(value.into());
    self
}
```

- Prefix: `with_<field_name>`
- Takes `mut self`, returns `Self` (where `Self` is the **builder** type)
- Use `impl Into<T>` for string-like and convertible parameters where appropriate
- Do **not** generate `with_*` setters for required fields — they are set exclusively via the builder's `build()` method (see Step 6f)
- Do **not** place `with_*` setters on the target struct — all construction goes through the builder

#### e) No `Default` on truly public structs

Truly public structs must **not** derive or implement `Default`. `Default` on a public struct is public API — it provides an uncontrolled construction path that bypasses the builder (Step 6h). All construction must go through `Type::builder()...build()`.

This applies to **all** truly public structs, including all-optional ones (options structs, `IndexingPolicy`, etc.). Even if every field has a valid zero value, `Default` would allow callers to bypass the builder.

**Serde compatibility:** Removing struct-level `Default` does not break `Deserialize`. Instead, annotate each optional field with `#[serde(default)]` so serde uses the field type's own `Default` (e.g., `Option::None`, `Vec::new()`, `false`). Required fields must always be present in the JSON (the server always returns them).

**The builder type** (Step 6h) must **not** derive or implement `Default` either — that would allow `TypeBuilder::default()` to bypass the `Type::builder()` factory method. The `builder()` factory constructs the builder via struct literal syntax (possible because it lives in the same module). The builder's private fields prevent external struct literal construction.

**All-optional structs** (options structs, `IndexingPolicy`, etc.) — use the builder:

```rust
let options = ItemOptions::builder()
    .with_session_token(token)
    .with_priority(PriorityLevel::High)
    .build();

let policy = IndexingPolicy::builder()
    .with_automatic(true)
    .with_indexing_mode(IndexingMode::Consistent)
    .build();
```

**Structs with required fields** (`ContainerProperties`, `DatabaseProperties`, etc.) — use the builder with required params on `build()`:

```rust
let props = ContainerProperties::builder()
    .with_default_ttl(Duration::from_secs(3600))
    .build("myContainer", "/partitionKey");

let db = DatabaseProperties::builder()
    .build("myDatabase");
```

#### f) Required fields and the builder's `build()` method

A **required field** is one that must be set for the struct to be semantically valid. Required fields are set exclusively via the builder's `build()` method — they must **not** appear as `with_*` setters on the builder.

**Determining which fields are required:**

- For **new** structs being created: the developer specifies which fields are required. If unclear, ask.
- For **existing** structs: infer from the combination of:
  1. Doc comments or server documentation that say "required"
  2. Non-`Option` fields whose `Default` value is meaningless (e.g., `id: ""`, `paths: vec![]`, `dimensions: 0`)
  3. Usage patterns where every call site always sets the field
  4. Serde behavior — if the server rejects the default value, the field is required

**Indicators that a field is NOT required:**

- The field type is `Option<T>`
- The field has a semantically valid default (e.g., `bool` defaulting to `false`, an enum with a `#[default]` variant)
- The struct is an options/configuration struct where all fields are optional overrides

**Rules for all truly public structs** (regardless of whether they have required fields):

1. **Provide `pub fn builder()`** returning a `<Type>Builder` (see Step 6h).
2. **Do not derive or implement `Default`** on the target struct — `Default` is public API that provides an uncontrolled construction path bypassing the builder. For serde deserialization of optional fields, annotate each optional field with `#[serde(default)]` rather than relying on a struct-level `Default`.
3. **Do not place `with_*` setters on the target struct** — they belong on the builder only.
4. **Still provide getter methods for all fields** (required and optional) on the target struct — external consumers need to read them.

**Additional rules for structs with required fields:**

5. **Do not provide `with_*` setters for required fields on the builder** — required fields can only be set via `build()`. This prevents partially-constructed instances where a required field is still at its meaningless default.
6. **Define required fields as parameters on `build()`** — use `impl Into<T>` for ergonomic conversion (e.g., `impl Into<Cow<'static, str>>` for string fields, `impl Into<PartitionKeyDefinition>` for partition keys).

**Example — struct with required fields:**

```rust
/// Properties of a Cosmos DB container.
///
/// # Required fields
///
/// * `id` — The unique identifier for the container.
/// * `partition_key` — The partition key definition.
///
/// Use [`ContainerProperties::builder()`] to construct an instance.
impl ContainerProperties {
    pub fn builder() -> ContainerPropertiesBuilder {
        ContainerPropertiesBuilder {
            indexing_policy: None,
            unique_key_policy: None,
            conflict_resolution_policy: None,
            vector_embedding_policy: None,
            default_ttl: None,
            analytical_storage_ttl: None,
        }
    }

    // Getters for ALL fields (required + optional):
    pub fn id(&self) -> &str { &self.id }
    pub fn partition_key(&self) -> &PartitionKeyDefinition { &self.partition_key }
    pub fn indexing_policy(&self) -> Option<&IndexingPolicy> { self.indexing_policy.as_ref() }
    // ... no with_*() setters on the target struct
}

/// Builder for [`ContainerProperties`].
pub struct ContainerPropertiesBuilder {
    indexing_policy: Option<IndexingPolicy>,
    unique_key_policy: Option<UniqueKeyPolicy>,
    conflict_resolution_policy: Option<ConflictResolutionPolicy>,
    vector_embedding_policy: Option<VectorEmbeddingPolicy>,
    default_ttl: Option<Duration>,
    analytical_storage_ttl: Option<Duration>,
}

impl ContainerPropertiesBuilder {
    // with_* setters ONLY for optional fields:
    pub fn with_indexing_policy(mut self, value: IndexingPolicy) -> Self {
        self.indexing_policy = Some(value);
        self
    }

    pub fn with_default_ttl(mut self, value: Duration) -> Self {
        self.default_ttl = Some(value);
        self
    }

    // ... no with_id(), no with_partition_key()

    /// Build a [`ContainerProperties`] with the required fields.
    pub fn build(
        self,
        id: impl Into<Cow<'static, str>>,
        partition_key: impl Into<PartitionKeyDefinition>,
    ) -> ContainerProperties {
        ContainerProperties {
            id: id.into(),
            partition_key: partition_key.into(),
            indexing_policy: self.indexing_policy,
            unique_key_policy: self.unique_key_policy,
            conflict_resolution_policy: self.conflict_resolution_policy,
            vector_embedding_policy: self.vector_embedding_policy,
            default_ttl: self.default_ttl,
            analytical_storage_ttl: self.analytical_storage_ttl,
            system_properties: SystemProperties::default(),
        }
    }
}
```

**Example — all-optional struct (usage from an external crate):**

```rust
// All fields are optional, so build() takes no parameters.
let options = ItemOptions::builder()
    .with_session_token("token-abc")
    .with_consistency_level(ConsistencyLevel::Session)
    .build();

// Or with no options set at all:
let defaults = ItemOptions::builder().build();
```

**Serde compatibility note:** Removing struct-level `Default` does **not** break serde deserialization as long as:

- Required fields are always present in the JSON (they are — the server always returns them)
- Optional fields use `#[serde(default)]` and `#[serde(skip_serializing_if = "Option::is_none")]` (or equivalent for `Vec`, `bool`, etc.)

#### g) Exemptions

**Newtype structs** are exempt from rules (a), (b), (c), (d), (e), (f), and (h) above. Since a newtype wraps a single value, the full named-field struct rules (private fields, getters, builders, `#[non_exhaustive]`) do not apply. Instead, newtypes should:

- Keep the inner field **private**.
- Provide construction via `new()`, `From` impls, or associated constants.
- Provide access to the inner value via a getter (e.g., `value()`) or `Into`/`AsRef` impls.
- **Omit** `#[non_exhaustive]`.

**Builder types** (`*Builder` structs) are exempt from `#[non_exhaustive]` and getters. Builder fields are always private (making `#[non_exhaustive]` unnecessary — private fields already prevent external struct literal construction). Builders must **not** derive or implement `Default` — that would allow `TypeBuilder::default()` to bypass `Type::builder()`. The `builder()` factory method uses struct literal syntax internally (same module) to construct the builder.

All other truly public structs — including options structs, serde model structs, and configuration structs — must use a separate builder type for construction per Steps 6d–6f and 6h. There are no further exemptions.

Serde derive macros (`Serialize`, `Deserialize`) work on private fields — no `pub(crate)` is needed for serde compatibility.

#### h) Builder type requirements

Every truly public struct must provide a **separate builder type** for construction, following the [Azure SDK Rust builder guidelines](https://azure.github.io/azure-sdk/rust_introduction.html).

**Factory method on the target type:**

- `pub fn builder() -> <Type>Builder` — returns a new builder instance via struct literal syntax (possible because `builder()` is defined in the same module as the builder). This is the **only** way to obtain a builder.

**Builder type rules:**

1. Named `<Type>Builder` (e.g., `ContainerProperties` → `ContainerPropertiesBuilder`, `ItemOptions` → `ItemOptionsBuilder`).
2. All fields are **private**. Private fields prevent external struct literal construction, making `#[non_exhaustive]` unnecessary on the builder.
3. Does **not** derive or implement `Default` — `TypeBuilder::default()` would be an uncontrolled construction path bypassing `Type::builder()`. The `builder()` factory method constructs the builder via struct literal syntax internally.
4. Provides `pub fn with_<field>(mut self, value: ...) -> Self` setters for each **optional** field.
5. Provides a terminal `pub fn build(self, ...) -> <Type>` method:
   - For all-optional structs: `build(self)` takes **no** arguments.
   - For structs with required fields: `build(self, required_args)` takes all required fields as parameters. Use `impl Into<T>` for ergonomic conversion.
   - Returns the constructed target type (or `azure_core::Result<Type>` if construction can fail).
6. Does **not** provide `with_*` setters for required fields — they are set exclusively via `build()`.

**Construction patterns:**

```rust
// All-optional struct:
let options = ItemOptions::builder()
    .with_session_token(token)
    .build();

// Struct with required fields:
let props = ContainerProperties::builder()
    .with_default_ttl(Duration::from_secs(3600))
    .build("myContainer", "/partitionKey");

// Struct with required fields, no optional overrides:
let db = DatabaseProperties::builder()
    .build("myDatabase");
```

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
5. For each truly public struct, generate a builder type (per Steps 6d–6h):
   a. Generate a `pub fn builder()` factory method on the target struct returning `<Type>Builder`.
   b. Generate a `<Type>Builder` struct with private fields (no `Default` derive).
   c. Generate `with_*` setters on the builder for each optional field.
   d. Generate a `build(self, ...)` method on the builder — with required fields as parameters (if any), returning the target type.
   e. Remove `pub fn new(...)` from the target struct (if it exists).
   f. Remove `pub fn with_*(...)` from the target struct (if they exist).
   g. Remove `#[derive(Default)]` (or manual `impl Default`) from the target struct.
   h. Add `#[serde(default)]` on each optional field if the struct derives `Deserialize`.
   i. Update all call sites in the crate and tests:
      - `Type::default().with_opt(x)` → `Type::builder().with_opt(x).build()`
      - `Type::new(v).with_opt(x)` → `Type::builder().with_opt(x).build(v)`
      - `Type::default()` → `Type::builder().build()`
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
- New builder type generated (`<Type>Builder`)
- New `builder()` factory method added
- `Default` derive removed from target struct
- `with_*` setters moved from target struct to builder
- `new()` constructor removed (replaced by `builder().build()`)
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
- Removing `Default` from a struct (consumers calling `Type::default()` will break — must switch to `Type::builder().build()`)
- Removing `new()` (consumers calling `Type::new(v)` will break — must switch to `Type::builder().build(v)`)
- Removing `with_*` setters from the target struct (consumers chaining `Type::new(v).with_opt(x)` must switch to `Type::builder().with_opt(x).build(v)`)
- Adding `builder()` and the builder type is an additive, non-breaking change

## Notes

- Never modify files in `generated/` subdirectories.
- `#[non_exhaustive]` is **only** for truly public structs. Effectively-scoped and internal structs must omit it — it adds no value when external code cannot reach the struct.
- Fields on effectively-scoped structs can remain `pub` without additional restriction — the struct-level visibility already limits access, and repeating `pub(crate)` on every field is unnecessary noise.
- Serde derive macros (`Serialize`, `Deserialize`) work on private fields — no `pub(crate)` is needed for serde compatibility.
- Builder-pattern setters use the `with_*` prefix per [Azure SDK Rust guidelines](https://azure.github.io/azure-sdk/rust_introduction.html) (not bare field names).
- Builder types follow the [Azure SDK Rust builder guidelines](https://azure.github.io/azure-sdk/rust_introduction.html): builders must be their own type, required params go on `build()`, and all builder fields use `with_*` setters.
- All truly public structs use a separate builder type for construction. The `Type::builder().with_*().build(...)` pattern applies uniformly — the only difference between all-optional and required-field structs is whether `build()` takes arguments.
- No truly public struct derives `Default` — it would be an uncontrolled public construction path bypassing the builder. Field-level `#[serde(default)]` replaces struct-level `Default` for deserialization.
- Builder fields are always private, making `#[non_exhaustive]` unnecessary on builders. Builders must not derive `Default` — `Type::builder()` is the sole entry point.
- Reference `sdk/cosmos/AGENTS.md` for canonical model, options, and builder patterns.
- Breaking changes in public surface area require explicit acknowledgment from the developer before merging.
- When generating **new** structs, apply these rules from the start — it is far easier to design with private fields, getters, and builders than to retrofit them later.
- For **new** structs, explicitly ask the developer which fields are required if not obvious from the context.
- For **existing** structs, infer required fields from: (1) doc comments mentioning "required", (2) server rejection of default values, (3) every call site always setting the field, (4) non-`Option` type with no semantically valid zero value.
