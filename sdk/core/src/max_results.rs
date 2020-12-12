use crate::AppendToUrlQuery;
use std::num::NonZeroU32;

// This type forbids zero as value.
// Zero is invalid in Azure and would throw an error
// if specified.
// Azure has a soft cap on 5k. There is no harm
// to go over but maybe we should inform the user
// that they are specifing a value outside the allowed range.
// Right now we simply ignore it.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct MaxResults(NonZeroU32);

impl MaxResults {
    pub fn new(max_results: NonZeroU32) -> Self {
        Self(max_results)
    }
}

impl AppendToUrlQuery for MaxResults {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("maxresults", &format!("{}", self.0));
    }
}

impl From<NonZeroU32> for MaxResults {
    fn from(max_results: NonZeroU32) -> Self {
        Self::new(max_results)
    }
}
