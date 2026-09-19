---
agent: "agent"
description: "Add documentation comments patch support to a code generator"
---

# Add documentation comments patch support

Add support for generating a companion unified diff that inserts documentation comments into a
generated code artifact that otherwise omits them. Adapt these requirements to the repository's
language, parser, intermediate model, renderer, command-line interface, and artifact names.

## Understand the generator

- Inspect the complete generation path before changing it: source extraction, intermediate models,
  rendering, output writing, check or comparison modes, tests, and user documentation.
- Reuse structured documentation, attribute or decorator, declaration, and source-location data
  already available in the generator. Do not recover these distinctions by parsing rendered text
  when the generator can preserve them explicitly.
- Keep documentation available until both the documentation-free artifact and its comments patch
  have been rendered from the same ordered representation.
- Preserve existing generated output except for the new companion artifact and any explicitly
  requested integration changes.

## Generate the comments patch

- Write a valid unified diff whose base is the generated artifact without documentation comments.
- Make the patch insertion-only: applying it adds documentation comments without replacing or
  deleting generated code.
- Emit one hunk for each contiguous documentation-comment block. Do not merge separately documented
  declarations into one hunk, including adjacent members.
- After a documentation block, include as unchanged context:
  1. every immediately following attribute, annotation, or decorator line, from zero to many; then
  2. the first declaration-bearing code line for the documented element.
- A declaration-bearing code line is the first line that introduces the documented API element or
  scope. Examples include a namespace or module, a type such as a struct or class, a callable such as
  a function or method, a field or property, a constant, a macro, or the language's equivalent.
- Documentation lines, attributes, annotations, decorators, generated-file metadata, headings,
  markup fences, blank lines, comments unrelated to the documentation block, and structural-only
  delimiters are not declaration-bearing code lines.
- Treat attributes or decorators as anchors only when they are contiguous between the documentation
  block and its declaration. Preserve their original order and include all of them.
- Include the declaration line even when there are no attributes or decorators.
- Keep unified-diff paths, hunk ranges, line counts, prefixes, and newline handling valid for the
  target artifact. Documentation lines do not exist in the base artifact; unchanged attributes,
  declarations, and other code do.
- Produce an empty patch file when no documentation comments are available.
- Integrate the artifact with existing write, check, clean, or opt-out behavior consistently. If the
  generator already has a documentation switch, use it rather than introducing an overlapping
  control.

For example, a hunk should have this conceptual shape:

```diff
+documentation comment
 attribute or decorator
 another attribute or decorator
 declaration that introduces the documented element
```

The leading spaces shown on unchanged lines are unified-diff context prefixes, not source
indentation.

## Test and document the behavior

- Add focused tests for documentation followed directly by a declaration.
- Add focused tests for multiple stacked attributes or decorators before a declaration.
- Verify that documented namespaces or modules, types, and members use the same anchoring rules.
- Verify that adjacent documented declarations remain separate hunks, including when each has
  attributes or decorators.
- Verify nested and indented declarations, correct hunk line numbers, and exact context prefixes.
- Verify that no documentation produces an empty patch.
- Apply the generated patch to a representative generated artifact and confirm the result contains
  the expected documentation in the correct locations without changing code.
- Run the smallest relevant formatter, linter, unit tests, and end-to-end generation check.
- Update user and maintainer documentation to describe the artifact, how to apply it, and the
  one-documentation-block-per-hunk anchoring contract.
