// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import type { HttpJson, Level, LogJson, SpanJson } from "./types";

/** Levels from most to least severe. */
export const LEVELS: Level[] = ["error", "warn", "info", "debug", "trace"];

/** Whether `level` is `min` or more severe. */
export function atLeast(level: Level, min: Level): boolean {
    return LEVELS.indexOf(level) <= LEVELS.indexOf(min);
}

export function isLevel(value: string | null): value is Level {
    return LEVELS.includes(value as Level);
}

/** A row of the requests table: a request, or a log line between requests. */
export type EventRow =
    | { kind: "http"; t: number; http: HttpJson }
    | { kind: "log"; t: number; log: LogJson; index: number };

/**
 * Merges requests (already in display order by start time) with logs by
 * time. On a tie the request comes first, because a request's own logs are
 * written after it starts. Logs keep their capture order. `descending` lists
 * the same merge newest first; `http` is then in descending order too.
 */
export function interleave(http: HttpJson[], logs: LogJson[], descending = false): EventRow[] {
    if (descending) return interleave([...http].reverse(), logs).reverse();
    const out: EventRow[] = [];
    let j = 0;
    for (const r of http) {
        for (; j < logs.length && logs[j]!.t_ms < r.start_ms; j++) {
            out.push({ kind: "log", t: logs[j]!.t_ms, log: logs[j]!, index: j });
        }
        out.push({ kind: "http", t: r.start_ms, http: r });
    }
    for (; j < logs.length; j++)
        out.push({ kind: "log", t: logs[j]!.t_ms, log: logs[j]!, index: j });
    return out;
}

/** A span with its children, in start order. */
export interface SpanNode {
    span: SpanJson;
    children: SpanNode[];
}

/** Builds the span forest. Spans whose parent is missing become roots. */
export function spanTree(spans: SpanJson[]): SpanNode[] {
    const nodes = new Map(spans.map((s) => [s.id, { span: s, children: [] as SpanNode[] }]));
    const roots: SpanNode[] = [];
    for (const node of nodes.values()) {
        const parent = node.span.parent === undefined ? undefined : nodes.get(node.span.parent);
        (parent && parent !== node ? parent.children : roots).push(node);
    }
    const sort = (list: SpanNode[]) => {
        list.sort((x, y) => x.span.start_ms - y.span.start_ms || x.span.id - y.span.id);
        list.forEach((n) => sort(n.children));
    };
    sort(roots);
    return roots;
}

/** The names from the outermost span down to span `id`, such as `["op", "attempt"]`. */
export function spanPath(spans: SpanJson[], id: number | undefined): string[] {
    const byId = new Map(spans.map((s) => [s.id, s]));
    const path: string[] = [];
    const seen = new Set<number>();
    for (let s = id === undefined ? undefined : byId.get(id); s && !seen.has(s.id);) {
        seen.add(s.id);
        path.unshift(s.name);
        s = s.parent === undefined ? undefined : byId.get(s.parent);
    }
    return path;
}

/** A run of consecutive log rows between requests, shown collapsed. */
export interface LogGroup {
    kind: "logs";
    /** Stable across re-sorts: the capture index of the group's first log. */
    key: number;
    logs: Extract<EventRow, { kind: "log" }>[];
}

/** Folds each run of consecutive log rows into one group. */
export function groupLogs(rows: EventRow[]): (Extract<EventRow, { kind: "http" }> | LogGroup)[] {
    const out: (Extract<EventRow, { kind: "http" }> | LogGroup)[] = [];
    for (const row of rows) {
        if (row.kind === "http") {
            out.push(row);
            continue;
        }
        const last = out[out.length - 1];
        if (last?.kind === "logs") last.logs.push(row);
        else out.push({ kind: "logs", key: row.index, logs: [row] });
    }
    for (const g of out) if (g.kind === "logs") g.key = Math.min(...g.logs.map((l) => l.index));
    return out;
}
