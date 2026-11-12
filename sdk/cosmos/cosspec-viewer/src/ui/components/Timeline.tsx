// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { useEffect, useMemo, useRef, useState, type PointerEvent } from "react";
import type { HttpJson, LogJson } from "../../model/types";
import { layout, ticks } from "../../timeline/layout";
import { ms, outcomeClass, requestLabel, urlPath } from "../format";

const ROW = 18;
const LANE_GAP = 8;
const AXIS = 22;
const GUTTER = 150;
const LOG_LANE = 22;

export function host(url: string): string {
    try {
        return new URL(url).host;
    } catch {
        return url;
    }
}

/**
 * Requests on a time axis: one lane per client label and region, with
 * concurrent requests on separate rows so parallel sends (such as hedging) are
 * visible. Wheel zooms around the pointer, dragging pans, double-click fits.
 */
export function Timeline(props: {
    http: HttpJson[];
    duration: number;
    logs?: LogJson[];
    selected: number | null;
    onSelect: (sequence: number) => void;
}) {
    const { http, duration, logs } = props;
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
    const [view, setView] = useState<[number, number]>([0, horizon]);
    const [width, setWidth] = useState(900);
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

    useEffect(() => setView([0, horizon]), [horizon]);

    const [from, to] = view;
    const plot = width - GUTTER - 8;
    const x = (t: number) => GUTTER + ((t - from) / (to - from)) * plot;
    const hasLogs = !!logs?.length;
    const laneTop = (row: number, laneIndex: number) => AXIS + row * ROW + laneIndex * LANE_GAP;
    const laneOf = (row: number) =>
        plan.lanes.findIndex((l) => row >= l.firstRow && row < l.firstRow + l.rows);
    const rowsHeight = AXIS + plan.rows * ROW + plan.lanes.length * LANE_GAP;
    const height = rowsHeight + (hasLogs ? LOG_LANE + LANE_GAP : 0) + 4;

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

    return (
        <div className="timeline" ref={box}>
            <div className="timeline-tools">
                <button onClick={() => zoom(0.5)} title="Zoom in">
                    +
                </button>
                <button onClick={() => zoom(2)} title="Zoom out">
                    −
                </button>
                <button onClick={() => setView([0, horizon])} title="Fit">
                    Fit
                </button>
                <span className="muted small">
                    {ms(Math.max(0, from))} – {ms(to)} · scroll to zoom, drag to pan
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
                onDoubleClick={() => setView([0, horizon])}
            >
                <defs>
                    <pattern
                        id="hatch"
                        width="6"
                        height="6"
                        patternUnits="userSpaceOnUse"
                        patternTransform="rotate(45)"
                    >
                        <line x1="0" y1="0" x2="0" y2="6" className="hatch-line" />
                    </pattern>
                    <clipPath id="plot">
                        <rect x={GUTTER} y={0} width={plot} height={height} />
                    </clipPath>
                </defs>
                {ticks(from, to).map((t) => (
                    <g key={t}>
                        <line className="tick" x1={x(t)} x2={x(t)} y1={AXIS - 4} y2={height} />
                        <text className="tick-label" x={x(t)} y={12} textAnchor="middle">
                            {ms(t)}
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
                <g clipPath="url(#plot)">
                    {plan.bars.map((bar) => {
                        const r = bySeq.get(bar.key)!;
                        const y = laneTop(bar.row, laneOf(bar.row)) + 2;
                        const left = x(bar.start);
                        const w = Math.max(2, x(bar.end) - left);
                        const cls = outcomeClass(r.outcome);
                        return (
                            <g
                                key={bar.key}
                                className={`bar o-${cls} ${props.selected === bar.key ? "selected" : ""} ${bar.open ? "open" : ""}`}
                                onClick={() => props.onSelect(bar.key)}
                            >
                                <title>
                                    {requestLabel(r)}
                                    {"\n"}
                                    {r.method} {urlPath(r.url)}
                                    {bar.open ? "\n(never finished)" : ""}
                                </title>
                                <rect x={left} y={y} width={w} height={ROW - 4} rx={2} />
                                {cls === "cancelled" ? (
                                    <rect
                                        x={left}
                                        y={y}
                                        width={w}
                                        height={ROW - 4}
                                        rx={2}
                                        fill="url(#hatch)"
                                    />
                                ) : null}
                                {w > 60 ? (
                                    <text x={left + 4} y={y + 11} className="bar-label">
                                        {r.operation ?? r.method}
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
                                  y1={rowsHeight + 2}
                                  y2={rowsHeight + LOG_LANE}
                              >
                                  <title>
                                      {ms(log.t_ms)} {log.level} {log.target}: {log.message}
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

function Legend() {
    const items: [string, string][] = [
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
                    <span className={`swatch o-${cls}`} /> {text}
                </span>
            ))}
        </span>
    );
}
