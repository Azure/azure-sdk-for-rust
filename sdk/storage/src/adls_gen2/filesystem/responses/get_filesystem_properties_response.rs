use crate::filesystem::{namespace_enabled_from_headers, properties_from_headers};
use azure_core::headers::*;
use azure_core::RequestId;
use chrono::{DateTime, Utc};

response_from_headers!(GetFilesystemPropertiesResponse,
    date_from_headers => date: DateTime<Utc>,
    etag_from_headers => etag: String,
    last_modified_from_headers => last_modified: DateTime<Utc>,
    request_id_from_headers => request_id: RequestId,
    properties_from_headers => properties: String,
    namespace_enabled_from_headers => namespace_enabled: bool
);
