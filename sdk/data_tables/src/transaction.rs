use crate::transaction_operation::TransactionOperation;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub(crate) struct TransactionOperations {
    batch_uuid: Uuid,
    change_set_uuid: Uuid,
    transaction_operations: Vec<TransactionOperation>,
}

impl TransactionOperations {
    pub(crate) fn new() -> Self {
        Self {
            batch_uuid: Uuid::new_v4(),
            change_set_uuid: Uuid::new_v4(),
            transaction_operations: Vec::new(),
        }
    }

    pub(crate) fn add(&mut self, transaction_operation: TransactionOperation) -> &mut Self {
        self.transaction_operations.push(transaction_operation);
        self
    }

    pub(crate) fn batch_uuid(&self) -> &Uuid {
        &self.batch_uuid
    }

    pub(crate) fn to_string(&self) -> azure_core::Result<String> {
        let mut s = String::new();

        s.push_str("--batch_");
        s.push_str(&self.batch_uuid.hyphenated().to_string());
        s.push_str("\nContent-Type: multipart/mixed; boundary=changeset_");
        s.push_str(&self.change_set_uuid.hyphenated().to_string());
        s.push_str("\n\n");

        for transaction_operation in self.transaction_operations.iter() {
            s.push_str("--changeset_");
            s.push_str(&self.change_set_uuid.hyphenated().to_string());
            s.push_str("\nContent-Type: application/http\nContent-Transfer-Encoding: binary\n\n");

            s.push_str(transaction_operation.request.method().as_ref());
            s.push(' ');
            s.push_str(transaction_operation.request.url().as_ref());
            s.push_str(" HTTP/1.1\n");

            for (header_name, header_value) in transaction_operation.request.headers().iter() {
                s.push_str(header_name.as_str());
                s.push_str(": ");
                s.push_str(header_value.as_str());
                s.push('\n');
            }

            s.push('\n');
            match transaction_operation.request.body() {
                azure_core::Body::Bytes(body) => {
                    if !body.is_empty() {
                        s.push_str(std::str::from_utf8(body)?);
                        s.push('\n');
                    }
                }
                #[cfg(not(target_arch = "wasm32"))]
                azure_core::Body::SeekableStream(_) => todo!(),
            }
        }

        s.push_str("--changeset_");
        s.push_str(&self.change_set_uuid.hyphenated().to_string());
        s.push_str("--\n--batch_");
        s.push_str(&self.batch_uuid.hyphenated().to_string());
        s.push('\n');

        Ok(s)
    }
}
