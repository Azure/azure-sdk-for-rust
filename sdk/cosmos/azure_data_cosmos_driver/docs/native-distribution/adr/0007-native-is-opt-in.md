# ADR 0007 — Native transport ships in a new SDK major; no parallel managed transport

**Status:** Accepted (proposed for review)

## Context
`Microsoft.Azure.Cosmos` (pure managed) and the Go SDK run today on any platform with no native dependency. Introducing a platform-specific native library changes the portability and trust profile of the SDK. The question is how consumers move onto native **without being force-migrated mid-stream**, and whether a managed and a native transport are maintained side by side. The switch to the Rust driver is a breaking change in how the SDK talks to the service — it lands at a **major version**, not as a per-call toggle.

## Decision
- Native transport is introduced in a **new SDK major version**. Adopting that major **is** adopting native: there is no per-client / per-call opt-in switch, and no managed transport kept in parallel *inside the same major*.
- Existing majors keep their current managed / pure transport unchanged. No consumer is **force-migrated by a routine (minor/patch) bump** — they move to native deliberately, by choosing to take the new major.
- The supported surface is bounded by the platform matrix (ADR 0008). On an unsupported platform/RID the native major fails with a **clear, actionable error**; it does **not** silently fall back to a managed path. If a pure-managed / pure-Go audience must be served long-term, that is a **separate, explicitly limited package**, not a hidden fallback inside the flagship.
- **Integrity and contract failures are fail-loud:** a signature/checksum failure, a corrupt library, or an ABI mismatch (ADR 0005) on a library that *is* present surfaces a clear error and aborts — never a silent downgrade that could mask tampering or misconfiguration.

## Consequences
- "Which transport am I on" has one answer per major: the native major is native, full stop — no per-deployment drift between managed and native, and no doubled transport matrix to test and support.
- The native path matures across the **prerelease line of the new major** (dogfood → preview → GA) rather than behind a runtime flag in an existing major.
- Migration is a deliberate major-version decision that goes through the usual major-version compatibility review — not a surprise on upgrade.
- Platforms outside the matrix are a clear error in the native major; serving them is a separate-package decision, not an in-SDK fallback.

## Alternatives considered
- **Per-client / per-call opt-in toggle inside one major (native *or* managed)** — rejected: keeps two transports alive in parallel, doubles the test/support matrix, and does not match how the switch actually happens — moving to the Rust driver is a major-version event.
- **Silent runtime fallback to managed on unsupported platforms** — rejected: hides capability gaps and muddies the trust model; an explicit error (or a separate limited package) is clearer.
- **Native-on by default within an existing major** — rejected: that is a force-migration by upgrade.
