use std::collections::HashMap;

use url::Url;

use azure_storage_blob::models::UserDelegationKey;

pub(crate) const ACCOUNT: &str = "testaccount";

pub(crate) fn make_key() -> UserDelegationKey {
    UserDelegationKey {
        signed_oid: "oid-11111111-1111-1111-1111-111111111111".to_string(),
        signed_tid: "tid-22222222-2222-2222-2222-222222222222".to_string(),
        signed_start: "2099-01-01T00:00:00Z".to_string(),
        signed_expiry: "2099-01-08T00:00:00Z".to_string(),
        signed_service: "b".to_string(),
        signed_version: "2022-11-02".to_string(),
        // 32 zero bytes in standard base64
        value: "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".to_string(),
    }
}

pub(crate) fn url_params(url: &Url) -> HashMap<String, String> {
    url.query_pairs()
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect()
}
