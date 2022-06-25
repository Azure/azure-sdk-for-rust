use crate::TransactionOperation;
use uuid::Uuid;

#[derive(Debug)]
pub struct Transaction {
    batch_uuid: Uuid,
    change_set_uuid: Uuid,
    transaction_operations: Vec<TransactionOperation>,
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            batch_uuid: Uuid::new_v4(),
            change_set_uuid: Uuid::new_v4(),
            transaction_operations: Vec::new(),
        }
    }
}

impl Transaction {
    pub fn add(&mut self, transaction_operation: TransactionOperation) -> &mut Self {
        self.transaction_operations.push(transaction_operation);
        self
    }

    pub(crate) fn batch_uuid(&self) -> &Uuid {
        &self.batch_uuid
    }

    pub(crate) fn to_string(&self) -> azure_core::Result<String> {
        let mut s = String::new();

        s.push_str(&format!(
            "--batch_{}\nContent-Type: multipart/mixed; boundary=changeset_{}\n\n",
            self.batch_uuid.hyphenated(),
            self.change_set_uuid.hyphenated()
        ));

        for transaction_operation in self.transaction_operations.iter() {
            s.push_str(&format!("--changeset_{}\nContent-Type: application/http\nContent-Transfer-Encoding: binary\n\n", self.change_set_uuid.hyphenated()));
            s.push_str(&format!(
                "{:?} {} HTTP/1.1\n",
                transaction_operation.request.method(),
                transaction_operation.request.url()
            ));
            for (header_name, header_value) in transaction_operation.request.headers().iter() {
                s.push_str(&format!(
                    "{}: {}\n",
                    header_name.as_str(),
                    header_value.as_str()
                ));
            }

            s.push('\n');
            match transaction_operation.request.body() {
                azure_core::Body::Bytes(body) => {
                    if !body.is_empty() {
                        s.push_str(std::str::from_utf8(body)?);
                        s.push('\n');
                    }
                }
                azure_core::Body::SeekableStream(_) => todo!(),
            }
        }

        s.push_str(&format!(
            "\n--changeset_{}--\n--batch_{}\n",
            self.change_set_uuid.hyphenated(),
            self.batch_uuid.hyphenated(),
        ));

        Ok(s)
    }
}
