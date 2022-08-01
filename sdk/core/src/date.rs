//! Azure date and time parsing and formatting

// RFC 3339 vs ISO 8601
// https://ijmacd.github.io/rfc3339-iso8601/

use crate::error::{ErrorKind, ResultExt};
use time::{
    format_description::{
        well_known::{Rfc2822, Rfc3339},
        FormatItem,
    },
    macros::format_description,
    OffsetDateTime, PrimitiveDateTime, UtcOffset,
};

/// RFC 2822: Internet Message Format
///
/// https://www.rfc-editor.org/rfc/rfc2822#section-3.3
/// https://www.rfc-editor.org/rfc/rfc2822#section-4.3
/// A "GMT" zone is obsolete, instead "+0000" is used.
///
/// Date: Fri, 21 Nov 1997 10:01:10 -0600
pub fn parse_rfc2822(s: &str) -> crate::Result<OffsetDateTime> {
    OffsetDateTime::parse(s, &Rfc2822).with_context(ErrorKind::DataConversion, || {
        format!("unable to parse smtp date '{s}")
    })
}

/// RFC 2822: Internet Message Format
///
/// https://www.rfc-editor.org/rfc/rfc2822#section-3.3
/// https://www.rfc-editor.org/rfc/rfc2822#section-4.3
/// A "GMT" zone is obsolete, instead "+0000" is used.
///
/// Date: Fri, 21 Nov 1997 10:01:10 -0600
pub fn to_rfc2822(date: &OffsetDateTime) -> String {
    // known format does not panic
    date.format(&Rfc2822).unwrap()
}

/// RFC 3339: Date and Time on the Internet: Timestamps
///
/// https://www.rfc-editor.org/rfc/rfc3339
///
/// 1985-04-12T23:20:50.52Z
pub fn parse_rfc3339(s: &str) -> crate::Result<OffsetDateTime> {
    OffsetDateTime::parse(s, &Rfc3339).with_context(ErrorKind::DataConversion, || {
        format!("unable to parse internet date '{s}")
    })
}

/// Date format for the Internet
///
/// https://www.rfc-editor.org/rfc/rfc3339
///
/// 1985-04-12T23:20:50.52Z
pub fn to_rfc3339(date: &OffsetDateTime) -> String {
    // known format does not panic
    date.format(&Rfc3339).unwrap()
}

/// Preferred HTTP date format
///
/// https://httpwg.org/specs/rfc9110.html#http.date
///
/// In Azure REST API specification it is specified as `"format": "date-time-rfc1123"`. In .NET it is:
///
/// https://docs.microsoft.com/dotnet/api/system.globalization.datetimeformatinfo.rfc1123pattern
///
/// Sun, 06 Nov 1994 08:49:37 GMT
pub fn parse_http_date(s: &str) -> crate::Result<OffsetDateTime> {
    Ok(PrimitiveDateTime::parse(s, HTTP_DATE_FORMAT)
        .with_context(ErrorKind::DataConversion, || {
            format!("unable to parse http date '{s}")
        })?
        .assume_utc())
}

const HTTP_DATE_FORMAT: &[FormatItem] = format_description!(
    "[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] GMT"
);

/// Preferred HTTP date format
///
/// https://httpwg.org/specs/rfc9110.html#http.date
///
/// In Azure REST API specification it is specified as `"format": "date-time-rfc1123"`. In .NET it is:
///
/// https://docs.microsoft.com/dotnet/api/system.globalization.datetimeformatinfo.rfc1123pattern
///
/// Sun, 06 Nov 1994 08:49:37 GMT
pub fn to_http_date(date: &OffsetDateTime) -> String {
    date.to_offset(UtcOffset::UTC);
    // known format does not panic
    date.format(&HTTP_DATE_FORMAT).unwrap()
}

/// Similar to preferred HTTP date format, but includes milliseconds
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

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn test_roundtrip_internet_date() -> crate::Result<()> {
        let s = "2019-10-12T07:20:50.52Z";
        let dt = OffsetDateTime::parse(s, &Rfc3339)?;
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
    fn test_to_http_date() -> crate::Result<()> {
        let dt = datetime!(1994-11-06 08:49:37 UTC);
        assert_eq!("Sun, 06 Nov 1994 08:49:37 GMT", to_http_date(&dt));
        Ok(())
    }

    #[test]
    fn test_parse_http_date() -> crate::Result<()> {
        let dt = datetime!(1994-11-06 08:49:37 UTC);
        assert_eq!(parse_http_date("Sun, 06 Nov 1994 08:49:37 GMT")?, dt);
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
            parse_http_date(creation_time)?
        );
        Ok(())
    }
}
