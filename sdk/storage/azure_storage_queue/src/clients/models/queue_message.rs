use serde::Deserialize;

use super::pub_models::QueueMessage;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueueMessagesList {
    #[serde(rename = "QueueMessage")]
    queue_message: Vec<QueueMessage>,
}
