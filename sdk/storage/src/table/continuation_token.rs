use azure_core::errors::AzureError;
use http::HeaderMap;
use std::borrow::Cow;
use url::Url;

const HEADER_NEXTPARTITIONKEY: &str = "x-ms-continuation-NextPartitionKey";
const HEADER_NEXTROWKEY: &str = "x-ms-continuation-NextRowKey";
const QUERY_PARAM_NEXTPARTITIONKEY: &str = "NextPartitionKey";
const QUERY_PARAM_NEXTROWKEY: &str = "NextRowKey";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuationToken {
    pub(crate) previous_url: Url,
    pub(crate) new_url: Url,
}

impl ContinuationToken {
    pub fn new(previous_url: Url, next_partition_key: &str, next_row_key: Option<&str>) -> Self {
        let mut partition_key_replaced = false;
        let mut row_key_replaced = false;

        let v: Vec<(_, _)> = previous_url
            .query_pairs()
            // filter_map allows us to strip the QUERY_PARAM_NEXTROWKEY
            // if next_row_key is empty.
            .filter_map(|(k, v)| {
                match k.as_ref() {
                    QUERY_PARAM_NEXTPARTITIONKEY => {
                        partition_key_replaced = true;
                        Some(Cow::Borrowed(next_partition_key))
                    }
                    QUERY_PARAM_NEXTROWKEY => {
                        row_key_replaced = true;
                        next_row_key.map(|next_row_key| Cow::Borrowed(next_row_key))
                    }
                    _ => Some(v),
                }
                .map(|new_v| (k, new_v))
            })
            .collect();

        let mut new_url = previous_url.clone();
        new_url.query_pairs_mut().clear().extend_pairs(v);
        if !partition_key_replaced {
            new_url
                .query_pairs_mut()
                .append_pair(QUERY_PARAM_NEXTPARTITIONKEY, &next_partition_key);
        }
        if !row_key_replaced {
            if let Some(next_row_key) = next_row_key {
                new_url
                    .query_pairs_mut()
                    .append_pair(QUERY_PARAM_NEXTROWKEY, &next_row_key);
            }
        }

        Self {
            previous_url,
            new_url,
        }
    }

    pub fn previous_url(&self) -> &Url {
        &self.previous_url
    }
    pub fn new_url(&self) -> &Url {
        &self.new_url
    }

    pub fn previous_partition_key(&self) -> Option<String> {
        self.previous_url.query_pairs().find_map(|(k, v)| {
            if k == QUERY_PARAM_NEXTPARTITIONKEY {
                Some(v.into_owned())
            } else {
                None
            }
        })
    }

    pub fn previous_row_key(&self) -> Option<String> {
        self.new_url.query_pairs().find_map(|(k, v)| {
            if k == QUERY_PARAM_NEXTROWKEY {
                Some(v.into_owned())
            } else {
                None
            }
        })
    }

    pub fn next_partition_key(&self) -> String {
        self.new_url
            .query_pairs()
            .find_map(|(k, v)| {
                if k == QUERY_PARAM_NEXTPARTITIONKEY {
                    Some(v)
                } else {
                    None
                }
            })
            .unwrap()
            .into_owned()
    }

    pub fn next_row_key(&self) -> String {
        self.new_url
            .query_pairs()
            .find_map(|(k, v)| {
                if k == QUERY_PARAM_NEXTROWKEY {
                    Some(v)
                } else {
                    None
                }
            })
            .unwrap()
            .into_owned()
    }

    pub(crate) fn parse_from_headers_optional(
        previous_url: Url,
        headers: &HeaderMap,
    ) -> Result<Option<Self>, AzureError> {
        let result = if let Some(partition_key) = headers.get(HEADER_NEXTPARTITIONKEY) {
            debug!("partition_key == {:?}", partition_key.to_str());

            let row_key = match headers.get(HEADER_NEXTROWKEY) {
                Some(row_key) => Some(row_key.to_str()?),
                None => None,
            };
            debug!("row_key == {:?}", row_key);

            Some(Self::new(previous_url, partition_key.to_str()?, row_key))
        } else {
            None
        };

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use url::{Position, Url};

    #[test]
    fn parse() {
        let u = Url::parse(
            "http://www.microsoft.com/?some=value&NextPartitionKey=p1&NextRowKey=r1&someother=cc",
        )
        .unwrap();
        let c: ContinuationToken = ContinuationToken::new(u, "new_pp", Some("new_rk"));
        assert_eq!(&format!("{}", c.new_url()),
            "http://www.microsoft.com/?some=value&NextPartitionKey=new_pp&NextRowKey=new_rk&someother=cc");

        let u = Url::parse("https://myaccount.table.core.windows.net/mytable()?$filter=query-expression&$select=comma-separated-property-names").unwrap();
        let c: ContinuationToken = ContinuationToken::new(u, "new_pp", Some("new_rk"));
        assert_eq!(&format!("{}", c.new_url()),
            "https://myaccount.table.core.windows.net/mytable()?%24filter=query-expression&%24select=comma-separated-property-names&NextPartitionKey=new_pp&NextRowKey=new_rk");

        assert_eq!("/mytable()?%24filter=query-expression&%24select=comma-separated-property-names&NextPartitionKey=new_pp&NextRowKey=new_rk",
        &c.new_url[Position::BeforePath..]);
    }
}
