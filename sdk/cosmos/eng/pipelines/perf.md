# Weekly Cosmos performance gate

The optional `CosmosPerf` stage in `sdk/cosmos/ci.yml` builds and executes
`azure_data_cosmos_perf` at the pipeline's exact `Build.SourceVersion`. It
runs all 14 workload types concurrently for 2,700 seconds, excluding the first
300 seconds from the gate. Setup and cleanup are outside that window.

## Onboarding status

The infrastructure and thresholds are **not provisioned or calibrated by this
change**. `EnableWeeklyPerf` defaults to false. `perf-thresholds.json` is
explicitly unapproved and cannot produce a passing gate. Do not enable the
weekly parameter until the following prerequisites are complete.

1. Identify the Azure resource ID for the existing single-region Gateway V2
   account referenced by `thinclient-test-endpoint` / `thinclient-test-key`.
   Do not use the unrelated `rust-ci` fixed-account catalog or create another
   account. Confirm its capacity supports the proposed profile.
2. Provision a dedicated Linux Azure VM in that account's region and register it
   in an approved Azure DevOps agent pool. `PerfPool` defaults to `cosmos-perf`.
   The initial profile proposes `Standard_D8s_v5`, Session consistency, three
   containers at 10,000 RU/s each, 1,000 reference documents, 1,024-byte payload
   fields, and four workers per operation. These are calibration inputs, not
   established production baselines. Review capacity/cost and adjust the profile
   before calibration if necessary.
3. Create/approve a read-only Azure service connection for account metadata in
   the account's owning tenant/subscription. `PerfServiceConnection` defaults to
   `cosmos-perf-reader`. Data-plane keys cannot supply this permission. The VM
   must reach Azure Instance Metadata Service directly, without a proxy.
4. Add `cosmos-perf-account-resource-id` and `cosmos-perf-baseline-commit`
   to the existing protected variable group
   `Test Secrets for Cosmos Live Tests - user administered`. The latter must be
   a full reviewed commit SHA containing the workload profile and threshold
   file, available in the pipeline checkout.
5. Configure an **exclusive-lock check on that same shared variable group**.
   Both the existing Cosmos live-test stage and the new perf stage consume it.
   This check is essential: a dedicated agent alone cannot prevent other runs
   from loading the account. Require every other consumer of this account to
   use the same lock. The perf stage also waits for the current run's shared-account
   live tests to finish and uses sequential lock behavior.
6. Require approvals on the secret group, agent pool, and service connection.
   Untrusted PR code must not automatically run with these credentials or on a
   privileged persistent agent. Review the source SHA, use an isolated/reset
   runner, and restrict network/identity access before approving such a run.
7. Use the manual calibration flow below, review its measurements and thresholds,
   then enable `EnableWeeklyPerf` on the `- weekly` definition. The repository's
   existing external weekly schedule remains responsible for queuing that
   definition; this change does not add another schedule.

Onboarding checks and approvals are external ADO/Azure configuration, not YAML
settings that can be validated locally. Do not claim the gate is operational
until a real run has demonstrated them.

## Calibration and execution

Queue a manual pipeline run with `CalibratePerf=true`. The profile comes from the
pinned baseline revision; calibration omits thresholds and reports
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
to that revision. Missing entries, duplicates, unapproved baselines, and
profile mismatches are errors, not implicit skips.

The weekly run tests its exact source commit. For manual PR validation, queue the
same pipeline at the PR's exact head commit using Azure DevOps source-version
selection and `RunPerfTests=true`. Ensure that commit is available to the pipeline
repository. Synthetic `refs/pull/*` merge checkouts are rejected rather than
being mislabeled as PR-head measurements. The resulting artifacts include the
source SHA and separate reviewed baseline SHA. Ordinary PR builds do not run
this workload automatically.

The selected baseline supplies both the profile and thresholds via `git show`;
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

The preflight compares live account read/write regions with the VM's IMDS
region and checks SKU, OS, endpoint identity, and consistency. A point-read
probe must actually use Gateway V2. During the workload, data-plane request
diagnostics must remain local. Legitimate operation-specific wire behavior
is not forced into Gateway V2 just to satisfy a protocol label.

## Artifacts and cleanup

The `cosmos-perf-<job-attempt>` pipeline artifact includes the reviewed input
files, verified environment, interval measurements, console log, and atomic
`summary.json`. This local summary is authoritative; no results container
competes with the measured account. Calibration and failures remain visible.
Intervals are diagnostic; only the complete aggregate drives thresholds.

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
PowerShell fixtures reject wrong regions, accounts, SKU/OS, and consistency.
They do not replace an approved live Gateway V2 smoke/full run or ADO template
expansion. Validate a deliberately impossible threshold and a preflight mismatch
before enabling the scheduled gate.
