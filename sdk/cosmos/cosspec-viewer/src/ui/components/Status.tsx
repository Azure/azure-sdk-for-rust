// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import type { Totals, TrialStatus } from "../../model/types";

export const STATUS_ORDER: TrialStatus[] = [
    "failed",
    "passed",
    "ignored",
    "skipped",
    "not_run",
    "pending",
];

const STATUS_TEXT: Record<TrialStatus, string> = {
    passed: "passed",
    failed: "failed",
    skipped: "skipped",
    ignored: "known issue",
    not_run: "not run",
    pending: "pending",
};

export function statusText(status: TrialStatus): string {
    return STATUS_TEXT[status];
}

export function StatusBadge(props: { status: TrialStatus; title?: string }) {
    return (
        <span className={`badge st-${props.status}`} title={props.title ?? undefined}>
            {STATUS_TEXT[props.status]}
        </span>
    );
}

/** A stacked bar of trial outcomes. */
export function TotalsBar(props: { totals: Totals }) {
    const total = STATUS_ORDER.reduce((n, s) => n + props.totals[s], 0);
    if (total === 0) return <div className="totals-bar empty" />;
    return (
        <div
            className="totals-bar"
            title={STATUS_ORDER.filter((s) => props.totals[s])
                .map((s) => `${props.totals[s]} ${STATUS_TEXT[s]}`)
                .join(", ")}
        >
            {STATUS_ORDER.map((s) =>
                props.totals[s] ? (
                    <span key={s} className={`st-${s}`} style={{ flexGrow: props.totals[s] }} />
                ) : null,
            )}
        </div>
    );
}

export function TotalsText(props: { totals: Totals }) {
    return (
        <span className="totals-text">
            {STATUS_ORDER.filter((s) => props.totals[s]).map((s) => (
                <span key={s} className={`fg-${s}`}>
                    {props.totals[s]} {STATUS_TEXT[s]}
                </span>
            ))}
        </span>
    );
}
