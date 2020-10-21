mod utilities;

pub use utilities::*;

pub const SERVER: &str = "server"; // -> [String]
pub const RANGE_GET_CONTENT_MD5: &str = "x-ms-range-get-content-md5"; //=> [bool] }
pub const LEASE_ID: &str = "x-ms-lease-id"; //=> [LeaseId] }
pub const CLIENT_REQUEST_ID: &str = "x-ms-client-request-id"; //=> [String] }
pub const BLOB_PUBLIC_ACCESS: &str = "x-ms-blob-public-access"; // [PublicAccess]
pub const REQUEST_ID: &str = "x-ms-request-id"; //=> [String] }
pub const LEASE_STATUS: &str = "x-ms-lease-status"; //=> [LeaseStatus] }
pub const LEASE_STATE: &str = "x-ms-lease-state"; //=> [LeaseState] }
pub const LEASE_DURATION: &str = "x-ms-lease-duration"; //=> [LeaseDuration] }
pub const HAS_IMMUTABILITY_POLICY: &str = "x-ms-has-immutability-policy";
pub const HAS_LEGAL_HOLD: &str = "x-ms-has-legal-hold";
pub const META_PREFIX: &str = "x-ms-meta-";
pub const LEASE_ACTION: &str = "x-ms-lease-action"; //=> [LeaseAction] }
pub const LEASE_BREAK_PERIOD: &str = "x-ms-lease-break-period"; //=> [u32] }
pub const PROPOSED_LEASE_ID: &str = "x-ms-proposed-lease-id"; //=> [LeaseId] }
pub const LEASE_TIME: &str = "x-ms-lease-time";
pub const CREATION_TIME: &str = "x-ms-creation-time";
pub const COPY_ID: &str = "x-ms-copy-id";
pub const COPY_STATUS_DESCRIPTION: &str = "x-ms-copy-status-description";
pub const COPY_COMPLETION_TIME: &str = "x-ms-copy-completion-time";
pub const COPY_PROGRESS: &str = "x-ms-copy-progress";
pub const COPY_SOURCE: &str = "x-ms-copy-source";
pub const COPY_STATUS: &str = "x-ms-copy-status";
pub const CONTENT_MD5: &str = "Content-MD5"; //=> [String] }
pub const SOURCE_CONTENT_MD5: &str = "x-ms-source-content-md5"; //=> [String] }
pub const SERVER_ENCRYPTED: &str = "x-ms-server-encrypted";
pub const BLOB_TYPE: &str = "x-ms-blob-type";
pub const CONTENT_CRC64: &str = "x-ms-content-crc64";
pub const BLOB_CONTENT_LENGTH: &str = "x-ms-blob-content-length";
pub const BLOB_ACCESS_TIER: &str = "x-ms-access-tier";
pub const BLOB_SEQUENCE_NUMBER: &str = "x-ms-blob-sequence-number";
pub const IF_SEQUENCE_NUMBER_LE: &str = "x-ms-if-sequence-number-le";
pub const IF_SEQUENCE_NUMBER_LT: &str = "x-ms-if-sequence-number-lt";
pub const IF_SEQUENCE_NUMBER_EQ: &str = "x-ms-if-sequence-number-eq";
pub const PAGE_WRITE: &str = "x-ms-page-write";
pub const REQUEST_SERVER_ENCRYPTED: &str = "x-ms-request-server-encrypted";
pub const DELETE_TYPE_PERMANENT: &str = "x-ms-delete-type-permanent";
pub const DELETE_SNAPSHOTS: &str = "x-ms-delete-snapshots";
pub const SKU_NAME: &str = "x-ms-sku-name";
pub const ACCOUNT_KIND: &str = "x-ms-account-kind";
pub const APPEND_POSITION: &str = "x-ms-blob-condition-appendpos";
pub const CACHE_CONTROL: &str = "x-ms-blob-cache-control";
pub const CONTENT_DISPOSITION: &str = "x-ms-blob-content-disposition";
pub const ACTIVITY_ID: &str = "x-ms-activity-id";
pub const HEADER_CONTINUATION: &str = "x-ms-continuation";
pub const SESSION_TOKEN: &str = "x-ms-session-token";
pub const REQUIRES_SYNC: &str = "x-ms-requires-sync";
pub const VERSION: &str = "x-ms-version";
