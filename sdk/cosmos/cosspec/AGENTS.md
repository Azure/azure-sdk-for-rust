# cosspec agent instructions

These rules add to the [Cosmos SDK instructions](../AGENTS.md). Read
[spec 0030] for the design and the [README](README.md) to run specs.

## Approach

`cosspec` is the normative specification of the Azure Cosmos DB Rust SDK's
behavior, written as executable tests. Each spec states one behavior in its
doc comment and demonstrates it in a short body. Specs request what they need
(injectables such as `SharedContainer`, `ExclusiveContainer`, and `HttpTap`,
plus profiles, datasets, and capabilities), and the framework provides it on
any backend: the in-memory emulator, the Azure Cosmos DB Emulator, or live
accounts. Specs never depend on SDK internals, so they stay stable when the
implementation is refactored and change only when the behavior does.

Humans must be able to review every spec and the framework behind it. As
agents merge more code, these specs are how people check what the SDK does.

## Rules

These rules are strict. When a change can't follow them, stop and ask.

### 1. The goal is a clear, complete, normative specification

- Write each spec as the definitive statement of one behavior. Its doc
  comment states the behavior in plain language, as a rule the SDK must
  follow, and its body proves exactly that.
- Make specs complete. Cover the behavior's observable outcomes, including
  errors, retries, and the HTTP traffic that matters, and add specs for
  missing cases instead of stretching an existing one.
- Assert observable behavior: responses, items, status codes, feed ranges,
  and `HttpTap` evidence. Never assert on SDK internals.
- Never rely on SDK diagnostics to prove behavior; use real evidence from
  types like `HttpTap`. Diagnostics assertions may be added alongside that
  evidence, not instead of it.
- Never weaken, loosen, or delete an assertion to make a spec pass. If the
  SDK is wrong, mark the spec `#[known_issue("<accurate reason>", ..)]`,
  scoped to the affected profiles, and record it under Known issues in
  [spec 0030]. Remove the attribute in the change that fixes the behavior.
- When behavior changes on purpose, update the spec's doc comment and body in
  the same change so the diff reads as a change to the specification.
- Keep specs backend-agnostic. Declare needed capabilities with
  `#[requires(..)]` and profiles with `#[service_profiles(..)]`; never branch
  on the backend in a spec body.

### 2. Use and grow the framework to keep specs concise

- Keep spec bodies short and linear: arrange, act, assert. A reviewer should
  understand a spec without reading anything but the DSL.
- Reuse existing injectables, profiles, datasets, `faults` builders, and
  assertions before writing setup or checks inline.
- When the same setup or check would obscure the behavior under test, or
  repeats across specs, add or extend a framework helper instead of copying
  code into specs.
- Prefer plain SDK calls in spec bodies. Helpers remove noise; they must not
  hide the SDK call being specified.
- Use `SharedContainer` only for read-only specs; use the `Exclusive*`
  injectables for anything that writes or changes topology.
- After adding, renaming, or changing a spec, regenerate the catalog with
  `AZURE_COSMOS_SPEC_UPDATE_CATALOG=1 cargo test -p cosspec --test spec`
  and commit `CATALOG.md` and `catalog.json`.

### 3. Keep the framework simple enough to review

- Keep the DSL small enough that a reviewer can hold all of it in their head.
  Respect the [helper budget]: every attribute, injectable method, `faults`
  builder, profile, dataset, and assertion is listed in [spec 0030], and a new
  one must be added there in the same change.
- Give each helper one obvious meaning that matches its name. No hidden
  retries, sleeps, or tolerance that could mask SDK behavior.
- Fail loudly on misuse. Using a capability a spec didn't declare is an
  `InvalidConfiguration` failure, not a silent skip or fallback.
- Keep backend differences inside the backends. A new backend operation goes
  behind a capability, and specs see only the capability.
- Keep `http_tap` self-contained so it can become its own crate.
- Prefer plain, explicit code over clever abstractions, macros, or generics.
  Document every public item.
- Treat the result file format as versioned. Additive changes keep version
  `1`; anything breaking needs a new version and an update to [spec 0030].

<!-- LINKS -->
[spec 0030]: ../docs/specs/0030-cosmos-spec-tests.md
[helper budget]: ../docs/specs/0030-cosmos-spec-tests.md#helper-budget
