#[async_trait::async_trait]
pub trait ContextHolder {
    async fn get_context(&self) -> AppContext;
}

#[derive(Debug, Clone)]
pub struct AppContext {
    id: String,
    groups: Vec<String>,
}

impl AppContext {
    pub fn new(id: String, groups: Vec<String>) -> AppContext {
        AppContext { id, groups }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_groups(&self) -> Vec<String> {
        self.groups.clone()
    }
}
