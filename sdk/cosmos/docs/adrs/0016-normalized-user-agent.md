# ADR-0016 - Driver-owned normalized User-Agent format

**Status:** Approved (approval takes effect when this PR is merged)
**Date:** 2026-10-02

## Context

All future Cosmos SDKs use the shared driver to construct their User-Agent.
Telemetry needs a consistent, easily parsed format that identifies both the
wrapping SDK and the driver, allows language-specific runtime metadata, and
remains human-readable within a 255-byte limit.

## Decision

The driver owns construction of the following format for every language SDK:

```text
{sdk}/{version} (drv={driver}; {os}; {arch}; {rustc}[; ft={base64url}][; {key}={value}]...) [user-suffix]
```

The leading product name must start with `azsdk-`. A wrapping SDK supplies its
own name and version, regardless of implementation language. Direct driver
usage defaults to `azsdk-rust-cosmos-driver/{driver-version}`.

The parenthesized segment starts with `drv=<driver-version>`, identifying this
as a driver-generated User-Agent. OS, architecture, and Rust compiler version
follow as three positional values, in that order. Optional key-value entries
follow them. When platform metadata is present, the first `)` unambiguously
ends the SDK-provided portion.

`ft` carries the enabled feature bitmask, retaining the existing cross-language
bit assignments. Encode the unsigned integer as big-endian bytes, remove
leading zero bytes, and use standard URL-safe base64 without padding. For
example, PPCB plus HTTP/2 is `0x12`, encoded as `ft=Eg`. Omit `ft` when no bits
are set. Use the `base64` crate rather than a custom encoding implementation.

SDKs may supply additional properties, such as `rt=.NET 8.0.1`. Keys are
lowercase ASCII identifiers of at most 16 characters, beginning with a letter
and continuing with letters, digits, hyphens, or underscores. `drv` and `ft`
are reserved. Values are 1-32 printable ASCII characters and may contain
spaces, but not leading or trailing spaces. SDK-provided values never contain
`;`, `(`, or `)`. Reject invalid custom properties. Preserve insertion order;
repeated keys replace their earlier value in place.

`UserAgentBuilder` collects inputs and permits overriding built-in values;
`build()` normalizes inputs, applies the size budget, and produces an immutable
`UserAgent`. Sanitize wrapping identifiers and overridden built-in values so
they cannot introduce delimiters or extra product tokens.

If the full User-Agent exceeds 255 bytes, truncate in this order:

1. Remove platform metadata segments whole from right to left: custom
   properties, feature flags, compiler version, architecture, OS, and finally
   `drv`. Preserve the remaining segments, semicolon-space separators, and
   parentheses.
2. If all platform metadata is removed, omit its parentheses entirely and use
   `{sdk}/{version} [user-suffix]`. Shorten the suffix only if this is still too
   large; omit the separating space when no suffix fits.
3. If the SDK identifier alone exceeds the limit, shorten it to 255 bytes and
   send only that identifier. This applies equally to a wrapping identifier and
   the driver's direct identifier.

The suffix remains opaque and is never normalized. It is shortened only after
all platform metadata is gone. Never truncate a partial metadata segment or
leave empty parentheses. If metadata is absent, parsers treat the first space
as the boundary between the product token and the suffix.

For example:

```text
azsdk-rust-cosmos-driver/1.0.0 (drv=1.0.0; linux; x86_64; 1.98.1)
azsdk-dotnet-cosmos/3.40.0 (drv=1.0.0; linux; x86_64; 1.98.1; ft=Eg; rt=.NET 8.0.1) myapp
```

## Alternatives considered

- **Separate formats per SDK or legacy-format compatibility.** Rejected:
  driver adoption provides one shared construction path and parsing contract.
- **Trailing feature tokens or hexadecimal feature flags.** Rejected:
  metadata belongs inside the delimited segment; base64url is compact and
  decodable with standard tools.
- **Truncate the completed header or normalize the user suffix.** Rejected:
  this can break the metadata structure or alter operator-supplied identity.
- **Shorten the SDK identifier before platform metadata.** Rejected:
  SDK identity takes priority over platform details; suffix shortening is
  allowed only after the platform section is completely removed.

## Consequences

Telemetry can parse the SDK product, split the metadata on semicolons, and
decode feature flags consistently across SDKs. Parsers must adopt this format
instead of the previous product-token layout and trailing `|F<HEX>` encoding.
Platform metadata, including `drv` and feature flags, may be absent under size
pressure. The suffix may then be shortened or omitted. Parsers must support
both the parenthesized format and the identifier-only or identifier-plus-suffix
fallback, and must not assume every positional value is present.
