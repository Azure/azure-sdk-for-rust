use azure_core::error::{Error, ErrorKind};

/// A resource quota for the given resource kind
///
/// A collection of this type is often returned in responses allowing you to
/// know how much of a given resource you can use.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum ResourceQuota {
    Databases(u64),
    StoredProcedures(u64),
    Collections(u64),
    DocumentSize(u64),
    DocumentsSize(u64),
    DocumentsCount(i64),
    CollectionSize(u64),
    Users(u64),
    Permissions(u64),
    Triggers(u64),
    Functions(u64),
    ClientEncryptionKeys(u64),
    InteropUsers(u64),
    AuthPolicyElements(u64),
}

const DATABASES: &str = "databases=";
const STORED_PROCEDURES: &str = "storedProcedures=";
const COLLECTIONS: &str = "collections=";
const DOCUMENT_SIZE: &str = "documentSize=";
const DOCUMENTS_SIZE: &str = "documentsSize=";
const DOCUMENTS_COUNT: &str = "documentsCount=";
const COLLECTION_SIZE: &str = "collectionSize=";
const USERS: &str = "users=";
const PERMISSIONS: &str = "permissions=";
const TRIGGERS: &str = "triggers=";
const FUNCTIONS: &str = "functions=";
const CLIENT_ENCRYPTION_KEYS: &str = "clientEncryptionKeys=";
const INTEROP_USERS: &str = "interopUsers=";
const AUTH_POLICY_ELEMENTS: &str = "authPolicyElements=";

/// Parse a collection of [`ResourceQuota`] from a string
pub(crate) fn resource_quotas_from_str(
    full_string: &str,
) -> azure_core::Result<Vec<ResourceQuota>> {
    debug!("resource_quotas_from_str(\"{}\") called", full_string);
    let tokens: Vec<&str> = full_string.split(';').collect();
    let mut v = Vec::with_capacity(tokens.len());

    let parseu64 = |s| str::parse(s).map_err(|e| parse_int_error(e, s, full_string));
    let parsei64 = |s| str::parse(s).map_err(|e| parse_int_error(e, s, full_string));

    for token in tokens.into_iter().filter(|token| !token.is_empty()) {
        trace!("processing token == {}", token);

        if let Some(stripped) = token.strip_prefix(DATABASES) {
            v.push(ResourceQuota::Databases(parseu64(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(STORED_PROCEDURES) {
            v.push(ResourceQuota::StoredProcedures(parseu64(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(COLLECTIONS) {
            v.push(ResourceQuota::Collections(parseu64(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(DOCUMENT_SIZE) {
            v.push(ResourceQuota::DocumentSize(parseu64(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(DOCUMENTS_SIZE) {
            v.push(ResourceQuota::DocumentsSize(parseu64(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(DOCUMENTS_COUNT) {
            v.push(ResourceQuota::DocumentsCount(parsei64(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(COLLECTION_SIZE) {
            v.push(ResourceQuota::CollectionSize(parseu64(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(USERS) {
            v.push(ResourceQuota::Users(parseu64(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(PERMISSIONS) {
            v.push(ResourceQuota::Permissions(parseu64(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(TRIGGERS) {
            v.push(ResourceQuota::Triggers(parseu64(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(FUNCTIONS) {
            v.push(ResourceQuota::Functions(parseu64(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(CLIENT_ENCRYPTION_KEYS) {
            v.push(ResourceQuota::ClientEncryptionKeys(parseu64(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(INTEROP_USERS) {
            v.push(ResourceQuota::InteropUsers(parseu64(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(AUTH_POLICY_ELEMENTS) {
            v.push(ResourceQuota::AuthPolicyElements(parseu64(stripped)?));
        } else {
            return Err(Error::with_message(ErrorKind::DataConversion, || {
                format!(
                    "resource quota has an unrecognized part - part: \"{}\" full string: \"{}\"",
                    token, full_string
                )
            }));
        }

        trace!("v == {:#?}", v);
    }

    Ok(v)
}

fn parse_int_error(e: std::num::ParseIntError, n: &str, resource_quota: &str) -> Error {
    Error::full(
        ErrorKind::DataConversion,
        e,
        format!(
            "failed to convert '{}' as int when parsing resource quote '{}'",
            n, resource_quota
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_resource_quota() {
        let resource_quota = resource_quotas_from_str("storedProcedures=25;").unwrap();
        assert_eq!(resource_quota, vec![ResourceQuota::StoredProcedures(25)]);

        let resource_quota = resource_quotas_from_str(
            "databases=100;collections=5000;users=500000;permissions=2000000;clientEncryptionKeys=13;",
        )
        .unwrap();

        assert_eq!(
            resource_quota,
            vec![
                ResourceQuota::Databases(100),
                ResourceQuota::Collections(5000),
                ResourceQuota::Users(500000),
                ResourceQuota::Permissions(2000000),
                ResourceQuota::ClientEncryptionKeys(13)
            ]
        );

        let resource_quota = resource_quotas_from_str("collections=27;").unwrap();
        assert_eq!(resource_quota, vec![ResourceQuota::Collections(27)]);

        let resource_quota =
            resource_quotas_from_str("documentSize=0;documentsSize=2;collectionSize=3;").unwrap();

        assert_eq!(
            resource_quota,
            vec![
                ResourceQuota::DocumentSize(0),
                ResourceQuota::DocumentsSize(2),
                ResourceQuota::CollectionSize(3)
            ]
        );

        let resource_quota = resource_quotas_from_str("users=500000;").unwrap();
        assert_eq!(resource_quota, vec![ResourceQuota::Users(500000)]);

        let resource_quota = resource_quotas_from_str("permissions=2000000;").unwrap();
        assert_eq!(resource_quota, vec![ResourceQuota::Permissions(2000000)]);

        let resource_quota = resource_quotas_from_str("triggers=25;").unwrap();
        assert_eq!(resource_quota, vec![ResourceQuota::Triggers(25)]);

        let resource_quota = resource_quotas_from_str("functions=26;").unwrap();
        assert_eq!(resource_quota, vec![ResourceQuota::Functions(26)]);

        let resource_quota = resource_quotas_from_str("clientEncryptionKeys=13;").unwrap();
        assert_eq!(
            resource_quota,
            vec![ResourceQuota::ClientEncryptionKeys(13)]
        );
    }
}
