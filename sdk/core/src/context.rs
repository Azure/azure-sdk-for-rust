/// Pipeline execution context.
///
/// During a pipeline execution, context will be passed from the function starting the
/// pipeline down to each pipeline policy. Contrarily to the Request, the context can be mutated
/// by each pipeline policy and is not reset between retries. It can be used to pass the whole
/// pipeline execution history between policies.
/// For example, it could be used to signal that an execution failed because a CosmosDB endpoint is
/// down and the appropriate policy should try the next one).
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
