#![cfg(feature = "arrow")]

use azure_data_kusto::prelude::{KustoResponseDataSetV2, ResultTable};
use std::path::PathBuf;

#[test]
fn asd() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/inputs/dataframe.json");

    let data = std::fs::read_to_string(path).unwrap();
    let tables: Vec<ResultTable> = serde_json::from_str(&data).unwrap();
    let response = KustoResponseDataSetV2 { tables };
    let record_batches = response.into_record_batches().collect::<Vec<_>>();
    println!("{:?}", record_batches)
}
