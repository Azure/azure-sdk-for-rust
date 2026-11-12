// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import type { Detected } from "../model/detect";
import type { ResultFile, RunSummary } from "../model/types";

/**
 * Where the viewer reads results from. Screens only use this interface, so new
 * sources (a ZIP file, a remote URL, data inlined in a report) plug in without
 * screen changes.
 */
export interface ResultSource {
    /** Describes the source for the header, for example a path or folder name. */
    readonly label: string;
    /** Finds the runs the source holds. */
    detect(): Promise<Detected>;
    /** Reads one result file of `run`; `file` is the manifest's `result_file`. */
    result(run: RunSummary, file: string): Promise<ResultFile>;
    /** Calls `onChange` when results change on disk. Returns an unsubscribe function. */
    subscribe?(onChange: () => void): () => void;
    /** Whether `detect` and `result` re-read files, so a refresh shows changes. */
    readonly live: boolean;
}
