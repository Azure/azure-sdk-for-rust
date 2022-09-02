use time::Duration as TimeSpan;

use time::OffsetDateTime;

pub const MAX_MESSAGE_ID_LENGTH: usize = 128;

pub const MAX_PARTITION_KEY_LENGTH: usize = 128;

pub const MAX_SESSION_ID_LENGTH: usize = 128;

pub const PATH_DELIMITER: &str = r#"/"#;

pub const RULE_NAME_MAXIMUM_LENGTH: usize = 50;

pub const MAXIMUM_SQL_RULE_FILTER_STATEMENT_LENGTH: usize = 1024;

pub const MAXIMUM_SQL_RULE_ACTION_STATEMENT_LENGTH: usize = 1024;

pub const DEFAULT_CLIENT_PREFETCH_COUNT: usize = 0;

pub const MAX_DEAD_LETTER_REASON_LENGTH: usize = 4096;

pub const DEFAULT_LAST_PEEKED_SEQUENCE_NUMBER: i64 = 0;

pub const DEFAULT_OPERATION_TIMEOUT: TimeSpan = TimeSpan::minutes(1);

pub const CLIENT_PUMP_RENEW_LOCK_TIMEOUT: TimeSpan = TimeSpan::minutes(5);

pub const MAXIMUM_RENEW_BUFFER_DURATION: TimeSpan = TimeSpan::seconds(10);

pub const DEFAULT_RETRY_DELTA_BACKOFF: TimeSpan = TimeSpan::seconds(3);

pub const NO_MESSAGE_BACKOFF_TIME_SPAN: TimeSpan = TimeSpan::seconds(5);

pub const SAS_TOKEN_TYPE: &str = "servicebus.windows.net:sastoken";

pub const JSON_WEB_TOKEN_TYPE: &str = "jwt";

pub const AAD_SERVICE_BUS_AUDIENCE: &str = "https://servicebus.azure.net/";

/// Represents 00:00:00 UTC Thursday 1, January 1970.
pub const EPOCH_TIME: OffsetDateTime = time::OffsetDateTime::UNIX_EPOCH;

pub const WELL_KNOWN_PUBLIC_PORTS_LIMIT: i32 = 1023;

pub const DEFAULT_SCOPE: &str = "https://servicebus.azure.net/.default";
