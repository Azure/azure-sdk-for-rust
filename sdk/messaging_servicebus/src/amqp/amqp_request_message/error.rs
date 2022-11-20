#[derive(Debug, Clone, thiserror::Error)]
pub enum CorrelationFilterError {
    #[error("Correlation filter must include at least one entry")]
    EmptyFilter,
}
