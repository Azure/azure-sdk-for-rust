use crate::errors::AzureError;
use chrono::{DateTime, FixedOffset};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StoredAccessPolicyList {
    pub stored_access: Vec<StoredAccessPolicy>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StoredAccessPolicy {
    pub id: String,
    pub start: DateTime<FixedOffset>,
    pub expiry: DateTime<FixedOffset>,
    pub permission: String,
}

impl StoredAccessPolicy {
    pub fn new<A, B>(
        id: A,
        start: DateTime<FixedOffset>,
        expiry: DateTime<FixedOffset>,
        permission: B,
    ) -> StoredAccessPolicy
    where
        A: Into<String>,
        B: Into<String>,
    {
        StoredAccessPolicy {
            id: id.into(),
            start,
            expiry,
            permission: permission.into(),
        }
    }
}

impl StoredAccessPolicyList {
    pub fn new() -> StoredAccessPolicyList {
        StoredAccessPolicyList::default()
    }

    pub fn from_xml(xml: &str) -> Result<StoredAccessPolicyList, AzureError> {
        let mut sal = StoredAccessPolicyList {
            stored_access: Vec::new(),
        };
        let sis: SignedIdentifiers = serde_xml_rs::de::from_reader(xml.as_bytes())?;

        if let Some(sis) = sis.signed_identifiers {
            for si in sis {
                let sa = StoredAccessPolicy {
                    id: si.id,
                    start: DateTime::parse_from_rfc3339(&si.access_policy.start)?,
                    expiry: DateTime::parse_from_rfc3339(&si.access_policy.expiry)?,
                    permission: si.access_policy.permission,
                };

                sal.stored_access.push(sa);
            }
        }

        Ok(sal)
    }

    pub fn to_xml(&self) -> String {
        let mut s = String::new();
        s.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<SignedIdentifiers>\n");
        for sa in &self.stored_access {
            s.push_str("\t<SignedIdentifier>\n");
            s.push_str(&format!("\t\t<Id>{}</Id>\n\t\t<AccessPolicy>\n", sa.id));
            s.push_str(&format!(
                "\t\t\t<Start>{}</Start>\n",
                sa.start.format("%Y-%m-%dT%H:%M:%SZ")
            ));
            s.push_str(&format!(
                "\t\t\t<Expiry>{}</Expiry>\n",
                sa.expiry.format("%Y-%m-%dT%H:%M:%SZ")
            ));
            s.push_str(&format!(
                "\t\t\t<Permission>{}</Permission>\n",
                sa.permission
            ));
            s.push_str("\t\t</AccessPolicy>\n\t</SignedIdentifier>\n");
        }

        s.push_str("</SignedIdentifiers>");
        s
    }
}

#[derive(Debug, Deserialize)]
struct SignedIdentifiers {
    #[serde(rename = "SignedIdentifier")]
    signed_identifiers: Option<Vec<SignedIdentifier>>,
}

#[derive(Debug, Deserialize)]
struct SignedIdentifier {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "AccessPolicy")]
    access_policy: AccessPolicy,
}

#[derive(Debug, Deserialize)]
struct AccessPolicy {
    #[serde(rename = "Start")]
    start: String,
    #[serde(rename = "Expiry")]
    expiry: String,
    #[serde(rename = "Permission")]
    permission: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_from_xml() {
        let resp = "<?xml version=\"1.0\" encoding=\"utf-8\"?>  
    <SignedIdentifiers>  
      <SignedIdentifier>   
          <Id>MTIzNDU2Nzg5MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTI=</Id>  
          <AccessPolicy>  
            <Start>2009-09-28T08:49:37.0000000Z</Start>  
            <Expiry>2009-09-29T08:49:37.0000000Z</Expiry>  
            <Permission>rwd</Permission>  
          </AccessPolicy>  
      </SignedIdentifier>  
      <SignedIdentifier>   
          <Id>000zNDU2Nzg5MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTI=</Id>  
          <AccessPolicy>  
            <Start>2018-09-28T08:49:37.0000000Z</Start>  
            <Expiry>2020-09-29T08:49:37.0000000Z</Expiry>  
            <Permission>rd</Permission>  
          </AccessPolicy>  
      </SignedIdentifier>    
    </SignedIdentifiers>";

        let sis: SignedIdentifiers = serde_xml_rs::de::from_reader(resp.as_bytes()).unwrap();
        assert!(sis.signed_identifiers.unwrap().len() == 2);

        let sal = StoredAccessPolicyList::from_xml(resp).unwrap();

        let _sxml = sal.to_xml();
    }
}
