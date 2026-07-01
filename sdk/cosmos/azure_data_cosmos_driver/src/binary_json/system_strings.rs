// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The Cosmos binary JSON **system-string dictionary**.
//!
//! System strings are a fixed, hardcoded dictionary of common Cosmos property
//! names and GeoJSON keywords. A 1-byte system-string type marker (in the range
//! [`SYSTEM_STRING_1BYTE_MIN`]..[`SYSTEM_STRING_1BYTE_MAX`])
//! encodes a string by **index** into this table: `index = marker - SYSTEM_STRING_1BYTE_MIN`.
//!
//! The table **must** match the service byte-for-byte and in the same order; an
//! off-by-one silently produces wrong property keys. It is transcribed verbatim
//! from the .NET reference implementation
//! `Microsoft.Azure.Cosmos/src/Json/JsonBinaryEncoding.SystemStrings.cs`
//! (`SystemStrings.Strings`).
//!
//! > **Note on size:** the design spec estimated ~128 entries, but the
//! > authoritative .NET table currently holds exactly [`SYSTEM_STRING_COUNT`]
//! > (32) entries, all addressable by a single 1-byte marker. There is no
//! > 2-byte system-string marker range today (the 2-byte range
//! > [`USER_STRING_2BYTE_MIN`](crate::binary_json::markers::USER_STRING_2BYTE_MIN)
//! > is for *user* strings). If the service later grows the table past 32
//! > entries, the encoder/decoder do not need to change to remain *correct*
//! > (the encoder never emits system strings, and the decoder simply gains
//! > more valid indices); only this constant table would be extended.

use super::markers::{SYSTEM_STRING_1BYTE_MAX, SYSTEM_STRING_1BYTE_MIN};

/// The number of entries in the system-string dictionary.
pub const SYSTEM_STRING_COUNT: usize = 32;

/// The system-string dictionary, indexed by system-string id.
///
/// Entry `i` is encoded by the 1-byte type marker
/// `SYSTEM_STRING_1BYTE_MIN + i`. The order is significant and matches the
/// service; do not sort or reorder.
pub const SYSTEM_STRINGS: [&str; SYSTEM_STRING_COUNT] = [
    "$s",                 // 0
    "$t",                 // 1
    "$v",                 // 2
    "_attachments",       // 3
    "_etag",              // 4
    "_rid",               // 5
    "_self",              // 6
    "_ts",                // 7
    "attachments/",       // 8
    "coordinates",        // 9
    "geometry",           // 10
    "GeometryCollection", // 11
    "id",                 // 12
    "url",                // 13
    "Value",              // 14
    "label",              // 15
    "LineString",         // 16
    "link",               // 17
    "MultiLineString",    // 18
    "MultiPoint",         // 19
    "MultiPolygon",       // 20
    "name",               // 21
    "Name",               // 22
    "Type",               // 23
    "Point",              // 24
    "Polygon",            // 25
    "properties",         // 26
    "type",               // 27
    "value",              // 28
    "Feature",            // 29
    "FeatureCollection",  // 30
    "_id",                // 31
];

/// Returns the system string for a dictionary `index`, or `None` if the index
/// is out of range.
///
/// This is the lookup the **decoder** performs when it reads a 1-byte
/// system-string marker.
///
/// # Examples
///
/// ```
/// use azure_data_cosmos_driver::binary_json::system_strings::system_string;
///
/// assert_eq!(system_string(12), Some("id"));
/// assert_eq!(system_string(5), Some("_rid"));
/// assert_eq!(system_string(32), None);
/// ```
pub fn system_string(index: usize) -> Option<&'static str> {
    SYSTEM_STRINGS.get(index).copied()
}

/// Returns the system string addressed by a 1-byte system-string `marker`
/// (in `SYSTEM_STRING_1BYTE_MIN..SYSTEM_STRING_1BYTE_MAX`), or `None` if the
/// marker is outside that range or addresses an index past the table.
///
/// Convenience wrapper that performs the `marker - SYSTEM_STRING_1BYTE_MIN`
/// index arithmetic for the decoder.
pub fn system_string_for_marker(marker: u8) -> Option<&'static str> {
    if (SYSTEM_STRING_1BYTE_MIN..SYSTEM_STRING_1BYTE_MAX).contains(&marker) {
        system_string(usize::from(marker - SYSTEM_STRING_1BYTE_MIN))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The table size is fixed at the documented count.
    #[test]
    fn table_has_expected_count() {
        assert_eq!(SYSTEM_STRINGS.len(), SYSTEM_STRING_COUNT);
        assert_eq!(SYSTEM_STRING_COUNT, 32);
    }

    /// Well-known property names land at their authoritative .NET indices. A
    /// failure here means the transcription drifted from the service table.
    #[test]
    fn well_known_entries_at_expected_indices() {
        assert_eq!(SYSTEM_STRINGS[0], "$s");
        assert_eq!(SYSTEM_STRINGS[3], "_attachments");
        assert_eq!(SYSTEM_STRINGS[4], "_etag");
        assert_eq!(SYSTEM_STRINGS[5], "_rid");
        assert_eq!(SYSTEM_STRINGS[6], "_self");
        assert_eq!(SYSTEM_STRINGS[7], "_ts");
        assert_eq!(SYSTEM_STRINGS[12], "id");
        assert_eq!(SYSTEM_STRINGS[31], "_id");
    }

    /// Cross-check every entry's byte length against the buckets the .NET
    /// `GetSystemStringIdLength{N}` reverse-lookup functions sort them into.
    /// This guards the transcription independently of the forward order.
    #[test]
    fn entry_lengths_match_dotnet_buckets() {
        // (index, expected utf-8 byte length) for every entry, taken from the
        // .NET length-bucketed lookup functions.
        let expected: [(usize, usize); SYSTEM_STRING_COUNT] = [
            (0, 2),   // $s
            (1, 2),   // $t
            (2, 2),   // $v
            (3, 12),  // _attachments
            (4, 5),   // _etag
            (5, 4),   // _rid
            (6, 5),   // _self
            (7, 3),   // _ts
            (8, 12),  // attachments/
            (9, 11),  // coordinates
            (10, 8),  // geometry
            (11, 18), // GeometryCollection
            (12, 2),  // id
            (13, 3),  // url
            (14, 5),  // Value
            (15, 5),  // label
            (16, 10), // LineString
            (17, 4),  // link
            (18, 15), // MultiLineString
            (19, 10), // MultiPoint
            (20, 12), // MultiPolygon
            (21, 4),  // name
            (22, 4),  // Name
            (23, 4),  // Type
            (24, 5),  // Point
            (25, 7),  // Polygon
            (26, 10), // properties
            (27, 4),  // type
            (28, 5),  // value
            (29, 7),  // Feature
            (30, 17), // FeatureCollection
            (31, 3),  // _id
        ];
        for (index, len) in expected {
            assert_eq!(
                SYSTEM_STRINGS[index].len(),
                len,
                "entry {index} ({:?}) has unexpected length",
                SYSTEM_STRINGS[index],
            );
        }
    }

    /// No two entries are identical (the dictionary is a set).
    #[test]
    fn entries_are_unique() {
        for i in 0..SYSTEM_STRINGS.len() {
            for j in (i + 1)..SYSTEM_STRINGS.len() {
                assert_ne!(
                    SYSTEM_STRINGS[i], SYSTEM_STRINGS[j],
                    "duplicate system string at indices {i} and {j}",
                );
            }
        }
    }

    #[test]
    fn system_string_lookup_bounds() {
        assert_eq!(system_string(0), Some("$s"));
        assert_eq!(system_string(31), Some("_id"));
        assert_eq!(system_string(32), None);
        assert_eq!(system_string(usize::MAX), None);
    }

    /// The marker convenience wrapper maps the 1-byte system-string range onto
    /// the table and rejects markers outside it.
    #[test]
    fn marker_lookup_maps_range() {
        assert_eq!(
            system_string_for_marker(SYSTEM_STRING_1BYTE_MIN),
            Some("$s")
        );
        // Last valid system-string marker addresses the last entry.
        assert_eq!(
            system_string_for_marker(SYSTEM_STRING_1BYTE_MAX - 1),
            Some("_id"),
        );
        // The id marker: SYSTEM_STRING_1BYTE_MIN + 12.
        assert_eq!(
            system_string_for_marker(SYSTEM_STRING_1BYTE_MIN + 12),
            Some("id"),
        );
        // Out of range: a user-string marker is not a system string.
        assert_eq!(system_string_for_marker(SYSTEM_STRING_1BYTE_MAX), None);
        assert_eq!(system_string_for_marker(0x00), None);
    }
}
