// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { runState } from "../../model/detect";
import type { RunSummary } from "../../model/types";
import { shortId } from "../App";
import { TotalsBar, TotalsText } from "../components/Status";
import { between, ms, when } from "../format";
import { href } from "../router";

/** Every run in the source, newest first. */
export function RunsScreen(props: { runs: RunSummary[] }) {
    return (
        <section>
            <h2>Runs</h2>
            <table className="grid">
                <thead>
                    <tr>
                        <th>Run</th>
                        <th>Started</th>
                        <th>Duration</th>
                        <th>Backends</th>
                        <th>Trials</th>
                        <th className="wide">Outcome</th>
                    </tr>
                </thead>
                <tbody>
                    {props.runs.map((run) => {
                        const m = run.manifest;
                        const state = runState(m);
                        return (
                            <tr
                                key={run.id}
                                className="link-row"
                                onClick={() => (location.hash = href(["run", run.id]))}
                            >
                                <td>
                                    <a href={href(["run", run.id])} title={run.id}>
                                        <code>{shortId(run.id)}</code>
                                    </a>
                                    {state !== "completed" ? (
                                        <span className={`pill ${state}`}>{state}</span>
                                    ) : null}
                                    {run.synthesized ? (
                                        <span
                                            className="pill"
                                            title="No manifest.json; rebuilt from result files"
                                        >
                                            no manifest
                                        </span>
                                    ) : null}
                                </td>
                                <td>{when(m.started_at)}</td>
                                <td>{ms(between(m.started_at, m.finished_at))}</td>
                                <td>{m.backends.map((b) => b.name).join(", ") || "–"}</td>
                                <td className="num">{m.trials.length}</td>
                                <td>
                                    <TotalsBar totals={m.totals} />
                                    <TotalsText totals={m.totals} />
                                </td>
                            </tr>
                        );
                    })}
                </tbody>
            </table>
        </section>
    );
}
