use azure_core::headers::*;
use azure_core::RequestId;
use chrono::{DateTime, Utc};

response_from_headers!(DeleteFilesystemResponse,
    date_from_headers => date: DateTime<Utc>,
    request_id_from_headers => request_id: RequestId
);
