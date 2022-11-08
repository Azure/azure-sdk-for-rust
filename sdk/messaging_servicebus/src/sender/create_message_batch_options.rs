/// The set of options that can be specified to influence the way in which an service bus message
/// batch behaves and is sent to the Queue/Topic.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateMessageBatchOptions {
    /// The maximum size of the batch, in bytes.
    pub max_size_in_bytes: Option<u64>,
}
