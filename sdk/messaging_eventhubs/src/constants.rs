// use time::Duration as TimeSpan;

use time::OffsetDateTime;

pub(crate) const MAX_MESSAGE_ID_LENGTH: usize = 128;

pub(crate) const MAX_PARTITION_KEY_LENGTH: usize = 128;

pub(crate) const MAX_SESSION_ID_LENGTH: usize = 128;

// pub(crate) const PATH_DELIMITER: &str = r#"/"#;

// pub(crate) const RULE_NAME_MAXIMUM_LENGTH: usize = 50;

// pub(crate) const MAXIMUM_SQL_RULE_FILTER_STATEMENT_LENGTH: usize = 1024;

// pub(crate) const MAXIMUM_SQL_RULE_ACTION_STATEMENT_LENGTH: usize = 1024;

// pub(crate) const DEFAULT_CLIENT_PREFETCH_COUNT: usize = 0;

// pub(crate) const MAX_DEAD_LETTER_REASON_LENGTH: usize = 4096;

pub(crate) const DEFAULT_LAST_PEEKED_SEQUENCE_NUMBER: i64 = 0;

// pub(crate) const DEFAULT_OPERATION_TIMEOUT: TimeSpan = TimeSpan::minutes(1);

// pub(crate) const CLIENT_PUMP_RENEW_LOCK_TIMEOUT: TimeSpan = TimeSpan::minutes(5);

// pub(crate) const MAXIMUM_RENEW_BUFFER_DURATION: TimeSpan = TimeSpan::seconds(10);

// pub(crate) const DEFAULT_RETRY_DELTA_BACKOFF: TimeSpan = TimeSpan::seconds(3);

// pub(crate) const NO_MESSAGE_BACKOFF_TIME_SPAN: TimeSpan = TimeSpan::seconds(5);

pub(crate) const SAS_TOKEN_TYPE: &str = "servicebus.windows.net:sastoken";

pub(crate) const JSON_WEB_TOKEN_TYPE: &str = "jwt";

// pub(crate) const AAD_SERVICE_BUS_AUDIENCE: &str = "https://servicebus.azure.net/";

// /// Represents 00:00:00 UTC Thursday 1, January 1970.
// pub(crate) const EPOCH_TIME: OffsetDateTime = time::OffsetDateTime::UNIX_EPOCH;

// pub(crate) const WELL_KNOWN_PUBLIC_PORTS_LIMIT: i32 = 1023;

// pub(crate) const DEFAULT_SCOPE: &str = "https://servicebus.azure.net/.default";

/// `time::OffsetDateTime` doesn't implement `Default`. This value is taken from the
/// dotnet SDK `DateTime.MinValue` (Azure.Messaging.ServiceBus).
pub(crate) const DEFAULT_OFFSET_DATE_TIME: OffsetDateTime =
    time::macros::datetime!(0001-01-01 12:00:00 AM +00:00);

pub(crate) const MAX_OFFSET_DATE_TIME: OffsetDateTime =
    time::macros::datetime!(9999-12-31 11:59:59 PM +00:00);
