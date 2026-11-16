// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { detect, joinPath, type Detected, type FileTree } from "../model/detect";
import { isResultFile, type ResultFile, type RunLogsFile, type RunSummary } from "../model/types";
import type { ResultSource } from "./source";

/** Reads results from a directory the user picked or dropped. Files never leave the browser. */
export class DirectorySource implements ResultSource {
    constructor(
        readonly label: string,
        private readonly tree: FileTree,
        readonly live: boolean,
    ) {}

    /** From `<input webkitdirectory>` or dropped files, each carrying its relative path. */
    static fromFiles(files: { path: string; file: File }[]): DirectorySource {
        const tree = new FileMapTree(files);
        return new DirectorySource(tree.rootName || "dropped files", tree, false);
    }

    /** From the File System Access API; re-reading the handle sees new files. */
    static fromHandle(handle: FileSystemDirectoryHandle): DirectorySource {
        return new DirectorySource(handle.name, new HandleTree(handle), true);
    }

    detect(): Promise<Detected> {
        return detect(this.tree);
    }

    async result(run: RunSummary, file: string): Promise<ResultFile> {
        const text = await this.tree.read(joinPath(run.path, file));
        if (text === null) throw new Error(`${joinPath(run.path, file)} not found`);
        const result: unknown = JSON.parse(text);
        if (!isResultFile(result)) throw new Error(`${file} is not a cosspec result file`);
        return result;
    }

    async runLogs(run: RunSummary, file: string): Promise<RunLogsFile> {
        const text = await this.tree.read(joinPath(run.path, file));
        if (text === null) throw new Error(`${joinPath(run.path, file)} not found`);
        return JSON.parse(text) as RunLogsFile;
    }
}

/**
 * A tree over files with relative paths. When every path starts with the same
 * folder (the folder the user picked), that folder becomes the root.
 */
export class FileMapTree implements FileTree {
    private readonly files = new Map<string, File | Blob>();
    readonly rootName: string;

    constructor(files: { path: string; file: File | Blob }[]) {
        const paths = files.map((f) => f.path.replace(/^\/+/, ""));
        const first = paths.map((p) => p.split("/")[0]);
        const shared =
            paths.length > 0 &&
            paths.every((p) => p.includes("/")) &&
            first.every((f) => f === first[0]);
        this.rootName = shared ? (first[0] ?? "") : "";
        files.forEach((f, i) => {
            const path = paths[i] ?? "";
            this.files.set(shared ? path.slice(this.rootName.length + 1) : path, f.file);
        });
    }

    async read(path: string): Promise<string | null> {
        return (await this.files.get(path)?.text()) ?? null;
    }

    async list(path: string): Promise<{ name: string; dir: boolean }[]> {
        const prefix = path ? `${path}/` : "";
        const entries = new Map<string, boolean>();
        for (const key of this.files.keys()) {
            if (!key.startsWith(prefix)) continue;
            const [name, ...rest] = key.slice(prefix.length).split("/");
            if (name) entries.set(name, entries.get(name) || rest.length > 0);
        }
        return [...entries].map(([name, dir]) => ({ name, dir }));
    }
}

class HandleTree implements FileTree {
    constructor(private readonly root: FileSystemDirectoryHandle) {}

    private async dir(path: string): Promise<FileSystemDirectoryHandle | null> {
        let dir = this.root;
        for (const part of path.split("/").filter(Boolean)) {
            try {
                dir = await dir.getDirectoryHandle(part);
            } catch {
                return null;
            }
        }
        return dir;
    }

    async read(path: string): Promise<string | null> {
        const parts = path.split("/");
        const name = parts.pop() ?? "";
        const dir = await this.dir(parts.join("/"));
        try {
            return (await (await dir?.getFileHandle(name))?.getFile())?.text() ?? null;
        } catch {
            return null;
        }
    }

    async list(path: string): Promise<{ name: string; dir: boolean }[]> {
        const dir = await this.dir(path);
        const entries: { name: string; dir: boolean }[] = [];
        if (!dir) return entries;
        for await (const [name, handle] of dir.entries()) {
            entries.push({ name, dir: handle.kind === "directory" });
        }
        return entries;
    }
}

/** Collects every file under dropped items, with paths relative to the drop. */
export async function droppedFiles(
    items: DataTransferItemList,
): Promise<{ path: string; file: File }[]> {
    const files: { path: string; file: File }[] = [];
    const walk = async (entry: FileSystemEntry, prefix: string): Promise<void> => {
        if (entry.isFile) {
            const file = await new Promise<File>((resolve, reject) =>
                (entry as FileSystemFileEntry).file(resolve, reject),
            );
            files.push({ path: `${prefix}${entry.name}`, file });
        } else if (entry.isDirectory) {
            const reader = (entry as FileSystemDirectoryEntry).createReader();
            // `readEntries` returns entries in batches until it returns none.
            for (;;) {
                const batch = await new Promise<FileSystemEntry[]>((resolve, reject) =>
                    reader.readEntries(resolve, reject),
                );
                if (batch.length === 0) break;
                for (const child of batch) await walk(child, `${prefix}${entry.name}/`);
            }
        }
    };
    const entries = [...items]
        .map((item) => item.webkitGetAsEntry())
        .filter((e): e is FileSystemEntry => e !== null);
    for (const entry of entries) await walk(entry, "");
    return files;
}

/** Whether the browser supports `showDirectoryPicker` (Chromium-based browsers). */
export function canPickDirectory(): boolean {
    return typeof (window as { showDirectoryPicker?: unknown }).showDirectoryPicker === "function";
}

export async function pickDirectory(): Promise<FileSystemDirectoryHandle> {
    const picker = (
        window as unknown as {
            showDirectoryPicker: (options?: {
                mode?: "read";
            }) => Promise<FileSystemDirectoryHandle>;
        }
    ).showDirectoryPicker;
    return picker({ mode: "read" });
}

const DB = "cosspec-viewer";
const STORE = "handles";
const KEY = "last";

function openDb(): Promise<IDBDatabase> {
    return new Promise((resolve, reject) => {
        const request = indexedDB.open(DB, 1);
        request.onupgradeneeded = () => request.result.createObjectStore(STORE);
        request.onsuccess = () => resolve(request.result);
        request.onerror = () => reject(request.error);
    });
}

/** Remembers the last picked directory so it can be reopened with one click. */
export async function rememberHandle(handle: FileSystemDirectoryHandle): Promise<void> {
    const db = await openDb();
    await new Promise<void>((resolve, reject) => {
        const tx = db.transaction(STORE, "readwrite");
        tx.objectStore(STORE).put(handle, KEY);
        tx.oncomplete = () => resolve();
        tx.onerror = () => reject(tx.error);
    });
}

export async function rememberedHandle(): Promise<FileSystemDirectoryHandle | null> {
    try {
        const db = await openDb();
        return await new Promise((resolve) => {
            const request = db.transaction(STORE).objectStore(STORE).get(KEY);
            request.onsuccess = () =>
                resolve((request.result as FileSystemDirectoryHandle) ?? null);
            request.onerror = () => resolve(null);
        });
    } catch {
        return null;
    }
}

/** Asks again for read access to a remembered handle; needs a user gesture. */
export async function requestAccess(handle: FileSystemDirectoryHandle): Promise<boolean> {
    const h = handle as FileSystemDirectoryHandle & {
        requestPermission?: (d: { mode: "read" }) => Promise<PermissionState>;
    };
    return (await h.requestPermission?.({ mode: "read" })) !== "denied";
}
