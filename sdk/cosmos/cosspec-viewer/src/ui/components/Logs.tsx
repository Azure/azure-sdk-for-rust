// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { useMemo, useState } from "react";
import { atLeast, LEVELS, spanPath, spanTree, type SpanNode } from "../../model/logs";
import type { Level, LogJson, SpanJson } from "../../model/types";
import { ms, rel } from "../format";

/** The level as a word, so it never relies on color alone. */
export function LevelBadge(props: { level: Level }) {
    return <span className={`level lv-${props.level}`}>{props.level.toUpperCase()}</span>;
}

export function LevelSelect(props: { value: Level; onChange: (level: Level) => void }) {
    return (
        <select
            value={props.value}
            title="Minimum log level"
            onChange={(e) => props.onChange(e.currentTarget.value as Level)}
        >
            {LEVELS.map((l) => (
                <option key={l} value={l}>
                    {l} and above
                </option>
            ))}
        </select>
    );
}

/** A log's fields as `key=value` pairs, expandable when long. */
export function Fields(props: { fields: Record<string, unknown> | undefined }) {
    const entries = Object.entries(props.fields ?? {});
    if (!entries.length) return null;
    const text = entries
        .map(([k, v]) => `${k}=${typeof v === "string" ? v : JSON.stringify(v)}`)
        .join(" ");
    if (text.length <= 120) return <code className="muted fields"> {text}</code>;
    return (
        <details className="fields">
            <summary>
                <code className="muted">{text.slice(0, 100)}…</code>
            </summary>
            <pre className="code">{JSON.stringify(props.fields, null, 2)}</pre>
        </details>
    );
}

/**
 * Every captured log, with level, target, and text filters. `onActivity`
 * makes activity IDs links to the matching request.
 */
export function LogsTable(props: {
    logs: LogJson[];
    spans?: SpanJson[];
    origin?: number;
    onActivity?: (activityId: string) => void;
    showTrial?: boolean;
}) {
    const { logs, spans = [], origin = 0 } = props;
    const [level, setLevel] = useState<Level>("trace");
    const [target, setTarget] = useState("");
    const [text, setText] = useState("");
    const targets = useMemo(() => [...new Set(logs.map((l) => l.target))].sort(), [logs]);
    const q = text.toLowerCase();
    const rows = logs.filter(
        (l) =>
            atLeast(l.level, level) &&
            (!target || l.target === target) &&
            (!q ||
                l.message.toLowerCase().includes(q) ||
                l.target.toLowerCase().includes(q) ||
                JSON.stringify(l.fields ?? {})
                    .toLowerCase()
                    .includes(q)),
    );
    return (
        <>
            <div className="toolbar">
                <LevelSelect value={level} onChange={setLevel} />
                <select value={target} onChange={(e) => setTarget(e.currentTarget.value)}>
                    <option value="">All targets</option>
                    {targets.map((t) => (
                        <option key={t}>{t}</option>
                    ))}
                </select>
                <input
                    type="search"
                    placeholder="Filter logs"
                    value={text}
                    onChange={(e) => setText(e.currentTarget.value)}
                />
                <span className="muted small">
                    {rows.length} of {logs.length}
                </span>
            </div>
            {rows.length ? (
                <table className="grid compact logs">
                    <thead>
                        <tr>
                            <th>Time</th>
                            <th>Level</th>
                            {props.showTrial ? <th>Trial</th> : null}
                            <th>Target</th>
                            <th>Message</th>
                        </tr>
                    </thead>
                    <tbody>
                        {rows.map((l, i) => {
                            const path = spanPath(spans, l.span);
                            return (
                                <tr key={i} className={`log-row lv-${l.level}`}>
                                    <td className="num">{rel(l.t_ms, origin)}</td>
                                    <td>
                                        <LevelBadge level={l.level} />
                                    </td>
                                    {props.showTrial ? (
                                        <td className="muted">{l.trial ?? "–"}</td>
                                    ) : null}
                                    <td
                                        className="muted"
                                        title={l.file ? `${l.file}:${l.line ?? "?"}` : undefined}
                                    >
                                        {l.target}
                                    </td>
                                    <td>
                                        {path.length ? (
                                            <span className="muted span-path">
                                                {path.join(" › ")}:{" "}
                                            </span>
                                        ) : null}
                                        {l.message}
                                        {l.activity_id && props.onActivity ? (
                                            <>
                                                {" "}
                                                <button
                                                    className="small"
                                                    title="Show the request with this activity ID"
                                                    onClick={() =>
                                                        props.onActivity!(l.activity_id!)
                                                    }
                                                >
                                                    {l.activity_id.slice(0, 8)}…
                                                </button>
                                            </>
                                        ) : null}
                                        <Fields fields={l.fields} />
                                    </td>
                                </tr>
                            );
                        })}
                    </tbody>
                </table>
            ) : (
                <div className="center muted">No logs match.</div>
            )}
        </>
    );
}

/** Spans as an indented tree with start time and duration. */
export function SpansTree(props: { spans: SpanJson[]; origin?: number }) {
    const roots = useMemo(() => spanTree(props.spans), [props.spans]);
    const rows: [SpanNode, number][] = [];
    const walk = (nodes: SpanNode[], depth: number) =>
        nodes.forEach((n) => {
            rows.push([n, depth]);
            walk(n.children, depth + 1);
        });
    walk(roots, 0);
    return (
        <table className="grid compact spans">
            <thead>
                <tr>
                    <th>Span</th>
                    <th>Start</th>
                    <th>Duration</th>
                    <th>Target</th>
                    <th>Fields</th>
                </tr>
            </thead>
            <tbody>
                {rows.map(([{ span: s }, depth]) => (
                    <tr key={s.id}>
                        <td style={{ paddingLeft: `${0.5 + depth * 1.2}rem` }}>
                            {depth ? <span className="muted">└ </span> : null}
                            {s.name} <LevelBadge level={s.level} />
                        </td>
                        <td className="num">{rel(s.start_ms, props.origin ?? 0)}</td>
                        <td className="num">
                            {s.end_ms === null ? "still open" : ms(s.end_ms - s.start_ms)}
                        </td>
                        <td className="muted">{s.target}</td>
                        <td>
                            <Fields fields={s.fields} />
                        </td>
                    </tr>
                ))}
            </tbody>
        </table>
    );
}
