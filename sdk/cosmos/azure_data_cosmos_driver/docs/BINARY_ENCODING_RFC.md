# Cosmos Binary JSON Encoding — Wire Format Specification

**Status:** Draft · **Version:** 0.1 · **Audience:** SDK / codec implementers (Rust, .NET, C++, Java, Go, Python)

> This document is a **normative, self-contained** description of the Cosmos
> Binary JSON wire format. A conforming encoder/decoder can be implemented from
> this document alone, without reference to any SDK source. It is modeled on the
> style of [RFC 8949 (CBOR)](https://datatracker.ietf.org/doc/html/rfc8949) and
> the [Protocol Buffers encoding guide](https://protobuf.dev/programming-guides/encoding/).
>
> **Provenance.** The wire constants are transcribed from the .NET reference
> implementation (`Microsoft.Azure.Cosmos/src/Json/JsonBinaryEncoding.TypeMarker.cs`)
> and cross-checked against the Rust codec (`azure_data_cosmos_driver::binary_json`).
> Details that could not be confirmed from the Rust implementation alone are
> tagged **`[CROSS-VERIFY: .NET/C++]`** and MUST be validated against the .NET
> and C++ sources before this draft is promoted to a stable version.

---

## 1. Introduction

### 1.1 Purpose

Cosmos Binary JSON is a compact, self-describing binary serialization of the
JSON data model used by Azure Cosmos DB for item request and response bodies. It
is **information-preserving with respect to the JSON value model** (null,
boolean, number, string, array, object) while being smaller and faster to
parse than UTF-8 JSON text. The service and every language SDK MUST agree on
this format byte-for-byte.

### 1.2 Scope

This specification defines:

- the byte-level layout of every value kind (§3–§6),
- the **canonical** encoding a conforming encoder emits when multiple encodings
  are valid (§7),
- decoder conformance requirements, including bounds and resource limits (§8),
- security considerations for decoding untrusted input (§9).

It does **not** define: transport framing, HTTP/RNTBD negotiation headers,
per-account dictionary (user-string) construction policy, or the query wire
protocol. Those are layered above this format.

### 1.3 Requirements language

The key words **MUST**, **MUST NOT**, **SHOULD**, **SHOULD NOT**, and **MAY** are
to be interpreted as described in [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

---

## 2. Notation and terminology

- **byte** — an 8-bit octet, written as two hexadecimal digits, e.g. `C8`.
- **marker** (or **type marker**) — the single leading byte that selects how the
  following bytes are interpreted. Every encoded value begins with exactly one
  marker (except literal small integers, where the marker byte *is* the value —
  see §4.1).
- **spaced-hex** — the human-reviewable notation for a byte sequence used
  throughout this document and in the shared test corpus, e.g. `80 CC 00 00 00
  00 00 00 0C 40`.
- **preamble** — the single byte `0x80` that MUST prefix a complete buffer (§3.1).
- **little-endian (LE)** — multi-byte integers and floats are serialized
  least-significant-byte first. This is **normative and independent of host
  architecture**: an encoder on a big-endian host MUST still emit LE.
- **value model** — the abstract JSON value: `null | bool | number | string |
  array | object`. Numbers are IEEE-754 doubles or integers in the ranges
  encodable by the number markers in §4.

---

## 3. Structure of an encoded value

### 3.1 Buffer preamble and auto-detection

A complete Cosmos Binary JSON buffer MUST begin with the **preamble byte**
`0x80`, followed by exactly one encoded value:

```
buffer = 0x80  value
```

Because `0x80` is also the marker for a zero-length encoded string (§4.4), the
preamble is **always consumed first**; a top-level empty string is therefore
`80 80` (preamble + zero-length-string marker).

Consumers distinguish binary from UTF-8 JSON text by inspecting the **first
byte**: a payload whose first byte is `0x80` is Cosmos Binary JSON; any other
first byte (`{`, `[`, `"`, digit, `t`, `f`, `n`, whitespace, …) is UTF-8 text.
This is the `is_binary` predicate.

> **Note.** UTF-8 text can never legitimately begin with `0x80` (a continuation
> byte), so the discriminator is unambiguous.

### 3.2 Marker byte taxonomy

The 256 marker values are partitioned into contiguous ranges. Ranges are written
`[MIN, MAX)` — MIN inclusive, MAX exclusive.

| Range         | Meaning                                                        | Section |
| ------------- | ------------------------------------------------------------- | ------- |
| `[0x00,0x20)` | Literal small integer (`value == marker`, range 0–31)         | §4.1    |
| `[0x20,0x40)` | 1-byte **system** string (index into fixed dictionary)        | §4.4.3  |
| `[0x40,0x60)` | 1-byte **user** string (index into per-buffer dictionary)     | §4.4.4  |
| `[0x60,0x68)` | 2-byte user string                                            | §4.4.4  |
| `[0x68,0x80)` | base64 / GUID-string / compressed-string forms                | §4.4.5–6|
| `[0x80,0xC0)` | Encoded-length string (`len == marker & 0x7F`, 0–63)          | §4.4.1  |
| `[0xC0,0xC8)` | Length-prefixed strings, reference strings, `NumberUInt64`     | §4.4.1, §6, §4.2 |
| `[0xC8,0xD0)` | Fixed-width numbers (`UInt8`,`Int16/32/64`,`Double`,`Float*`)  | §4.2–3  |
| `[0xD0,0xE0)` | null, bool, GUID value, extended sized ints, binary blobs     | §4.1, §4.2, §4.7 |
| `[0xE0,0xE8)` | Arrays                                                         | §5.1    |
| `[0xE8,0xF0)` | Objects                                                        | §5.2    |
| `[0xF0,0xF8)` | Uniform (typed) number arrays                                 | §5.3    |
| `[0xF8,0xFF]` | Reserved; `0xFF` == Invalid                                    | §8      |

The complete marker constant table is given in Appendix C.

---

## 4. Scalars

### 4.1 Null, boolean, and literal small integers

| Value           | Marker  | Sample (with preamble) |
| --------------- | ------- | ---------------------- |
| `null`          | `D0`    | `80 D0`                |
| `false`         | `D1`    | `80 D1`                |
| `true`          | `D2`    | `80 D2`                |
| integer `0`–`31`| `00`–`1F` | `0` → `80 00`; `31` → `80 1F` |

For an integer `n` in `[0, 31]`, the marker byte itself is the value: the encoded
form is the single byte `n`. This is the most compact integer encoding and is the
canonical form for that range (§7).

### 4.2 Integers

Two families of integer markers exist, distinguished only by their historical
range. Both are LE.

| Marker            | Byte | Width | Signedness | Sample                              |
| ----------------- | ---- | ----- | ---------- | ----------------------------------- |
| `NumberUInt8`     | `C8` | 1     | unsigned   | `200` → `80 C8 C8`                   |
| `NumberInt16`     | `C9` | 2     | signed     | `-1000` → `80 C9 18 FC`             |
| `NumberInt32`     | `CA` | 4     | signed     | `70000` → `80 CA 70 11 01 00`       |
| `NumberInt64`     | `CB` | 8     | signed     | `-5000000000` → `80 CB 00 0E FA D5 FE FF FF FF` |
| `NumberUInt64`    | `C7` | 8     | unsigned   | `18446744073709551614` → `80 C7 FE FF FF FF FF FF FF FF` |
| `UInt8` (ext.)    | `D7` | 1     | unsigned   | `[CROSS-VERIFY: .NET/C++]`          |
| `Int8` (ext.)     | `D8` | 1     | signed     | `-5` → `80 D8 FB`                   |
| `Int16` (ext.)    | `D9` | 2     | signed     | `-1000` → `80 D9 18 FC`             |
| `Int32` (ext.)    | `DA` | 4     | signed     | `-70000` → `80 DA 90 EE FE FF`      |
| `Int64` (ext.)    | `DB` | 8     | signed     | `-5000000000` → `80 DB 00 0E FA D5 FE FF FF FF` |
| `UInt32` (ext.)   | `DC` | 4     | unsigned   | `4294967294` → `80 DC FE FF FF FF`  |

Decoders MUST accept **both** families. Encoders emit the canonical family per
§7. The `NumberUInt64` marker (`C7`) is the only encoding able to carry
unsigned 64-bit values above `i64::MAX`.

### 4.3 Floating-point numbers

| Marker         | Byte | Width | Encoding                     | Sample                          |
| -------------- | ---- | ----- | ---------------------------- | ------------------------------- |
| `NumberDouble` | `CC` | 8     | IEEE-754 binary64 (LE)       | `3.5` → `80 CC 00 00 00 00 00 00 0C 40` |
| `Float32`      | `CD` | 4     | IEEE-754 binary32 (LE)       | `1.5` → `80 CD 00 00 C0 3F`     |
| `Float64`      | `CE` | 8     | IEEE-754 binary64 (LE)       | `-2.25` → `80 CE 00 00 00 00 00 00 02 C0` |
| `Float16`      | `CF` | 2     | IEEE-754 binary16 (LE)       | `[CROSS-VERIFY: .NET/C++]`      |

`NumberDouble` (`CC`) is the canonical JSON-number float form (§7).

**Non-finite values.** JSON has no representation for `NaN` or `±Infinity`.
A conforming encoder MUST NOT emit a non-finite double; instead it MUST encode
`null` (`D0`), mirroring `serde_json` / JavaScript `JSON.stringify`. A conforming
decoder MUST reject a non-finite `NumberDouble`/`Float*` payload as an invalid
number. `[CROSS-VERIFY: .NET/C++]` — confirm the .NET encoder/decoder policy is
identical.

### 4.4 Strings

JSON strings are UTF-8. Several encodings exist; a decoder MUST accept all of
them, and an encoder selects the canonical one per §7.

#### 4.4.1 Length-framed strings

| Form               | Marker | Length field       | Sample                     |
| ------------------ | ------ | ------------------ | -------------------------- |
| Encoded-length     | `80`–`BF` | in marker: `len = marker & 0x7F` (0–63) | `""` → `80 80`; `"hi"` → `80 82 68 69` |
| `StrL1`            | `C0`   | u8                 | `"hello"` → `80 C0 05 68 65 6C 6C 6F` |
| `StrL2`            | `C1`   | u16 LE             | 300×`"a"` → `80 C1 2C 01 …` |
| `StrL4`            | `C2`   | u32 LE             | (large strings)            |

The string's UTF-8 bytes follow the length field verbatim.

#### 4.4.2 GUID strings

| Marker | Byte | Meaning                              | Sample |
| ------ | ---- | ------------------------------------ | ------ |
| lower  | `75` | 36-char lowercase GUID string        | `80 75 00 01 … 0F` → `"00010203-0405-0607-0809-0a0b0c0d0e0f"` |
| upper  | `76` | 36-char uppercase GUID string        | `80 76 …` → uppercase |
| quoted | `77` | double-quoted lowercase GUID string  | `80 77 …` → `"\"…\""` |

The 16 raw GUID bytes follow the marker; the decoder formats them as a hyphenated
GUID string (byte order per Appendix B). This is distinct from a **GUID value**
(§4.7).

#### 4.4.3 System strings

Markers `[0x20, 0x40)` encode a string by **index into a fixed, well-known
dictionary** of common Cosmos property names (`id`, `_rid`, `_etag`, …). The
encoded form is a single byte; `marker - 0x20` is the dictionary index.

Example: `"id"` has system index `0x0C`, so `"id"` → `80 2C` (`0x20 + 0x0C`).

The full system-string table is normative and given in Appendix D.
`[CROSS-VERIFY: .NET/C++]` — the index assignments MUST match `JsonBinaryEncoding`.

#### 4.4.4 User (per-buffer dictionary) strings

Markers `[0x40, 0x60)` (1-byte) and `[0x60, 0x68)` (2-byte) encode a string by
index into a **per-buffer user-string dictionary**. The dictionary and its
construction policy are out of scope for this document (a decoder receives the
dictionary alongside the buffer, or the buffer contains no user-dictionary
references). `[CROSS-VERIFY: .NET/C++]`

#### 4.4.5 base64 strings

| Marker | Byte | Alphabet     | Length field | Sample |
| ------ | ---- | ------------ | ------------ | ------ |
| `Base64Len1`     | `71` | standard     | u8  | `"Zm9v"` → `80 71 01 00 66 6F 6F` |
| `Base64Len2`     | `72` | standard     | u16 | `"Zm9vYmFy"` → `80 72 02 00 00 66 6F 6F 62 61 72` |
| `Base64UrlLen1`  | `73` | URL-safe     | u8  | `"-__-"` → `80 73 01 00 FB FF FE` |
| `Base64UrlLen2`  | `74` | URL-safe     | u16 | (as above, 2-byte length) |

The payload is the **decoded** bytes; the decoder re-encodes them to a base64
string using the marker's alphabet. Padding handling (`=`) and the "omitted
padding" length-field convention are illustrated in Appendix B.
`[CROSS-VERIFY: .NET/C++]` for the exact padding/length-offset encoding.

#### 4.4.6 Compressed strings

Restricted-alphabet strings are bit-packed relative to a base character:

| Marker | Byte | Packing                         | Sample |
| ------ | ---- | ------------------------------- | ------ |
| lower-hex | `78` | 4-bit hex digits (lowercase) | `"1a2b"` → `80 78 04 A1 B2` |
| upper-hex | `79` | 4-bit hex digits (uppercase) | `"1A2B"` → `80 79 04 A1 B2` |
| date-time | `7A` | 4-bit date-time charset      | `"2024-01"` → `80 7A 07 13 53 1C 02` |
| packed-4bit | `7B` | 4 bits/char + base         | `"0123"` → `80 7B 04 30 10 32` |
| packed-5bit | `7C` | 5 bits/char + base         | `"abc"` → `80 7C 03 61 20 08` |
| packed-6bit | `7D` | 6 bits/char + base         | `"abcd"` → `80 7D 04 61 40 20 0C` |
| packed-7bit-L1 | `7E` | 7 bits/char, u8 length  | `"Hi"` → `80 7E 02 C8 34` |
| packed-7bit-L2 | `7F` | 7 bits/char, u16 length | `"Hi"` → `80 7F 02 00 C8 34` |

The byte immediately after the marker is the **character count** (for L1/L2 forms
a 1-/2-byte length), followed by the packed bits. Appendix B gives a worked
unpacking example. `[CROSS-VERIFY: .NET/C++]` for the exact base/charset tables.

### 4.7 GUID value and binary blobs

| Marker | Byte | Meaning                          | Sample |
| ------ | ---- | -------------------------------- | ------ |
| `Guid` | `D3` | raw 16-byte GUID **value**       | `80 D3 00 01 … 0F` → `"03020100-0504-0706-0809-0a0b0c0d0e0f"` |
| `Binary1` | `DD` | blob, u8 length prefix        | `0xDEADBEEF` → `80 DD 04 DE AD BE EF` → `"3q2+7w=="` |
| `Binary2` | `DE` | blob, u16 length prefix       | `80 DE 03 00 01 02 03` → `"AQID"` |
| `Binary4` | `DF` | blob, u32 length prefix       | (large blobs) |

A **binary blob** decodes to a **standard base64 string** in the JSON value
model. Note the byte order of a GUID *value* (`D3`) differs from a GUID *string*
(§4.4.2) — see Appendix B. `[CROSS-VERIFY: .NET/C++]`

---

## 5. Containers

### 5.1 Arrays

| Marker  | Byte | Framing                                  | Sample |
| ------- | ---- | ---------------------------------------- | ------ |
| `Arr0`  | `E0` | empty                                    | `[]` → `80 E0` |
| `Arr1`  | `E1` | exactly one element, no length/count     | `[true]` → `80 E1 D2` |
| `ArrL1` | `E2` | u8 **byte-length** prefix                | `[0,1,null]` → `80 E2 03 00 01 D0` |
| `ArrL2` | `E3` | u16 byte-length                          |        |
| `ArrL4` | `E4` | u32 byte-length                          |        |
| `ArrLC1`| `E5` | u8 byte-length **+** u8 item-count       | `[0,1,null]` → `80 E5 03 03 00 01 D0` |
| `ArrLC2`| `E6` | u16 byte-length + u16 count              |        |
| `ArrLC4`| `E7` | u32 byte-length + u32 count              |        |

For `L*` forms, the length is the **byte length of the element region** (not the
element count); the decoder reads elements until it has consumed exactly that
many bytes. For `LC*` forms, both the byte length **and** the element count are
given, and a conforming decoder MUST verify that reading `count` elements
consumes exactly the declared byte length (§8).

### 5.2 Objects

| Marker  | Byte | Framing                             | Sample |
| ------- | ---- | ----------------------------------- | ------ |
| `Obj0`  | `E8` | empty                               | `{}` → `80 E8` |
| `Obj1`  | `E9` | exactly one name/value pair         | `{"id":true}` → `80 E9 2C D2` |
| `ObjL1` | `EA` | u8 byte-length                      | `{"id":0,"type":1}` → `80 EA 04 2C 00 3B 01` |
| `ObjL2` | `EB` | u16 byte-length                     |        |
| `ObjL4` | `EC` | u32 byte-length                     |        |
| `ObjLC1`| `ED` | u8 byte-length + u8 pair-count      | `{"id":0,"type":1}` → `80 ED 04 02 2C 00 3B 01` |
| `ObjLC2`| `EE` | u16 byte-length + u16 count         |        |
| `ObjLC4`| `EF` | u32 byte-length + u32 count         |        |

Members are encoded as **name, value, name, value, …**. Each **name** is itself
an encoded string (any string form, including system strings — note `2C` = system
`"id"` in the samples). Member ordering is preserved as encoded.

### 5.3 Uniform (typed) number arrays

A homogeneous array of numbers of a single width is encoded compactly by writing
the item type marker once, then the raw item bytes with no per-item markers.

| Marker      | Byte | Layout                                                    | Sample |
| ----------- | ---- | -------------------------------------------------------- | ------ |
| `ArrNumC1`  | `F0` | `itemMarker`, u8 count, then `count` bare numbers         | `[1,2,3]` (Int32) → `80 F0 DA 03 01 00 00 00 02 00 00 00 03 00 00 00` |
| `ArrNumC2`  | `F1` | `itemMarker`, u16 count, then bare numbers                | `[-1,0,1000]` (Int16) → `80 F1 D9 03 00 FF FF 00 00 E8 03` |
| `ArrArrNumC1C1` | `F2` | innerMarker, itemMarker, u8 inner-count, u8 outer-count, then inner arrays | `[[1,2],[3,4]]` → `80 F2 F0 DA 02 02 01 00 00 00 …` |
| `ArrArrNumC2C2` | `F3` | as above with u16 counts                             |        |

Further examples: `[10,20,30]` (UInt8) → `80 F0 D7 03 0A 14 1E`; empty uniform
array → `80 F0 DA 00`; `[1.5,-0.25]` (Float32) → `80 F0 CD 02 00 00 C0 3F 00 00
80 BE`.

**Item markers.** The uniform-array item type MUST be one of the **extended**
number markers (`Int8`/`UInt8`/`Int16`/`Int32`/`Int64`/`UInt32`/`Float32`/
`Float64`, i.e. `D7`–`DC`, `CD`, `CE`). The self-describing `Number*` markers
(`C7`–`CC`) MUST NOT appear as a uniform-array item type and a conforming decoder
MUST reject them there. `[CROSS-VERIFY: .NET/C++]`

---

## 6. Reference strings

A string that already appeared earlier in the same buffer MAY be encoded as a
**back-reference** to its byte offset, saving space for repeated keys/values.

| Marker  | Byte | Offset field |
| ------- | ---- | ------------ |
| `StrR1` | `C3` | u8 offset    |
| `StrR2` | `C4` | u16 offset   |
| `StrR3` | `C5` | u24 offset (3 bytes LE) |
| `StrR4` | `C6` | u32 offset   |

The offset is an **absolute byte offset into the buffer**, measured in the same
frame as the preamble (the preamble is offset `0`). The referenced offset MUST
hold a non-reference string; reference-to-reference chains are prohibited, which
makes cycles impossible and bounds resolution without recursion.

**Decoder resource bound (normative).** Because many references can point at one
large string, a naïve decoder can be forced into O(S²) output for a size-`S`
buffer. A conforming decoder MUST bound total materialized reference bytes by a
budget proportional to the input size (e.g. `max(16 × buffer_len, 64 KiB)`) and
fail with an invalid-length error once exceeded (§9).

---

## 7. Canonical encoding

Multiple valid encodings exist for the same value (e.g. the integer `5` can be a
literal `05`, `NumberUInt8`, `Int8`, `Int16`, …; the string `"id"` can be a
system string, an encoded-length string, `StrL1`, …). A **decoder MUST accept
all valid encodings**. An **encoder MUST be deterministic**: for a given value it
MUST emit exactly one, canonical encoding. The canonical rules are:

1. **Integers in `[0,31]`** → literal small integer (single byte).
2. **Other integers** → the narrowest fixed-width `Number*` marker that holds the
   value (`NumberUInt8` → `NumberInt16` → `NumberInt32` → `NumberInt64`; values
   above `i64::MAX` use `NumberUInt64`). `[CROSS-VERIFY: .NET/C++]` — confirm the
   .NET encoder prefers `Number*` over the extended `D7`–`DC` markers.
3. **Floating-point (non-integer) numbers** → `NumberDouble` (`CC`).
4. **Non-finite floats** → `null` (§4.3).
5. **Strings** → system string if the value is in the system dictionary;
   otherwise encoded-length (`< 64` bytes), `StrL1`, `StrL2`, or `StrL4` by
   length. Compressed/base64/GUID/reference forms are **decoder-accepted but
   encoder-optional** optimizations; a minimal conforming encoder need not emit
   them. `[CROSS-VERIFY: .NET/C++]` — enumerate which optimizations the .NET
   encoder applies by default.
6. **Containers** → `LC*` (length-and-count) framing at the narrowest width that
   fits. `[CROSS-VERIFY: .NET/C++]`

> **Why canonical encoding matters.** A snapshot / golden-vector test asserts
> `encode(value) == expected_bytes`. Without a fixed canonical rule, two
> conforming encoders could produce different (both valid) buffers and the test
> would be meaningless. §7 is the contract that makes cross-SDK byte-equality
> tests possible.

---

## 8. Decoder conformance requirements

A conforming decoder MUST:

1. **Reject a missing preamble.** The first byte MUST be `0x80` (§3.1).
2. **Reject trailing bytes.** After decoding the single top-level value, no bytes
   may remain.
3. **Bounds-check every read.** A length/offset field that would read past the
   end of the buffer MUST fail with an unexpected-EOF / invalid-length error, not
   panic or read out of bounds.
4. **Enforce a maximum nesting depth** to prevent stack exhaustion on deeply
   nested containers. The reference limit is `256`. Both the reference (`Value`)
   decoder and any streaming decoder MUST reject at the **same** depth.
5. **Validate `LC*` containers.** After reading `count` elements/members, the
   cursor MUST be exactly at the declared byte-length boundary; a mismatch MUST
   fail (this catches malformed length+count buffers that a count-only decoder
   would silently under-read).
6. **Enforce the reference-string budget** (§6).
7. **Bound uniform-array output.** For `ArrArrNum*` with a zero inner count, the
   outer count MUST NOT exceed the remaining buffer bytes (else a few bytes could
   materialize `u16::MAX` empty arrays).
8. **Reject the `Invalid` marker (`0xFF`)** and any unassigned marker with an
   invalid-marker error.
9. **Reject non-finite numbers** (§4.3).

A conforming decoder MUST NOT panic, hang, or allocate unboundedly on any input,
because it parses **untrusted** service/network bytes (§9).

---

## 9. Security considerations

Decoders process bytes that may originate from a compromised or buggy service, a
MITM, or a corrupted cache. The threats and required mitigations:

| Threat                                   | Mitigation (normative)                         |
| ---------------------------------------- | ---------------------------------------------- |
| Out-of-bounds read via a large length    | Bounds-check every read against buffer end (§8.3) |
| Stack exhaustion via deep nesting        | Max depth limit, rejected identically by all decoders (§8.4) |
| O(S²) memory via many back-references     | Per-decode reference-expansion budget (§6)     |
| Output amplification via empty uniform arrays | Bound outer count by remaining bytes (§8.7) |
| Malformed length+count under-read        | Assert cursor == declared end (§8.5)           |

Encoders SHOULD reject values that cannot be represented (e.g. integer widths
beyond `u32::MAX` container framing) rather than silently truncating.

---

## Appendix A — Golden test vectors (shared corpus)

The normative, cross-SDK test corpus lives in machine-readable form at
`azure_data_cosmos_driver/testdata/binary_json_vectors.json`. Each entry pairs a
`name`, a spaced-hex `binary` buffer (including the `0x80` preamble), and the
`json` value it decodes to. A conforming decoder MUST reproduce every `json` from
its `binary`; a conforming encoder MUST reproduce the canonical `binary` for
every `json` that is in canonical form (§7).

A representative subset (see the file for the full set):

| name | binary | json |
| ---- | ------ | ---- |
| null | `80 D0` | `null` |
| true | `80 D2` | `true` |
| literal_int_max | `80 1F` | `31` |
| uint8 | `80 C8 C8` | `200` |
| double | `80 CC 00 00 00 00 00 00 0C 40` | `3.5` |
| system_string_id | `80 2C` | `"id"` |
| str_l1_hello | `80 C0 05 68 65 6C 6C 6F` | `"hello"` |
| binary_deadbeef | `80 DD 04 DE AD BE EF` | `"3q2+7w=="` |
| uniform_int32 | `80 F0 DA 03 01 00 00 00 02 00 00 00 03 00 00 00` | `[1,2,3]` |
| object_lc1 | `80 ED 04 02 2C 00 3B 01` | `{"id":0,"type":1}` |
| nested_containers | `80 E2 05 E1 00 E9 2C 01` | `[[0],{"id":1}]` |

## Appendix B — Worked examples (byte-by-byte)

**`{"id":0,"type":1}` as `ObjLC1` (`80 ED 04 02 2C 00 3B 01`):**

```
80        preamble
ED        ObjLC1 marker  (u8 byte-length + u8 count)
04        byte-length of member region = 4
02        member (pair) count = 2
2C        name: system string 0x0C ("id")     ← 0x20 + 0x0C
00        value: literal small int 0
3B        name: system string 0x1B ("type")   ← 0x20 + 0x1B  [CROSS-VERIFY]
01        value: literal small int 1
```

**`[1,2,3]` as a uniform Int32 array (`80 F0 DA 03 01 00 00 00 …`):**

```
80        preamble
F0        ArrNumC1 (uniform number array, u8 count)
DA        item type marker = Int32
03        item count = 3
01 00 00 00   1  (Int32 LE)
02 00 00 00   2
03 00 00 00   3
```

**base64 with omitted padding (`80 71 01 FD 41` → `"QQ"`):** the length field
uses a signed offset convention to signal padding omission; see the codec's
base64 decoder for the exact rule. `[CROSS-VERIFY: .NET/C++]`

## Appendix C — Complete marker constant table

*(Authoritative values; transcribed from `markers.rs` / `JsonBinaryEncoding.TypeMarker.cs`.)*

```
Literal int      0x00–0x1F
System string 1B 0x20–0x3F
User string 1B   0x40–0x5F
User string 2B   0x60–0x67
Base64Len1       0x71   Base64Len2      0x72
Base64UrlLen1    0x73   Base64UrlLen2   0x74
GuidLower        0x75   GuidUpper       0x76   GuidQuoted 0x77
CompLowerHex     0x78   CompUpperHex    0x79   CompDateTime 0x7A
Packed4/5/6bit   0x7B/0x7C/0x7D
Packed7bitL1/L2  0x7E/0x7F
Encoded-len str  0x80–0xBF   (len = marker & 0x7F)
StrL1/L2/L4      0xC0/0xC1/0xC2
StrR1/R2/R3/R4   0xC3/0xC4/0xC5/0xC6
NumberUInt64     0xC7
NumberUInt8      0xC8   NumberInt16 0xC9   NumberInt32 0xCA   NumberInt64 0xCB
NumberDouble     0xCC   Float32 0xCD   Float64 0xCE   Float16 0xCF
Null 0xD0  False 0xD1  True 0xD2  Guid 0xD3
UInt8 0xD7  Int8 0xD8  Int16 0xD9  Int32 0xDA  Int64 0xDB  UInt32 0xDC
Binary1/2/4      0xDD/0xDE/0xDF
Arr0..ArrLC4     0xE0–0xE7
Obj0..ObjLC4     0xE8–0xEF
ArrNumC1/C2      0xF0/0xF1
ArrArrNumC1C1/C2C2 0xF2/0xF3
Invalid          0xFF
```

## Appendix D — System string dictionary

The fixed system-string table (index → string) is normative and MUST match
`JsonBinaryEncoding` across SDKs. It is defined in
`azure_data_cosmos_driver/src/binary_json/system_strings.rs`.
`[CROSS-VERIFY: .NET/C++]` — reproduce the full table here once confirmed against
the .NET source (only `id` = index `0x0C` and `type` = index `0x1B` are shown
inline in this draft's examples).

---

## Open items before promotion to stable

- Resolve every `[CROSS-VERIFY: .NET/C++]` tag against the .NET
  (`Microsoft.Azure.Cosmos/src/Json/`) and C++ reference implementations.
- Complete Appendix D (full system-string table).
- Confirm the canonical-encoding rules in §7 match the .NET encoder's actual
  output (needed for cross-SDK byte-equality snapshot tests).
- Specify the exact base64 padding/length-offset convention (Appendix B).
- Specify the compressed-string base/charset tables (§4.4.6).
