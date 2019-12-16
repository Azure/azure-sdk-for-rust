use azure_sdk_core::errors::AzureError;
use azure_sdk_core::RequestId;
use chrono::{DateTime, Utc};

response_from_headers!(DeleteBlobResponse ,
		       
                       delete_type_permanent_from_headers -> delete_type_permanent: bool,
		       request_id_from_headers -> request_id: RequestId,
		       date_from_headers -> date: DateTime<Utc>
);
