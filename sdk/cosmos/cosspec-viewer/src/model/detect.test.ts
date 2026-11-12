// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { describe, expect, test } from "bun:test";
import { join } from "node:path";
import { FileMapTree } from "../sources/directory";
import { detect, totals } from "./detect";
import { fsTree } from "./fs-tree";

const root = join(import.meta.dir, "../../fixtures/results");
const items = "01a1d1d0-1d68-7053-b49f-137f00c471c0";
const resilience = "01a1d1d0-1fc7-75b3-a2c5-3322f3e92583";

describe("detect", () => {
    test("a directory of runs lists every run, newest first", async () => {
        const { mode, runs } = await detect(fsTree(root));
        expect(mode).toBe("runs");
        expect(runs).toHaveLength(5);
        const real = runs.filter((r) => !r.synthesized).map((r) => r.id);
        expect(real).toEqual([...real].sort().reverse());
    });

    test("a run directory is a single run", async () => {
        const { mode, runs } = await detect(fsTree(join(root, resilience)));
        expect(mode).toBe("run");
        expect(runs[0]).toMatchObject({ id: resilience, path: "", synthesized: false });
        expect(runs[0]!.manifest.totals.failed).toBe(2);
    });

    test("a run without a manifest gets one from its result files", async () => {
        const { runs } = await detect(fsTree(root));
        const legacy = runs.find((r) => r.id === "legacy-run")!;
        expect(legacy.synthesized).toBe(true);
        expect(legacy.path).toBe("legacy-run");
        const original = runs.find((r) => r.id === items)!.manifest;
        expect(legacy.manifest.trials.map((t) => t.test_name).sort()).toEqual(
            original.trials.map((t) => t.test_name).sort(),
        );
        expect(legacy.manifest.totals).toEqual(totals(original.trials));
        expect(legacy.manifest.trials.every((t) => t.result_file)).toBe(true);
    });

    test("a directory with no runs finds none", async () => {
        const { mode, runs } = await detect(fsTree(join(root, "..", "..", "src")));
        expect(mode).toBe("runs");
        expect(runs).toEqual([]);
    });
});

describe("FileMapTree", () => {
    const file = (text: string) => new Blob([text]);

    test("strips the picked folder", async () => {
        const tree = new FileMapTree([
            { path: "cosspec-results/a/manifest.json", file: file("{}") },
            { path: "cosspec-results/a/x.json", file: file("x") },
            { path: "cosspec-results/b/y.json", file: file("y") },
        ]);
        expect(tree.rootName).toBe("cosspec-results");
        expect(await tree.list("")).toEqual([
            { name: "a", dir: true },
            { name: "b", dir: true },
        ]);
        expect(await tree.list("a")).toEqual([
            { name: "manifest.json", dir: false },
            { name: "x.json", dir: false },
        ]);
        expect(await tree.read("b/y.json")).toBe("y");
        expect(await tree.read("missing.json")).toBeNull();
    });

    test("keeps several dropped folders side by side", async () => {
        const tree = new FileMapTree([
            { path: "a/x.json", file: file("x") },
            { path: "b/y.json", file: file("y") },
        ]);
        expect(tree.rootName).toBe("");
        expect((await tree.list("")).map((e) => e.name)).toEqual(["a", "b"]);
    });
});
