---
agent: "agent"
description: "Add standard source map support to a code generator"
---

# Add source map support

Add support for generating a companion source map for a generated code or API artifact. Follow the
[ECMA-426 2024 source map specification](https://tc39.es/ecma426/2024/) and adapt these requirements
to the repository's implementation language, parser, intermediate model, renderer, command-line
interface, and artifact names.

## Understand and preserve source locations

- Inspect the complete generation path before changing it: source extraction, intermediate models,
  rendering, output writing, check or comparison modes, tests, and user documentation.
- Preserve each original declaration's source file, zero-based line, and zero-based column in the
  generator's structured intermediate representation.
- Carry locations through sorting, re-exporting, lifting, normalization, filtering, and rendering.
  Do not infer source locations by reparsing generated text when structured locations can be
  retained.
- Use the actual source text and exact source spans when an upstream parser, documentation model, or
  compiler representation is lossy, reformatted, expanded, or elides declaration bodies.
- Resolve declarations to the source line that introduced them, not merely to a related body or
  generated representation. Account for language-specific equivalents of out-of-line declarations,
  repeated names in different scopes, conditional declarations, path overrides, macro arms,
  procedural or generated helpers, and multiple declarations that render similarly.
- Respect both the beginning and ending line and column of source spans. Add regressions for any
  source-location ambiguity discovered during implementation.

## Generate a standard source map

- Generate a version 3 source map next to the generated artifact, conventionally named by appending
  `.map` to the artifact name.
- Set `file` to the generated artifact's filename.
- Store every `sources` entry as a normalized path relative to the repository root. Do not store
  paths relative to the current working directory, output directory, or individual source file.
- When the source map is inside the repository, set `sourceRoot` to the normalized relative path
  from the source map's directory to the repository root. Use an empty string when both directories
  are the same.
- When the source map is outside the repository, omit `sourceRoot`; keep all `sources` entries
  repository-root-relative so consumers can resolve them against an explicitly supplied repository
  root.
- Encode `mappings` according to ECMA-426 using Base64 VLQ segments and zero-based generated and
  original positions. Prefer a mature ecosystem implementation when one fits the repository;
  otherwise isolate the encoder in a reusable module with direct conformance tests.
- Sort mappings by generated line and column. Keep output deterministic, including `sources`
  ordering; first mapped occurrence is a reasonable stable order.
- Omit optional fields such as `names` and `sourcesContent` unless the product requirements or
  consumer need them. If included, implement their standard semantics and test them.
- Serialize valid JSON with the repository's normal newline and path-separator conventions.

## Choose what to map

- Map every generated declaration-bearing code line to the exact original declaration line and
  column from which it was produced.
- A declaration-bearing code line is a generated line that introduces an API element or scope.
  Examples include a namespace or module, a type such as a struct or class, a callable such as a
  function or method, a field or property, a constant, a macro, or the language's equivalent.
- Map declarations represented by nested or child lines independently so members with the same name
  under different parents retain distinct mappings.
- Leave non-declaration lines unmapped. These normally include documentation comments, attributes,
  annotations, decorators, generated-file metadata, headings, markup fences, blank lines, and
  structural-only opening or closing delimiters.
- Keep generated line accounting based on the final emitted artifact. Filtering documentation or
  adding metadata before the code body must not shift mappings onto the wrong lines.
- Do not emit approximate mappings. If a rendered declaration has no reliable source location,
  preserve correctness by leaving it unmapped and fix the extraction path when the declaration is
  required to be mapped.

## Integrate, test, and document

- Keep source map rendering generic enough to support additional generated artifact formats.
- Generate the map by default for the applicable artifact format.
- Add an opt-out switch or equivalent configuration when the generator has a command-line or
  configurable output surface. Accept the option as a no-op for formats that do not emit maps unless
  the repository's existing CLI conventions require an error.
- Integrate source maps with existing write, check, clean, and stale-artifact behavior.
- Add unit tests for JSON fields, zero-based line and column mappings, Base64 VLQ positive and
  negative deltas, multiple source files, multiple segments on one line, unmapped line gaps,
  deterministic ordering, and malformed or unordered input handling.
- Add path tests for output at the repository root, nested inside the repository, and outside the
  repository. Verify the combination of `sourceRoot` and every `sources` value resolves according to
  the repository-root-relative contract.
- Add extraction and rendering tests for every supported declaration category and for the difficult
  source-location cases identified while inspecting the generator.
- Decode the generated map with an independent standards-compatible consumer when practical and
  verify representative generated declarations resolve to their exact original files, lines, and
  columns.
- Run the smallest relevant formatter, linter, unit tests, and end-to-end generation check.
- Update user and maintainer documentation with the standard link, artifact location, mapping scope,
  path-resolution behavior, opt-out behavior, and consumer guidance.
