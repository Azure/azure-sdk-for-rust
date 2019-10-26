/// This specifies the collection performance level. It can either be custom or fixed. For
/// more details see
/// [https://docs.microsoft.com/en-us/rest/api/cosmos-db/create-a-collection](https://docs.microsoft.com/en-us/rest/api/cosmos-db/create-a-collection).
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Offer {
    Throughput(u64),
    S1,
    S2,
    S3,
}
