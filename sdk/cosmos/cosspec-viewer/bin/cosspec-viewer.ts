#!/usr/bin/env bun
// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/**
 * Serves the cosspec results viewer and the results under one directory.
 *
 *     bun bin/cosspec-viewer.ts [--results <path>] [--port <port>] [--open]
 *
 * `--results` is a run directory or a directory of runs; it defaults to the
 * repository's `target/cosspec-results`. The server listens on 127.0.0.1 only
 * and serves only the files the runs' manifests list.
 */

import { stat } from "node:fs/promises";
import { join, resolve, sep } from "node:path";
import { parseArgs } from "node:util";
import { detect, joinPath, type Detected } from "../src/model/detect";
import { fsTree } from "../src/model/fs-tree";
import { MANIFEST_FILE } from "../src/model/types";

const usage = `usage: cosspec-viewer [--results <path>] [--port <port>] [--open]

  --results <path>  a run directory or a directory of runs
                    (default: <repo>/target/cosspec-results)
  --port <port>     port to listen on; 0 picks a free port (default: 4173)
  --open            open the viewer in the default browser`;

const { values } = parseArgs({
    options: {
        results: { type: "string" },
        port: { type: "string", default: "4173" },
        open: { type: "boolean", default: false },
        help: { type: "boolean", short: "h", default: false },
    },
});
if (values.help) {
    console.log(usage);
    process.exit(0);
}

const repoRoot = resolve(import.meta.dir, "../../../..");
const root = resolve(values.results ?? join(repoRoot, "target", "cosspec-results"));
const dist = resolve(import.meta.dir, "../dist");
const port = Number(values.port);
if (!Number.isInteger(port) || port < 0 || port > 65535) {
    console.error(`error: invalid --port ${values.port}\n\n${usage}`);
    process.exit(2);
}
if (!(await stat(root).catch(() => null))?.isDirectory()) {
    console.error(`error: ${root} is not a directory`);
    process.exit(2);
}
if (!(await Bun.file(join(dist, "index.html")).exists())) {
    console.error(
        `error: ${dist} has no build; run \`bun run build\` first or use \`bun run start\``,
    );
    process.exit(2);
}

const tree = fsTree(root);

/** A cheap fingerprint of the results directory, used to notice changes. */
async function fingerprint(): Promise<string> {
    const mtime = async (path: string) => (await stat(join(root, path)).catch(() => null))?.mtimeMs;
    const parts = [`.:${await mtime(MANIFEST_FILE)}:${(await tree.list("")).length}`];
    for (const entry of await tree.list("")) {
        if (!entry.dir) continue;
        const count = (await tree.list(entry.name)).length;
        parts.push(`${entry.name}:${await mtime(joinPath(entry.name, MANIFEST_FILE))}:${count}`);
    }
    return parts.join("|");
}

let detected: Detected = await detect(tree);
let allowed = allowList(detected);
let last = await fingerprint();
const listeners = new Set<ReadableStreamDefaultController<string>>();

function allowList(d: Detected): Map<string, { path: string; files: Set<string> }> {
    return new Map(
        d.runs.map((run) => [
            run.id,
            {
                path: run.path,
                files: new Set(
                    run.manifest.trials.flatMap((t) => (t.result_file ? [t.result_file] : [])),
                ),
            },
        ]),
    );
}

// Polling is simple and works the same on every platform and file system.
setInterval(async () => {
    const next = await fingerprint();
    if (next === last) return;
    last = next;
    detected = await detect(tree);
    allowed = allowList(detected);
    for (const listener of listeners) listener.enqueue("event: change\ndata: {}\n\n");
}, 1000);

function events(): Response {
    let self: ReadableStreamDefaultController<string>;
    let ping: ReturnType<typeof setInterval>;
    const stream = new ReadableStream<string>({
        start(controller) {
            self = controller;
            listeners.add(controller);
            controller.enqueue(": connected\n\n");
            ping = setInterval(() => controller.enqueue(": ping\n\n"), 15000);
        },
        cancel() {
            clearInterval(ping);
            listeners.delete(self);
        },
    });
    return new Response(stream, {
        headers: {
            "content-type": "text/event-stream",
            "cache-control": "no-store",
        },
    });
}

function resultFile(runId: string, file: string): Response {
    const run = allowed.get(runId);
    if (!run || !run.files.has(file)) return new Response("not found", { status: 404 });
    return new Response(Bun.file(join(root, run.path, file)), {
        headers: { "content-type": "application/json", "cache-control": "no-store" },
    });
}

async function staticFile(pathname: string): Promise<Response> {
    const path = resolve(dist, `.${decodeURIComponent(pathname)}`);
    if (path !== dist && !path.startsWith(dist + sep)) {
        return new Response("not found", { status: 404 });
    }
    const file = Bun.file(pathname === "/" ? join(dist, "index.html") : path);
    return (await file.exists()) ? new Response(file) : new Response("not found", { status: 404 });
}

const server = Bun.serve({
    hostname: "127.0.0.1",
    port,
    idleTimeout: 0,
    async fetch(request) {
        const url = new URL(request.url);
        const path = url.pathname;
        if (path === "/api/source") {
            return Response.json(detected, {
                headers: { "x-cosspec-results": root, "cache-control": "no-store" },
            });
        }
        if (path === "/api/events") return events();
        const file = /^\/api\/runs\/([^/]+)\/files\/([^/]+)$/.exec(path);
        if (file) return resultFile(decodeURIComponent(file[1]!), decodeURIComponent(file[2]!));
        if (path.startsWith("/api/")) return new Response("not found", { status: 404 });
        return staticFile(path);
    },
});

const address = `http://127.0.0.1:${server.port}/`;
const runs = detected.runs.length;
console.log(
    `cosspec-viewer: ${address}\n  results: ${root} (${
        detected.mode === "run" ? "one run" : `${runs} run${runs === 1 ? "" : "s"}`
    })`,
);
if (values.open) {
    const command =
        process.platform === "darwin"
            ? ["open", address]
            : process.platform === "win32"
              ? ["cmd", "/c", "start", "", address]
              : ["xdg-open", address];
    Bun.spawn(command, { stdout: "ignore", stderr: "ignore" });
}
