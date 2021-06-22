use crate::headers;
use azure_core::AddAsHeader;
use http::request::Builder;

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

impl AddAsHeader for Offer {
    fn add_as_header(&self, builder: Builder) -> Builder {
        match self {
            Offer::Throughput(throughput) => {
                builder.header(headers::HEADER_OFFER_THROUGHPUT, *throughput)
            }
            Offer::S1 => builder.header(headers::HEADER_OFFER_TYPE, "S1"),
            Offer::S2 => builder.header(headers::HEADER_OFFER_TYPE, "S2"),
            Offer::S3 => builder.header(headers::HEADER_OFFER_TYPE, "S3"),
        }
    }
}
