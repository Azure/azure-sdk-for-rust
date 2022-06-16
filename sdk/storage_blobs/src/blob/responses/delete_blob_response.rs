use azure_core::{headers::*, RequestId};
use chrono::{DateTime, Utc};

#[cfg(not(feature = "azurite_workaround"))]
azure_storage::response_from_headers!(DeleteBlobResponse ,
               delete_type_permanent_from_headers => delete_type_permanent: bool,
               request_id_from_headers => request_id: RequestId,
               date_from_headers => date: DateTime<Utc>
);

#[cfg(feature = "azurite_workaround")]
azure_storage::response_from_headers!(DeleteBlobResponse ,
               delete_type_permanent_from_headers => delete_type_permanent: Option<bool>,
               request_id_from_headers => request_id: RequestId,
               date_from_headers => date: DateTime<Utc>
);
