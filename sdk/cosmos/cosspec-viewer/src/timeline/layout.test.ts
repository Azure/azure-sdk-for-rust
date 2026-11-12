// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { describe, expect, test } from "bun:test";
import { layout, ticks, type Span } from "./layout";

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
