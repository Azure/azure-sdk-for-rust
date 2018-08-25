use azure::core::RequestId;
use chrono::{DateTime, Utc};

response_from_headers!(DeleteBlobSnapshotResponse ,
		       
                       delete_type_permanent_from_headers -> delete_type_permanent: bool,
		       request_id_from_headers -> request_id: RequestId,
		       date_from_headers -> date: DateTime<Utc>
);
