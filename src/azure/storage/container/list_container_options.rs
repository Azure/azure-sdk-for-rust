#[derive(Debug, Clone, PartialEq)]
pub struct ListContainerOptions {
    pub max_results: u32,
    pub include_metadata: bool,
    pub next_marker: Option<String>,
    pub prefix: Option<String>,
    pub timeout: Option<u64>,
}

pub const LIST_CONTAINER_OPTIONS_DEFAULT: ListContainerOptions = ListContainerOptions {
    max_results: 5000,
    include_metadata: false,
    next_marker: None,
    prefix: None,
    timeout: None,
};
