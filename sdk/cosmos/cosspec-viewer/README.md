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
directory holding runs, such as `target/cosspec-results` or a CI artifact
with several runs. A directory of runs opens to the runs list; a single run
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
- **Run**: the run's arguments, backends, and checks, then its trials as a
  backend, profile, and spec tree (like the console) or as a spec-by-backend
  matrix. Filter by text, backend, profile, or status. Filters are part of
  the URL.
- **Trial**: the spec's summary, backend, profile, capabilities, and outcome,
  with the error, panic, and backtrace, or the skip or known-issue reason.
  Tabs show:
  - **Requests**: a timeline with one lane per client (`spec` or `harness`)
    and region, where requests in flight together get separate rows, so
    parallel requests such as hedges are visible. Colors show the outcome.
    Cancelled requests are hatched and requests that never finished are
    dashed. Scroll to zoom, drag to pan, and double-click to fit. Below it, a
    sortable table of the same requests. Select a request in either to see
    its URL, timing, activity ID, and bodies. JSON is pretty-printed, binary
    bodies are shown as hex, and bodies cut at 1 KiB are marked.
  - **Resources**: each account, database, and container, and whether it
    was created or borrowed from the pool.
  - **Logs**: shown when a result file has captured logs. Logs also appear as
    ticks in a lane under the timeline.
  - **Raw JSON**: the whole result file.

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
results inlined. Publishing results from CI isn't set up yet.

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
