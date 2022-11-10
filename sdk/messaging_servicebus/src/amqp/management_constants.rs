pub const MICROSOFT: &str = "com.microsoft";

pub mod request {
    pub const OPERATION: &str = "operation";
    pub const ASSOCIATED_LINK_NAME: &str = "associated-link-name";
}

pub mod response {
    pub const STATUS_CODE: &str = "statusCode";
    pub const STATUS_DESCRIPTION: &str = "statusDescription";
    pub const ERROR_CONDITION: &str = "errorCondition";
}

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
    pub const ADD_RULE_OPERATION: &str = concatcp!(MICROSOFT, ":add-rule");
    pub const REMOVE_RULE_OPERATION: &str = concatcp!(MICROSOFT, ":remove-rule");
    pub const ENUMERATE_RULES_OPERATION: &str = concatcp!(MICROSOFT, ":enumerate-rules");
    pub const SCHEDULE_MESSAGE_OPERATION: &str = concatcp!(MICROSOFT, ":schedule-message");
    pub const CANCEL_SCHEDULED_MESSAGE_OPERATION: &str =
        concatcp!(MICROSOFT, ":cancel-scheduled-message");
}
