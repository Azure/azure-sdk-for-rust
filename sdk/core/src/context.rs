/// Pipeline execution context.
#[derive(Clone, Debug, Default)]
pub struct Context {
    _priv: (),
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }
}
