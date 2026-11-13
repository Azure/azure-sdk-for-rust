// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { describe, expect, test } from "bun:test";
import { concurrency, layout, ticks, type Span } from "./layout";

const span = (
    key: number,
    start: number,
    end: number | null,
    label = "spec",
    region = "East US",
): Span => ({
    key,
    label,
    region,
    start,
    end,
});

describe("layout", () => {
    test("sequential requests share a row", () => {
        const { bars, rows } = layout([span(0, 0, 10), span(1, 10, 20)]);
        expect(rows).toBe(1);
        expect(bars.map((b) => b.row)).toEqual([0, 0]);
    });

    test("overlapping requests stack, and freed rows are reused", () => {
        const { bars, rows } = layout([span(0, 0, 10), span(1, 5, 15), span(2, 12, 20)]);
        expect(rows).toBe(2);
        expect(Object.fromEntries(bars.map((b) => [b.key, b.row]))).toEqual({ 0: 0, 1: 1, 2: 0 });
    });

    test("lanes split by label and region, spec before harness", () => {
        const { lanes } = layout([
            span(0, 0, 5, "harness"),
            span(1, 1, 5, "spec", "West US"),
            span(2, 2, 5, "spec", "East US"),
        ]);
        expect(lanes.map((l) => `${l.label}/${l.region}@${l.firstRow}`)).toEqual([
            "spec/West US@0",
            "spec/East US@1",
            "harness/East US@2",
        ]);
    });

    test("unfinished requests run to the horizon", () => {
        const { bars, horizon } = layout([span(0, 0, 10), span(1, 2, null)], 30);
        expect(horizon).toBe(30);
        expect(bars[1]).toMatchObject({ end: 30, open: true, row: 1 });
    });
});

describe("ticks", () => {
    test("picks round steps", () => {
        expect(ticks(0, 100, 5)).toEqual([0, 20, 40, 60, 80, 100]);
        expect(ticks(3, 9, 3)).toEqual([4, 6, 8]);
    });
});

describe("concurrency", () => {
    test("finds requests in flight together", () => {
        const { overlaps, partners } = concurrency(
            [
                { key: 1, start: 0, end: 10 },
                { key: 2, start: 4, end: 6 },
                { key: 3, start: 5, end: 12 },
                { key: 4, start: 12, end: 15 },
            ],
            20,
        );
        expect(overlaps).toEqual([{ start: 4, end: 10, peak: 3 }]);
        expect(partners.get(1)).toEqual([2, 3]);
        expect(partners.get(3)).toEqual([1, 2]);
        expect(partners.get(4)).toEqual([]);
    });

    test("requests that only touch don't overlap", () => {
        const { overlaps } = concurrency(
            [
                { key: 1, start: 0, end: 5 },
                { key: 2, start: 5, end: 9 },
            ],
            10,
        );
        expect(overlaps).toEqual([]);
    });

    test("unfinished requests stay in flight until the horizon", () => {
        const { overlaps, partners } = concurrency(
            [
                { key: 1, start: 0, end: null },
                { key: 2, start: 8, end: 9 },
            ],
            10,
        );
        expect(overlaps).toEqual([{ start: 8, end: 9, peak: 2 }]);
        expect(partners.get(2)).toEqual([1]);
    });
});
