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

    pub(crate) fn to_string(&self) -> Result<String, http::header::ToStrError> {
        let mut s = String::new();

        s.push_str(&format!(
            "--transaction_{}\nContent-Type: multipart/mixed; boundary=changeset_{}\n\n",
            self.batch_uuid.to_hyphenated_ref(),
            self.change_set_uuid.to_hyphenated_ref()
        ));

        for transaction_operation in self.transaction_operations.iter() {
            s.push_str(&format!("--changeset_{}\nContent-Type: application/http\nContent-Transfer-Encoding: binary\n\n", self.change_set_uuid.to_hyphenated_ref()));
            s.push_str(&format!(
                "{} {} HTTP/1.1\n",
                transaction_operation.request.method(),
                transaction_operation.request.uri()
            ));
            for (header_name, header_value) in transaction_operation.request.headers() {
                s.push_str(&format!("{}: {}\n", header_name, header_value.to_str()?));
            }

            s.push('\n');
            if !transaction_operation.request.body().is_empty() {
                s.push_str(transaction_operation.request.body());
                s.push('\n');
            }
        }

        s.push_str(&format!(
            "\n--changeset_{}--\n--transaction_{}\n",
            self.change_set_uuid.to_hyphenated_ref(),
            self.batch_uuid.to_hyphenated_ref(),
        ));

        Ok(s)
    }
}
