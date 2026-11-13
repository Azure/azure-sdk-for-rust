# Writing and reviewing spec tests

Spec tests are the normative specification of the Azure Cosmos DB Rust SDK's
behavior. Each one states a single behavior in its doc comment and proves it in
a short body that runs unchanged on every backend: the in-memory emulator, the
Azure Cosmos DB Emulator, or live accounts. They live in the
[`cosspec`](../cosspec/README.md) crate, under `tests/spec`.

This guide covers what you need to write a spec and to review one. The
[cosspec README](../cosspec/README.md) covers running specs and configuring
backends, and the [design spec](specs/0030-cosmos-spec-tests.md) has the full
design.

## Anatomy of a spec

```rust ignore
/// A continuation token taken before a partition split still resumes the query
/// after the split, with no item lost or repeated.
#[spec_test]
#[requires(SimpleSplitMerge)]
pub async fn continuation_survives_split(
    #[profile(basic_items)] container: ExclusiveContainer,
) -> SpecResult {
    // Read the first page, keep its continuation token, split, resume,
    // then compare against the dataset.
    assert_feed_ranges!(container, 1);
    // ...
    container.simple_partition_split().await?;
    assert_feed_ranges!(container, 2);
    // ...
    assert_eq!(items, container.dataset().all());
    Ok(())
}
```

- The **doc comment** is the specification. Its first paragraph states the
  behavior as a rule the SDK must follow. `#[spec_test]` rejects specs without
  one.
- The **parameters** are resources the framework provides; the spec never
  creates its own accounts, databases, or pooled containers.
- The **body** uses plain SDK calls and returns `SpecResult`, so `?` works on
  SDK errors.
- The spec's ID is `<module>.<function>`, for example
  `query.continuation_survives_split`. Each run of it on one backend and
  service profile is a trial, named like
  `query.continuation_survives_split[inmemory.MultiRead]`.

## Attributes

- `#[spec_test]` registers the function. It must be `async`, documented, and
  return a `Result` (normally `SpecResult`).
- `#[service_profiles(..)]` picks the account shapes to run on:
  `SingleRegion`, `MultiRead`, `MultiWrite`, or `all` (the default).
- `#[requires(..)]` lists the capabilities the spec needs. Backends without
  them skip the spec.
- `#[known_issue("reason", ..)]` marks a spec the SDK fails today. It's
  reported as ignored, optionally only on the listed profiles.
- `#[profile(..)]`, on a parameter, picks the container profile to create or
  borrow, such as `basic_items`.

## Resources

Specs ask for resources by type. The framework creates them before the body
runs and cleans them up afterwards.

- `SharedContainer`: a pooled, seeded, **read-only** container. It's created
  once per run and reused by every spec with the same profile, and any write
  fails the spec. Needs `#[profile(..)]`.
- `ExclusiveContainer`: a container created and seeded for this spec only, in
  the pooled database. Use it for anything that writes or changes topology.
  Without `#[profile(..)]` it's `empty`.
- `ExclusiveDatabase`: a database for this spec, deleted with everything in it
  afterwards. Create containers with `create_container(profile)`.
- `ExclusiveAccount`: an account for this spec, for changing account-wide
  state. Needs `AccountIsolation`.
- `HttpTap`: a recording of the HTTP traffic from the spec's clients.

Key methods:

- `client()` on every resource returns the SDK client. Its traffic goes to the
  spec's `HttpTap`.
- `dataset()` on containers returns the data the container was seeded with,
  for expected values: `all()`, `take(n)`, and `get("item-07")`.
- `client_with_faults([..])` on containers builds a separate client with
  client-side faults (see [Faults](#faults)).
- `query_all(query)` on containers runs a query across the whole container.
- Topology on `ExclusiveContainer`: `simple_partition_split()`,
  `simple_partition_merge()`, `split()`, and `feed_range_count()`.
- `regions()` on every resource lists the account's region aliases in
  preference order.
- Regions on `ExclusiveAccount`: `take_region_offline(alias)` and
  `bring_region_online(alias)`.

## Profiles and test data

- **Service profiles** describe the account: `SingleRegion` has one region,
  `MultiRead` has a write region and a read region, and `MultiWrite` has two
  writable regions. They don't fix a consistency level; specs that depend on
  one set it on the request.
- **Region aliases** (`RegionAlias`) name a profile's regions, because live
  accounts can be deployed anywhere. Every profile starts with
  `PrimaryReadWrite`; `MultiRead` adds `SecondaryReadOnly` and `MultiWrite`
  adds `SecondaryReadWrite`. Each backend maps aliases to real regions. Specs
  use only aliases. A spec that runs on both multi-region profiles names the
  second region `regions()[1]`. Using an alias the profile lacks fails the
  trial as an invalid configuration.
- **Container profiles** (`cosspec::profiles`) describe a container: its
  partition key path, starting physical partition count, and seed data.
  `empty` and `basic_items` are built in.
- **Datasets** (`cosspec::testdata`) are fixed, ordered sets of `Item`s.
  `basic_items` has 24 items, `item-00` to `item-23`, spread across six
  partition keys, with `value` equal to the item number.

Compare results against datasets rather than literals, so expectations stay
readable and tied to the seed data.

## Capabilities

A capability is something a backend can do. A spec lists the capabilities it
needs in `#[requires(..)]`, which does two things:

- **Filtering:** a backend and service profile without every required
  capability skips the spec with a reason. Set `AZURE_COSMOS_SPEC_STRICT=1` to
  turn skips into failures.
- **Enforcement:** calling a framework method that needs an undeclared
  capability fails the spec with `InvalidConfiguration`, even on a backend that
  has it.

| Capability | Enables |
| --- | --- |
| `MultiRegion` | Specs that need more than one region |
| `MultiWrite` | Specs that need every region to accept writes |
| `SimpleSplitMerge` | `simple_partition_split()`, `simple_partition_merge()` |
| `TopologyControl` | `split()`, splitting every partition |
| `RegionControl` | `take_region_offline(..)` and `bring_region_online(..)` |
| `AccountIsolation` | `ExclusiveAccount` |

## Faults

`cosspec::faults` builds client-side fault rules for `client_with_faults`:

```rust ignore
let slow_primary = faults::delay(Op::ReadItem, Duration::from_secs(2))
    .in_region(RegionAlias::PrimaryReadWrite);
let client = container.client_with_faults([slow_primary]).await?;
```

`delay`, `error`, and `throttle` choose the fault. `in_region(alias)` and
`times(n)` narrow it.

## Evidence and assertions

Prove behavior with what actually happened, not with what the SDK says
happened. `HttpTap` records every HTTP exchange from the spec's clients, with
its operation, region and region alias, timing, and outcome. Framework setup
traffic isn't included. Use `tap.operation("ReadItem")` to get one
operation's exchanges in start order. Each `HttpRecord` has `status()`,
`duration()`, `was_cancelled()`, and `overlaps(&other)`. Use `tap.clear()` to
drop traffic the spec made during its own setup.

Diagnostics can supplement this evidence but never replace it.

- `assert_eq!` and `assert!` check values, such as items against `dataset()`.
- `assert_items_unordered!(actual, expected)`: the same items, in any order.
- `assert_status!(result, 404)` or `assert_status!(result, 404, 1002)`: a call
  failed with this status and sub-status.
- `assert_feed_ranges!(container, n)`: the container has `n` feed ranges, read
  with a fresh client.
- `assert_http_count!(tap, "ReadItem", n)`: how many exchanges an operation
  made.
- `assert_http_regions!(tap, "ReadItem", [RegionAlias::PrimaryReadWrite, ..])`:
  the region aliases an operation was sent to, in order.
- `assert_http_overlap!(a, b)`: two exchanges were in flight at the same time.

## Lifecycle

- Each backend and service profile pair is a cell. A cell's account and
  database are opened once per run and shared by its trials, along with its
  shared containers.
- Exclusive resources are deleted when the spec ends, even if it fails. Pass
  `-- --no-cleanup` to keep everything for debugging.
- Resource names are UUIDv7s, so leftovers in live accounts can be found and
  cleaned up by age.
- Every trial writes a JSON result file with its outcome, resources, and HTTP
  traffic. Browse runs with the
  [`cosspec-viewer`](../cosspec-viewer/README.md).

## Known issues

When the SDK doesn't meet a spec, don't weaken the spec. Mark it with
`#[known_issue("<what the SDK does wrong>", <profiles>)]` so it stays visible
without blocking CI, and remove the attribute in the change that fixes the
SDK. Run known issues with `-- --include-ignored`.

## Reviewing a spec

- Does the doc comment state one behavior, as a rule, in plain language?
- Does the body prove exactly that behavior, no more and no less?
- Are the assertions strong enough? Expected values come from datasets, and
  claims about retries, regions, or concurrency use `HttpTap` evidence.
- Does it use the narrowest resources it needs? Use `SharedContainer` when it
  only reads.
- Do `#[requires(..)]` and `#[service_profiles(..)]` match what the spec does,
  without branching on the backend?
- Does it name regions only by alias, never by real region name?
- If behavior changed on purpose, do the doc comment and body change together?
- If there's a `#[known_issue]`, is its reason accurate and scoped to the
  right profiles?

## Reviewing framework changes

- Is a new helper worth it? It should remove noise that obscures the behavior
  under test, have one obvious meaning, and not hide the SDK call being
  specified.
- Does it fail loudly on misuse, with no hidden retries, sleeps, or
  tolerances that could mask SDK behavior?
- Do backend-specific operations stay behind a capability?
- Are changes to the result or manifest format additive? Breaking changes need
  a new format version.
- Is the design spec updated to list any new attribute, resource method,
  fault, profile, dataset, or assertion?
