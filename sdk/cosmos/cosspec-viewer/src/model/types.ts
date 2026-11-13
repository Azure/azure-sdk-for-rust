// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/**
 * Types for the files cosspec writes. They mirror `manifest.json` (manifest
 * version 1) and the per-trial result files (result version 1). Both formats
 * only grow additively within a version, so every field added later is
 * optional here.
 */

export type RunStatus = "running" | "completed";

export type TrialStatus = "pending" | "passed" | "failed" | "skipped" | "ignored" | "not_run";

export interface Totals {
    passed: number;
    failed: number;
    skipped: number;
    ignored: number;
    not_run: number;
    pending: number;
}

export interface BackendEntry {
    name: string;
    kind: string;
    profiles: string[];
}

export interface CheckEntry {
    name: string;
    status: TrialStatus;
    duration_ms: number | null;
    error: string | null;
}

export interface TrialEntry {
    test_name: string;
    spec_id: string;
    summary: string;
    source: string;
    backend: string;
    backend_kind: string;
    service_profile: string;
    requires: string[];
    status: TrialStatus;
    /** The skip reason or known-issue text. */
    reason: string | null;
    duration_ms: number | null;
    error: string | null;
    /** The result file, relative to the run directory. */
    result_file: string | null;
}

export interface Manifest {
    version: number;
    kind: "cosspec-run";
    run_id: string;
    started_at: string;
    finished_at: string | null;
    status: RunStatus;
    args: string[];
    cleanup: boolean;
    result_version: number;
    backends: BackendEntry[];
    totals: Totals;
    checks: CheckEntry[];
    trials: TrialEntry[];
}

export const MANIFEST_FILE = "manifest.json";
export const MANIFEST_KIND = "cosspec-run";
export const MANIFEST_VERSION = 1;
export const RESULT_VERSION = 1;

export type OutcomeJson =
    | { kind: "response"; status: number; sub_status: number | null }
    | { kind: "transport_error" }
    | { kind: "cancelled" }
    | { kind: "unknown" };

/** An HTTP body, cut to the first `BODY_LIMIT` bytes (4 KiB; 1 KiB in older runs). Exactly one of `text` and `base64` is set. */
export interface BodyJson {
    length: number;
    truncated: boolean;
    text?: string;
    base64?: string;
}

export interface HttpJson {
    label: string | null;
    sequence: number;
    method: string;
    url: string;
    operation: string | null;
    region: string | null;
    /** The region's alias, such as `PrimaryReadWrite`. Absent in older runs. */
    region_alias?: string | null;
    activity_id: string | null;
    start_ms: number;
    duration_ms: number | null;
    outcome: OutcomeJson | null;
    request_body?: BodyJson | null;
    response_body?: BodyJson | null;
}

export interface ResourceJson {
    kind: "account" | "database" | "container";
    origin: "created" | "pooled";
    name: string;
    database?: string;
    profile?: string;
    parameter?: string;
}

export interface PanicJson {
    message: string;
    location: string | null;
    backtrace: string;
}

/**
 * A captured log line. Not written by cosspec yet; the viewer shows logs when
 * a result file has them.
 */
export interface LogJson {
    /** Milliseconds from the trial start. */
    t_ms: number;
    level: "error" | "warn" | "info" | "debug" | "trace";
    target: string;
    message: string;
    fields?: Record<string, unknown>;
    span?: string;
    activity_id?: string;
}

/** A named stretch of a trial, such as `pool`, `body`, or `teardown`. */
export interface PhaseJson {
    name: string;
    /** Milliseconds from the trial start. */
    start_ms: number;
    /** Milliseconds from the trial start. */
    end_ms: number;
}

export interface ResultFile {
    version: number;
    id: string;
    test_name: string;
    summary: string;
    source: string;
    service_profile: string;
    backend: string;
    backend_kind?: string;
    cleanup?: boolean;
    requires: string[];
    known_issue: { reason: string; profiles: string[] } | null;
    started_at: string;
    duration_ms: number;
    status: "passed" | "failed";
    error: string | null;
    teardown_error: string | null;
    panic: PanicJson | null;
    resources: ResourceJson[];
    /** Absent in results written before phases were recorded. */
    phases?: PhaseJson[];
    http: HttpJson[];
    logs?: LogJson[];
}

/** One run found in a results location. */
export interface RunSummary {
    /** The run ID, unique within the source. */
    id: string;
    /** The run directory, relative to the source root (`""` for the root itself). */
    path: string;
    manifest: Manifest;
    /** Whether the manifest was rebuilt from result files because none was written. */
    synthesized: boolean;
}

export function isManifest(value: unknown): value is Manifest {
    const v = value as Partial<Manifest> | null;
    return (
        !!v && v.kind === MANIFEST_KIND && typeof v.run_id === "string" && Array.isArray(v.trials)
    );
}

export function isResultFile(value: unknown): value is ResultFile {
    const v = value as Partial<ResultFile> | null;
    return (
        !!v &&
        typeof v.version === "number" &&
        typeof v.test_name === "string" &&
        typeof v.id === "string" &&
        Array.isArray(v.http)
    );
}
