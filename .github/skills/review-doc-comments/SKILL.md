---
name: review-doc-comments
description: Write and review Rust public API documentation for clarity, accuracy, Microsoft style, useful links and sections, and buildable examples. Use for public API reviews even when doc comments did not change.
---

# Review public API doc comments

Write precise, standalone reference documentation for developers using the public API.
Prioritize human understanding while keeping comments easy for LLMs to interpret. Explain
what the API does, when to use it, and what callers must know. Avoid marketing, conversation,
keyword stuffing, and unnecessary detail.

## When to use

Use this skill when writing or revising public API documentation. Also use it when reviewing
changes to public signatures, types, fields, variants, behavior, errors, panics, or feature
availability, even if no doc comments changed.

## Review workflow

1. Identify the affected public surface, including re-exports and observable behavior changed
   by private implementation. Stay within the requested scope.
2. Read each affected API's complete documentation and relevant implementation, tests, and
   surrounding type or module docs. Include `///`, `//!`, and documentation attributes.
3. Apply every criterion below. Verify claims against code; do not invent guarantees, errors,
   panics, or safety requirements.
4. For reviews, report findings without editing. When asked to fix docs, preserve meaning and
   make focused edits. Never edit `generated/`; report the issue and route it to the TypeSpec
   specification or generator.
5. Validate relevant links and examples when possible. State what was checked and any
   limitations; never assume code compiles.

## Review criteria

### 1. Apply Microsoft style to reference documentation

Apply these guidelines directly:

1. **Express ideas simply.** Remove needless words and complexity without losing required
   constraints or qualifications.
2. **Use natural language.** Prefer familiar words and direct sentences. Explain necessary
   domain terms and avoid unexplained jargon.
3. **Be approachable.** Use contractions when natural and unambiguous, but avoid banter and
   forced familiarity.
4. **Lead with the useful fact.** Start with the API's purpose or behavior. Surface important
   requirements and caller actions early.
5. **Include enough, not everything.** Keep details needed to choose and use the API; remove
   repetition, filler, and cryptic compression.
6. **Use sentence case.** Preserve proper names and exact Rust identifier casing.
7. **Punctuate by context.** Punctuate sentences, not headings. Keep list punctuation
   consistent.
8. **Use serial commas.** Include the comma before the final conjunction in lists of three or
   more items.
9. **Use conventional spacing.** Use one space after sentences and colons, no spaces around
   em dashes, and preserve meaningful Markdown formatting.
10. **Write directly.** Prefer active voice and meaningful verbs. Remove openings such as
    "You can" and "There is". Use natural summaries such as "Returns", "Creates", or
    "Deletes" where appropriate.

Favor accuracy and readability over mechanical compliance.

### 2. Keep the focus on the caller

- Document observable behavior, inputs, outputs, defaults, units, limits, ownership effects,
  and usage requirements when relevant.
- Remove internal names, mechanics, and maintainer jargon unless callers need them.
- Ensure the comment is understandable without reading private code.

### 3. Remove irrelevant background and references

- Remove internal specifications, meeting history, issue discussions, implementation phases,
  and historical rationale that do not help callers.
- Put useful maintainer rationale in implementation comments or design documents.
- Keep relevant public protocol, service, compatibility, and deprecation guidance. Explain
  why linked material matters rather than making readers reconstruct the contract.

### 4. Avoid LLM-like prose

- Remove empty praise and inflated claims such as "seamlessly", "powerful", and "robust".
- Remove conversational lead-ins such as "Let's dive in", "Great question", "As mentioned
  earlier", and "It's worth noting that".
- Avoid rhetorical questions, slogans, excessive emphasis, repetitive summaries, elaborate
  headings, and patterns such as "Not X, not Y, Z."
- Make explanations standalone; do not rely on chat history, patch descriptions, or unstated
  antecedents.
- Preserve clear prose. Do not report subjective rewrites as bugs or speculate about
  authorship.

For example, replace internal, promotional wording with the caller-visible behavior:

```text
/// Downloads the blob's contents.
///
/// Create a [`StorageClient`] to access blobs in a storage account.
```

Verify the behavior and symbol resolution before proposing text.

### 5. Link Rust symbols

- Use rustdoc intra-doc links for linkable public items: `` [`StorageClient`] `` rather than
  plain text or code formatting.
- Qualify or disambiguate targets when needed, such as `` [`StorageClient::new`] `` or
  `` [`new`](StorageClient::new) ``.
- Verify links resolve from the documenting item's scope to public documentation.
- Format parameters, variables, and literals as code, not links. Do not place Markdown links
  in code fences.

### 6. Document the contract with conventional sections

Start with a concise summary, then a blank line and useful details. Add only applicable
rustdoc sections:

- **`# Errors`**: meaningful failure conditions and relevant propagated errors, not generic
  statements.
- **`# Panics`**: verified direct or propagated panic conditions.
- **`# Safety`**: caller obligations for `unsafe` APIs and implementor obligations for unsafe
  traits.
- **`# Examples`**: self-contained usage examples when helpful.

Keep errors, panics, and safety obligations distinct. Document other important contract
details, such as cancellation or feature requirements, without empty boilerplate. Follow
surrounding conventions.

### 7. Make examples executable or at least buildable

- Prefer runnable doctests with useful assertions when no external resources are required.
- Use `rust,no_run` for examples requiring Azure services, credentials, networking, or other
  setup. They must still compile. Explain prerequisites without exposing secrets.
- Verify imports, features, public signatures, async runtime setup, and error handling. Use
  hidden `#` lines only for supporting scaffolding.
- Do not present placeholders or pseudocode as buildable Rust.
- Do not use `ignore`, `text`, or `compile_fail` to hide broken examples. Reserve
  `compile_fail` for intentional rejected-code demonstrations.
- Validate each affected package from the repository root:

  ```bash
  cargo test -p <package-name> --doc --all-features
  cargo doc -p <package-name> --no-deps --all-features
  ```

Inspect warnings, rendered headings, and examples. Test promised feature combinations
separately; `--all-features` does not verify them. Do not run live Azure operations solely
for documentation review.

## Review output

For each finding, give the API and location, unmet criterion, caller impact, and a concrete
correction. Distinguish incorrect contracts, broken links, and non-compiling examples from
lower-priority style issues. Follow the surrounding review's severity threshold.

Summarize the reviewed scope, including unchanged comments, and validation results or
limitations. If nothing actionable is found, say so without inventing style nits.
