pub mod operations;

#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub sku_name: String,
    pub kind: String,
}
