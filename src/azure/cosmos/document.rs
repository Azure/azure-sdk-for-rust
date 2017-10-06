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
    #[serde(rename = "_rid")] pub rid: String,
    #[serde(rename = "_ts")] pub ts: u64,
    #[serde(rename = "_self")] pub _self: String,
    #[serde(rename = "_etag")] pub etag: String,
    #[serde(rename = "_attachments")] pub attachments: String,
}
