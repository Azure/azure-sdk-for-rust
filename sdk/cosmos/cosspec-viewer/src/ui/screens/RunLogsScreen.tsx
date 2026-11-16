// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { useEffect, useState } from "react";
import type { RunLogsFile, RunSummary } from "../../model/types";
import type { ResultSource } from "../../sources/source";
import { LogsTable } from "../components/Logs";

/**
 * Logs no trial owned: pooled setup, end-of-run cleanup, and events a trial
 * emitted after it finished (tagged with that trial).
 */
export function RunLogsScreen(props: { source: ResultSource; run: RunSummary; version: number }) {
    const { source, run } = props;
    const file = run.manifest.run_logs;
    const [logs, setLogs] = useState<RunLogsFile | null>(null);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        setError(null);
        if (!file) return;
        let live = true;
        source.runLogs(run, file).then(
            (l) => live && setLogs(l),
            (e: unknown) => live && setError(String(e)),
        );
        return () => {
            live = false;
        };
    }, [source, run.id, file, props.version]);

    if (!file) return <div className="center muted">Run {run.id} didn't capture logs.</div>;
    if (error) return <div className="error-panel">{error}</div>;
    if (!logs) return <div className="center muted">Loading…</div>;
    return (
        <section>
            <h2>Run logs</h2>
            <p className="muted">
                Events outside every trial: pooled setup, cleanup at the end of the run, and events
                a trial emitted after it finished. Times count from the run start.
                {run.manifest.log_filter ? (
                    <>
                        {" "}
                        Filter: <code>{run.manifest.log_filter}</code>
                    </>
                ) : null}
            </p>
            <LogsTable logs={logs.logs} showTrial />
        </section>
    );
}
