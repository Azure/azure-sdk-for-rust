use crate::primitives::sub_queue::SubQueue;

// const RULES_SUB_PATH: &str = "Rules";
// const SUB_QUEUE_PREFIX: &str = "$";
// const DEAD_LETTER_QUEUE_SUFFIX: &str = "DeadLetterQueue";
// const TRANSFER: &str = "Transfer";

const PATH_DELIMITER: &str = "/";
const SUBSCRIPTIONS_SUB_PATH: &str = "Subscriptions";
const DEAD_LETTER_QUEUE_NAME: &str = "$DeadLetterQueue"; // SubQueuePrefix + DeadLetterQueueSuffix;
const TRANSFER_DEAD_LETTER_QUEUE_NAME: &str = "$Transfer/$DeadLetterQueue"; // SubQueuePrefix + Transfer + PathDelimiter + DeadLetterQueueName;

pub(crate) fn format_subscription_path(topic_name: &str, subscription_name: &str) -> String {
    format!(
        "{topic_name}{PATH_DELIMITER}{SUBSCRIPTIONS_SUB_PATH}{PATH_DELIMITER}{subscription_name}"
    )
}

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

/// Formats the endpoint for the given service endpoint and entity path.
///
/// The `url::Url::join` is not used because it returns a Result that is likely infallible.
pub(crate) fn format_endpoint(
    service_endpoint: impl AsRef<str>,
    entity_path: impl AsRef<str>,
) -> String {
    let service_endpoint = service_endpoint.as_ref();
    let entity_path = entity_path.as_ref();
    match (
        service_endpoint.ends_with('/'),
        entity_path.starts_with('/'),
    ) {
        (true, true) => format!("{}{}", service_endpoint, &entity_path[1..]),
        (true, false) => format!("{}{}", service_endpoint, entity_path),
        (false, true) => format!("{}{}", service_endpoint, entity_path),
        (false, false) => format!("{}/{}", service_endpoint, entity_path),
    }
}
