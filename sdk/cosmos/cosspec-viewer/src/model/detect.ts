// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import {
    isManifest,
    isResultFile,
    MANIFEST_FILE,
    MANIFEST_KIND,
    MANIFEST_VERSION,
    RESULT_VERSION,
    type Manifest,
    type ResultFile,
    type RunSummary,
    type Totals,
    type TrialEntry,
} from "./types";

/** A read-only directory tree. Paths are `/`-separated and relative to its root. */
export interface FileTree {
    /** Reads a file as text, or returns `null` if it doesn't exist. */
    read(path: string): Promise<string | null>;
    /** Lists a directory's immediate entries; empty if it doesn't exist. */
    list(path: string): Promise<{ name: string; dir: boolean }[]>;
}

/** What a results location holds. */
export interface Detected {
    /** `run` when the location is one run directory, `runs` when it holds several. */
    mode: "run" | "runs";
    runs: RunSummary[];
}

export function joinPath(dir: string, name: string): string {
    return dir ? `${dir}/${name}` : name;
}

function baseName(path: string): string {
    return path.split("/").filter(Boolean).pop() ?? "";
}

function parse(text: string | null): unknown {
    if (text === null) return null;
    try {
        return JSON.parse(text);
    } catch {
        return null;
    }
}

/**
 * Finds the runs in `tree`.
 *
 * - If the root holds a `manifest.json`, it is a single run.
 * - Otherwise each immediate subdirectory holding a manifest is a run,
 *   newest first.
 * - A directory without a manifest that holds result files is still a run;
 *   its manifest is rebuilt from them and flagged `synthesized`.
 */
export async function detect(tree: FileTree): Promise<Detected> {
    const root = await loadRun(tree, "");
    if (root) return { mode: "run", runs: [root] };
    const entries = await tree.list("");
    const runs = (
        await Promise.all(entries.filter((e) => e.dir).map((e) => loadRun(tree, e.name)))
    ).filter((run): run is RunSummary => run !== null);
    return { mode: "runs", runs: sortRuns(runs) };
}

/** Sorts runs newest first by start time, then by ID (UUIDv7s sort by time). */
export function sortRuns(runs: RunSummary[]): RunSummary[] {
    return [...runs].sort(
        (a, b) =>
            b.manifest.started_at.localeCompare(a.manifest.started_at) || b.id.localeCompare(a.id),
    );
}

/** Loads the run in directory `path`, or returns `null` if it isn't one. */
export async function loadRun(tree: FileTree, path: string): Promise<RunSummary | null> {
    const manifest = parse(await tree.read(joinPath(path, MANIFEST_FILE)));
    if (isManifest(manifest)) {
        return { id: manifest.run_id, path, manifest, synthesized: false };
    }
    const files = (await tree.list(path)).filter(
        (e) => !e.dir && e.name.endsWith(".json") && e.name !== MANIFEST_FILE,
    );
    const results: [string, ResultFile][] = [];
    for (const file of files) {
        const result = parse(await tree.read(joinPath(path, file.name)));
        if (isResultFile(result)) results.push([file.name, result]);
    }
    if (results.length === 0) return null;
    const id = baseName(path) || "run";
    return { id, path, manifest: synthesize(id, results), synthesized: true };
}

/** Builds a manifest from result files, for runs written before manifests existed. */
export function synthesize(runId: string, results: [string, ResultFile][]): Manifest {
    const trials: TrialEntry[] = results
        .map(([file, r]) => ({
            test_name: r.test_name,
            spec_id: r.id,
            summary: r.summary,
            source: r.source,
            backend: r.backend,
            backend_kind: r.backend_kind ?? r.backend,
            service_profile: r.service_profile,
            requires: r.requires,
            status: r.status,
            reason: r.known_issue?.reason ?? null,
            duration_ms: r.duration_ms,
            error: r.error ?? r.teardown_error ?? r.panic?.message ?? null,
            result_file: file,
        }))
        .sort((a, b) => a.test_name.localeCompare(b.test_name));
    const backends = new Map<string, { kind: string; profiles: Set<string> }>();
    for (const t of trials) {
        const b = backends.get(t.backend) ?? { kind: t.backend_kind, profiles: new Set() };
        b.profiles.add(t.service_profile);
        backends.set(t.backend, b);
    }
    const starts = results.map(([, r]) => Date.parse(r.started_at)).filter(Number.isFinite);
    const ends = results
        .map(([, r]) => Date.parse(r.started_at) + r.duration_ms)
        .filter(Number.isFinite);
    const iso = (ms: number[], pick: (...v: number[]) => number) =>
        ms.length ? new Date(pick(...ms)).toISOString() : "";
    return {
        version: MANIFEST_VERSION,
        kind: MANIFEST_KIND,
        run_id: runId,
        started_at: iso(starts, Math.min),
        finished_at: iso(ends, Math.max) || null,
        status: "completed",
        args: [],
        cleanup: results[0]?.[1].cleanup ?? true,
        result_version: RESULT_VERSION,
        backends: [...backends].map(([name, b]) => ({
            name,
            kind: b.kind,
            profiles: [...b.profiles],
        })),
        totals: totals(trials),
        checks: [],
        trials,
    };
}

export function totals(trials: TrialEntry[]): Totals {
    const t: Totals = { passed: 0, failed: 0, skipped: 0, ignored: 0, not_run: 0, pending: 0 };
    for (const trial of trials) t[trial.status] += 1;
    return t;
}

/**
 * The run's state. A manifest still `running` hours after its run started
 * most likely belongs to a run that crashed.
 */
export function runState(manifest: Manifest): "completed" | "running" | "incomplete" {
    if (manifest.status === "completed") return "completed";
    const age = Date.now() - Date.parse(manifest.started_at);
    return age > 6 * 60 * 60 * 1000 ? "incomplete" : "running";
}
