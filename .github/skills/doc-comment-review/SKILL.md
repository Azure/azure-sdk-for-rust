---
name: doc-comment-review
description: Review Rust public API doc comments for human-first clarity, Microsoft style, user relevance, Rust links and sections, and buildable examples. Use whenever reviewing code that affects public APIs, even if no doc comments changed.
---

# Review public API doc comments

Write for humans first, LLMs second. The primary audience is a developer using the
public API. Help that developer understand what the API does, when to use it, and
what they must know to use it correctly. Consider how models interpret the comments
as well, while prioritizing human understanding.

Doc comments are standalone reference documentation. Use a professional, natural,
precise tone, with enough detail to support correct use. They are not blog posts,
marketing copy, message board replies, or snippets of a conversation. Do not optimize
for keyword density, punchy phrasing, or the shortest possible text.

## When to use

Use this skill whenever a code review involves public APIs, including changes to
signatures, types, fields, variants, behavior, errors, panics, or feature availability.
Review the affected API's existing documentation even if the diff contains no doc
comment changes. Also use it when writing or revising public API documentation.

## Review workflow

1. Identify the affected public surface, including re-exports and APIs whose observable
   behavior changes through private implementation changes. Stay within the requested
   review scope.
2. Read the complete doc comments and relevant implementation, tests, and surrounding
   type or module documentation. Include `///`, `//!`, and other Rust documentation
   attributes where applicable. Do not limit the review to changed comment lines.
3. Apply every criterion below. Compare documented claims with the implementation;
   do not invent guarantees, error conditions, panic conditions, or safety requirements.
4. In a review-only task, report findings and proposed corrections without editing.
   When asked to fix documentation, preserve technical meaning and make focused edits.
   Never hand-edit files under `generated/`; report the issue and route the correction
   to its TypeSpec specification or generator as appropriate.
5. Validate relevant links and examples when execution is available. Report what was
   checked, what was not checked, and any blockers. Do not claim a snippet compiles
   merely because it looks plausible.

## Review criteria

### 1. Apply Microsoft style to reference documentation

Use the [Microsoft Style Guide](https://learn.microsoft.com/style-guide/).
The following checklist paraphrases and adapts its
[Top 10 tips for style and voice](https://learn.microsoft.com/style-guide/top-10-tips-style-voice)
so routine reviews do not require fetching the guide:

1. **Express the idea simply.** Remove unnecessary wording and complexity without
   dropping constraints, qualifications, or other information needed for correct use.
2. **Use natural language.** Prefer familiar words and straightforward sentences.
   Explain necessary domain terms; avoid unexplained jargon. Read prose aloud to
   catch awkward wording, but do not turn reference content into a conversation.
3. **Be approachable.** Contractions are preferred when natural and unambiguous.
   Avoid stiff or impersonal phrasing without adding greetings, banter, or forced
   familiarity.
4. **Lead with the useful fact.** Begin with a concise summary of the API's purpose
   or behavior. Put important requirements and caller actions where readers can
   find them quickly.
5. **Include enough, not everything.** Keep details that help callers choose and use
   the API. Remove repetition and filler; do not compress explanations into cryptic,
   information-dense fragments.
6. **Use sentence case.** Capitalize headings normally, preserving proper names and
   the exact spelling and case of Rust identifiers.
7. **Punctuate by context.** Punctuate prose sentences. Do not add periods or colons
   to headings. Keep list punctuation consistent with whether entries are sentences.
8. **Use serial commas.** Include the comma before the final conjunction in lists
   of three or more items.
9. **Keep spacing conventional.** Use one space between prose sentences and after
   colons. If using an em dash, do not put spaces around it. Preserve meaningful
   Markdown indentation and code formatting.
10. **Make statements direct.** Prefer active voice and meaningful verbs; remove
    unnecessary openings such as "You can" and "There is". Function summaries often
    start with "Returns", "Creates", or "Deletes"; type summaries can naturally
    describe what the type represents. Do not force every sentence into an imperative.

Apply these tips in service of accuracy and readability, not as mechanical rewrite
rules. A useful explanation is better than a slogan.

### 2. Keep the focus on the caller

- Describe observable behavior, inputs, outputs, defaults, units, limits, ownership
  implications, and requirements when relevant to using the API.
- Remove internal component names, implementation mechanics, and maintainer jargon
  that do not help callers. Keep necessary user-facing concepts, explaining them
  through their effect on usage rather than their internal design.
- Check that a reader can understand the comment without reading private code.

### 3. Remove irrelevant background and references

- Remove references to internal specifications, design meetings, implementation
  phases, issue discussions, and historical decisions that do not help users.
- Move maintainer rationale to ordinary implementation comments or design documents
  when it is worth preserving; it does not belong in public API reference prose.
- Keep relevant public protocol or service documentation, and actionable compatibility
  or deprecation guidance. Explain their relevance instead of requiring readers to
  reconstruct the contract from a link.

### 4. Avoid LLM-like prose

- Remove generic praise and inflated claims such as "seamlessly", "powerful", or
  "robust" when they convey no concrete behavior. Review meaning, not a banned-word list.
- Remove conversational lead-ins and replies such as "Let's dive in", "Great question",
  "As mentioned earlier", and "It's worth noting that". State the relevant fact instead.
- Avoid rhetorical questions, slogans, excessive emphasis, repetitive summaries,
  and elaborate headings around simple facts.
- Avoid rhetorical cliches, including patterns such as "Not X, not Y, Z." and
  "X. Never Y". These examples are not exhaustive. State behavior and requirements
  directly; prioritize clarity over rhetorical effect.
- Use complete, standalone explanations. Do not rely on a prior chat, patch description,
  or unstated antecedent. Balance detail and brevity instead of maximizing either.
- Preserve clear existing prose. Do not report subjective rewrites as correctness bugs
  or speculate about whether a person or model wrote the text.

For example, assuming the API actually has the stated behavior, replace this excerpt:

```text
/// Let's dive in! This powerful helper seamlessly leverages our internal request
/// adapter, introduced in phase 2 of the design spec, to unlock blob downloads.
/// You can use StorageClient to get started.
```

With user-focused reference content:

```text
/// Downloads the blob's contents.
///
/// Create a [`StorageClient`] to access blobs in a storage account.
```

These are illustrative comment excerpts, not complete runnable examples. Verify
the behavior and symbol resolution in the actual API before proposing a replacement.

### 5. Link Rust symbols

- Use rustdoc intra-doc links for references to types, traits, methods, modules,
  constants, and other linkable Rust items: prefer `` [`StorageClient`] `` over
  plain `StorageClient` or code formatting alone.
- Use qualified targets when needed, such as `` [`StorageClient::new`] `` or
  `` [`new`](StorageClient::new) ``. Use rustdoc disambiguators if names collide.
- Verify links resolve from the documenting item's scope and lead to public,
  user-accessible documentation. Do not link to private implementation details.
- Format parameter names, local variables, and literal values as code, not links,
  unless they actually name a linkable item. Do not add Markdown links inside code fences.

### 6. Document the contract with conventional sections

Use a concise summary, a blank line, then useful details. Follow established rustdoc
patterns with single-`#` headings inside doc comments, without empty boilerplate:

- **`# Errors`** for APIs returning `Result`, including async APIs and result aliases.
  Describe meaningful failure conditions and relevant propagated failures, not merely
  "Returns an error if the operation fails". Verify claims against actual behavior.
- **`# Panics`** when the API can panic. State the triggering conditions, including
  relevant panics propagated from called code. Do not invent panic cases or add a
  section claiming panics where none are known.
- **`# Safety`** for `unsafe` functions and methods. State the obligations the caller
  must satisfy to avoid undefined behavior; "This function is unsafe" is insufficient.
  For unsafe traits, document implementor obligations as well.
- **`# Examples`** for usage examples when they help. Explain enough context to make
  the example understandable on its own.

Keep errors, panics, and safety obligations distinct. Document other important contract
details, such as cancellation or feature requirements, where relevant without forcing
a section for every possible topic. Match consistent patterns in surrounding APIs.

### 7. Make examples executable or at least buildable

- Prefer ordinary Rust doctests that run and assert useful behavior when they need no
  external services or credentials.
- Use `rust,no_run` for examples requiring Azure services, credentials, network access,
  or other external setup. These must still compile; `no_run` skips execution, not
  type checking. Explain prerequisites without including secrets.
- Check imports, dependency and feature availability, current public API signatures,
  async runtime setup, and error handling. Use rustdoc hidden `#` lines for necessary
  scaffolding without hiding the central operation.
- Avoid undefined placeholders, omitted expressions, or pseudocode in examples
  presented as buildable Rust. Use valid values or environment variables where needed.
- Do not use `ignore`, `text`, or `compile_fail` to hide a broken usage example.
  Reserve `compile_fail` for intentional demonstrations of rejected code and label
  genuinely non-Rust illustrations honestly.
- From the repository root, validate each affected package with:

  ```bash
  cargo test -p <package-name> --doc --all-features
  cargo doc -p <package-name> --no-deps --all-features
  ```

  Inspect rustdoc warnings for unresolved or private links and check rendered headings
  and examples. Check specific feature combinations if the documentation promises
  them; an all-features build alone does not verify those promises. Do not run live
  Azure operations just to review documentation.

## Review output

For each finding, identify the API and file location, the unmet criterion, its effect
on a caller, and a concrete correction. Distinguish incorrect contracts, broken links,
and non-compiling examples from lower-priority clarity or style suggestions. Respect
the surrounding review's severity and reporting conventions.

Summarize the scope reviewed, including unchanged comments, and validation results or
limitations. If no actionable issues are found, say so without manufacturing style nits.
