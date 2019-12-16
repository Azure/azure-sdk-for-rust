use azure_sdk_core::errors::AzureError;
use azure_sdk_core::RequestId;
use chrono::{DateTime, Utc};

response_from_headers!(ClearPageResponse,
		       
		       etag_from_headers -> etag: String,
		       last_modified_from_headers -> last_modified: DateTime<Utc>,
		       sequence_number_from_headers -> sequence_number: u64,
		       request_id_from_headers -> request_id: RequestId,
		       date_from_headers -> date: DateTime<Utc>
);
