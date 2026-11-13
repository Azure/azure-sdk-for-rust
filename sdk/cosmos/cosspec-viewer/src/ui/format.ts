// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import type { HttpJson, OutcomeJson } from "../model/types";

/** The region an exchange went to: its alias with the real name, when known. */
export function region(r: Pick<HttpJson, "region" | "region_alias">): string | null {
    if (r.region_alias && r.region) return `${r.region_alias} (${r.region})`;
    return r.region_alias ?? r.region;
}

export function ms(value: number | null | undefined): string {
    if (value === null || value === undefined) return "–";
    if (value < 1) return `${value.toFixed(2)} ms`;
    if (value < 1000) return `${value.toFixed(1)} ms`;
    if (value < 60_000) return `${(value / 1000).toFixed(2)} s`;
    return `${Math.floor(value / 60_000)}m ${((value % 60_000) / 1000).toFixed(0)}s`;
}

/** A time relative to `origin`, signed when before it. */
export function rel(value: number, origin: number): string {
    const d = value - origin;
    return d < -0.0005 ? `−${ms(-d)}` : ms(Math.max(0, d));
}

export function when(iso: string | null | undefined): string {
    if (!iso) return "–";
    const date = new Date(iso);
    return Number.isNaN(date.getTime()) ? iso : date.toLocaleString();
}

export function between(from: string, to: string | null): number | null {
    if (!to) return null;
    const d = Date.parse(to) - Date.parse(from);
    return Number.isFinite(d) ? d : null;
}

/** A CSS class for an outcome: colors on the timeline and in tables. */
export type OutcomeClass =
    "ok" | "client" | "throttled" | "server" | "transport" | "cancelled" | "pending";

export function outcomeClass(outcome: OutcomeJson | null): OutcomeClass {
    if (!outcome) return "pending";
    switch (outcome.kind) {
        case "response":
            if (outcome.status === 429 || outcome.status === 449) return "throttled";
            if (outcome.status >= 500) return "server";
            if (outcome.status >= 400) return "client";
            return "ok";
        case "transport_error":
            return "transport";
        case "cancelled":
            return "cancelled";
        default:
            return "pending";
    }
}

/**
 * A symbol for an outcome, so success and failure don't rely on color alone.
 */
export function outcomeSymbol(outcome: OutcomeJson | null): string {
    switch (outcomeClass(outcome)) {
        case "ok":
            return "✓";
        case "cancelled":
            return "⊘";
        case "pending":
            return "…";
        default:
            return "✗";
    }
}

export function outcomeText(outcome: OutcomeJson | null): string {
    if (!outcome) return "in flight";
    switch (outcome.kind) {
        case "response":
            return outcome.sub_status
                ? `${outcome.status}/${outcome.sub_status}`
                : `${outcome.status}`;
        case "transport_error":
            return "transport error";
        case "cancelled":
            return "cancelled";
        default:
            return "unknown";
    }
}

export function urlPath(url: string): string {
    try {
        const u = new URL(url);
        return `${u.pathname}${u.search}`;
    } catch {
        return url;
    }
}

export function requestLabel(r: HttpJson): string {
    return `#${r.sequence} ${r.operation ?? r.method} ${outcomeText(r.outcome)} · ${ms(r.duration_ms)}${
        region(r) ? ` · ${region(r)}` : ""
    }`;
}
