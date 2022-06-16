use azure_core::{headers::*, RequestId};
use chrono::{DateTime, Utc};

azure_storage::response_from_headers!(ClearPageResponse,
               etag_from_headers => etag: String,
               last_modified_from_headers => last_modified: DateTime<Utc>,
               sequence_number_from_headers => sequence_number: u64,
               request_id_from_headers => request_id: RequestId,
               date_from_headers => date: DateTime<Utc>
);
