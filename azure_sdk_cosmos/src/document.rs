use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum IndexingDirective {
    Include,
    Exclude,
}

impl Display for IndexingDirective {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            IndexingDirective::Include => write!(f, "Include"),
            IndexingDirective::Exclude => write!(f, "Exclude"),
        }
    }
}

impl FromStr for IndexingDirective {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Include" => Ok(IndexingDirective::Include),
            "Exclude" => Ok(IndexingDirective::Exclude),
            _ => Err(format!("{} is not valid IndexingDirective value", s)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DocumentAttributes {
    id: String,
    #[serde(rename = "_rid")]
    rid: String,
    #[serde(rename = "_ts")]
    ts: u64,
    #[serde(rename = "_self")]
    _self: String,
    #[serde(rename = "_etag")]
    etag: String,
    #[serde(rename = "_attachments")]
    attachments: String,
}

impl DocumentAttributes {
    pub fn id(&self) -> &str {
        &self.id
    }

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

    pub fn set_id<T>(&mut self, value: T)
    where
        T: Into<String>,
    {
        self.id = value.into();
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

    pub(crate) fn try_extract(
        from: &mut ::serde_json::Map<String, ::serde_json::Value>,
    ) -> Option<DocumentAttributes> {
        let id = from.get("id")?.as_str()?.to_owned();
        let rid = from.remove("_rid")?.as_str()?.to_owned();
        let ts = from.remove("_ts")?.as_u64()?;
        let _self = from.remove("_self")?.as_str()?.to_owned();
        let etag = from.remove("_etag")?.as_str()?.to_owned();
        let attachments = from.remove("_attachments")?.as_str()?.to_owned();

        Some(DocumentAttributes {
            id,
            rid,
            ts,
            _self,
            etag,
            attachments,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_mutate() {
        use super::*;

        let mut a = DocumentAttributes {
            id: "id".to_owned(),
            rid: "rid".to_owned(),
            ts: 100,
            _self: "_self".to_owned(),
            etag: "etag".to_owned(),
            attachments: "attachments".to_owned(),
        };

        a.set_id("new_id");
        a.set_attachments("new_attachments".to_owned());
    }
}
