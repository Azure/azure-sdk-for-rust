// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { useCallback, useEffect, useState } from "react";
import type { Detected } from "../model/detect";
import { ServerSource } from "../sources/server";
import type { ResultSource } from "../sources/source";
import { href, useRoute } from "./router";
import { OpenScreen } from "./screens/OpenScreen";
import { RunScreen } from "./screens/RunScreen";
import { RunsScreen } from "./screens/RunsScreen";
import { TrialScreen } from "./screens/TrialScreen";

type Boot = { state: "connecting" } | { state: "open" } | { state: "ready"; source: ResultSource };

export function App() {
    const [boot, setBoot] = useState<Boot>({ state: "connecting" });
    const [detected, setDetected] = useState<Detected | null>(null);
    const [error, setError] = useState<string | null>(null);
    const [version, setVersion] = useState(0);
    const route = useRoute();

    useEffect(() => {
        void ServerSource.connect().then((source) =>
            setBoot(source ? { state: "ready", source } : { state: "open" }),
        );
    }, []);

    const source = boot.state === "ready" ? boot.source : null;

    const refresh = useCallback(() => {
        if (!source) return;
        source.detect().then(
            (d) => {
                setDetected(d);
                setError(null);
                setVersion((v) => v + 1);
            },
            (e: unknown) => setError(String(e)),
        );
    }, [source]);

    useEffect(() => {
        refresh();
        return source?.subscribe?.(refresh);
    }, [source, refresh]);

    if (boot.state === "connecting") return <div className="center muted">Connecting…</div>;
    if (boot.state === "open" || !source) {
        return <OpenScreen onOpen={(s) => setBoot({ state: "ready", source: s })} />;
    }

    const single = detected?.mode === "run";
    const [screen, runId, , testName] = route.path;
    const run =
        detected?.runs.find((r) => r.id === runId) ?? (single ? detected?.runs[0] : undefined);

    let body;
    if (error) body = <div className="error-panel">{error}</div>;
    else if (!detected) body = <div className="center muted">Loading…</div>;
    else if (detected.runs.length === 0) {
        body = (
            <div className="center muted">
                No cosspec runs found in <code>{source.label}</code>.
            </div>
        );
    } else if (screen === "run" && run && testName) {
        body = (
            <TrialScreen
                source={source}
                run={run}
                testName={testName}
                route={route}
                version={version}
            />
        );
    } else if ((screen === "run" && run) || single) {
        body = <RunScreen run={run ?? detected.runs[0]!} route={route} />;
    } else if (screen === "run") {
        body = (
            <div className="center muted">
                Run {runId} isn't in {source.label}.
            </div>
        );
    } else {
        body = <RunsScreen runs={detected.runs} />;
    }

    return (
        <div className="app">
            <header className="top-bar">
                <a className="brand" href={single && run ? href(["run", run.id]) : href(["runs"])}>
                    cosspec viewer
                </a>
                <Breadcrumbs
                    single={single}
                    runId={run?.id}
                    testName={screen === "run" ? testName : undefined}
                />
                <span className="spacer" />
                <span className="muted source" title={source.label}>
                    {source.label}
                </span>
                {source.live ? (
                    source.subscribe ? (
                        <span className="pill live" title="Updates as results change on disk">
                            live
                        </span>
                    ) : (
                        <button onClick={refresh}>Refresh</button>
                    )
                ) : null}
                {source instanceof ServerSource ? null : (
                    <button onClick={() => setBoot({ state: "open" })}>Open…</button>
                )}
            </header>
            <main>{body}</main>
        </div>
    );
}

function Breadcrumbs(props: { single: boolean; runId?: string; testName?: string }) {
    const crumbs: [string, string][] = [];
    if (!props.single) crumbs.push(["Runs", href(["runs"])]);
    if (props.runId) crumbs.push([shortId(props.runId), href(["run", props.runId])]);
    if (props.runId && props.testName) {
        crumbs.push([props.testName, href(["run", props.runId, "trial", props.testName])]);
    }
    return (
        <nav className="crumbs">
            {crumbs.map(([text, link], i) => (
                <span key={link}>
                    {i > 0 ? <span className="sep">›</span> : null}
                    <a href={link} title={text}>
                        {text}
                    </a>
                </span>
            ))}
        </nav>
    );
}

export function shortId(id: string): string {
    return /^[0-9a-f]{8}-/.test(id) ? id.slice(0, 13) : id;
}
