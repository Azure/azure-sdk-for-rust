// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { useEffect, useRef, useState, type DragEvent } from "react";
import {
    canPickDirectory,
    DirectorySource,
    droppedFiles,
    pickDirectory,
    rememberedHandle,
    rememberHandle,
    requestAccess,
} from "../../sources/directory";
import type { ResultSource } from "../../sources/source";

/**
 * Shown when the UI runs without the local server: pick, drop, or reopen a
 * results folder. Files are read in the browser and never uploaded.
 */
export function OpenScreen(props: { onOpen: (source: ResultSource) => void }) {
    const [over, setOver] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [remembered, setRemembered] = useState<FileSystemDirectoryHandle | null>(null);
    const input = useRef<HTMLInputElement>(null);

    useEffect(() => {
        input.current?.setAttribute("webkitdirectory", "");
        void rememberedHandle().then(setRemembered, () => setRemembered(null));
    }, []);

    const openHandle = async (handle: FileSystemDirectoryHandle) => {
        if (!(await requestAccess(handle))) {
            setError(`Access to ${handle.name} was denied.`);
            return;
        }
        await rememberHandle(handle).catch(() => undefined);
        props.onOpen(DirectorySource.fromHandle(handle));
    };

    const pick = async () => {
        try {
            await openHandle(await pickDirectory());
        } catch (e) {
            if ((e as DOMException).name !== "AbortError") setError(String(e));
        }
    };

    const onFiles = (list: FileList | null) => {
        const files = Array.from(list ?? [], (file) => ({
            path: file.webkitRelativePath || file.name,
            file,
        }));
        if (files.length) props.onOpen(DirectorySource.fromFiles(files));
    };

    const onDrop = async (e: DragEvent) => {
        e.preventDefault();
        setOver(false);
        try {
            const files = await droppedFiles(e.dataTransfer.items);
            if (files.length) props.onOpen(DirectorySource.fromFiles(files));
            else setError("Drop a folder of cosspec results.");
        } catch (err) {
            setError(String(err));
        }
    };

    return (
        <div className="open">
            <h1>cosspec viewer</h1>
            <p className="muted">
                Open a cosspec run directory, or a directory of runs such as{" "}
                <code>target/cosspec-results</code>. Files stay in your browser.
            </p>
            <div
                className={`dropzone ${over ? "over" : ""}`}
                onDragOver={(e) => {
                    e.preventDefault();
                    setOver(true);
                }}
                onDragLeave={() => setOver(false)}
                onDrop={(e) => void onDrop(e)}
            >
                <p>Drop a results folder here</p>
                <div className="actions">
                    {canPickDirectory() ? (
                        <button className="primary" onClick={() => void pick()}>
                            Choose folder…
                        </button>
                    ) : null}
                    <button onClick={() => input.current?.click()}>
                        {canPickDirectory() ? "Upload folder (snapshot)…" : "Choose folder…"}
                    </button>
                    {remembered ? (
                        <button onClick={() => void openHandle(remembered)}>
                            Reopen {remembered.name}
                        </button>
                    ) : null}
                </div>
                <input
                    ref={input}
                    type="file"
                    multiple
                    hidden
                    onChange={(e) => onFiles(e.currentTarget.files)}
                />
            </div>
            {error ? <div className="error-panel">{error}</div> : null}
            <p className="muted small">
                Or serve results live: <code>bun run start -- --results &lt;dir&gt;</code>
            </p>
        </div>
    );
}
