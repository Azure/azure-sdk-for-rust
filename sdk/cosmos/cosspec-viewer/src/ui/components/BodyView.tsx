// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { useState } from "react";
import type { BodyJson } from "../../model/types";

function bytesOf(base64: string): Uint8Array {
    const raw = atob(base64);
    return Uint8Array.from(raw, (c) => c.charCodeAt(0));
}

export function hexDump(bytes: Uint8Array): string {
    const lines: string[] = [];
    for (let i = 0; i < bytes.length; i += 16) {
        const chunk = bytes.subarray(i, i + 16);
        const hex = Array.from(chunk, (b) => b.toString(16).padStart(2, "0")).join(" ");
        const ascii = Array.from(chunk, (b) =>
            b >= 0x20 && b < 0x7f ? String.fromCharCode(b) : ".",
        ).join("");
        lines.push(`${i.toString(16).padStart(6, "0")}  ${hex.padEnd(47)}  ${ascii}`);
    }
    return lines.join("\n");
}

/** Pretty-prints JSON when the (possibly cut) text parses; otherwise returns it as is. */
export function pretty(text: string): string {
    try {
        return JSON.stringify(JSON.parse(text), null, 2);
    } catch {
        return text;
    }
}

/** An HTTP body: JSON pretty-printed, binary as a hex dump, with a note when it was cut. */
export function BodyView(props: { title: string; body: BodyJson | null | undefined }) {
    const { body } = props;
    const [raw, setRaw] = useState(false);
    if (!body) {
        return (
            <div className="body">
                <h4>{props.title}</h4>
                <div className="muted small">No body.</div>
            </div>
        );
    }
    const text =
        body.text !== undefined
            ? raw
                ? body.text
                : pretty(body.text)
            : body.base64
              ? hexDump(bytesOf(body.base64))
              : "";
    const shown =
        body.text !== undefined
            ? new TextEncoder().encode(body.text).length
            : body.base64
              ? bytesOf(body.base64).length
              : 0;
    return (
        <div className="body">
            <h4>
                {props.title}{" "}
                <span className="muted small">
                    {body.length} bytes{body.base64 ? " · binary" : ""}
                </span>
                <span className="spacer" />
                {body.text !== undefined ? (
                    <button className="small" onClick={() => setRaw(!raw)}>
                        {raw ? "Pretty" : "Raw"}
                    </button>
                ) : null}
                <button
                    className="small"
                    onClick={() =>
                        void navigator.clipboard?.writeText(body.text ?? body.base64 ?? "")
                    }
                >
                    Copy
                </button>
            </h4>
            {body.truncated ? (
                <div className="banner warn small">
                    Showing the first {shown} of {body.length} bytes; cosspec didn't record the
                    rest.
                </div>
            ) : null}
            <pre className="code">{text}</pre>
        </div>
    );
}
