use azure_core::{headers::*, RequestId};
use chrono::{DateTime, Utc};

azure_storage::response_from_headers!(ReleaseBlobLeaseResponse ,
               etag_from_headers => etag: String,
               last_modified_from_headers => last_modified: DateTime<Utc>,
               request_id_from_headers => request_id: RequestId,
               date_from_headers => date: DateTime<Utc>
);
