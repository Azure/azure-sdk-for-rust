// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Date and time parsing and formatting functions.

pub use time::{error::ComponentRange, Duration, OffsetDateTime};
use time::{
    format_description::{well_known::Rfc3339, FormatItem},
    macros::format_description,
    PrimitiveDateTime,
};
use typespec::error::{ErrorKind, ResultExt};

// Serde modules.
pub use time::serde::rfc3339;
pub use time::serde::timestamp;

// RFC 3339 vs ISO 8601: <https://ijmacd.github.io/rfc3339-iso8601/>
pub mod iso8601;
pub mod rfc7231;
pub mod unix_time;

pub use unix_time::parse_unix_time;

/// RFC 3339: Date and Time on the Internet: Timestamps.
///
/// <https://www.rfc-editor.org/rfc/rfc3339>
///
/// In [TypeSpec](https://aka.ms/typespec) properties are specified as `utcDateTime` or `offsetDateTime`.
/// In OpenAPI 2.0 specifications properties are specified as `"format": "date-time"`.
///
/// Example string: `1985-04-12T23:20:50.52Z`.
pub fn parse_rfc3339(s: &str) -> crate::Result<OffsetDateTime> {
    OffsetDateTime::parse(s, &Rfc3339).with_context(ErrorKind::DataConversion, || {
        format!("unable to parse rfc3339 date '{s}")
    })
}

/// RFC 3339: Date and Time on the Internet: Timestamps.
///
/// <https://www.rfc-editor.org/rfc/rfc3339>
///
/// In [TypeSpec](https://aka.ms/typespec) properties are specified as `utcDateTime` or `offsetDateTime`.
/// In OpenAPI 2.0 specifications properties are specified as `"format": "date-time"`.
///
/// Example string: `1985-04-12T23:20:50.52Z`.
pub fn to_rfc3339(date: &OffsetDateTime) -> String {
    // known format does not panic
    date.format(&Rfc3339).unwrap()
}

/// RFC 7231: Requirements for Internet Hosts - Application and Support.
///
/// <https://datatracker.ietf.org/doc/html/rfc7231#section-7.1.1.1>
///
/// In [TypeSpec](https://aka.ms/typespec) headers are specified as `utcDateTime`.
/// In REST API specifications headers are specified as `"format": "date-time-rfc1123"`.
///
/// This format is also the preferred HTTP date-based header format.
/// * <https://datatracker.ietf.org/doc/html/rfc7231#section-7.1.1.2>
/// * <https://datatracker.ietf.org/doc/html/rfc7232>
///
/// Example string: `Sun, 06 Nov 1994 08:49:37 GMT`.
pub fn parse_rfc7231(s: &str) -> crate::Result<OffsetDateTime> {
    Ok(PrimitiveDateTime::parse(s, RFC7231_FORMAT)
        .with_context(ErrorKind::DataConversion, || {
            format!("unable to parse rfc7231 date '{s}")
        })?
        .assume_utc())
}

const RFC7231_FORMAT: &[FormatItem] = format_description!(
    "[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] GMT"
);

/// RFC 7231: Requirements for Internet Hosts - Application and Support.
///
/// <https://datatracker.ietf.org/doc/html/rfc7231#section-7.1.1.1>
///
/// In [TypeSpec](https://aka.ms/typespec) headers are specified as `utcDateTime`.
/// In REST API specifications headers are specified as `"format": "date-time-rfc1123"`.
///
/// This format is also the preferred HTTP date-based header format.
/// * <https://datatracker.ietf.org/doc/html/rfc7231#section-7.1.1.2>
/// * <https://datatracker.ietf.org/doc/html/rfc7232>
///
/// Example string: `Sun, 06 Nov 1994 08:49:37 GMT`.
pub fn to_rfc7231(date: &OffsetDateTime) -> String {
    // known format does not panic
    date.format(&RFC7231_FORMAT).unwrap()
}

/// Similar to RFC 7231, but includes milliseconds.
///
/// <https://learn.microsoft.com/rest/api/cosmos-db/patch-a-document>
///
/// x-ms-last-state-change-utc: Fri, 25 Mar 2016 21:27:20.035 GMT
pub fn parse_last_state_change(s: &str) -> crate::Result<OffsetDateTime> {
    Ok(PrimitiveDateTime::parse(s, LAST_STATE_CHANGE_FORMAT)
        .with_context(ErrorKind::DataConversion, || {
            format!("unable to parse last state change date '{s}")
        })?
        .assume_utc())
}

// cspell:ignore subsecond
const LAST_STATE_CHANGE_FORMAT: &[FormatItem] = format_description!(
    "[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second].[subsecond digits:3] GMT"
);

/// Similar to preferred HTTP date format, but includes milliseconds.
///
/// <https://learn.microsoft.com/rest/api/cosmos-db/patch-a-document>
///
/// x-ms-last-state-change-utc: Fri, 25 Mar 2016 21:27:20.035 GMT
pub fn to_last_state_change(date: &OffsetDateTime) -> String {
    // known format does not panic
    date.format(LAST_STATE_CHANGE_FORMAT).unwrap()
}

/// Create a duration from the number of minutes.
pub fn duration_from_minutes(minutes: u64) -> Duration {
    Duration::minutes(i64::try_from(minutes).unwrap())
}

/// Create a duration from the number of hours.
pub fn duration_from_hours(hours: u64) -> Duration {
    Duration::hours(i64::try_from(hours).unwrap())
}

/// Create a duration from the number of days.
pub fn duration_from_days(days: u64) -> Duration {
    Duration::days(i64::try_from(days).unwrap())
}

/// Get the difference between two dates.
pub fn diff(first: OffsetDateTime, second: OffsetDateTime) -> Duration {
    (first - second).abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::from_json;
    use serde::{Deserialize, Serialize};
    use time::macros::datetime;

    #[derive(Serialize, Deserialize)]
    struct ExampleState {
        #[serde(with = "crate::time::rfc3339")]
        created_time: time::OffsetDateTime,

        // Note: Must specify "default" in serde options when using "with"
        #[serde(default, with = "crate::time::rfc3339::option")]
        deleted_time: Option<time::OffsetDateTime>,
    }

    #[test]
    fn roundtrip_rfc3339() -> crate::Result<()> {
        let s = "2019-10-12T07:20:50.52Z";
        let dt = parse_rfc3339(s)?;
        assert_eq!(s, to_rfc3339(&dt));
        Ok(())
    }

    #[test]
    fn roundtrip_rfc3339_offset() -> crate::Result<()> {
        let s = "2019-10-12T00:20:50.52-08:00";
        let dt = parse_rfc3339(s)?;
        assert!(!dt.offset().is_utc());
        assert_eq!(s, to_rfc3339(&dt));
        Ok(())
    }

    #[test]
    fn device_update_dates() -> crate::Result<()> {
        let created = parse_rfc3339("1999-09-10T21:59:22Z")?;
        let last_action = parse_rfc3339("1999-09-10T03:05:07.3845533+01:00")?;
        assert_eq!(created, datetime!(1999-09-10 21:59:22 UTC));
        assert_eq!(last_action, datetime!(1999-09-10 03:05:07.3845533 +01));
        Ok(())
    }

    #[test]
    fn test_to_rfc7231() -> crate::Result<()> {
        let dt = datetime!(1994-11-06 08:49:37 UTC);
        assert_eq!("Sun, 06 Nov 1994 08:49:37 GMT", to_rfc7231(&dt));
        Ok(())
    }

    #[test]
    fn test_parse_rfc7231() -> crate::Result<()> {
        let dt = datetime!(1994-11-06 08:49:37 UTC);
        assert_eq!(parse_rfc7231("Sun, 06 Nov 1994 08:49:37 GMT")?, dt);
        Ok(())
    }

    #[test]
    fn parse_rfc7231_offset() {
        assert!(parse_rfc7231("Sun, 06 Nov 1994 00:49:37 PST").is_err());
    }

    #[test]
    fn roundtrip_rfc7231() -> crate::Result<()> {
        let s = "Sat, 12 Oct 2019 07:20:50 GMT";
        let dt = parse_rfc7231(s)?;
        assert_eq!(s, to_rfc7231(&dt));
        Ok(())
    }

    #[test]
    #[ignore = "https://github.com/Azure/azure-sdk-for-rust/issues/1982"]
    fn roundtrip_rfc7231_offset() -> crate::Result<()> {
        let s = "Sat, 12 Oct 2019 07:20:50 PST";
        let dt = parse_rfc7231(s)?;
        assert!(!dt.offset().is_utc());
        assert_eq!(s, to_rfc7231(&dt));
        Ok(())
    }

    #[test]
    fn test_parse_last_state_change() -> crate::Result<()> {
        assert_eq!(
            datetime!(2020-01-15 23:39:44.369 UTC),
            parse_last_state_change("Wed, 15 Jan 2020 23:39:44.369 GMT")?
        );
        Ok(())
    }

    #[test]
    fn list_blob_creation_time() -> crate::Result<()> {
        let creation_time = "Thu, 01 Jul 2021 10:45:02 GMT";
        assert_eq!(
            datetime!(2021-07-01 10:45:02 UTC),
            parse_rfc7231(creation_time)?
        );
        Ok(())
    }

    #[test]
    fn serde_rfc3339_none_optional() -> crate::Result<()> {
        let json_state = r#"{
            "created_time": "2021-07-01T10:45:02Z"
        }"#;

        let state: ExampleState = from_json(json_state)?;

        assert_eq!(parse_rfc3339("2021-07-01T10:45:02Z")?, state.created_time);
        assert_eq!(state.deleted_time, None);

        Ok(())
    }

    #[test]
    fn serde_rfc3339_some_optional() -> crate::Result<()> {
        let json_state = r#"{
            "created_time": "2021-07-01T10:45:02Z",
            "deleted_time": "2022-03-28T11:05:31Z"
        }"#;

        let state: ExampleState = from_json(json_state)?;

        assert_eq!(parse_rfc3339("2021-07-01T10:45:02Z")?, state.created_time);
        assert_eq!(
            state.deleted_time,
            Some(parse_rfc3339("2022-03-28T11:05:31Z")?)
        );

        Ok(())
    }
}
