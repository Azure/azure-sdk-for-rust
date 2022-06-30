use azure_core::Request;

#[derive(Debug, Clone)]
pub struct TransactionOperation {
    pub(crate) request: Request,
}

impl TransactionOperation {
    pub(crate) fn new(request: Request) -> Self {
        Self { request }
    }
}
