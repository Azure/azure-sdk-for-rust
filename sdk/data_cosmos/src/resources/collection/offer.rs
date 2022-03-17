use crate::headers;
use azure_core::Header;

/// The collection performance level.
///
/// It can either be custom or fixed. You can find more details [here](https://docs.microsoft.com/rest/api/cosmos-db/create-a-collection).
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Offer {
    /// A Custom level of throughput
    Throughput(u64),
    /// Legacy throughput level 1
    S1,
    /// Legacy throughput level 2
    S2,
    /// Legacy throughput level 3
    S3,
}

impl Header for Offer {
    fn name(&self) -> &'static str {
        match self {
            Offer::Throughput(_) => headers::HEADER_OFFER_THROUGHPUT,
            _ => headers::HEADER_OFFER_TYPE,
        }
    }

    fn value(&self) -> String {
        match self {
            Offer::Throughput(throughput) => throughput.to_string(),
            Offer::S1 => "S1".to_owned(),
            Offer::S2 => "S2".to_owned(),
            Offer::S3 => "S3".to_owned(),
        }
    }
}
