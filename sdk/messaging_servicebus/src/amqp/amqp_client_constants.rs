use const_format::concatcp;
// use std::time::Duration as StdDuration;

use crate::amqp::amqp_constants::VENDOR;

// AMQP Management Operation
pub const MANAGEMENT_ADDRESS: &str = "$management";
// pub const ENTITY_TYPE_MANAGEMENT: &str = "entity-mgmt";
// pub const ENTITY_NAME_KEY: &str = "name";
// pub const PARTITION_NAME_KEY: &str = "partition";
// pub const MANAGEMENT_OPERATION_KEY: &str = "operation";
// pub const READ_OPERATION_VALUE: &str = "READ";
// pub const MANAGEMENT_ENTITY_TYPE_KEY: &str = "type";
// pub const MANAGEMENT_SECURITY_TOKEN_KEY: &str = "security_token";

// Filters
// pub const FILTER_OFFSET_PART_NAME: &str = "amqp.annotation.x-opt-offset";
// pub const FILTER_OFFSET: &str = concatcp!(FILTER_OFFSET_PART_NAME, " > ");
// pub const FILTER_INCLUSIVE_OFFSET: &str = concatcp!(FILTER_OFFSET_PART_NAME, " >= ");
// pub const FILTER_OFFSET_FORMAT_STRING: &str = concatcp!(FILTER_OFFSET, "'{0}'");
// pub const FILTER_INCLUSIVE_OFFSET_FORMAT_STRING: &str = concatcp!(FILTER_INCLUSIVE_OFFSET, "'{0}'");
// pub const FILTER_RECEIVED_AT_PART_NAME_V1: &str = "amqp.annotation.x-opt-enqueuedtimeutc";
// pub const FILTER_RECEIVED_AT_PART_NAME_V2: &str = "amqp.annotation.x-opt-enqueued-time";
// pub const FILTER_RECEIVED_AT: &str = concatcp!(FILTER_RECEIVED_AT_PART_NAME_V2, " > ");
// pub const FILTER_RECEIVED_AT_FORMAT_STRING: &str = concatcp!(FILTER_RECEIVED_AT, "{0}");
pub const SESSION_FILTER_NAME: &str = concatcp!(VENDOR, ":session-filter");
// pub const MESSAGE_RECEIPTS_FILTER_NAME: &str = concatcp!(VENDOR, ":message-receipts-filter");
// pub const CLIENT_SIDE_CURSOR_FILTER_NAME: &str = concatcp!(VENDOR, ":client-side-filter");
// pub static CLIENT_MINIMUM_TOKEN_REFRESH_INTERVAL: StdDuration = StdDuration::from_secs(4 * 60);

// Properties
// pub const ATTACH_EPOCH: &str = concatcp!(VENDOR, ":epoch");
// pub const BATCH_FLUSH_INTERVAL_NAME: &str = concatcp!(VENDOR, ":batch-flush-interval");
// pub const ENTITY_TYPE_NAME: &str = concatcp!(VENDOR, ":entity-type");
// pub const TRANSFER_DESTINATION_ADDRESS: &str = concatcp!(VENDOR, ":transfer-destination-address");
// pub const TIMEOUT_NAME: &str = concatcp!(VENDOR, ":timeout");
// pub const TRACKING_ID_NAME: &str = concatcp!(VENDOR, ":tracking-id");

// Error codes
pub const DEAD_LETTER_NAME: &str = concatcp!(VENDOR, ":dead-letter");
// pub const TIMEOUT_ERROR: &str = concatcp!(VENDOR, ":timeout");
// pub const ADDRESS_ALREADY_IN_USE_ERROR: &str = concatcp!(VENDOR, ":address-already-in-use");
// pub const AUTHORIZATION_FAILED_ERROR: &str = concatcp!(VENDOR, ":auth-failed");
// pub const MESSAGE_LOCK_LOST_ERROR: &str = concatcp!(VENDOR, ":message-lock-lost");
// pub const SESSION_LOCK_LOST_ERROR: &str = concatcp!(VENDOR, ":session-lock-lost");
// pub const STORE_LOCK_LOST_ERROR: &str = concatcp!(VENDOR, ":store-lock-lost");
// pub const SESSION_CANNOT_BE_LOCKED_ERROR: &str = concatcp!(VENDOR, ":session-cannot-be-locked");
// pub const NO_MATCHING_SUBSCRIPTION_ERROR: &str = concatcp!(VENDOR, ":no-matching-subscription");
// pub const SERVER_BUSY_ERROR: &str = concatcp!(VENDOR, ":server-busy");
// pub const ARGUMENT_ERROR: &str = concatcp!(VENDOR, ":argument-error");
// pub const ARGUMENT_OUT_OF_RANGE_ERROR: &str = concatcp!(VENDOR, ":argument-out-of-range");
// pub const PARTITION_NOT_OWNED_ERROR: &str = concatcp!(VENDOR, ":partition-not-owned");
// pub const ENTITY_DISABLED_ERROR: &str = concatcp!(VENDOR, ":entity-disabled");
// pub const PUBLISHER_REVOKED_ERROR: &str = concatcp!(VENDOR, ":publisher-revoked");
// pub const OPERATION_CANCELLED_ERROR: &str = concatcp!(VENDOR, ":operation-cancelled");
// pub const ENTITY_ALREADY_EXISTS_ERROR: &str = concatcp!(VENDOR, ":entity-already-exists");
// pub const RELAY_NOT_FOUND_ERROR: &str = concatcp!(VENDOR, ":relay-not-found");
// pub const MESSAGE_NOT_FOUND_ERROR: &str = concatcp!(VENDOR, ":message-not-found");
pub const LOCKED_UNTIL_UTC: &str = concatcp!(VENDOR, ":locked-until-utc");
