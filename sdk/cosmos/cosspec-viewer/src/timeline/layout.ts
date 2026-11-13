// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/** One request to place on the timeline. Times are milliseconds from the trial start. */
export interface Span {
    key: number;
    /** The client label, for example `spec` or `harness`. */
    label: string;
    /** The region the request went to. */
    region: string;
    start: number;
    /** `null` while the request never finished. */
    end: number | null;
}

/** A span with its row. */
export interface Bar {
    key: number;
    /** The row, counted across all lanes from the top. */
    row: number;
    start: number;
    end: number;
    /** Whether the request never finished, so `end` is the horizon. */
    open: boolean;
}

/** A group of rows for one label and region. */
export interface Lane {
    label: string;
    region: string;
    firstRow: number;
    rows: number;
}

export interface Layout {
    lanes: Lane[];
    bars: Bar[];
    rows: number;
    /** The latest time on the timeline. */
    horizon: number;
}

const LABEL_ORDER = ["spec", "harness"];

function labelRank(label: string): number {
    const i = LABEL_ORDER.indexOf(label);
    return i === -1 ? LABEL_ORDER.length : i;
}

/**
 * Lays requests out in lanes: one lane per label and region, with labels in
 * the order `spec`, `harness`, then the rest. Within a lane, requests that
 * overlap in time go on separate rows, so concurrency (such as hedging) is
 * visible. Rows are reused greedily.
 *
 * @param minHorizon The least horizon, for example the trial's duration.
 */
export function layout(spans: Span[], minHorizon = 0): Layout {
    const horizon = spans.reduce((h, s) => Math.max(h, s.end ?? s.start, s.start), minHorizon);
    const byLane = new Map<string, Span[]>();
    for (const span of spans) {
        const key = `${span.label}\u0000${span.region}`;
        const lane = byLane.get(key);
        if (lane) lane.push(span);
        else byLane.set(key, [span]);
    }
    const ordered = [...byLane.values()]
        .map((lane) => lane.sort((a, b) => a.start - b.start || a.key - b.key))
        .sort(
            (a, b) =>
                labelRank(a[0]!.label) - labelRank(b[0]!.label) ||
                a[0]!.label.localeCompare(b[0]!.label) ||
                a[0]!.start - b[0]!.start ||
                a[0]!.region.localeCompare(b[0]!.region),
        );

    const lanes: Lane[] = [];
    const bars: Bar[] = [];
    let next = 0;
    for (const spansInLane of ordered) {
        const ends: number[] = [];
        for (const span of spansInLane) {
            const end = span.end ?? horizon;
            let row = ends.findIndex((rowEnd) => rowEnd <= span.start);
            if (row === -1) row = ends.push(end) - 1;
            else ends[row] = end;
            bars.push({
                key: span.key,
                row: next + row,
                start: span.start,
                end,
                open: span.end === null,
            });
        }
        const first = spansInLane[0]!;
        lanes.push({ label: first.label, region: first.region, firstRow: next, rows: ends.length });
        next += ends.length;
    }
    return { lanes, bars, rows: next, horizon };
}

/** Evenly spaced, round tick values between `from` and `to`. */
export function ticks(from: number, to: number, target = 8): number[] {
    const span = to - from;
    if (!(span > 0)) return [from];
    const raw = span / target;
    const magnitude = 10 ** Math.floor(Math.log10(raw));
    const step = [1, 2, 5, 10].map((m) => m * magnitude).find((s) => s >= raw) ?? raw;
    const result: number[] = [];
    for (let t = Math.ceil(from / step) * step; t <= to + 1e-9; t += step) {
        result.push(Number(t.toFixed(6)));
    }
    return result;
}

/** A stretch of time when two or more requests were in flight together. */
export interface Overlap {
    start: number;
    end: number;
    /** The most requests in flight at once during the stretch. */
    peak: number;
}

export interface Concurrency {
    /** Stretches with two or more requests in flight, in time order. */
    overlaps: Overlap[];
    /** For each request, the requests in flight at the same time as it. */
    partners: Map<number, number[]>;
}

/**
 * Finds requests that were in flight at the same time, such as a hedged read
 * and the request it races. Requests that only touch (one ends as the next
 * starts) don't overlap. `end` is `null` for a request that never finished;
 * it counts as in flight until `horizon`.
 */
export function concurrency(
    spans: { key: number; start: number; end: number | null }[],
    horizon: number,
): Concurrency {
    const closed = spans.map((s) => ({ ...s, end: Math.max(s.end ?? horizon, s.start) }));
    const partners = new Map<number, number[]>(closed.map((s) => [s.key, []]));
    const byStart = [...closed].sort((a, b) => a.start - b.start || a.key - b.key);
    for (let i = 0; i < byStart.length; i++) {
        const a = byStart[i]!;
        for (let j = i + 1; j < byStart.length && byStart[j]!.start < a.end; j++) {
            const b = byStart[j]!;
            if (b.end > a.start && b.start < a.end) {
                partners.get(a.key)!.push(b.key);
                partners.get(b.key)!.push(a.key);
            }
        }
    }
    for (const list of partners.values()) list.sort((a, b) => a - b);

    // Sweep the start and end events; ends sort before starts at the same time.
    const events = closed
        .flatMap((s) => [
            { t: s.start, d: 1 },
            { t: s.end, d: -1 },
        ])
        .sort((a, b) => a.t - b.t || a.d - b.d);
    const overlaps: Overlap[] = [];
    let live = 0;
    let open: Overlap | null = null;
    for (const { t, d } of events) {
        live += d;
        if (live >= 2) {
            if (open) open.peak = Math.max(open.peak, live);
            else open = { start: t, end: t, peak: live };
        } else if (open) {
            open.end = t;
            if (open.end > open.start) overlaps.push(open);
            open = null;
        }
    }
    return { overlaps, partners };
}
