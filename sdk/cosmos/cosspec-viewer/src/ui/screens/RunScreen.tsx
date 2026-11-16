// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { useMemo } from "react";
import { runState } from "../../model/detect";
import type { RunSummary, TrialEntry, TrialStatus } from "../../model/types";
import {
    StatusBadge,
    STATUS_ORDER,
    STATUS_SYMBOL,
    statusText,
    TotalsBar,
    TotalsText,
} from "../components/Status";
import { between, ms, when } from "../format";
import { href, setQuery, type Route } from "../router";

/**
 * One run: its header, then trials as a backend › profile › spec tree (like
 * the console) or as a spec × (backend, profile) matrix. Filters live in the
 * URL so a filtered view can be shared.
 */
export function RunScreen(props: { run: RunSummary; route: Route }) {
    const { run, route } = props;
    const m = run.manifest;
    const view = route.query.get("view") === "matrix" ? "matrix" : "tree";
    const text = route.query.get("q") ?? "";
    const statuses = new Set((route.query.get("status") ?? "").split(",").filter(Boolean));
    const backend = route.query.get("backend") ?? "";
    const profile = route.query.get("profile") ?? "";

    const trials = useMemo(() => {
        const q = text.toLowerCase();
        return m.trials.filter(
            (t) =>
                (!statuses.size || statuses.has(t.status)) &&
                (!backend || t.backend === backend) &&
                (!profile || t.service_profile === profile) &&
                (!q ||
                    t.test_name.toLowerCase().includes(q) ||
                    t.summary.toLowerCase().includes(q) ||
                    t.requires.some((r) => r.toLowerCase().includes(q))),
        );
    }, [m.trials, text, statuses.size, [...statuses].join(), backend, profile]);

    const profiles = [...new Set(m.trials.map((t) => t.service_profile))].sort();
    const toggleStatus = (s: TrialStatus) => {
        const next = new Set(statuses);
        if (next.has(s)) next.delete(s);
        else next.add(s);
        setQuery(route, { status: [...next].join(",") });
    };

    return (
        <section>
            <div className="run-header">
                <div>
                    <h2>
                        Run <code title={run.id}>{run.id}</code>
                        {runState(m) !== "completed" ? (
                            <span className={`pill ${runState(m)}`}>{runState(m)}</span>
                        ) : null}
                        {run.synthesized ? <span className="pill">no manifest</span> : null}
                    </h2>
                    <dl className="facts">
                        <dt>Started</dt>
                        <dd>{when(m.started_at)}</dd>
                        <dt>Duration</dt>
                        <dd>{ms(between(m.started_at, m.finished_at))}</dd>
                        <dt>Backends</dt>
                        <dd>
                            {m.backends.map((b) => (
                                <span key={b.name} className="pill" title={b.profiles.join(", ")}>
                                    {b.name} <span className="muted">({b.kind})</span>
                                </span>
                            ))}
                        </dd>
                        <dt>Cleanup</dt>
                        <dd>{m.cleanup ? "yes" : "no (--no-cleanup)"}</dd>
                        <dt>Logs</dt>
                        <dd>
                            {m.log_filter ? (
                                <>
                                    <code>{m.log_filter}</code>
                                    {m.run_logs ? (
                                        <>
                                            {" "}
                                            <a href={href(["run", run.id, "logs"])}>Run logs</a>
                                        </>
                                    ) : null}
                                </>
                            ) : (
                                "not captured"
                            )}
                        </dd>
                        <dt>Arguments</dt>
                        <dd>
                            <code>{m.args.join(" ") || "–"}</code>
                        </dd>
                    </dl>
                </div>
                <div className="run-totals">
                    <TotalsBar totals={m.totals} />
                    <TotalsText totals={m.totals} />
                </div>
            </div>

            {m.checks.length ? (
                <div className="checks">
                    {m.checks.map((c) => (
                        <div key={c.name} className="check">
                            <StatusBadge status={c.status} /> <code>{c.name}</code>{" "}
                            {c.error ? <pre>{c.error}</pre> : null}
                        </div>
                    ))}
                </div>
            ) : null}

            <div className="toolbar">
                <div className="segmented">
                    <a
                        className={view === "tree" ? "on" : ""}
                        href={withQuery(route, { view: null })}
                    >
                        Tree
                    </a>
                    <a
                        className={view === "matrix" ? "on" : ""}
                        href={withQuery(route, { view: "matrix" })}
                    >
                        Matrix
                    </a>
                </div>
                <input
                    type="search"
                    placeholder="Filter specs or capabilities"
                    value={text}
                    onChange={(e) => setQuery(route, { q: e.currentTarget.value })}
                />
                <select
                    value={backend}
                    onChange={(e) => setQuery(route, { backend: e.currentTarget.value })}
                >
                    <option value="">All backends</option>
                    {m.backends.map((b) => (
                        <option key={b.name}>{b.name}</option>
                    ))}
                </select>
                <select
                    value={profile}
                    onChange={(e) => setQuery(route, { profile: e.currentTarget.value })}
                >
                    <option value="">All profiles</option>
                    {profiles.map((p) => (
                        <option key={p}>{p}</option>
                    ))}
                </select>
                <span className="status-filters">
                    {STATUS_ORDER.filter((s) => m.totals[s]).map((s) => (
                        <label key={s} className={`chip ${statuses.has(s) ? "on" : ""} fg-${s}`}>
                            <input
                                type="checkbox"
                                checked={statuses.has(s)}
                                onChange={() => toggleStatus(s)}
                            />
                            {STATUS_SYMBOL[s]} {statusText(s)} {m.totals[s]}
                        </label>
                    ))}
                </span>
            </div>

            {trials.length === 0 ? (
                <div className="center muted">No trials match.</div>
            ) : view === "matrix" ? (
                <Matrix run={run} trials={trials} />
            ) : (
                <Tree run={run} trials={trials} />
            )}
        </section>
    );
}

function withQuery(route: Route, updates: Record<string, string | null>): string {
    const query: Record<string, string> = Object.fromEntries(route.query);
    for (const [k, v] of Object.entries(updates)) {
        if (v) query[k] = v;
        else delete query[k];
    }
    return href(route.path, query);
}

function trialHref(run: RunSummary, t: TrialEntry): string {
    return href(["run", run.id, "trial", t.test_name]);
}

function group<K, T>(items: T[], key: (t: T) => K): Map<K, T[]> {
    const map = new Map<K, T[]>();
    for (const item of items) {
        const k = key(item);
        map.set(k, [...(map.get(k) ?? []), item]);
    }
    return map;
}

function Tree(props: { run: RunSummary; trials: TrialEntry[] }) {
    return (
        <div className="tree">
            {[...group(props.trials, (t) => t.backend)].map(([backend, byBackend]) => (
                <details key={backend} open>
                    <summary>
                        <strong>{backend}</strong>{" "}
                        <span className="muted">{byBackend[0]?.backend_kind}</span>
                    </summary>
                    {[...group(byBackend, (t) => t.service_profile)].map(([profile, byProfile]) => (
                        <details key={profile} open className="nested">
                            <summary>{profile}</summary>
                            <table className="grid compact">
                                <tbody>
                                    {byProfile.map((t) => (
                                        <tr key={t.test_name}>
                                            <td className="spec">
                                                <a href={trialHref(props.run, t)} title={t.summary}>
                                                    {t.spec_id}
                                                </a>
                                                {t.requires.map((r) => (
                                                    <span key={r} className="cap">
                                                        {r}
                                                    </span>
                                                ))}
                                            </td>
                                            <td>
                                                <StatusBadge
                                                    status={t.status}
                                                    title={t.reason ?? undefined}
                                                />
                                            </td>
                                            <td className="num">{ms(t.duration_ms)}</td>
                                            <td className="reason">
                                                {t.error ? firstLine(t.error) : (t.reason ?? "")}
                                            </td>
                                        </tr>
                                    ))}
                                </tbody>
                            </table>
                        </details>
                    ))}
                </details>
            ))}
        </div>
    );
}

function Matrix(props: { run: RunSummary; trials: TrialEntry[] }) {
    const columns = [...new Set(props.trials.map((t) => `${t.backend}\u0000${t.service_profile}`))];
    const rows = group(props.trials, (t) => t.spec_id);
    return (
        <div className="matrix-wrap">
            <table className="grid matrix">
                <thead>
                    <tr>
                        <th>Spec</th>
                        {columns.map((c) => {
                            const [b, p] = c.split("\u0000");
                            return (
                                <th key={c}>
                                    {b}
                                    <br />
                                    <span className="muted">{p}</span>
                                </th>
                            );
                        })}
                    </tr>
                </thead>
                <tbody>
                    {[...rows].map(([spec, cells]) => (
                        <tr key={spec}>
                            <td className="spec" title={cells[0]?.summary}>
                                {spec}
                            </td>
                            {columns.map((c) => {
                                const t = cells.find(
                                    (x) => `${x.backend}\u0000${x.service_profile}` === c,
                                );
                                return (
                                    <td key={c} className="cell">
                                        {t ? (
                                            <a
                                                className={`dot st-${t.status}`}
                                                href={trialHref(props.run, t)}
                                                title={`${statusText(t.status)}${t.reason ? `: ${t.reason}` : ""}`}
                                                aria-label={statusText(t.status)}
                                            >
                                                {STATUS_SYMBOL[t.status]}
                                            </a>
                                        ) : null}
                                    </td>
                                );
                            })}
                        </tr>
                    ))}
                </tbody>
            </table>
        </div>
    );
}

function firstLine(s: string): string {
    return s.split("\n", 1)[0] ?? "";
}
