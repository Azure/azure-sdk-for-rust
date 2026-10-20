# Weekly Cosmos performance gate

The `CosmosPerf` stage in `sdk/cosmos/ci.yml` runs automatically in the internal
`- weekly` pipeline definition, including manual runs of that definition. It builds and executes
`azure_data_cosmos_perf` at the pipeline's exact `Build.SourceVersion`. It
runs all 14 workload types concurrently for 2,700 seconds, excluding the first
300 seconds from the gate. Setup and cleanup are outside that window.

## Onboarding status

Weekly execution requires no opt-in parameter. The job reuses the existing
`azsdk-pool-mms-ubuntu-2204-perf` pool and fixed-account credentials; it does not
require a new agent pool, Azure service connection, or ARM account resource ID.
Agent placement and account configuration are verified before creating workload
resources. The initial
`perf-thresholds.json` is explicitly unapproved: weekly runs collect calibration
measurements until a reviewed baseline is configured, without reporting a
performance-gate pass. Confirm these prerequisites:

1. Verify the existing Gateway V2 account referenced by
   `thinclient-test-endpoint` / `thinclient-test-key` in
   `Test Secrets for Cosmos Live Tests - user administered`. These are the only
   Azure credentials needed. Do not use the unrelated `rust-ci` fixed-account
   catalog or create another account. Single- and multi-region accounts are
   supported, provided the agent's region is advertised for both reads and writes.
2. Authorize the pipeline to use the existing Linux perf pool.
   `PerfPool` defaults to `azsdk-pool-mms-ubuntu-2204-perf`, using its
   `ubuntu-22.04` image. The agent must be in a readable, writable account region and match the
   profile's VM SKU; preflight fails on a mismatch instead of accepting noisy
   or cross-region measurements. An alternate existing pool can be supplied
   through `PerfPool` only if it satisfies the same checks.
   The initial profile proposes `Standard_D8s_v5`, Session consistency, three
   containers at 10,000 RU/s each, 1,000 reference documents, 1,024-byte payload
   fields, and four workers per operation. These are calibration inputs, not
   established production baselines. Review capacity/cost and adjust the profile
   before calibration if necessary. The agent must reach Azure Instance Metadata
   Service directly, without a proxy.
3. Configure an **exclusive-lock check on that same shared variable group**.
   Both the existing Cosmos live-test stage and the new perf stage consume it.
   This check is essential: a separate agent alone cannot prevent other runs
   from loading the account. Require every other consumer of this account to
   use the same lock. The perf stage also waits for the current run's shared-account
   live tests to finish and uses sequential lock behavior.
4. Require appropriate protected-resource checks on the secret group and agent pool.
   Untrusted PR code must not automatically run with these credentials or on a
   privileged persistent agent. Review the source SHA, use an isolated/reset
   runner, and restrict network/identity access before approving such a run.
5. After calibration, add `cosmos-perf-baseline-commit` to the protected variable
   group via its Key Vault mapping: a full reviewed commit SHA containing the
   workload profile and approved threshold file, available in the checkout.
   Without it, weekly runs collect calibration using the source commit's profile.
   The next weekly run enforces the pinned approved thresholds automatically.
   The existing external weekly schedule queues the definition; no new schedule
   is required.

Onboarding checks and approvals are external ADO/Azure configuration, not YAML
settings that can be validated locally. Do not claim the gate is operational
until a real run has demonstrated them.

## Calibration and execution

Weekly runs automatically collect calibration if no reviewed baseline revision
is configured, or if the pinned revision contains the initial empty, unapproved
threshold template. They enforce thresholds once the pinned baseline is approved.
Malformed or partially configured pinned baselines fail; they do not silently
downgrade to calibration.

For explicit manual calibration, set `CalibratePerf=true`. The profile comes from
the pinned baseline revision when configured, otherwise the source commit.
Calibration omits thresholds and reports
`calibration`, never `passed`. A successful collection is marked
`SucceededWithIssues` in ADO to distinguish it from a passing validation.
Repeat against the same source/profile/environment to measure variability.
No universal numbers or automatically reduced thresholds are supplied.

For the reviewed `perf-thresholds.json`, set `approved` to true, copy the exact
`profile` object from the report, and provide one entry per operation:

```json
{
  "operation": "ReadItem",
  "min_successful_ops_per_second": 100,
  "max_p99_ms": 25,
  "max_error_rate": 0.001
}
```

These numbers illustrate the schema only; **they are not recommended thresholds**.
Repeat the entry with measured, reviewed values for every operation. Commit
the configuration through normal review and pin `cosmos-perf-baseline-commit`
to that revision. In validation mode, missing entries, duplicates, unapproved
baselines, and profile mismatches are errors, not implicit skips. Candidate threshold edits
cannot enable gating without a separately configured reviewed baseline revision.

The weekly run tests its exact source commit. For manual PR validation, queue the
same pipeline at the PR's exact head commit using Azure DevOps source-version
selection and `RunPerfTests=true`. Ensure that commit is available to the pipeline
repository. Exact `refs/pull/<number>/head` checkouts are accepted; synthetic
`refs/pull/<number>/merge` checkouts are rejected rather than
being mislabeled as PR-head measurements. The resulting artifacts include the
source SHA and separate reviewed baseline SHA. Ordinary PR builds do not run
this workload automatically. Weekly perf waits for Build and the shared-account
live tests to finish, but still runs when those stages fail, so unrelated test
or packaging failures do not suppress measurements. Cancelled runs do not
start perf. Those earlier failures still fail the overall pipeline.

When configured, the selected baseline supplies both the profile and thresholds via `git show`;
candidate threshold-file edits do not change the gate. Candidate code itself
is still executed, so this is a review/approval boundary, not a sandbox against
malicious source changes.

## Workload and metrics

All worker groups share one long-lived SDK client and a synchronized start.
Warm-up does not restart connections or caches. A seeded `reference` container
is read-only during measurement; `mutations` holds worker-private upsert,
replace, and patch items; `lifecycle` holds create, delete, and batch items.
No TTL is applied to reference data. All three containers are created in a new,
uniquely named run-owned database with dedicated manual throughput.

Create deletes its own item after timing; delete creates its own target before
timing. Supporting requests consume shared CPU/RUs and affect achieved throughput
but are not labeled as the timed operation's latency. Ambiguous lifecycle errors
stop the run rather than reusing potentially committed state or accumulating
abandoned items. The batch atomically executes create, upsert, replace, read,
and delete against one item/partition, and checks all five response statuses.

The query corpus is fixed. One query/change-feed invocation consumes at most
the configured number of pages and must return seeded items. Aggregate queries
are single-partition because cross-partition aggregates are unsupported by this
SDK; DISTINCT uses `TOP 100` to satisfy its bounded-window requirement. One
logical query or batch counts as one operation, not one document, page, or
internal request.

| Metric | Definition |
| --- | --- |
| Successful ops/sec | Successes started at or after second 300 and completed before second 2700, divided by 2400, including stalls. |
| p99 | The merged histogram of those successful logical invocation latencies, not an average of interval percentiles. |
| Error rate | Failures plus unfinished measured requests divided by measured successes plus failures/unfinished requests. |
| Deadline in-flight | A measured request still incomplete at the common cutoff is unsuccessful, even if the server may later commit it. |
| Cancellation | Aborted measured requests count as errors exactly once and have a separate cancellation counter; aborted warm-up requests stay excluded. |
| Warm-up | Requests started before second 300 are excluded even if they complete later. |

Comparisons are inclusive (`>=` throughput floor, `<=` latency/error ceilings).
Every operation requires successful samples. Threshold violations, early
termination, worker failure, nonlocal/unknown data-plane routing, and missing
required artifacts fail the run. Timeout and cleanup time cannot inflate the
measurement denominator. Background metadata requests are not classified as
item throughput or data-plane locality evidence.

The preflight signs `GET /` against the configured Cosmos HTTPS endpoint using
the existing account key. It requires the VM's IMDS region to be present in both
`readableLocations` and `writableLocations` and checks SKU, OS, the authenticated
account name, and `userConsistencyPolicy`. It does not query Azure Resource
Manager, create an Azure account, or require an ARM resource-ID setting.
Redirects, non-success responses, and missing metadata are errors. A point-read
probe must actually use Gateway V2. During the workload, data-plane request
diagnostics must remain local. Legitimate operation-specific wire behavior
is not forced into Gateway V2 just to satisfy a protocol label.

Preflight records the selected local region and every other advertised region.
The shared client excludes those other regions through its default operation
options, covering point operations, queries, change feed, and batch. A single-write
account therefore requires the agent in its write region; multi-write accounts
can use any region that is both readable and writable. Runtime diagnostics still
reject nonlocal traffic, including traffic caused by a topology change during a run.

## Artifacts and cleanup

The `cosmos-perf-<job-attempt>` pipeline artifact includes the reviewed input
files, verified environment, interval measurements, console log, and atomic
`summary.json`. This local summary is authoritative; no results container
competes with the measured account. Calibration and failures remain visible.
Intervals are diagnostic; only the complete aggregate drives thresholds.

Each invocation creates a fresh `cosmos-perf-<UUID>` database with its own
`reference`, `mutations`, and `lifecycle` containers inside the existing account.
No databases or containers from earlier runs are reused.

The crate persists database ownership before issuing creation, including for lost
creation responses, and deletes its database on ordinary completion/failure. The pipeline's
`always()` cleanup uses the account key and an ownership manifest matching the
current build ID; it never deletes the account or arbitrary existing databases.
An abrupt agent loss can prevent both cleanup paths. Before removing a stale
`cosmos-perf-<UUID>` database, verify its run manifest/build ID and confirm that
the corresponding pipeline is no longer active. Do not sweep by prefix alone.

## Local checks

```bash
cargo test -p azure_data_cosmos_perf --all-features
pwsh sdk/cosmos/eng/pipelines/prepare-cosmos-perf.tests.ps1
```

The crate tests execute all workloads concurrently through the SDK's in-memory
emulator and cover warm-up/cutoff accounting and threshold semantics. The
PowerShell fixtures verify request signing and metadata retrieval, reject
wrong regions/accounts/SKU/OS/consistency, and check that cleanup can delete
only this run's database, never the account or unrelated databases.
They do not replace an approved live Gateway V2 smoke/full run or ADO template
expansion. Validate a deliberately impossible threshold and a preflight mismatch
before enabling the scheduled gate.
