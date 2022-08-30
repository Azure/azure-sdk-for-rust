use std::time::Duration;

use time::OffsetDateTime;

pub const MAX_MESSAGE_ID_LENGTH: i32 = 128;

pub const MAX_PARTITION_KEY_LENGTH: i32 = 128;

pub const MAX_SESSION_ID_LENGTH: i32 = 128;

pub const PATH_DELIMITER: &str = r#"/"#;

pub const RULE_NAME_MAXIMUM_LENGTH: i32 = 50;

pub const MAXIMUM_SQL_RULE_FILTER_STATEMENT_LENGTH: i32 = 1024;

pub const MAXIMUM_SQL_RULE_ACTION_STATEMENT_LENGTH: i32 = 1024;

pub const DEFAULT_CLIENT_PREFETCH_COUNT: i32 = 0;

pub const MAX_DEAD_LETTER_REASON_LENGTH: i32 = 4096;

pub const DEFAULT_LAST_PEEKED_SEQUENCE_NUMBER: i64 = 0;

pub static DEFAULT_OPERATION_TIMEOUT: Duration = Duration::from_secs(1 * 60);

pub static CLIENT_PUMP_RENEW_LOCK_TIMEOUT: Duration = Duration::from_secs(5 * 60);

pub static MAXIMUM_RENEW_BUFFER_DURATION: Duration = Duration::from_secs(10);

pub static DEFAULT_RETRY_DELTA_BACKOFF: Duration = Duration::from_secs(3);

pub static NO_MESSAGE_BACKOFF_TIME_SPAN: Duration = Duration::from_secs(5);

pub const SAS_TOKEN_TYPE: &str = "servicebus.windows.net:sastoken";

pub const JSON_WEB_TOKEN_TYPE: &str = "jwt";

pub const AAD_SERVICE_BUS_AUDIENCE: &str = "https://servicebus.azure.net/";

/// Represents 00:00:00 UTC Thursday 1, January 1970.
pub static EPOCH_TIME: OffsetDateTime = time::OffsetDateTime::UNIX_EPOCH;

pub const WELL_KNOWN_PUBLIC_PORTS_LIMIT: i32 = 1023;

pub const DEFAULT_SCOPE: &str = "https://servicebus.azure.net/.default";
