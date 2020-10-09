use azure_sdk_core::errors::AzureError;
use azure_sdk_core::prelude::IfMatchCondition;
use http::HeaderMap;

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DocumentAttributes {
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "_ts")]
    pub ts: u64,
    #[serde(rename = "_self")]
    pub _self: String,
    #[serde(rename = "_etag")]
    pub etag: String,
    #[serde(rename = "_attachments")]
    pub attachments: String,
}

impl DocumentAttributes {
    pub fn rid(&self) -> &str {
        &self.rid
    }

    pub fn ts(&self) -> u64 {
        self.ts
    }

    pub fn _self(&self) -> &str {
        &self._self
    }

    pub fn etag(&self) -> &str {
        &self.etag
    }

    pub fn attachments(&self) -> &str {
        &self.attachments
    }

    pub fn set_rid<T>(&mut self, value: T)
    where
        T: Into<String>,
    {
        self.rid = value.into();
    }

    pub fn set_ts(&mut self, value: u64) {
        self.ts = value;
    }

    pub fn set_self<T>(&mut self, value: T)
    where
        T: Into<String>,
    {
        self._self = value.into();
    }

    pub fn set_etag<T>(&mut self, value: T)
    where
        T: Into<String>,
    {
        self.etag = value.into();
    }

    pub fn set_attachments<T>(&mut self, value: T)
    where
        T: Into<String>,
    {
        self.attachments = value.into();
    }
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for DocumentAttributes {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let body = value.1;
        Ok(serde_json::from_slice(body)?)
    }
}

impl<'a> std::convert::From<&'a DocumentAttributes> for IfMatchCondition<'a> {
    fn from(document_attributes: &'a DocumentAttributes) -> Self {
        IfMatchCondition::Match(&document_attributes.etag)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_mutate() {
        use super::*;

        let mut a = DocumentAttributes {
            rid: "rid".to_owned(),
            ts: 100,
            _self: "_self".to_owned(),
            etag: "etag".to_owned(),
            attachments: "attachments".to_owned(),
        };

        a.set_attachments("new_attachments".to_owned());
    }
}
