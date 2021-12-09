/// Represents the components that make up an Azure Storage Shared Access Signature's query parameters.
/// You can construct a new instance using TODO
#[derive(Debug, PartialEq)]
pub(crate) struct TableSasQueryOptions {
    pub(crate) table_name: String,

    /// The end of RowKey range.
    pub(crate) end_row_key: Option<String>,

    /// The start of RowKey range.
    pub(crate) start_row_key: Option<String>,

    /// The end of PartionKey range.
    pub(crate) end_partition_key: Option<String>,

    /// The start of PartionKey range.
    pub(crate) start_partition_key: Option<String>,
}

impl TableSasQueryOptions {
    pub(crate) fn new(table_name: impl Into<String>) -> Self {
        Self {
            table_name: table_name.into(),
            end_row_key: Default::default(),
            start_row_key: Default::default(),
            end_partition_key: Default::default(),
            start_partition_key: Default::default(),
        }
    }

    setters! {
        end_row_key: String  => Some(end_row_key),
        start_row_key: String => Some(start_row_key),
        end_partition_key: String  => Some(end_partition_key),
        start_partition_key: String => Some(start_partition_key),
    }
}
