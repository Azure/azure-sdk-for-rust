use http::request::Request;

#[derive(Debug)]
pub struct TransactionOperation {
    pub(crate) request: Request<String>,
}

impl TransactionOperation {
    pub(crate) fn new(request: Request<String>) -> Self {
        Self { request }
    }
}
