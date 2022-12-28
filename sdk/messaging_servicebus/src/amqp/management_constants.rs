pub const MICROSOFT: &str = "com.microsoft";

pub mod request {
    // pub const OPERATION: &str = "operation";
    pub const ASSOCIATED_LINK_NAME: &str = "associated-link-name";
}

// pub mod response {
//     pub const STATUS_CODE: &str = "statusCode";
//     pub const STATUS_DESCRIPTION: &str = "statusDescription";
//     pub const ERROR_CONDITION: &str = "errorCondition";
// }

pub mod operations {
    use super::MICROSOFT;
    use const_format::concatcp;

    pub const RENEW_LOCK_OPERATION: &str = concatcp!(MICROSOFT, ":renew-lock");
    pub const RECEIVE_BY_SEQUENCE_NUMBER_OPERATION: &str =
        concatcp!(MICROSOFT, ":receive-by-sequence-number");
    pub const UPDATE_DISPOSITION_OPERATION: &str = concatcp!(MICROSOFT, ":update-disposition");
    pub const RENEW_SESSION_LOCK_OPERATION: &str = concatcp!(MICROSOFT, ":renew-session-lock");
    pub const SET_SESSION_STATE_OPERATION: &str = concatcp!(MICROSOFT, ":set-session-state");
    pub const GET_SESSION_STATE_OPERATION: &str = concatcp!(MICROSOFT, ":get-session-state");
    pub const PEEK_MESSAGE_OPERATION: &str = concatcp!(MICROSOFT, ":peek-message");
    pub const SCHEDULE_MESSAGE_OPERATION: &str = concatcp!(MICROSOFT, ":schedule-message");
    pub const CANCEL_SCHEDULED_MESSAGE_OPERATION: &str =
        concatcp!(MICROSOFT, ":cancel-scheduled-message");

    pub const ADD_RULE_OPERATION: &str = concatcp!(MICROSOFT, ":add-rule");
    pub const REMOVE_RULE_OPERATION: &str = concatcp!(MICROSOFT, ":remove-rule");
    pub const ENUMERATE_RULES_OPERATION: &str = concatcp!(MICROSOFT, ":enumerate-rules");
}

pub mod properties {
    use super::MICROSOFT;
    use const_format::concatcp;

    pub const SERVER_TIMEOUT: &str = concatcp!(MICROSOFT, ":server-timeout");
    // pub const TRACKING_ID: &str = concatcp!(MICROSOFT, ":tracking-id");

    pub const SESSION_STATE: &str = "session-state";
    pub const LOCK_TOKEN: &str = "lock-token";
    pub const LOCK_TOKENS: &str = "lock-tokens";
    pub const SEQUENCE_NUMBERS: &str = "sequence-numbers";
    pub const EXPIRATIONS: &str = "expirations";
    pub const EXPIRATION: &str = "expiration";
    pub const SESSION_ID: &str = "session-id";
    pub const MESSAGE_ID: &str = "message-id";
    pub const PARTITION_KEY: &str = "partition-key";
    pub const VIA_PARTITION_KEY: &str = "via-partition-key";

    pub const RECEIVER_SETTLE_MODE: &str = "receiver-settle-mode";
    pub const MESSAGE: &str = "message";
    pub const MESSAGES: &str = "messages";
    pub const DISPOSITION_STATUS: &str = "disposition-status";
    pub const PROPERTIES_TO_MODIFY: &str = "properties-to-modify";
    pub const DEAD_LETTER_REASON: &str = "deadletter-reason";
    pub const DEAD_LETTER_DESCRIPTION: &str = "deadletter-description";

    pub const FROM_SEQUENCE_NUMBER: &str = "from-sequence-number";
    pub const MESSAGE_COUNT: &str = "message-count";

    pub const CORRELATION_ID: &str = "correlation-id";
    pub const TO: &str = "to";
    pub const REPLY_TO: &str = "reply-to";
    pub const LABEL: &str = "label";
    pub const REPLY_TO_SESSION_ID: &str = "reply-to-session-id";
    pub const CONTENT_TYPE: &str = "content-type";
    pub const CORRELATION_RULE_FILTER_PROPERTIES: &str = "properties";

    pub const SKIP: &str = "skip";
    pub const TOP: &str = "top";
    pub const RULES: &str = "rules";
    pub const RULE_NAME: &str = "rule-name";
    pub const RULE_DESCRIPTION: &str = "rule-description";
    // pub const RULE_CREATED_AT: &str = "rule-created-at";
    pub const SQL_RULE_FILTER: &str = "sql-filter";
    pub const SQL_RULE_ACTION: &str = "sql-rule-action";
    pub const CORRELATION_RULE_FILTER: &str = "correlation-filter";
    pub const EXPRESSION: &str = "expression";
}
