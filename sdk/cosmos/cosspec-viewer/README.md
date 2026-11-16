# cosspec-viewer

<!-- cspell:ignore bunx -->

Browse [cosspec] spec-test results in a browser. See which trials passed or
failed on each backend and service profile, then open a trial to see its
requests on a timeline, each request and response body, the resources it
used, and any error, panic, or known issue.

It reads the files cosspec writes: `manifest.json` in each run directory and
one result file per trial. See
[Spec results](../docs/specs/0030-cosmos-spec-tests.md#spec-results) for the
formats. Runs without a `manifest.json`, from before manifests existed, still
open; the viewer rebuilds a manifest from their result files.

This is an internal tool; it isn't published.

## Getting started

### Prerequisites

- [Bun] 1.4 or later.

### Install dependencies

```sh
cd sdk/cosmos/cosspec-viewer
bun install
```

### Browse results

Run some specs, then start the viewer:

```sh
cargo test -p cosspec --all-features --test spec
cd sdk/cosmos/cosspec-viewer
bun run start -- --open
```

`start` builds the app, then serves it and the results on
`http://127.0.0.1:4173/`. It only listens on the loopback address. Results
reload in the browser as the files change, so you can watch a run in progress.

| Option            | Default                         | Description                             |
| ----------------- | ------------------------------- | --------------------------------------- |
| `--results <dir>` | `<repo>/target/cosspec-results` | A run directory, or a directory of runs |
| `--port <port>`   | `4173`                          | The port to listen on                   |
| `--open`          |                                 | Opens the viewer in your browser        |

`--results` can point at one run (a directory with a `manifest.json`) or at a
directory holding runs, such as `target/cosspec-results`. For CI results,
download a `cosspec-results-*` artifact from the build, extract it, and pass
the extracted folder. A directory of runs opens to the runs list; a single run
opens straight to its trials.

After the first `start`, `bun run serve -- <options>` skips the build.

### Open results without the server

The built app in `dist/` also works from any static web server, for example
`bunx vite preview` or an internal site. Without the cosspec-viewer server, it
asks for a results folder: choose a folder,
drop one on the page, or reopen the last folder. Files are read in the
browser and never uploaded. In browsers with the File System Access API, such
as Edge and Chrome, a chosen folder stays live: **Refresh** re-reads it.
Otherwise the folder is a snapshot.

## Screens

- **Runs**: every run, newest first, with when it ran, its backends, and its
  outcome counts. Runs still going, interrupted, or without a manifest are
  marked.
- **Run**: the run's arguments, backends, log filter, and checks, then its
  trials as a backend, profile, and spec tree (like the console) or as a
  spec-by-backend matrix. Filter by text, backend, profile, or status.
  Filters are part of the URL.
- **Trial**: the spec's summary, backend, profile, capabilities, and outcome,
  with the error, panic, and backtrace, or the skip or known-issue reason.
  Tabs show:
  - **Events** (the default): the requests view below, with the trial's
    logs interleaved by time at or above the chosen level. Time order
    approximates causal order; it doesn't prove it. Each run of logs between
    requests starts collapsed to one "N log events…" row, which counts any
    warnings and errors; select it, or **Expand all logs**, to see them.
    Logs also appear as ticks in a lane under the timeline.
  - **Requests**: a timeline with one lane per client (`spec` or `harness`)
    and region, where requests in flight together get separate rows, so
    parallel requests such as hedges are visible. Times count from the start
    of the spec body, and a band above the lanes shows the trial's phases:
    waiting for pooled resources, the body, and teardown. **Fit** shows the
    body and **All** the whole trial. Scroll to zoom, drag to pan, and
    double-click to fit. **Overlaps** shades spans where requests were in
    flight together and labels how many; hover or select a request to
    outline the requests it overlaps and dim the rest. Below it, a sortable
    table of the same requests lists each one's overlaps. Select a request in
    either to see its URL, timing, overlaps, activity ID, and bodies. JSON is
    pretty-printed, binary bodies are shown as hex, and bodies cosspec cut
    short are marked.
  - **Resources**: each account, database, and container, and whether it
    was created or borrowed from the pool.
  - **Logs**: every captured log, filtered by level, target, or text, with
    its span path and fields. Select an activity ID to jump to its request.
  - **Spans**: shown when the trial captured `tracing` spans, as a tree with
    start times and durations.
  - **Raw JSON**: the whole result file.
- **Run logs**: linked from the run, the events no trial owned, such as
  pooled setup and end-of-run cleanup. Events a trial emitted after it
  finished show that trial.

Outcomes and log levels never rely on color alone. Statuses carry a symbol
(✓ passed, ✗ failed, ⊘ skipped, ⚠ ignored), requests show their status code
and a symbol, and each request outcome and status total has its own fill
pattern. Log levels are spelled out, and log rows between requests are
italic with a dotted rule.

Every screen has its own URL, such as
`#/run/<run-id>/trial/<test-name>?req=12`, so you can share a link to a
selected request with someone who has the same results.

## How it gets the files

A browser can't read a local folder by itself, so the viewer has two
sources behind one interface (`src/sources/source.ts`):

- **Local server** (`bin/cosspec-viewer.ts`): serves the app and an API over
  the results directory. It finds runs from `manifest.json` files and only
  serves the result files the manifests list. It watches for changes and
  pushes them to the browser with server-sent events.
- **Folder picker**: reads a folder the user chooses or drops, entirely in
  the browser.

Other sources could be added without changing the screens, such as a ZIP file
of a run, a run published at a URL by CI, or a single HTML report with the
results inlined.

## Contributing

```sh
bun run dev         # Vite dev server with hot reload; start `bun run serve` too for the API
bun test            # Unit tests, using the runs in fixtures/results
bun run typecheck
bun run format      # Prettier; `format:check` checks without writing
bun run build
```

`bun run dev` proxies `/api` to `http://127.0.0.1:4173`, or to
`COSSPEC_VIEWER_API` if set. The app is React and TypeScript, built with
Vite, with no other runtime dependencies.

`fixtures/results` holds real runs from the in-memory emulator and the
Azure Cosmos DB Emulator, including failures, known issues, skipped trials,
and a run without a manifest (`legacy-run`). Regenerate them with
`cargo test -p cosspec --all-features --test spec -- <filter> --results-dir <dir>`
when the formats change.

[Bun]: https://bun.sh/
[cosspec]: ../cosspec/README.md
