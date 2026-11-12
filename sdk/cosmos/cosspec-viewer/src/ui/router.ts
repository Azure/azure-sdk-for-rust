// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { useEffect, useState } from "react";

/**
 * Hash routes, such as `#/run/<id>/trial/<test name>?req=12`. Hash URLs work
 * from the local server and from static hosting alike, and deep links survive
 * a reload.
 */
export interface Route {
    path: string[];
    query: URLSearchParams;
}

export function parseHash(hash: string): Route {
    const [path = "", query = ""] = hash.replace(/^#\/?/, "").split("?", 2);
    return {
        path: path.split("/").filter(Boolean).map(decodeURIComponent),
        query: new URLSearchParams(query),
    };
}

export function href(
    path: string[],
    query: Record<string, string | null | undefined> = {},
): string {
    const params = new URLSearchParams();
    for (const [k, v] of Object.entries(query)) if (v) params.set(k, v);
    const qs = params.toString();
    return `#/${path.map(encodeURIComponent).join("/")}${qs ? `?${qs}` : ""}`;
}

export function useRoute(): Route {
    const [route, setRoute] = useState(() => parseHash(location.hash));
    useEffect(() => {
        const update = () => setRoute(parseHash(location.hash));
        window.addEventListener("hashchange", update);
        return () => window.removeEventListener("hashchange", update);
    }, []);
    return route;
}

/** Changes query parameters in place, without adding a history entry. */
export function setQuery(route: Route, updates: Record<string, string | null | undefined>): void {
    const query: Record<string, string> = Object.fromEntries(route.query);
    for (const [k, v] of Object.entries(updates)) {
        if (v) query[k] = v;
        else delete query[k];
    }
    history.replaceState(null, "", href(route.path, query));
    window.dispatchEvent(new HashChangeEvent("hashchange"));
}
