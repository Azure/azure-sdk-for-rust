use azure_core::Request;

#[derive(Debug)]
pub struct TransactionOperation {
    pub(crate) request: Request,
}

impl TransactionOperation {
    pub(crate) fn new(request: Request) -> Self {
        Self { request }
    }
}
