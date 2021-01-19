use crate::errors::TokenParsingError;

/// A resource quota for the given resource kind
///
/// A collection of this type is often returned in responses allowing you to
/// know how much of a given resource you can use.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

/// Parse a collection of [`ResourceQuota`] from a string
pub(crate) fn resource_quotas_from_str(s: &str) -> Result<Vec<ResourceQuota>, failure::Error> {
    debug!("resource_quotas_from_str(\"{}\") called", s);
    let tokens: Vec<&str> = s.split(';').collect();
    let mut v = Vec::with_capacity(tokens.len());

    for token in tokens.into_iter().filter(|token| !token.is_empty()) {
        debug!("processing token == {}", token);

        if let Some(stripped) = token.strip_prefix(DATABASES) {
            v.push(ResourceQuota::Databases(str::parse(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(STORED_PROCEDURES) {
            v.push(ResourceQuota::StoredProcedures(str::parse(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(COLLECTIONS) {
            v.push(ResourceQuota::Collections(str::parse(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(DOCUMENT_SIZE) {
            v.push(ResourceQuota::DocumentSize(str::parse(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(DOCUMENTS_SIZE) {
            v.push(ResourceQuota::DocumentsSize(str::parse(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(DOCUMENTS_COUNT) {
            v.push(ResourceQuota::DocumentsCount(str::parse(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(COLLECTION_SIZE) {
            v.push(ResourceQuota::CollectionSize(str::parse(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(USERS) {
            v.push(ResourceQuota::Users(str::parse(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(PERMISSIONS) {
            v.push(ResourceQuota::Permissions(str::parse(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(TRIGGERS) {
            v.push(ResourceQuota::Triggers(str::parse(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(FUNCTIONS) {
            v.push(ResourceQuota::Functions(str::parse(stripped)?));
        } else if let Some(stripped) = token.strip_prefix(CLIENT_ENCRYPTION_KEYS) {
            v.push(ResourceQuota::ClientEncryptionKeys(str::parse(stripped)?));
        } else {
            return Err(TokenParsingError::UnsupportedToken {
                token: token.to_string(),
                s: s.to_owned(),
            }
            .into());
        }

        debug!("v == {:#?}", v);
    }

    Ok(v)
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
