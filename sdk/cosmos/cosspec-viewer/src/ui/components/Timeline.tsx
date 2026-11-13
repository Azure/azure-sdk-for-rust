// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { useEffect, useMemo, useRef, useState, type PointerEvent } from "react";
import type { HttpJson, LogJson, PhaseJson } from "../../model/types";
import { layout, ticks, type Concurrency } from "../../timeline/layout";
import {
    ms,
    outcomeClass,
    outcomeSymbol,
    outcomeText,
    rel,
    requestLabel,
    urlPath,
    type OutcomeClass,
} from "../format";

const ROW = 18;
const LANE_GAP = 8;
const AXIS = 22;
const PHASE_ROW = 16;
const GUTTER = 150;
const LOG_LANE = 22;

/** Each outcome's fill pattern, so outcomes differ by more than color. */
const PATTERN: Record<OutcomeClass, string | null> = {
    ok: null,
    client: "p-diagonal",
    throttled: "p-dots",
    server: "p-cross",
    transport: "p-horizontal",
    cancelled: "p-vertical",
    pending: null,
};

export function host(url: string): string {
    try {
        return new URL(url).host;
    } catch {
        return url;
    }
}

/**
 * Requests on a time axis: one lane per client label and region, with
 * concurrent requests on separate rows. Stretches where requests overlap, such
 * as a hedge racing the original request, are shaded across every lane, and
 * pointing at or selecting a request outlines the requests it overlaps. Times
 * are relative to `origin`, normally the start of the spec body. Wheel zooms
 * around the pointer, dragging pans, double-click fits.
 */
export function Timeline(props: {
    http: HttpJson[];
    duration: number;
    origin: number;
    phases: PhaseJson[];
    concurrency: Concurrency;
    logs?: LogJson[];
    selected: number | null;
    onSelect: (sequence: number) => void;
}) {
    const { http, duration, logs, origin, phases, concurrency } = props;
    const plan = useMemo(
        () =>
            layout(
                http.map((r) => ({
                    key: r.sequence,
                    label: r.label ?? "client",
                    region: r.region ?? host(r.url),
                    start: r.start_ms,
                    end: r.duration_ms === null ? null : r.start_ms + r.duration_ms,
                })),
                duration,
            ),
        [http, duration],
    );
    const bySeq = useMemo(() => new Map(http.map((r) => [r.sequence, r])), [http]);
    const horizon = Math.max(plan.horizon, 1);
    const fit: [number, number] = [Math.min(origin, horizon - 1), horizon];
    const [view, setView] = useState<[number, number]>(fit);
    const [width, setWidth] = useState(900);
    const [hovered, setHovered] = useState<number | null>(null);
    const [shade, setShade] = useState(true);
    const drag = useRef<{ x: number; view: [number, number] } | null>(null);
    const svg = useRef<SVGSVGElement>(null);
    const box = useRef<HTMLDivElement>(null);

    useEffect(() => {
        const el = box.current;
        if (!el) return;
        const observer = new ResizeObserver(
            ([e]) => e && setWidth(Math.max(320, e.contentRect.width)),
        );
        observer.observe(el);
        return () => observer.disconnect();
    }, []);

    useEffect(() => setView(fit), [horizon, origin]);

    const [from, to] = view;
    const plot = width - GUTTER - 8;
    const x = (t: number) => GUTTER + ((t - from) / (to - from)) * plot;
    const hasLogs = !!logs?.length;
    const top = AXIS + (phases.length ? PHASE_ROW + 4 : 0);
    const laneTop = (row: number, laneIndex: number) => top + row * ROW + laneIndex * LANE_GAP;
    const laneOf = (row: number) =>
        plan.lanes.findIndex((l) => row >= l.firstRow && row < l.firstRow + l.rows);
    const rowsHeight = top + plan.rows * ROW + plan.lanes.length * LANE_GAP;
    const height = rowsHeight + (hasLogs ? LOG_LANE + LANE_GAP : 0) + 4;

    const focus = hovered ?? props.selected;
    const partners = new Set(focus === null ? [] : (concurrency.partners.get(focus) ?? []));

    const timeAt = (clientX: number) => {
        const left = svg.current?.getBoundingClientRect().left ?? 0;
        return from + ((clientX - left - GUTTER) / plot) * (to - from);
    };
    const clamp = (a: number, b: number): [number, number] => {
        const span = Math.min(b - a, horizon * 1.05);
        let lo = Math.max(-horizon * 0.025, a);
        if (lo + span > horizon * 1.025) lo = horizon * 1.025 - span;
        return [lo, lo + span];
    };
    const onWheel = (e: globalThis.WheelEvent) => {
        e.preventDefault();
        const at = timeAt(e.clientX);
        const k = Math.exp(e.deltaY * 0.002);
        const span = Math.max((to - from) * k, horizon / 10_000, 0.001);
        const ratio = (at - from) / (to - from);
        setView(clamp(at - ratio * span, at - ratio * span + span));
    };
    const wheel = useRef(onWheel);
    wheel.current = onWheel;
    useEffect(() => {
        const el = svg.current;
        if (!el) return;
        const listener = (e: globalThis.WheelEvent) => wheel.current(e);
        el.addEventListener("wheel", listener, { passive: false });
        return () => el.removeEventListener("wheel", listener);
    }, []);

    const onDown = (e: PointerEvent) => {
        drag.current = { x: e.clientX, view };
    };
    const onMove = (e: PointerEvent) => {
        const d = drag.current;
        if (!d) return;
        const dt = ((e.clientX - d.x) / plot) * (d.view[1] - d.view[0]);
        if (Math.abs(e.clientX - d.x) > 3) {
            svg.current?.setPointerCapture(e.pointerId);
            setView(clamp(d.view[0] - dt, d.view[1] - dt));
        }
    };
    const zoom = (k: number) => {
        const mid = (from + to) / 2;
        const span = (to - from) * k;
        setView(clamp(mid - span / 2, mid + span / 2));
    };
    const overlapCount = concurrency.overlaps.length;

    return (
        <div className="timeline" ref={box}>
            <PatternDefs />
            <div className="timeline-tools">
                <button onClick={() => zoom(0.5)} title="Zoom in">
                    +
                </button>
                <button onClick={() => zoom(2)} title="Zoom out">
                    −
                </button>
                <button onClick={() => setView(fit)} title="Fit the spec body and teardown">
                    Fit
                </button>
                {origin > 0 ? (
                    <button
                        onClick={() => setView([0, horizon])}
                        title="Include acquiring pooled resources"
                    >
                        All
                    </button>
                ) : null}
                <label className="small" title="Shade stretches where requests overlap">
                    <input
                        type="checkbox"
                        checked={shade}
                        onChange={(e) => setShade(e.currentTarget.checked)}
                    />{" "}
                    Overlaps ({overlapCount})
                </label>
                <span className="muted small">
                    {rel(from, origin)} – {rel(to, origin)} · scroll to zoom, drag to pan
                </span>
                <span className="spacer" />
                <Legend />
            </div>
            <svg
                ref={svg}
                width={width}
                height={height}
                onPointerDown={onDown}
                onPointerMove={onMove}
                onPointerUp={() => (drag.current = null)}
                onDoubleClick={() => setView(fit)}
            >
                <defs>
                    <clipPath id="plot">
                        <rect x={GUTTER} y={0} width={plot} height={height} />
                    </clipPath>
                </defs>
                {ticks(from - origin, to - origin).map((t) => (
                    <g key={t}>
                        <line
                            className={`tick ${t === 0 && origin > 0 ? "origin" : ""}`}
                            x1={x(t + origin)}
                            x2={x(t + origin)}
                            y1={AXIS - 4}
                            y2={height}
                        />
                        <text className="tick-label" x={x(t + origin)} y={12} textAnchor="middle">
                            {rel(t, 0)}
                        </text>
                    </g>
                ))}
                {plan.lanes.map((lane, i) => (
                    <g key={`${lane.label}/${lane.region}`}>
                        <rect
                            className={`lane lane-${i % 2}`}
                            x={0}
                            y={laneTop(lane.firstRow, i) - 2}
                            width={width}
                            height={lane.rows * ROW + 4}
                        />
                        <text className="lane-label" x={6} y={laneTop(lane.firstRow, i) + 13}>
                            <tspan className={`label-${lane.label}`}>{lane.label}</tspan>
                            <tspan className="muted"> {lane.region}</tspan>
                        </text>
                    </g>
                ))}
                {phases.length ? (
                    <text className="lane-label" x={6} y={AXIS + 12}>
                        phases
                    </text>
                ) : null}
                <g clipPath="url(#plot)">
                    {phases.map((p) => {
                        const left = x(p.start_ms);
                        const w = Math.max(1, x(p.end_ms) - left);
                        return (
                            <g key={p.name} className={`phase phase-${p.name}`}>
                                <title>
                                    {p.name}: {ms(p.end_ms - p.start_ms)}
                                    {p.name === "pool"
                                        ? " waiting for the pooled account and database, shared with other trials"
                                        : ""}
                                </title>
                                <rect x={left} y={AXIS} width={w} height={PHASE_ROW} />
                                {w > 40 ? (
                                    <text x={left + 4} y={AXIS + 12} className="phase-label">
                                        {p.name} {ms(p.end_ms - p.start_ms)}
                                    </text>
                                ) : null}
                            </g>
                        );
                    })}
                    {shade
                        ? concurrency.overlaps.map((o) => (
                              <g key={o.start} className="overlap">
                                  <title>
                                      {o.peak} requests in flight together for {ms(o.end - o.start)}
                                  </title>
                                  <rect
                                      x={x(o.start)}
                                      y={top - 2}
                                      width={Math.max(2, x(o.end) - x(o.start))}
                                      height={rowsHeight - top + 2}
                                  />
                                  <rect
                                      x={x(o.start)}
                                      y={top - 2}
                                      width={Math.max(2, x(o.end) - x(o.start))}
                                      height={rowsHeight - top + 2}
                                      fill="url(#p-overlap)"
                                  />
                                  {x(o.end) - x(o.start) > 18 ? (
                                      <text
                                          x={x(o.start) + 2}
                                          y={rowsHeight - 3}
                                          className="overlap-label"
                                      >
                                          ×{o.peak}
                                      </text>
                                  ) : null}
                              </g>
                          ))
                        : null}
                    {plan.bars.map((bar) => {
                        const r = bySeq.get(bar.key)!;
                        const y = laneTop(bar.row, laneOf(bar.row)) + 2;
                        const left = x(bar.start);
                        const w = Math.max(2, x(bar.end) - left);
                        const cls = outcomeClass(r.outcome);
                        const pattern = PATTERN[cls];
                        const role =
                            focus === null
                                ? ""
                                : focus === bar.key
                                  ? "focus"
                                  : partners.has(bar.key)
                                    ? "partner"
                                    : partners.size
                                      ? "dim"
                                      : "";
                        const others = concurrency.partners.get(bar.key) ?? [];
                        const label = `${outcomeSymbol(r.outcome)} ${r.operation ?? r.method} ${outcomeText(r.outcome)}`;
                        return (
                            <g
                                key={bar.key}
                                className={`bar o-${cls} ${props.selected === bar.key ? "selected" : ""} ${bar.open ? "open" : ""} ${role}`}
                                onClick={() => props.onSelect(bar.key)}
                                onPointerEnter={() => setHovered(bar.key)}
                                onPointerLeave={() => setHovered(null)}
                            >
                                <title>
                                    {requestLabel(r)}
                                    {"\n"}
                                    {r.method} {urlPath(r.url)}
                                    {bar.open ? "\n(never finished)" : ""}
                                    {others.length
                                        ? `\noverlaps ${others.map((s) => `#${s}`).join(", ")}`
                                        : ""}
                                </title>
                                <rect
                                    className="fill"
                                    x={left}
                                    y={y}
                                    width={w}
                                    height={ROW - 4}
                                    rx={2}
                                />
                                {pattern ? (
                                    <rect
                                        x={left}
                                        y={y}
                                        width={w}
                                        height={ROW - 4}
                                        rx={2}
                                        fill={`url(#${pattern})`}
                                    />
                                ) : null}
                                {w > 120 ? (
                                    <text x={left + 4} y={y + 11} className="bar-label">
                                        {label}
                                    </text>
                                ) : w > 12 ? (
                                    <text x={left + 3} y={y + 11} className="bar-label">
                                        {outcomeSymbol(r.outcome)}
                                    </text>
                                ) : null}
                            </g>
                        );
                    })}
                    {hasLogs
                        ? logs!.map((log, i) => (
                              <line
                                  key={i}
                                  className={`log-tick lv-${log.level}`}
                                  x1={x(log.t_ms)}
                                  x2={x(log.t_ms)}
                                  y1={rowsHeight + 2 + LOG_TICK_INSET[log.level]}
                                  y2={rowsHeight + LOG_LANE}
                              >
                                  <title>
                                      {rel(log.t_ms, origin)} {log.level} {log.target}:{" "}
                                      {log.message}
                                  </title>
                              </line>
                          ))
                        : null}
                </g>
                {hasLogs ? (
                    <text className="lane-label" x={6} y={rowsHeight + 15}>
                        logs
                    </text>
                ) : null}
            </svg>
        </div>
    );
}

/** Taller ticks for more severe log levels, so level isn't shown by color alone. */
const LOG_TICK_INSET: Record<LogJson["level"], number> = {
    error: 0,
    warn: 5,
    info: 10,
    debug: 13,
    trace: 15,
};

function PatternDefs() {
    const lines = (id: string, angle: number, gap = 5) => (
        <pattern
            id={id}
            width={gap}
            height={gap}
            patternUnits="userSpaceOnUse"
            patternTransform={`rotate(${angle})`}
        >
            <line x1="0" y1="0" x2="0" y2={gap} className="pattern-line" />
        </pattern>
    );
    return (
        <svg className="pattern-defs" width="0" height="0" aria-hidden="true">
            <defs>
                {lines("p-diagonal", 45)}
                {lines("p-vertical", 0)}
                {lines("p-horizontal", 90, 4)}
                <pattern id="p-cross" width="6" height="6" patternUnits="userSpaceOnUse">
                    <path d="M0,0 L6,6 M6,0 L0,6" className="pattern-line" />
                </pattern>
                <pattern id="p-dots" width="5" height="5" patternUnits="userSpaceOnUse">
                    <circle cx="2.5" cy="2.5" r="1.2" className="pattern-dot" />
                </pattern>
                <pattern
                    id="p-overlap"
                    width="8"
                    height="8"
                    patternUnits="userSpaceOnUse"
                    patternTransform="rotate(-45)"
                >
                    <line x1="0" y1="0" x2="0" y2="8" className="overlap-line" />
                </pattern>
            </defs>
        </svg>
    );
}

function Legend() {
    const items: [OutcomeClass, string][] = [
        ["ok", "2xx/3xx"],
        ["client", "4xx"],
        ["throttled", "429/449"],
        ["server", "5xx"],
        ["transport", "transport error"],
        ["cancelled", "cancelled"],
        ["pending", "in flight"],
    ];
    return (
        <span className="legend">
            {items.map(([cls, text]) => (
                <span key={cls}>
                    <svg width="14" height="10" className={`swatch o-${cls}`} aria-hidden="true">
                        <rect className="fill" width="14" height="10" rx="2" />
                        {PATTERN[cls] ? (
                            <rect width="14" height="10" rx="2" fill={`url(#${PATTERN[cls]})`} />
                        ) : null}
                    </svg>{" "}
                    {text}
                </span>
            ))}
            <span>
                <svg width="14" height="10" className="swatch overlap" aria-hidden="true">
                    <rect width="14" height="10" />
                    <rect width="14" height="10" fill="url(#p-overlap)" />
                </svg>{" "}
                overlap
            </span>
        </span>
    );
}
