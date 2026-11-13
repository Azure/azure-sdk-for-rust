// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { describe, expect, test } from "bun:test";
import { region } from "./format";

describe("region", () => {
    test("shows the alias with the real region", () => {
        expect(region({ region: "eastus", region_alias: "PrimaryReadWrite" })).toBe(
            "PrimaryReadWrite (eastus)",
        );
    });

    test("falls back to the real region in runs without aliases", () => {
        expect(region({ region: "eastus" })).toBe("eastus");
        expect(region({ region: null, region_alias: null })).toBeNull();
    });
});
