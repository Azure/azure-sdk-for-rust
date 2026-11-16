// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { describe, expect, test } from "bun:test";
import { atLeast, groupLogs, interleave, spanPath, spanTree } from "./logs";
import type { HttpJson, LogJson, SpanJson } from "./types";

const req = (sequence: number, start_ms: number) => ({ sequence, start_ms }) as unknown as HttpJson;
const log = (message: string, t_ms: number) =>
    ({ message, t_ms, level: "debug", target: "t", fields: {} }) as LogJson;
const span = (id: number, name: string, start_ms: number, parent?: number) =>
    ({ id, name, start_ms, parent }) as SpanJson;

const order = (rows: ReturnType<typeof interleave>) =>
    rows.map((r) => (r.kind === "http" ? `#${r.http.sequence}` : r.log.message));

describe("interleave", () => {
    test("merges by time; a request goes before a log at the same time", () => {
        const rows = interleave(
            [req(1, 10), req(2, 30)],
            [log("a", 5), log("b", 10), log("c", 20), log("d", 40)],
        );
        expect(order(rows)).toEqual(["a", "#1", "b", "c", "#2", "d"]);
        expect(rows.find((r) => r.kind === "log" && r.log.message === "c")).toMatchObject({
            index: 2,
        });
    });

    test("descending reverses the merge", () => {
        const rows = interleave([req(2, 30), req(1, 10)], [log("a", 5), log("b", 20)], true);
        expect(order(rows)).toEqual(["#2", "b", "#1", "a"]);
    });
});

test("groupLogs folds runs of logs between requests", () => {
    const rows = groupLogs(
        interleave(
            [req(1, 10), req(2, 30)],
            [log("a", 5), log("b", 10), log("c", 20), log("d", 40)],
        ),
    );
    expect(rows.map((r) => (r.kind === "http" ? `#${r.http.sequence}` : r.logs.length))).toEqual([
        1,
        "#1",
        2,
        "#2",
        1,
    ]);
    expect(rows.map((r) => (r.kind === "logs" ? r.key : null))).toEqual([0, null, 1, null, 3]);
    const desc = groupLogs(
        interleave([req(2, 30), req(1, 10)], [log("b", 10), log("c", 20)], true),
    );
    expect(desc.find((r) => r.kind === "logs")).toMatchObject({ key: 0 });
});

test("atLeast compares severity", () => {
    expect(atLeast("warn", "info")).toBe(true);
    expect(atLeast("debug", "info")).toBe(false);
    expect(atLeast("trace", "trace")).toBe(true);
});

describe("spans", () => {
    const spans = [
        span(0, "op", 0),
        span(2, "late", 9, 0),
        span(1, "attempt", 1, 0),
        span(3, "orphan", 2, 7),
    ];

    test("spanTree nests children in start order and keeps orphans as roots", () => {
        const tree = spanTree(spans);
        expect(tree.map((n) => n.span.name)).toEqual(["op", "orphan"]);
        expect(tree[0]!.children.map((n) => n.span.name)).toEqual(["attempt", "late"]);
    });

    test("spanPath lists names from the root", () => {
        expect(spanPath(spans, 1)).toEqual(["op", "attempt"]);
        expect(spanPath(spans, undefined)).toEqual([]);
    });
});
