use crate::Context;

/// Pipeline internal execution context.
///
/// During a pipeline execution, context will be passed from the function
/// starting the pipeline down to each pipeline policy. This struct it the
/// one that will actually passed down a pipeline. It's the pipeline
/// responsibility to specify the BAG generics: it can be anything as
/// long as it's Send and Sync. For example, Cosmos uses the
/// PipelineContext to pass the ResourceType down to its
/// AuthorizationPolicy.
pub struct PipelineContext<BAG>
where
    BAG: Send + Sync,
{
    inner_context: Context,
    bag: BAG,
}

impl<BAG> PipelineContext<BAG>
where
    BAG: Send + Sync,
{
    pub fn new(inner_context: Context, bag: BAG) -> Self {
        Self { inner_context, bag }
    }

    pub fn set_bag(&mut self, bag: BAG) {
        self.bag = bag;
    }

    pub fn get_bag(&self) -> &BAG {
        &self.bag
    }

    pub fn get_bag_mut(&mut self) -> &mut BAG {
        &mut self.bag
    }

    pub fn set_inner_context(&mut self, inner_context: Context) {
        self.inner_context = inner_context;
    }

    pub fn get_inner_context(&self) -> &Context {
        &self.inner_context
    }

    pub fn get_inner_context_mut(&mut self) -> &mut Context {
        &mut self.inner_context
    }
}
