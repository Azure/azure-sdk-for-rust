use crate::key::UserDelegationKey;

pub mod builder;

/// Controls which protocols are permitted for a SAS token (`spr` query parameter).
///
/// Passed to [`crate::UserDelegationSasBuilder::protocol`] and signed into the string-to-sign.
#[derive(Clone, Copy, Default)]
pub enum SignedProtocol {
    /// HTTPS and HTTP. Encoded as `"https,http"`. This is the spec default.
    #[default]
    HttpsAndHttp,
    /// HTTPS only. Encoded as `"https"`.
    Https,
}

impl SignedProtocol {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            SignedProtocol::HttpsAndHttp => "https,http",
            SignedProtocol::Https => "https",
        }
    }
}

/// Common SAS query-string parameters threaded through resource URL builders.
pub(crate) struct SasUrlParams<'a> {
    pub permissions: &'a str,
    /// `None` omits `st` from the URL entirely (spec: `st` is optional).
    pub start: Option<&'a str>,
    pub expiry: &'a str,
    pub key: &'a UserDelegationKey,
    pub signature: &'a str,
    pub version: &'a str,
    pub ip: Option<&'a str>,
    pub protocol: SignedProtocol,
    pub authorized_user_object_id: Option<&'a str>,
    pub unauthorized_user_object_id: Option<&'a str>,
    pub delegated_user_object_id: Option<&'a str>,
    pub delegated_user_tenant_id: Option<&'a str>,
}

/// Shared fields passed to each resource's `string_to_sign` implementation.
///
/// <https://learn.microsoft.com/en-us/rest/api/storageservices/create-user-delegation-sas#specify-the-duration-of-signature-validity>
pub(crate) struct SasSigningContext<'a> {
    pub permissions: &'a str,
    pub start: &'a str,
    pub expiry: &'a str,
    pub canon: &'a str,
    pub key: &'a UserDelegationKey,
    pub version: &'a str,
    pub ip: Option<&'a str>,
    pub protocol: SignedProtocol,
    pub authorized_user_object_id: Option<&'a str>,
    pub unauthorized_user_object_id: Option<&'a str>,
    pub delegated_user_object_id: Option<&'a str>,
    pub delegated_user_tenant_id: Option<&'a str>,
}

/// Appends `rel` to the path of `base`, treating `base`'s path as a directory prefix.
///
/// `Url::join` follows RFC 3986 and strips the last path segment of the base before
/// resolving a relative reference. That works fine when the base has no meaningful path
/// (e.g. `https://account.blob.core.windows.net`), but breaks for Azurite-style emulator
/// endpoints like `https://127.0.0.1:10000/devstoreaccount1` — join would discard
/// `devstoreaccount1`. This helper avoids that by building the path directly.
pub(crate) fn append_path(base: &url::Url, rel: &str) -> url::Url {
    let mut url = base.clone();
    url.set_path(&format!("{}/{}", base.path().trim_end_matches('/'), rel));
    url
}

pub(crate) fn append_common_sas_params<T: url::form_urlencoded::Target>(
    query: &mut url::form_urlencoded::Serializer<'_, T>,
    params: &SasUrlParams<'_>,
) {
    if let Some(start) = params.start {
        query.append_pair("st", start);
    }
    query
        .append_pair("se", params.expiry)
        .append_pair("sp", params.permissions)
        .append_pair("spr", params.protocol.as_str())
        .append_pair("skoid", &params.key.signed_oid)
        .append_pair("sktid", &params.key.signed_tid)
        .append_pair("skt", &params.key.signed_start)
        .append_pair("ske", &params.key.signed_expiry)
        .append_pair("sks", &params.key.signed_service)
        .append_pair("skv", &params.key.signed_version);
    if let Some(ip) = params.ip {
        query.append_pair("sip", ip);
    }
    if let Some(saoid) = params.authorized_user_object_id {
        query.append_pair("saoid", saoid);
    }
    if let Some(suoid) = params.unauthorized_user_object_id {
        query.append_pair("suoid", suoid);
    }
    if let Some(sduoid) = params.delegated_user_object_id {
        query.append_pair("sduoid", sduoid);
    }
    if let Some(skdutid) = params.delegated_user_tenant_id {
        query.append_pair("skdutid", skdutid);
    }
}
