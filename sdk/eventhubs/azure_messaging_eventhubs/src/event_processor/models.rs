// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

// Note that this module returns azure_core errors, *not* eventhub errors. That is because these structures are used by checkpoint stores which always return azure_core errors.
use crate::StartPosition;
use azure_core::{
    error::ErrorKind as AzureErrorKind, fmt::to_ascii_lowercase, http::Etag, time::OffsetDateTime,
    Error, Result,
};
use std::collections::HashMap;

/// Represents a checkpoint in an Event Hub.
///
/// This structure is used to track the progress of event processing
/// by storing the offset and sequence number of the last processed event
/// for a specific partition. It helps in resuming event processing from
/// the correct position in case of failures or restarts.
#[derive(Debug, Default, Clone)]
pub struct Checkpoint {
    /// The fully qualified namespace of the Event Hub.
    pub fully_qualified_namespace: String,
    /// The name of the Event Hub.
    pub event_hub_name: String,
    /// The name of the consumer group.
    pub consumer_group: String,
    /// The identifier of the partition.
    pub partition_id: String,
    /// The offset of the last processed event.
    pub offset: Option<String>,
    /// The sequence number of the last processed event.
    pub sequence_number: Option<i64>,
}

macro_rules! check_non_empty_parameter(
    ($field:expr) => {
        if $field.is_empty() {
            return Err(Error::with_message(AzureErrorKind::Other,
                String::from("Required field ") + stringify!($field) + " is empty",
            ));
        }
    }
);

impl Checkpoint {
    /// Returns the prefix for the checkpoint blob name.
    ///
    /// The layout is `{namespace}/{event hub}/{consumer group}/checkpoint/`.
    ///
    /// The namespace, the event hub name, and the consumer group fold to
    /// lowercase. The fold applies to ASCII letters only, and it leaves all
    /// other characters unchanged. Event Hubs treats these three names as
    /// case insensitive, so the fold keeps one prefix for one Event Hub.
    pub fn get_checkpoint_blob_prefix_name(
        fully_qualified_namespace: &str,
        event_hub_name: &str,
        consumer_group: &str,
    ) -> Result<String> {
        check_non_empty_parameter!(fully_qualified_namespace);
        check_non_empty_parameter!(event_hub_name);
        check_non_empty_parameter!(consumer_group);
        Ok(format!(
            "{}/{}/{}/checkpoint/",
            to_ascii_lowercase(fully_qualified_namespace),
            to_ascii_lowercase(event_hub_name),
            to_ascii_lowercase(consumer_group)
        ))
    }

    /// Returns the full name of the checkpoint blob.
    ///
    /// The layout is
    /// `{namespace}/{event hub}/{consumer group}/checkpoint/{partition id}`.
    ///
    /// The namespace, the event hub name, and the consumer group fold to
    /// lowercase. The fold applies to ASCII letters only, and it leaves all
    /// other characters unchanged. The partition id keeps its case.
    pub fn get_checkpoint_blob_name(
        fully_qualified_namespace: &str,
        event_hub_name: &str,
        consumer_group: &str,
        partition_id: &str,
    ) -> Result<String> {
        check_non_empty_parameter!(partition_id);
        Ok(Self::get_checkpoint_blob_prefix_name(
            fully_qualified_namespace,
            event_hub_name,
            consumer_group,
        )? + partition_id)
    }
}

/// Represents the ownership information for a partition in an Event Hub.
///
/// This structure is used to manage and track the ownership of partitions
/// by different consumers in a consumer group. It helps in load balancing
/// and ensuring that each partition is processed by only one consumer at a time.
#[derive(Debug, Default, Clone)]
pub struct Ownership {
    /// The fully qualified namespace of the Event Hub.
    pub fully_qualified_namespace: String,
    /// The name of the Event Hub.
    pub event_hub_name: String,
    /// The name of the consumer group.
    pub consumer_group: String,
    /// The identifier of the partition.
    pub partition_id: String,

    /// The identifier of the owner (consumer) of the partition.
    pub owner_id: Option<String>,
    /// The ETag associated with the ownership.
    pub etag: Option<Etag>,
    /// The last modified time of the ownership.
    pub last_modified_time: Option<OffsetDateTime>,
}

impl Ownership {
    /// Returns the prefix for the ownership blob name.
    ///
    /// The layout is `{namespace}/{event hub}/{consumer group}/ownership/`.
    ///
    /// The namespace, the event hub name, and the consumer group fold to
    /// lowercase. The fold applies to ASCII letters only, and it leaves all
    /// other characters unchanged. Event Hubs treats these three names as
    /// case insensitive, so the fold keeps one prefix for one Event Hub.
    pub fn get_ownership_prefix_name(
        fully_qualified_namespace: &str,
        event_hub_name: &str,
        consumer_group: &str,
    ) -> Result<String> {
        check_non_empty_parameter!(fully_qualified_namespace);
        check_non_empty_parameter!(event_hub_name);
        check_non_empty_parameter!(consumer_group);
        Ok(format!(
            "{}/{}/{}/ownership/",
            to_ascii_lowercase(fully_qualified_namespace),
            to_ascii_lowercase(event_hub_name),
            to_ascii_lowercase(consumer_group)
        ))
    }

    /// Returns the full name of the ownership blob.
    ///
    /// The layout is
    /// `{namespace}/{event hub}/{consumer group}/ownership/{partition id}`.
    ///
    /// The namespace, the event hub name, and the consumer group fold to
    /// lowercase. The fold applies to ASCII letters only, and it leaves all
    /// other characters unchanged. The partition id keeps its case.
    pub fn get_ownership_name(
        fully_qualified_namespace: &str,
        event_hub_name: &str,
        consumer_group: &str,
        partition_id: &str,
    ) -> Result<String> {
        check_non_empty_parameter!(fully_qualified_namespace);
        check_non_empty_parameter!(event_hub_name);
        check_non_empty_parameter!(consumer_group);
        check_non_empty_parameter!(partition_id);
        Ok(Self::get_ownership_prefix_name(
            fully_qualified_namespace,
            event_hub_name,
            consumer_group,
        )? + partition_id)
    }
}

/// Represents the starting position for event processing in an Event Hub.
///
/// This structure is used to specify the starting position for each partition
/// when processing events. It allows for flexibility in choosing the starting
/// position based on various criteria, such as the latest event, a specific
/// offset, or a specific sequence number.
#[derive(Debug, Default)]
pub struct StartPositions {
    /// The starting position for each partition in the Event Hub.
    /// The key is the partition ID, and the value is the starting position.
    /// The starting position can be specified as a specific offset, sequence number,
    /// or the latest event.
    pub per_partition: HashMap<String, StartPosition>,

    /// The default starting position for all partitions in the Event Hub.
    /// This position is used if no specific starting position is provided for a partition.
    /// The default starting position can be specified as a specific offset, sequence number,
    /// or the latest event.
    pub default: StartPosition,
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::error::ErrorKind;

    const NS_MIXED: &str = "NS-Test.ServiceBus.Windows.Net";
    const NS_LOWER: &str = "ns-test.servicebus.windows.net";
    const HUB_MIXED: &str = "My-EventHub";
    const HUB_LOWER: &str = "my-eventhub";
    const GROUP_MIXED: &str = "$Default";
    const GROUP_LOWER: &str = "$default";
    const PARTITION: &str = "Partition-A";

    /// Each of the three key fields folds to lowercase on its own, so a
    /// difference in one field cannot move the blob key.
    #[test]
    fn key_fields_fold_to_lowercase_independently() {
        let rows = [
            ("namespace only", NS_MIXED, HUB_LOWER, GROUP_LOWER),
            ("event hub name only", NS_LOWER, HUB_MIXED, GROUP_LOWER),
            ("consumer group only", NS_LOWER, HUB_LOWER, GROUP_MIXED),
            ("all three", NS_MIXED, HUB_MIXED, GROUP_MIXED),
        ];
        let expected_checkpoint = format!("{NS_LOWER}/{HUB_LOWER}/{GROUP_LOWER}/checkpoint/");
        let expected_ownership = format!("{NS_LOWER}/{HUB_LOWER}/{GROUP_LOWER}/ownership/");

        // One run reports every row, so a partial fold shows all of its
        // damage at once.
        let mut mismatches = Vec::new();
        for (label, namespace, event_hub_name, consumer_group) in rows {
            let checkpoint = Checkpoint::get_checkpoint_blob_prefix_name(
                namespace,
                event_hub_name,
                consumer_group,
            )
            .unwrap();
            if checkpoint != expected_checkpoint {
                mismatches.push(format!(
                    "{label}: checkpoint prefix is {checkpoint:?}, expected {expected_checkpoint:?}"
                ));
            }

            let ownership =
                Ownership::get_ownership_prefix_name(namespace, event_hub_name, consumer_group)
                    .unwrap();
            if ownership != expected_ownership {
                mismatches.push(format!(
                    "{label}: ownership prefix is {ownership:?}, expected {expected_ownership:?}"
                ));
            }
        }

        assert!(
            mismatches.is_empty(),
            "the key fields did not fold to lowercase:\n{}",
            mismatches.join("\n")
        );
    }

    #[test]
    fn checkpoint_blob_name_folds_key_and_keeps_partition_id_case() {
        let name =
            Checkpoint::get_checkpoint_blob_name(NS_MIXED, HUB_MIXED, GROUP_MIXED, PARTITION);
        assert_eq!(
            name.unwrap(),
            "ns-test.servicebus.windows.net/my-eventhub/$default/checkpoint/Partition-A"
        );
    }

    #[test]
    fn ownership_name_folds_key_and_keeps_partition_id_case() {
        let name = Ownership::get_ownership_name(NS_MIXED, HUB_MIXED, GROUP_MIXED, PARTITION);
        assert_eq!(
            name.unwrap(),
            "ns-test.servicebus.windows.net/my-eventhub/$default/ownership/Partition-A"
        );
    }

    /// Two callers that spell the same Event Hub with a different case must
    /// land on one key set. This test does not pin the fold direction, which
    /// is the job of the tests above.
    #[test]
    fn key_is_stable_across_input_case() {
        assert_eq!(
            Checkpoint::get_checkpoint_blob_prefix_name(NS_LOWER, HUB_LOWER, GROUP_MIXED).unwrap(),
            Checkpoint::get_checkpoint_blob_prefix_name(NS_LOWER, HUB_LOWER, GROUP_LOWER).unwrap(),
            "the consumer group case moved the checkpoint prefix"
        );
        assert_eq!(
            Ownership::get_ownership_prefix_name(NS_LOWER, HUB_LOWER, GROUP_MIXED).unwrap(),
            Ownership::get_ownership_prefix_name(NS_LOWER, HUB_LOWER, GROUP_LOWER).unwrap(),
            "the consumer group case moved the ownership prefix"
        );
        assert_eq!(
            Checkpoint::get_checkpoint_blob_name(NS_LOWER, HUB_LOWER, GROUP_MIXED, PARTITION)
                .unwrap(),
            Checkpoint::get_checkpoint_blob_name(NS_LOWER, HUB_LOWER, GROUP_LOWER, PARTITION)
                .unwrap(),
            "the consumer group case moved the checkpoint blob name"
        );
        assert_eq!(
            Ownership::get_ownership_name(NS_LOWER, HUB_LOWER, GROUP_MIXED, PARTITION).unwrap(),
            Ownership::get_ownership_name(NS_LOWER, HUB_LOWER, GROUP_LOWER, PARTITION).unwrap(),
            "the consumer group case moved the ownership name"
        );

        assert_eq!(
            Checkpoint::get_checkpoint_blob_name(NS_MIXED, HUB_LOWER, GROUP_LOWER, PARTITION)
                .unwrap(),
            Checkpoint::get_checkpoint_blob_name(NS_LOWER, HUB_LOWER, GROUP_LOWER, PARTITION)
                .unwrap(),
            "the namespace case moved the checkpoint blob name"
        );
        assert_eq!(
            Checkpoint::get_checkpoint_blob_name(NS_LOWER, HUB_MIXED, GROUP_LOWER, PARTITION)
                .unwrap(),
            Checkpoint::get_checkpoint_blob_name(NS_LOWER, HUB_LOWER, GROUP_LOWER, PARTITION)
                .unwrap(),
            "the event hub name case moved the checkpoint blob name"
        );
    }

    /// An empty key field stays an error, and the message keeps the name of
    /// the field that is empty. Each case leaves exactly one field empty,
    /// because the two `*_name` functions check the fields in a different
    /// order.
    #[test]
    fn key_functions_reject_empty_parameters() {
        let cases: Vec<(Result<String>, &str)> = vec![
            (
                Checkpoint::get_checkpoint_blob_prefix_name("", "hub", "group"),
                "Required field fully_qualified_namespace is empty",
            ),
            (
                Checkpoint::get_checkpoint_blob_prefix_name(NS_LOWER, "", "group"),
                "Required field event_hub_name is empty",
            ),
            (
                Checkpoint::get_checkpoint_blob_prefix_name(NS_LOWER, "hub", ""),
                "Required field consumer_group is empty",
            ),
            (
                Ownership::get_ownership_prefix_name("", "hub", "group"),
                "Required field fully_qualified_namespace is empty",
            ),
            (
                Ownership::get_ownership_prefix_name(NS_LOWER, "", "group"),
                "Required field event_hub_name is empty",
            ),
            (
                Ownership::get_ownership_prefix_name(NS_LOWER, "hub", ""),
                "Required field consumer_group is empty",
            ),
            (
                Checkpoint::get_checkpoint_blob_name(NS_LOWER, "hub", "group", ""),
                "Required field partition_id is empty",
            ),
            (
                Ownership::get_ownership_name(NS_LOWER, "hub", "group", ""),
                "Required field partition_id is empty",
            ),
        ];

        for (result, expected_message) in cases {
            let error = result.expect_err(expected_message);
            assert_eq!(
                *error.kind(),
                ErrorKind::Other,
                "wrong error kind for {expected_message:?}"
            );
            assert_eq!(error.to_string(), expected_message);
        }
    }

    /// The fold is an ASCII fold, not a Unicode fold. This test guards the
    /// implementation choice. A non-ASCII identifier is not a supported
    /// Event Hubs scenario, because the service limits these names to ASCII.
    #[test]
    fn fold_is_ascii_only() {
        // A trailing capital sigma folds to the final sigma form under
        // `str::to_lowercase`, because that rule depends on the position in
        // the word. An ASCII fold leaves both Greek letters alone.
        let name = Checkpoint::get_checkpoint_blob_name(
            NS_LOWER,
            HUB_LOWER,
            "$Default-\u{0391}\u{03A3}",
            "0",
        );
        assert_eq!(
            name.unwrap(),
            "ns-test.servicebus.windows.net/my-eventhub/$default-\u{0391}\u{03A3}/checkpoint/0"
        );
    }
}
