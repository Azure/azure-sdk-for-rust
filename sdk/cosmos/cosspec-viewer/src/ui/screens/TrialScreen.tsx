// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { useEffect, useMemo, useState } from "react";
import type { HttpJson, LogJson, ResultFile, RunSummary } from "../../model/types";
import type { ResultSource } from "../../sources/source";
import { BodyView } from "../components/BodyView";
import { StatusBadge } from "../components/Status";
import { host, Timeline } from "../components/Timeline";
import { concurrency, type Concurrency } from "../../timeline/layout";
import {
    ms,
    outcomeClass,
    outcomeSymbol,
    outcomeText,
    region,
    rel,
    urlPath,
    when,
} from "../format";
import { href, setQuery, type Route } from "../router";

type Tab = "requests" | "resources" | "logs" | "raw";

/** One trial: what it checks, how it ended, and every HTTP request it made. */
export function TrialScreen(props: {
    source: ResultSource;
    run: RunSummary;
    testName: string;
    route: Route;
    version: number;
}) {
    const { source, run, testName, route } = props;
    const entry = run.manifest.trials.find((t) => t.test_name === testName);
    const [result, setResult] = useState<ResultFile | null>(null);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        setError(null);
        if (!entry?.result_file) {
            setResult(null);
            return;
        }
        let live = true;
        source.result(run, entry.result_file).then(
            (r) => live && setResult(r),
            (e: unknown) => live && setError(String(e)),
        );
        return () => {
            live = false;
        };
    }, [source, run.id, entry?.result_file, props.version]);

    if (!entry)
        return (
            <div className="center muted">
                {testName} isn't in run {run.id}.
            </div>
        );

    const tab = (route.query.get("tab") as Tab | null) ?? "requests";
    const selected = route.query.get("req");
    const known = result?.known_issue;

    return (
        <section>
            <div className="trial-header">
                <h2>
                    {entry.spec_id} <StatusBadge status={entry.status} />
                </h2>
                <p className="summary">{entry.summary}</p>
                <dl className="facts">
                    <dt>Backend</dt>
                    <dd>
                        {entry.backend} <span className="muted">({entry.backend_kind})</span>
                    </dd>
                    <dt>Profile</dt>
                    <dd>{entry.service_profile}</dd>
                    <dt>Requires</dt>
                    <dd>
                        {entry.requires.length
                            ? entry.requires.map((r) => (
                                  <span key={r} className="cap">
                                      {r}
                                  </span>
                              ))
                            : "–"}
                    </dd>
                    <dt>Duration</dt>
                    <dd>{ms(entry.duration_ms)}</dd>
                    {result?.phases?.length ? (
                        <>
                            <dt>Phases</dt>
                            <dd>
                                {result.phases
                                    .map((p) => `${p.name} ${ms(p.end_ms - p.start_ms)}`)
                                    .join(" · ")}
                            </dd>
                        </>
                    ) : null}
                    <dt>Started</dt>
                    <dd>{when(result?.started_at)}</dd>
                    <dt>Source</dt>
                    <dd>
                        <code>{entry.source}</code>
                    </dd>
                    <dt>Test</dt>
                    <dd>
                        <code>{entry.test_name}</code>
                    </dd>
                </dl>
            </div>

            {known || entry.status === "ignored" ? (
                <div className="banner info">
                    <strong>Known issue</strong>
                    {known?.profiles.length ? (
                        <span className="muted"> on {known.profiles.join(", ")}</span>
                    ) : null}
                    : {known?.reason ?? entry.reason}
                </div>
            ) : entry.status === "skipped" && entry.reason ? (
                <div className="banner">
                    <strong>Skipped:</strong> {entry.reason}
                </div>
            ) : null}

            {entry.error || result?.panic || result?.teardown_error ? (
                <div className="error-panel">
                    {entry.error ? <pre>{entry.error}</pre> : null}
                    {result?.panic ? (
                        <>
                            {result.panic.location ? (
                                <div className="muted small">
                                    at <code>{result.panic.location}</code>
                                </div>
                            ) : null}
                            {result.panic.message !== entry.error ? (
                                <pre>{result.panic.message}</pre>
                            ) : null}
                            {result.panic.backtrace ? (
                                <details>
                                    <summary>Backtrace</summary>
                                    <pre className="code">{result.panic.backtrace}</pre>
                                </details>
                            ) : null}
                        </>
                    ) : null}
                    {result?.teardown_error ? (
                        <div>
                            <strong>Teardown failed:</strong>
                            <pre>{result.teardown_error}</pre>
                        </div>
                    ) : null}
                </div>
            ) : null}

            {!entry.result_file ? (
                <div className="center muted">
                    This trial has no result file
                    {entry.status === "passed" || entry.status === "failed"
                        ? ""
                        : ` (${entry.status})`}
                    .
                </div>
            ) : error ? (
                <div className="error-panel">{error}</div>
            ) : !result ? (
                <div className="center muted">Loading…</div>
            ) : (
                <TrialBody
                    result={result}
                    route={route}
                    tab={tab}
                    selected={selected}
                    runId={run.id}
                />
            )}
        </section>
    );
}

function TrialBody(props: {
    result: ResultFile;
    route: Route;
    tab: Tab;
    selected: string | null;
    runId: string;
}) {
    const { result, route, tab } = props;
    const [showHarness, setShowHarness] = useState(route.query.get("harness") === "1");
    const logs = result.logs ?? [];
    const http = useMemo(
        () => result.http.filter((r) => showHarness || r.label !== "harness"),
        [result.http, showHarness],
    );
    const phases = result.phases ?? [];
    // Times count from the start of the spec body, not from when the trial
    // began waiting for pooled resources.
    const origin = phases.find((p) => p.name === "body")?.start_ms ?? 0;
    const overlap = useMemo(
        () =>
            concurrency(
                http.map((r) => ({
                    key: r.sequence,
                    start: r.start_ms,
                    end: r.duration_ms === null ? null : r.start_ms + r.duration_ms,
                })),
                Math.max(result.duration_ms, ...http.map((r) => r.start_ms)),
            ),
        [http, result.duration_ms],
    );
    const selected = props.selected === null ? null : Number(props.selected);
    const request = result.http.find((r) => r.sequence === selected) ?? null;
    const select = (seq: number | null) =>
        setQuery(route, { req: seq === null ? null : String(seq) });
    const tabs: [Tab, string][] = [
        ["requests", `Requests (${result.http.length})`],
        ["resources", `Resources (${result.resources.length})`],
        ...(logs.length ? ([["logs", `Logs (${logs.length})`]] as [Tab, string][]) : []),
        ["raw", "Raw JSON"],
    ];

    return (
        <>
            <div className="tabs">
                {tabs.map(([id, text]) => (
                    <a key={id} className={tab === id ? "on" : ""} href={tabHref(route, id)}>
                        {text}
                    </a>
                ))}
            </div>
            {tab === "requests" ? (
                <>
                    <div className="toolbar">
                        <label>
                            <input
                                type="checkbox"
                                checked={showHarness}
                                onChange={(e) => setShowHarness(e.currentTarget.checked)}
                            />{" "}
                            Show harness requests
                        </label>
                        <span className="muted small">
                            Harness requests set up and tear down resources; spec requests are the
                            test's own.
                        </span>
                    </div>
                    {http.length ? (
                        <Timeline
                            http={http}
                            duration={result.duration_ms}
                            origin={origin}
                            phases={phases}
                            concurrency={overlap}
                            logs={logs}
                            selected={selected}
                            onSelect={(s) => select(s === selected ? null : s)}
                        />
                    ) : (
                        <div className="center muted">No requests.</div>
                    )}
                    <div className={`split ${request ? "with-drawer" : ""}`}>
                        <RequestTable
                            http={http}
                            origin={origin}
                            concurrency={overlap}
                            selected={selected}
                            onSelect={select}
                        />
                        {request ? (
                            <RequestDrawer
                                request={request}
                                origin={origin}
                                overlaps={overlap.partners.get(request.sequence) ?? []}
                                onSelect={select}
                                onClose={() => select(null)}
                            />
                        ) : null}
                    </div>
                </>
            ) : tab === "resources" ? (
                <ResourcesTable result={result} />
            ) : tab === "logs" ? (
                <LogsTable logs={logs} />
            ) : (
                <pre className="code raw">{JSON.stringify(result, null, 2)}</pre>
            )}
        </>
    );
}

function tabHref(route: Route, tab: Tab): string {
    const query: Record<string, string> = Object.fromEntries(route.query);
    if (tab === "requests") delete query.tab;
    else query.tab = tab;
    return href(route.path, query);
}

type SortKey = "sequence" | "start" | "duration" | "status";

function RequestTable(props: {
    http: HttpJson[];
    origin: number;
    concurrency: Concurrency;
    selected: number | null;
    onSelect: (s: number) => void;
}) {
    const [sort, setSort] = useState<[SortKey, 1 | -1]>(["start", 1]);
    const rows = useMemo(() => {
        const key = (r: HttpJson): number => {
            switch (sort[0]) {
                case "sequence":
                    return r.sequence;
                case "start":
                    return r.start_ms;
                case "duration":
                    return r.duration_ms ?? Infinity;
                case "status":
                    return r.outcome?.kind === "response" ? r.outcome.status : 1000;
            }
        };
        return [...props.http].sort(
            (a, b) => (key(a) - key(b)) * sort[1] || a.sequence - b.sequence,
        );
    }, [props.http, sort]);

    const th = (key: SortKey, text: string) => (
        <th
            className="sortable"
            onClick={() => setSort([key, sort[0] === key ? (-sort[1] as 1 | -1) : 1])}
        >
            {text}
            {sort[0] === key ? (sort[1] === 1 ? " ▲" : " ▼") : ""}
        </th>
    );

    return (
        <table className="grid compact requests">
            <thead>
                <tr>
                    {th("sequence", "#")}
                    <th>Client</th>
                    {th("start", "Start")}
                    {th("duration", "Duration")}
                    <th>Operation</th>
                    <th>Region</th>
                    {th("status", "Status")}
                    <th title="Requests in flight at the same time">Overlaps</th>
                    <th>Path</th>
                </tr>
            </thead>
            <tbody>
                {rows.map((r) => {
                    const others = props.concurrency.partners.get(r.sequence) ?? [];
                    const partner =
                        props.selected !== null &&
                        (props.concurrency.partners.get(props.selected) ?? []).includes(r.sequence);
                    return (
                        <tr
                            key={r.sequence}
                            className={`link-row ${props.selected === r.sequence ? "selected" : ""} ${partner ? "partner" : ""}`}
                            onClick={() => props.onSelect(r.sequence)}
                        >
                            <td className="num">{r.sequence}</td>
                            <td className={`label-${r.label ?? "client"}`}>{r.label ?? "–"}</td>
                            <td className="num">{rel(r.start_ms, props.origin)}</td>
                            <td className="num">{ms(r.duration_ms)}</td>
                            <td>{r.operation ?? r.method}</td>
                            <td>{region(r) ?? <span className="muted">{host(r.url)}</span>}</td>
                            <td>
                                <Outcome request={r} />
                            </td>
                            <td>
                                {others.length ? (
                                    <span
                                        className="overlaps"
                                        title={`In flight with ${others.map((s) => `#${s}`).join(", ")}`}
                                    >
                                        ∥ {others.map((s) => `#${s}`).join(" ")}
                                    </span>
                                ) : null}
                            </td>
                            <td className="path" title={r.url}>
                                {urlPath(r.url)}
                            </td>
                        </tr>
                    );
                })}
            </tbody>
        </table>
    );
}

function Outcome(props: { request: HttpJson }) {
    const o = props.request.outcome;
    return (
        <span className={`outcome o-${outcomeClass(o)}`}>
            <span aria-hidden="true">{outcomeSymbol(o)} </span>
            {outcomeText(o)}
        </span>
    );
}

function RequestDrawer(props: {
    request: HttpJson;
    origin: number;
    overlaps: number[];
    onSelect: (s: number) => void;
    onClose: () => void;
}) {
    const r = props.request;
    return (
        <aside className="drawer">
            <header>
                <h3>
                    #{r.sequence} {r.operation ?? r.method} <Outcome request={r} />
                </h3>
                <button onClick={props.onClose} title="Close">
                    ✕
                </button>
            </header>
            <dl className="facts">
                <dt>Request</dt>
                <dd>
                    <code>
                        {r.method} {r.url}
                    </code>
                </dd>
                <dt>Client</dt>
                <dd>{r.label ?? "–"}</dd>
                <dt>Region</dt>
                <dd>{region(r) ?? "–"}</dd>
                <dt>Start</dt>
                <dd>
                    {rel(r.start_ms, props.origin)}
                    {props.origin ? (
                        <span className="muted"> ({ms(r.start_ms)} from the trial start)</span>
                    ) : null}
                </dd>
                <dt>Duration</dt>
                <dd>{r.duration_ms === null ? "never finished" : ms(r.duration_ms)}</dd>
                <dt>Overlaps</dt>
                <dd>
                    {props.overlaps.length
                        ? props.overlaps.map((s) => (
                              <button key={s} className="small" onClick={() => props.onSelect(s)}>
                                  #{s}
                              </button>
                          ))
                        : "–"}
                </dd>
                <dt>Activity ID</dt>
                <dd>{r.activity_id ? <code>{r.activity_id}</code> : "–"}</dd>
            </dl>
            <BodyView title="Request body" body={r.request_body} />
            <BodyView title="Response body" body={r.response_body} />
        </aside>
    );
}

function ResourcesTable(props: { result: ResultFile }) {
    if (!props.result.resources.length) return <div className="center muted">No resources.</div>;
    return (
        <table className="grid compact">
            <thead>
                <tr>
                    <th>Kind</th>
                    <th>Origin</th>
                    <th>Name</th>
                    <th>Database</th>
                    <th>Profile</th>
                    <th>Parameter</th>
                </tr>
            </thead>
            <tbody>
                {props.result.resources.map((r, i) => (
                    <tr key={i}>
                        <td>{r.kind}</td>
                        <td>
                            <span className={`pill ${r.origin}`}>{r.origin}</span>
                        </td>
                        <td>
                            <code>{r.name}</code>
                        </td>
                        <td>{r.database ? <code>{r.database}</code> : "–"}</td>
                        <td>{r.profile ?? "–"}</td>
                        <td>{r.parameter ?? "–"}</td>
                    </tr>
                ))}
            </tbody>
        </table>
    );
}

const LEVELS: LogJson["level"][] = ["error", "warn", "info", "debug", "trace"];

function LogsTable(props: { logs: LogJson[] }) {
    const [level, setLevel] = useState<LogJson["level"]>("trace");
    const [text, setText] = useState("");
    const max = LEVELS.indexOf(level);
    const q = text.toLowerCase();
    const rows = props.logs.filter(
        (l) =>
            LEVELS.indexOf(l.level) <= max &&
            (!q || l.message.toLowerCase().includes(q) || l.target.toLowerCase().includes(q)),
    );
    return (
        <>
            <div className="toolbar">
                <select
                    value={level}
                    onChange={(e) => setLevel(e.currentTarget.value as LogJson["level"])}
                >
                    {LEVELS.map((l) => (
                        <option key={l} value={l}>
                            {l} and above
                        </option>
                    ))}
                </select>
                <input
                    type="search"
                    placeholder="Filter logs"
                    value={text}
                    onChange={(e) => setText(e.currentTarget.value)}
                />
            </div>
            <table className="grid compact logs">
                <tbody>
                    {rows.map((l, i) => (
                        <tr key={i}>
                            <td className="num">{ms(l.t_ms)}</td>
                            <td className={`lv-${l.level}`}>{l.level}</td>
                            <td className="muted">{l.target}</td>
                            <td>
                                {l.message}
                                {l.fields && Object.keys(l.fields).length ? (
                                    <code className="muted"> {JSON.stringify(l.fields)}</code>
                                ) : null}
                            </td>
                        </tr>
                    ))}
                </tbody>
            </table>
        </>
    );
}
