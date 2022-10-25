pub(crate) fn generate_identifier(entity_path: &str) -> String {
    format!("{}-{}", entity_path, uuid::Uuid::new_v4())
}
