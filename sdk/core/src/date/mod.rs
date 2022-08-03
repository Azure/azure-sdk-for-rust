//! Azure date and time parsing and formatting

// RFC 3339 vs ISO 8601
// https://ijmacd.github.io/rfc3339-iso8601/

use crate::error::{ErrorKind, ResultExt};
use std::time::Duration;
use time::{
    format_description::{well_known::Rfc3339, FormatItem},
    macros::format_description,
    OffsetDateTime, PrimitiveDateTime, UtcOffset,
};

// Serde modules
pub use time::serde::rfc3339;
pub use time::serde::timestamp;
pub mod rfc1123;

/// RFC 3339: Date and Time on the Internet: Timestamps
///
/// https://www.rfc-editor.org/rfc/rfc3339
///
/// In Azure REST API specifications it is specified as `"format": "date-time"`.
///
/// 1985-04-12T23:20:50.52Z
pub fn parse_rfc3339(s: &str) -> crate::Result<OffsetDateTime> {
    OffsetDateTime::parse(s, &Rfc3339).with_context(ErrorKind::DataConversion, || {
        format!("unable to parse rfc3339 date '{s}")
    })
}

/// RFC 3339: Date and Time on the Internet: Timestamps
///
/// https://www.rfc-editor.org/rfc/rfc3339
///
/// In Azure REST API specifications it is specified as `"format": "date-time"`.
///
/// 1985-04-12T23:20:50.52Z
pub fn to_rfc3339(date: &OffsetDateTime) -> String {
    // known format does not panic
    date.format(&Rfc3339).unwrap()
}

/// RFC 1123: Requirements for Internet Hosts - Application and Support
///
/// https://www.rfc-editor.org/rfc/rfc1123
///
/// In Azure REST API specifications it is specified as `"format": "date-time-rfc1123"`.
///
/// In .NET it is the `rfc1123pattern`.
/// https://docs.microsoft.com/dotnet/api/system.globalization.datetimeformatinfo.rfc1123pattern
///
/// This format is also the preferred HTTP date format.
/// https://httpwg.org/specs/rfc9110.html#http.date
///
/// Sun, 06 Nov 1994 08:49:37 GMT
pub fn parse_rfc1123(s: &str) -> crate::Result<OffsetDateTime> {
    Ok(PrimitiveDateTime::parse(s, RFC1123_FORMAT)
        .with_context(ErrorKind::DataConversion, || {
            format!("unable to parse rfc1123 date '{s}")
        })?
        .assume_utc())
}

const RFC1123_FORMAT: &[FormatItem] = format_description!(
    "[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] GMT"
);

/// RFC 1123: Requirements for Internet Hosts - Application and Support
///
/// https://www.rfc-editor.org/rfc/rfc1123
///
/// In Azure REST API specifications it is specified as `"format": "date-time-rfc1123"`.
///
/// In .NET it is the `rfc1123pattern`.
/// https://docs.microsoft.com/dotnet/api/system.globalization.datetimeformatinfo.rfc1123pattern
///
/// This format is also the preferred HTTP date format.
/// https://httpwg.org/specs/rfc9110.html#http.date
///
/// Sun, 06 Nov 1994 08:49:37 GMT
pub fn to_rfc1123(date: &OffsetDateTime) -> String {
    date.to_offset(UtcOffset::UTC);
    // known format does not panic
    date.format(&RFC1123_FORMAT).unwrap()
}

/// Similar to RFC 1123, but includes milliseconds.
///
/// https://docs.microsoft.com/rest/api/cosmos-db/patch-a-document
///
/// x-ms-last-state-change-utc: Fri, 25 Mar 2016 21:27:20.035 GMT
pub fn parse_last_state_change(s: &str) -> crate::Result<OffsetDateTime> {
    Ok(PrimitiveDateTime::parse(s, LAST_STATE_CHANGE_FORMAT)
        .with_context(ErrorKind::DataConversion, || {
            format!("unable to parse last state change date '{s}")
        })?
        .assume_utc())
}

const LAST_STATE_CHANGE_FORMAT: &[FormatItem] = format_description!(
    "[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second].[subsecond digits:3] GMT"
);

/// Similar to preferred HTTP date format, but includes milliseconds
///
/// https://docs.microsoft.com/rest/api/cosmos-db/patch-a-document
///
/// x-ms-last-state-change-utc: Fri, 25 Mar 2016 21:27:20.035 GMT
pub fn to_last_state_change(date: &OffsetDateTime) -> String {
    date.to_offset(UtcOffset::UTC);
    // known format does not panic
    date.format(LAST_STATE_CHANGE_FORMAT).unwrap()
}

/// Assumes the local offset. Default to UTC if unable to get local offset.
pub fn assume_local(date: &PrimitiveDateTime) -> OffsetDateTime {
    date.assume_offset(UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC))
}

// Create a duration from the number of minutes.
pub fn duration_from_minutes(minutes: u64) -> Duration {
    Duration::from_secs(minutes * 60)
}

// Create a duration from the number of hours.
pub fn duration_from_hours(hours: u64) -> Duration {
    Duration::from_secs(hours * 3_600)
}

// Create a duration from the number of days.
pub fn duration_from_days(days: u64) -> Duration {
    Duration::from_secs(days * 86_400)
}

/// Get the difference between two dates.
pub fn diff(first: OffsetDateTime, second: OffsetDateTime) -> Duration {
    (first - second).unsigned_abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn test_roundtrip_rfc3339() -> crate::Result<()> {
        let s = "2019-10-12T07:20:50.52Z";
        let dt = parse_rfc3339(s)?;
        assert_eq!(s, to_rfc3339(&dt));
        Ok(())
    }

    #[test]
    fn test_device_update_dates() -> crate::Result<()> {
        let created = parse_rfc3339("1999-09-10T21:59:22Z")?;
        let last_action = parse_rfc3339("1999-09-10T03:05:07.3845533+01:00")?;
        assert_eq!(created, datetime!(1999-09-10 21:59:22 UTC));
        assert_eq!(last_action, datetime!(1999-09-10 03:05:07.3845533 +01));
        Ok(())
    }

    #[test]
    fn test_to_rfc1123() -> crate::Result<()> {
        let dt = datetime!(1994-11-06 08:49:37 UTC);
        assert_eq!("Sun, 06 Nov 1994 08:49:37 GMT", to_rfc1123(&dt));
        Ok(())
    }

    #[test]
    fn test_parse_rfc1123() -> crate::Result<()> {
        let dt = datetime!(1994-11-06 08:49:37 UTC);
        assert_eq!(parse_rfc1123("Sun, 06 Nov 1994 08:49:37 GMT")?, dt);
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
    fn test_list_blob_creation_time() -> crate::Result<()> {
        let creation_time = "Thu, 01 Jul 2021 10:45:02 GMT";
        assert_eq!(
            datetime!(2021-07-01 10:45:02 UTC),
            parse_rfc1123(creation_time)?
        );
        Ok(())
    }
}
