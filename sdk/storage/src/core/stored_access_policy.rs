use crate::xml::read_xml;
use azure_core::date;
use time::OffsetDateTime;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StoredAccessPolicyList {
    pub stored_access: Vec<StoredAccessPolicy>,
}

impl StoredAccessPolicyList {
    pub fn new(list: Vec<StoredAccessPolicy>) -> Self {
        Self {
            stored_access: list,
        }
    }

    pub fn from_xml(bytes: &[u8]) -> azure_core::Result<Self> {
        let sis: SignedIdentifiers = read_xml(bytes)?;
        Ok(sis.into())
    }

    pub fn to_xml(&self) -> String {
        let mut s = String::new();
        s.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<SignedIdentifiers>\n");
        for sa in &self.stored_access {
            s.push_str("\t<SignedIdentifier>\n");

            s.push_str("\t\t<Id>");
            s.push_str(&sa.id);
            s.push_str("\n\t\t</Id>\n");

            s.push_str("<AccessPolicy>\n");

            s.push_str("\t\t\t<Start>");
            s.push_str(&date::to_rfc3339(&sa.start));
            s.push_str("</Start>\n");

            s.push_str("\t\t\t<Expiry>");
            s.push_str(&date::to_rfc3339(&sa.expiry));
            s.push_str("</Expiry>\n");

            s.push_str("\t\t\t<Permission>");
            s.push_str(&sa.permission);
            s.push_str("</Permission>\n");
            s.push_str("\t\t</AccessPolicy>\n\t</SignedIdentifier>\n");
        }

        s.push_str("</SignedIdentifiers>");
        s
    }
}

impl From<SignedIdentifiers> for StoredAccessPolicyList {
    fn from(si: SignedIdentifiers) -> Self {
        let list = si
            .signed_identifiers
            .into_iter()
            .map(|si| StoredAccessPolicy {
                id: si.id,
                start: si.access_policy.start,
                expiry: si.access_policy.expiry,
                permission: si.access_policy.permission,
            })
            .collect();
        Self {
            stored_access: list,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StoredAccessPolicy {
    pub id: String,
    pub start: OffsetDateTime,
    pub expiry: OffsetDateTime,
    pub permission: String,
}

impl StoredAccessPolicy {
    pub fn new<A, B>(id: A, start: OffsetDateTime, expiry: OffsetDateTime, permission: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            id: id.into(),
            start,
            expiry,
            permission: permission.into(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct SignedIdentifiers {
    #[serde(rename = "SignedIdentifier", default)]
    signed_identifiers: Vec<SignedIdentifier>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SignedIdentifier {
    id: String,
    access_policy: AccessPolicy,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AccessPolicy {
    #[serde(with = "azure_core::date::rfc3339")]
    start: OffsetDateTime,
    #[serde(with = "azure_core::date::rfc3339")]
    expiry: OffsetDateTime,
    permission: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::xml::read_xml;

    #[test]
    fn parse_from_xml() {
        let resp = "<?xml version=\"1.0\" encoding=\"utf-8\"?>
    <SignedIdentifiers>
      <SignedIdentifier>
          <Id>MTIzNDU2Nzg5MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTI=</Id>
          <AccessPolicy>
            <Start>2009-09-28T08:49:37Z</Start>
            <Expiry>2009-09-29T08:49:37Z</Expiry>
            <Permission>rwd</Permission>
          </AccessPolicy>
      </SignedIdentifier>
      <SignedIdentifier>
          <Id>000zNDU2Nzg5MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTI=</Id>
          <AccessPolicy>
            <Start>2018-09-28T08:49:37Z</Start>
            <Expiry>2020-09-29T08:49:37Z</Expiry>
            <Permission>rd</Permission>
          </AccessPolicy>
      </SignedIdentifier>
    </SignedIdentifiers>";

        let sis: SignedIdentifiers = read_xml(resp.as_bytes()).unwrap();
        assert_eq!(sis.signed_identifiers.len(), 2);
        let sap: StoredAccessPolicyList = sis.into();
        let sxml = sap.to_xml();

        fn remove_whitespace(mut s: String) -> String {
            s.retain(|c| !c.is_whitespace());
            s
        }
        assert_eq!(remove_whitespace(sxml), remove_whitespace(resp.to_owned()));
    }
}
