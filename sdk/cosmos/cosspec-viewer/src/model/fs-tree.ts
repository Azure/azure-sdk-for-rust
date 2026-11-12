// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { readdir, readFile } from "node:fs/promises";
import { join } from "node:path";
import type { FileTree } from "./detect";

/** A `FileTree` over a directory on disk. For the server and tests, not the browser. */
export function fsTree(root: string): FileTree {
    return {
        async read(path) {
            return readFile(join(root, path), "utf8").catch(() => null);
        },
        async list(path) {
            const entries = await readdir(join(root, path), { withFileTypes: true }).catch(
                () => [],
            );
            return entries.map((e) => ({ name: e.name, dir: e.isDirectory() }));
        },
    };
}
