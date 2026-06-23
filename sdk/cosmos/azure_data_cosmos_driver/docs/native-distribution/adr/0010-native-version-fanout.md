# ADR 0010 — One native version fanned out to all feeds simultaneously; each SDK pins that exact version

**Status:** Proposed (for design review)

## Context
The native driver ships on its own SemVer (ADR 0005), while .NET, Go, and Java each release on their own cadence. A foundational question underlies the whole distribution model: when a new native version is built, is it published to every language feed **at once** off the single hand-off (ADR 0001), or may each language pull a pinned native version and publish on its **own** schedule? This decides whether "which driver am I running" has one answer across languages or N answers, and it shapes the fan-out pipeline (ADR 0009) — so it belongs in an ADR, not an open question.

## Decision
- A given native release (one build + signing, ADR 0001) is **fanned out to all language feeds simultaneously** from that single hand-off — .NET, Go, and (later) Java publish the *same* native version together.
- Each language SDK **pins the exact native version** from that fan-out and may cut its *own* managed/SDK releases independently between native releases; what it must not do is float onto a *different* native build.
- The native SemVer is the **one source of truth** for "which driver"; per-language package versions map to it but never fork it. Hosts enforce this at load by accepting only that exact ABI revision (ADR 0005) — there is no compatibility *range*, which keeps the test matrix a single point.

## Consequences
- No cross-language version skew: a fix in the native driver lands everywhere in one coordinated fan-out, not at N independent times.
- The fan-out pipeline (ADR 0009) publishes a release as an all-or-nothing set; a partial fan-out (some feeds updated, some not) is an explicit failure state to guard.
- Languages keep cadence freedom for their *own* surface area while staying lockstep on the native bytes.

## Alternatives considered
- Fully independent per-language cadence off pinned native versions — rejected for GA: maximizes skew and makes "which driver is this customer on" unanswerable across languages; revisit only if a language's release cadence makes lockstep impractical.
