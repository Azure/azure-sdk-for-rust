// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import type { Detected } from "../model/detect";
import type { ResultFile, RunSummary } from "../model/types";
import type { ResultSource } from "./source";

/** The API served by `bin/cosspec-viewer.ts`. */
export const API = {
    source: "api/source",
    file: (run: string, file: string) =>
        `api/runs/${encodeURIComponent(run)}/files/${encodeURIComponent(file)}`,
    events: "api/events",
};

/** Reads results through the local `cosspec-viewer` server. */
export class ServerSource implements ResultSource {
    readonly live = true;

    constructor(readonly label: string) {}

    /** Connects to the server, or returns `null` when the UI is served without one. */
    static async connect(): Promise<ServerSource | null> {
        try {
            const response = await fetch(API.source, { method: "HEAD" });
            // Static hosts may answer any path with index.html, so require the server's header.
            const label = response.headers.get("x-cosspec-results");
            return response.ok && label !== null ? new ServerSource(label) : null;
        } catch {
            return null;
        }
    }

    async detect(): Promise<Detected> {
        return json<Detected>(API.source);
    }

    async result(run: RunSummary, file: string): Promise<ResultFile> {
        return json<ResultFile>(API.file(run.id, file));
    }

    subscribe(onChange: () => void): () => void {
        const events = new EventSource(API.events);
        events.addEventListener("change", onChange);
        return () => events.close();
    }
}

async function json<T>(url: string): Promise<T> {
    const response = await fetch(url, { cache: "no-store" });
    if (!response.ok) throw new Error(`${url}: ${response.status} ${await response.text()}`);
    return (await response.json()) as T;
}
