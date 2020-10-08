use crate::{entity_path, TableEntity};
use serde::Serialize;
use serde_json;

const BATCH_MAX_SIZE: usize = 100;

quick_error! {
    #[derive(Debug)]
    pub enum BatchError {
        UnexpectedPartitionKey {
            display("Batch operation cannot be executed in multiple partitions")
        }
        TooManyOperations  {
            display("Batch operation size limit reached")
        }
        JSONError(err: serde_json::Error) {
            from()
            display("json error: {}", err)
            cause(err)
        }
    }
}

pub enum BatchOperation {
    Delete {
        row_key: String,
        etag: Option<String>,
    },

    Insert {
        row_key: String,
        payload: String,
    },

    Update {
        row_key: String,
        payload: String,
        etag: Option<String>,
    },
}

impl BatchOperation {
    fn into_payload(&self, uri_prefix: &str, table: &str, partition_key: &str, body: &mut String) {
        // todo: consider using the cloud_table request builder to generate payloads
        match *self {
            BatchOperation::Insert { ref payload, .. } => {
                body.push_str("POST ");
                body.push_str(uri_prefix);
                body.push_str(table);
                body.push_str(" HTTP/1.1\n");
                body.push_str("Accept: application/json;odata=nometadata\n");
                body.push_str("Content-Type: application/json\n\n");
                body.push_str(&payload);
                body.push_str("\n");
            }

            BatchOperation::Update {
                ref row_key,
                ref etag,
                ref payload,
            } => {
                body.push_str("PUT ");
                body.push_str(uri_prefix);
                body.push_str(&entity_path(table, partition_key, row_key));
                body.push_str(" HTTP/1.1\n");
                body.push_str("Accept: application/json;odata=nometadata\n");
                body.push_str("Content-Type: application/json\n");
                if let Some(etag) = etag {
                    body.push_str("If-Match: \"");
                    body.push_str(etag);
                    body.push_str("\"\n\n");
                } else {
                    body.push_str("If-Match: *\n\n");
                }
                body.push_str(&payload);
                body.push_str("\n");
            }

            BatchOperation::Delete {
                ref row_key,
                ref etag,
            } => {
                body.push_str("DELETE ");
                body.push_str(uri_prefix);
                body.push_str(&entity_path(table, partition_key, row_key));
                body.push_str(" HTTP/1.1\n");
                body.push_str("Accept: application/json;odata=nometadata\n");
                body.push_str("Content-Type: application/json\n");
                if let Some(etag) = etag {
                    body.push_str("If-Match: \"");
                    body.push_str(etag);
                    body.push_str("\"\n");
                } else {
                    body.push_str("If-Match: *\n");
                }
                body.push_str("\n");
            }
        }
    }
}

pub struct Batch {
    partition_key: String,
    items: Vec<BatchOperation>,
}

#[derive(Serialize)]
struct InsertPayload<'a, T> {
    #[serde(rename = "RowKey")]
    row_key: &'a str,
    #[serde(rename = "PartitionKey")]
    partition_key: &'a str,
    #[serde(flatten)]
    payload: &'a T,
}

impl Batch {
    /// Create a new changeset for the given partition key
    pub fn new(partition_key: String) -> Batch {
        Batch {
            partition_key: partition_key,
            items: vec![],
        }
    }

    /// Return if batch contains no operation
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Return the number of contained operation
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Return if batch has the mxiumum number of operation.
    /// Note, it only checks for the operation count. Batch also has a size constraint which
    /// is checked (TBD) only during execution.
    pub fn is_full(&self) -> bool {
        self.items.len() >= BATCH_MAX_SIZE
    }

    /// Add a new operation.
    pub fn add_operation(&mut self, op: BatchOperation) -> Result<&mut Self, BatchError> {
        self.items.push(op);
        if self.is_full() {
            Err(BatchError::TooManyOperations)
        } else {
            Ok(self)
        }
    }

    /// Add an insert operation
    pub fn add_insert<T>(&mut self, row_key: String, data: &T) -> Result<&mut Self, BatchError>
    where
        T: Serialize,
    {
        let payload = serde_json::to_string(&InsertPayload {
            partition_key: &self.partition_key,
            row_key: &row_key,
            payload: data,
        })?;
        self.add_operation(BatchOperation::Insert { row_key, payload })
    }

    /// Add an insert operation using a TableEntitiy
    pub fn add_insert_entity<T>(&mut self, entity: TableEntity<T>) -> Result<&mut Self, BatchError>
    where
        T: Serialize,
    {
        if entity.partition_key != self.partition_key {
            Err(BatchError::UnexpectedPartitionKey)
        } else {
            self.add_insert(entity.row_key, &entity.payload)
        }
    }

    /// Add an update operation
    pub fn add_update<T>(
        &mut self,
        row_key: String,
        data: &T,
        etag: Option<String>,
    ) -> Result<&mut Self, BatchError>
    where
        T: Serialize,
    {
        self.add_operation(BatchOperation::Update {
            row_key: row_key.to_owned(),
            payload: serde_json::to_string(data)?,
            etag: etag,
        })
    }

    /// Add an update operation using a TableEntitiy
    pub fn add_update_entity<T>(&mut self, entity: TableEntity<T>) -> Result<&mut Self, BatchError>
    where
        T: Serialize,
    {
        if entity.partition_key != self.partition_key {
            Err(BatchError::UnexpectedPartitionKey)
        } else {
            self.add_update(entity.row_key, &entity.payload, entity.etag)
        }
    }

    /// Add a delete operation
    pub fn add_delete(
        &mut self,
        row_key: String,
        etag: Option<String>,
    ) -> Result<&mut Self, BatchError> {
        self.add_operation(BatchOperation::Delete {
            row_key: row_key.to_owned(),
            etag: etag,
        })
    }

    /// Add a delete operation using a TableEntitiy
    pub fn add_delete_entity<T>(
        &mut self,
        entity: TableEntity<T>,
    ) -> Result<&mut Self, BatchError> {
        if entity.partition_key != self.partition_key {
            Err(BatchError::UnexpectedPartitionKey)
        } else {
            self.add_delete(entity.row_key, entity.etag)
        }
    }

    pub(crate) fn into_payload(self, uri_prefix: &str, table: &str) -> String {
        let mut payload = String::default();
        payload.push_str("--batch_a1e9d677-b28b-435e-a89e-87e6a768a431\n");
        payload.push_str("Content-Type: multipart/mixed; boundary=changeset_8a28b620-b4bb-458c-a177-0959fb14c977\n\n");

        for item in self.items {
            payload.push_str("--changeset_8a28b620-b4bb-458c-a177-0959fb14c977\n");
            payload.push_str("Content-Type: application/http\n");
            payload.push_str("Content-Transfer-Encoding: binary\n\n");
            item.into_payload(uri_prefix, table, &self.partition_key, &mut payload);
            payload.push_str("\n");
        }

        payload.push_str("--changeset_8a28b620-b4bb-458c-a177-0959fb14c977--\n");
        payload.push_str("--batch_a1e9d677-b28b-435e-a89e-87e6a768a431\n");

        println!("{}", payload);

        payload
    }
}
