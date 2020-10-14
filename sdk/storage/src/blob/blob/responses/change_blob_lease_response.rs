use azure_core::lease::LeaseId;
use azure_core::RequestId;
use chrono::{DateTime, Utc};

response_from_headers!(ChangeBlobLeaseResponse ,

               etag_from_headers -> etag: String,
               last_modified_from_headers -> last_modified: DateTime<Utc>,
                       lease_id_from_headers -> lease_id: LeaseId,
               request_id_from_headers -> request_id: RequestId,
               date_from_headers -> date: DateTime<Utc>
);
