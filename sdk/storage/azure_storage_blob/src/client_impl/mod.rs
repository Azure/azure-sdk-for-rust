pub mod block_blob_client;

/// Strategy for a multi-part transfer operation to ensure data remains unmodified over the course
/// of the operation.
#[derive(Clone, Debug, Default)]
pub enum ConcurrencyControlStrategy {
    /// An optimistic concurrency approach. Observes ETag values in responses and uses those values
    /// as appropriate in
    /// [conditional headers](https://learn.microsoft.com/rest/api/storageservices/specifying-conditional-headers-for-blob-service-operations)
    /// on network requests.
    /// If the remote resource is modified by another operation between network requests, the
    /// network request will fail and this transfer operation will fail fast.
    /// The state of the storage resource in this failure case will be unknown.
    #[default]
    ETagLock,

    /// A pessimistic concurrency approach. Provides the given lease ID on all requests. A leased
    /// resource cannot be modified by any request not containing the lease ID, though leases can
    /// be broken. The lease must already exist for this strategy.
    /// See [Lease Blob REST documentation](https://learn.microsoft.com/en-us/rest/api/storageservices/lease-blob)
    /// for more information on leasing.
    Lease(String),

    /// Not recommended. Does not perform any checks across network requests to ensure the remote
    /// resource remains unmodified through other operations.
    None,
}
