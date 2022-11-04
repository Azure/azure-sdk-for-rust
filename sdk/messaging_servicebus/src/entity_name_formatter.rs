use crate::primitives::sub_queue::SubQueue;

const PATH_DELIMITER: &str = "/";
const SUBSCRIPTIONS_SUB_PATH: &str = "Subscriptions";
const RULES_SUB_PATH: &str = "Rules";
const SUB_QUEUE_PREFIX: &str = "$";
const DEAD_LETTER_QUEUE_SUFFIX: &str = "DeadLetterQueue";
const DEAD_LETTER_QUEUE_NAME: &str = "$DeadLetterQueue"; // SubQueuePrefix + DeadLetterQueueSuffix;
const TRANSFER: &str = "Transfer";
const TRANSFER_DEAD_LETTER_QUEUE_NAME: &str = "$Transfer/$DeadLetterQueue"; // SubQueuePrefix + Transfer + PathDelimiter + DeadLetterQueueName;

pub(crate) fn format_entity_path(entity_path: String, sub_queue: SubQueue) -> String {
    match sub_queue {
        SubQueue::None => entity_path,
        SubQueue::DeadLetter => format_dead_letter_path(entity_path),
        SubQueue::TransferDeadLetter => format_transfer_dead_letter_path(entity_path),
    }
}

fn format_dead_letter_path(entity_path: String) -> String {
    format!(
        "{}{}{}",
        entity_path, PATH_DELIMITER, DEAD_LETTER_QUEUE_NAME
    )
}

fn format_transfer_dead_letter_path(entity_path: String) -> String {
    format!(
        "{}{}{}",
        entity_path, PATH_DELIMITER, TRANSFER_DEAD_LETTER_QUEUE_NAME
    )
}
