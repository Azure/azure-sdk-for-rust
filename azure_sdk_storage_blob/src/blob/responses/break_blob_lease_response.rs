use azure_sdk_core::errors::AzureError;
use azure_sdk_core::RequestId;
use chrono::{DateTime, Utc};

response_from_headers!(BreakBlobLeaseResponse ,
		       
		       etag_from_headers -> etag: String,
		       last_modified_from_headers -> last_modified: DateTime<Utc>,
                       lease_time_from_headers -> lease_time: u8,
		       request_id_from_headers -> request_id: RequestId,
		       date_from_headers -> date: DateTime<Utc>
);
