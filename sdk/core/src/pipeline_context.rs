use crate::Context;

/// Pipeline internal execution context.
///
/// During a pipeline execution, context will be passed from the function
/// starting the pipeline down to each pipeline policy. This struct it the
/// one that will actually passed down a pipeline. It's the pipeline
/// responsibility to specify the C generic data type: it can be anything as
/// long as it's Send and Sync. For example, Cosmos uses the
/// PipelineContext to pass the ResourceType down to its
/// AuthorizationPolicy.
pub struct PipelineContext<C>
where
    C: Send + Sync,
{
    inner_context: Context,
    contents: C,
}

impl<C> PipelineContext<C>
where
    C: Send + Sync,
{
    pub fn new(inner_context: Context, contents: C) -> Self {
        Self {
            inner_context,
            contents,
        }
    }

    pub fn set_contents(&mut self, contents: C) {
        self.contents = contents;
    }

    pub fn get_contents(&self) -> &C {
        &self.contents
    }

    pub fn get_contents_mut(&mut self) -> &mut C {
        &mut self.contents
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
