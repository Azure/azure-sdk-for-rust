#[derive(Clone)]
pub struct Context {
    // Temporary hack to make sure that Context is not initializeable
    // Soon Context will have proper data fields
    _priv: (),
}

impl Context {
    pub fn new() -> Self {
        Self { _priv: () }
    }
}
