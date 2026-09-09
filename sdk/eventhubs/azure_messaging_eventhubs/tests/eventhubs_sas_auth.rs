// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

// cspell:ignore sastoken skn fexample fmyhub myhub mykey WDCF Bfsxrm Lsorq

//! Tests for Event Hubs Shared Access Signature (SAS) authentication and for
//! the connection options that go with it: a minted pre-formed token, a
//! `Send`-only rule, a `Listen`-only rule, and a custom endpoint.
//!
//! The tests read these environment variables:
//!
//! - `EVENTHUBS_HOST`: the fully qualified namespace.
//! - `EVENTHUB_NAME`: the Event Hub name.
//! - `EVENTHUBS_CONNECTION_STRING`: a connection string with `Manage` rights.
//! - `EVENTHUBS_SEND_ONLY_CONNECTION_STRING`: the primary connection string of
//!   the namespace-scope `SendOnly` rule in `sdk/eventhubs/test-resources.bicep`.
//! - `EVENTHUBS_LISTEN_ONLY_CONNECTION_STRING`: the primary connection string of
//!   the namespace-scope `ListenOnly` rule in the same template.
//!
//! Operator precondition: that template sets `disableLocalAuth: !tenantIsTME`,
//! so every SAS test here passes only against a namespace that has local
//! authentication enabled. Read the two rule connection strings from the portal
//! or from the Azure CLI, and export them before you run the file live.

use azure_core::Uuid;
use azure_core_test::{recorded, TestContext};
use azure_messaging_eventhubs::{
    models::{AmqpSimpleValue, EventData, ReceivedEventData},
    ConnectionString, ConsumerClient, EventHubsError, OpenReceiverOptions, ProducerClient,
    RetryOptions, SendEventOptions, StartLocation, StartPosition,
};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use futures::StreamExt;
use hmac::{Hmac, Mac};
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use sha2::Sha256;
use std::{
    env,
    error::Error,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::time::timeout;

/// Partition "1" belongs to `eventhubs_round_trip.rs` and partition "2" to
/// another test file, so this file keeps to its own of the four the hub has.
const TEST_PARTITION: &str = "3";

const RUN_MARKER_KEY: &str = "sas-auth-run";

const READ_DEADLINE: Duration = Duration::from_secs(60);
const OP_DEADLINE: Duration = Duration::from_secs(90);
const SAS_TTL_SECONDS: i64 = 3600;
const UNREACHABLE_ENDPOINT: &str = "amqps://127.0.0.1:1";

static SERIAL: async_lock::Mutex<()> = async_lock::Mutex::new(());

/// The percent-encoding set of Go's `url.QueryEscape`, which the broker uses.
const SAS_ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~');

/// Builds a SAS token for `amqps://{fqdn}/{eventhub}`, valid until `expiry`
/// (Unix seconds). This signer is independent of the crate's private one, so it
/// catches a change of the output. The `amqps://` audience is required: the
/// client claims that exact path, and SAS scope is a prefix match, so an
/// `sb://` token gives a spurious unauthorized failure.
fn mint_sas_token(fqdn: &str, eventhub: &str, key_name: &str, key: &str, expiry: i64) -> String {
    let audience = format!("amqps://{fqdn}/{eventhub}");
    let resource = utf8_percent_encode(&audience, SAS_ENCODE_SET)
        .to_string()
        .to_lowercase();
    let string_to_sign = format!("{resource}\n{expiry}");

    // The key signs as raw bytes, never base64-decoded first.
    let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes())
        .expect("HMAC-SHA256 takes a key of any length");
    mac.update(string_to_sign.as_bytes());
    let signature = BASE64_STANDARD.encode(mac.finalize().into_bytes());
    // The resource is lowercased, the signature is not.
    let signature = utf8_percent_encode(&signature, SAS_ENCODE_SET).to_string();

    format!("SharedAccessSignature sr={resource}&sig={signature}&se={expiry}&skn={key_name}")
}

/// Builds a key-free connection string, with the Event Hub it is scoped to.
fn preformed_sas_connection_string() -> Result<(String, String), Box<dyn Error>> {
    let parsed: ConnectionString = env::var("EVENTHUBS_CONNECTION_STRING")?.parse()?;
    let key_name = parsed
        .shared_access_key_name
        .clone()
        .ok_or("EVENTHUBS_CONNECTION_STRING has no SharedAccessKeyName to sign with")?;
    let key = parsed
        .shared_access_key
        .clone()
        .ok_or("EVENTHUBS_CONNECTION_STRING has no SharedAccessKey to sign with")?;
    let eventhub = env::var("EVENTHUB_NAME")
        .ok()
        .or_else(|| parsed.entity_path.clone())
        .ok_or("set EVENTHUB_NAME, or give EVENTHUBS_CONNECTION_STRING an EntityPath")?;

    let expiry = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64 + SAS_TTL_SECONDS;
    let token = mint_sas_token(
        &parsed.fully_qualified_namespace,
        &eventhub,
        &key_name,
        key.secret(),
        expiry,
    );

    Ok((
        format!(
            "Endpoint=sb://{}/;SharedAccessSignature={token}",
            parsed.fully_qualified_namespace
        ),
        eventhub,
    ))
}

/// Reports whether the broker refused the operation for lack of rights. The
/// public `ErrorKind` has no `Unauthorized` variant, and the broker refuses
/// at either hop with a different rendering: a refused attach as an AMQP
/// described error with `condition: UnauthorizedAccess`, a refused CBS
/// put-token as a management `status code: 401`. The full description carries a
/// TrackingId and a Timestamp that change every run, so match the stable part.
fn looks_unauthorized(error: &EventHubsError) -> bool {
    let rendered = format!("{error:?}").to_lowercase();
    rendered.contains("unauthorized") || rendered.contains("401")
}

fn run_marker(test_name: &str) -> String {
    format!("{test_name}-{}", Uuid::new_v4())
}

fn marked_event(run_marker: &str) -> EventData {
    EventData::builder()
        .with_body(b"eventhubs SAS auth test")
        .add_property(RUN_MARKER_KEY.to_string(), run_marker.to_string())
        .build()
}

fn tagged(event: &ReceivedEventData, run_marker: &str) -> bool {
    matches!(
        event
            .event_data()
            .properties()
            .and_then(|properties| properties.get(RUN_MARKER_KEY)),
        Some(AmqpSimpleValue::String(marker)) if marker.as_str() == run_marker
    )
}

/// Pins the minted token against the vector the crate's own signer is tested
/// with: the encoding, the field order, and the `sr`/`sig` case asymmetry.
#[test]
fn minted_sas_token_matches_the_crate_reference_vector() {
    let token = mint_sas_token(
        "example.servicebus.windows.net",
        "myhub",
        "RootManageSharedAccessKey",
        "mykey",
        1_700_000_000,
    );

    assert_eq!(
        token,
        "SharedAccessSignature \
         sr=amqps%3a%2f%2fexample.servicebus.windows.net%2fmyhub\
         &sig=SgJoMn7K6nWDCF6e1%2BfsxrmJLsorqPeZ3B8N1uQ31dc%3D\
         &se=1700000000\
         &skn=RootManageSharedAccessKey"
    );

    let connection_string =
        format!("Endpoint=sb://example.servicebus.windows.net/;SharedAccessSignature={token}");
    let parsed: ConnectionString = connection_string
        .parse()
        .expect("a pre-formed SAS connection string parses");

    assert!(parsed.shared_access_signature.is_some());
    assert!(parsed.shared_access_key.is_none());
    assert!(parsed.shared_access_key_name.is_none());
}

#[recorded::test(live)]
async fn preformed_sas_producer_sends(_ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const TEST_NAME: &str = "preformed_sas_producer_sends";
    let _serial = SERIAL.lock().await;

    let (connection_string, eventhub) = preformed_sas_connection_string()?;
    let producer = ProducerClient::builder()
        .with_application_id(TEST_NAME.to_string())
        .open_with_connection_string(&connection_string, Some(&eventhub))
        .await?;

    // A successful open proves nothing about the token: it ends at
    // `ensure_connection`, which never authorizes a path.
    let properties = producer.get_eventhub_properties().await?;
    let run_marker = run_marker(TEST_NAME);
    let result = producer
        .send_event(
            marked_event(&run_marker),
            Some(SendEventOptions {
                partition_id: Some(TEST_PARTITION.to_string()),
            }),
        )
        .await;

    producer.close().await?;

    assert!(!properties.partition_ids.is_empty());
    assert!(
        result.is_ok(),
        "the broker refused a send under a pre-formed SAS token: {result:?}"
    );
    Ok(())
}

#[recorded::test(live)]
async fn preformed_sas_consumer_receives(_ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const TEST_NAME: &str = "preformed_sas_consumer_receives";
    let _serial = SERIAL.lock().await;

    let (connection_string, eventhub) = preformed_sas_connection_string()?;
    let producer = ProducerClient::builder()
        .with_application_id(TEST_NAME.to_string())
        .open_with_connection_string(&connection_string, Some(&eventhub))
        .await?;
    let start = producer
        .get_partition_properties(TEST_PARTITION)
        .await?
        .last_enqueued_sequence_number;

    let run_marker = run_marker(TEST_NAME);
    producer
        .send_event(
            marked_event(&run_marker),
            Some(SendEventOptions {
                partition_id: Some(TEST_PARTITION.to_string()),
            }),
        )
        .await?;

    let consumer = ConsumerClient::builder()
        .with_application_id(TEST_NAME.to_string())
        .open_with_connection_string(&connection_string, Some(&eventhub))
        .await?;
    let receiver = consumer
        .open_receiver_on_partition(
            TEST_PARTITION.to_string(),
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: StartLocation::SequenceNumber(start),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .await?;

    let read = {
        let mut stream = receiver.stream_events();
        timeout(READ_DEADLINE, async {
            while let Some(event) = stream.next().await {
                if tagged(&event?, &run_marker) {
                    return Ok::<bool, EventHubsError>(true);
                }
            }
            Ok(false)
        })
        .await
    };

    receiver.close().await?;
    consumer.close().await?;
    producer.close().await?;

    assert!(
        matches!(read, Ok(Ok(true))),
        "a pre-formed SAS consumer did not read the tagged event: {read:?}"
    );
    Ok(())
}

#[recorded::test(live)]
async fn send_only_rule_can_send(_ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const TEST_NAME: &str = "send_only_rule_can_send";
    let _serial = SERIAL.lock().await;

    let connection_string = env::var("EVENTHUBS_SEND_ONLY_CONNECTION_STRING")?;
    let eventhub = env::var("EVENTHUB_NAME").ok();
    let producer = ProducerClient::builder()
        .with_application_id(TEST_NAME.to_string())
        .open_with_connection_string(&connection_string, eventhub.as_deref())
        .await?;

    // A `Send`-only rule has no read right, so this test must not ask for the
    // Event Hub or the partition properties: a management call would fail and
    // report a defect that is not there. Hence the constant partition.
    let run_marker = run_marker(TEST_NAME);
    let result = producer
        .send_event(
            marked_event(&run_marker),
            Some(SendEventOptions {
                partition_id: Some(TEST_PARTITION.to_string()),
            }),
        )
        .await;

    producer.close().await?;

    assert!(
        result.is_ok(),
        "the broker refused a send under a Send-only rule: {result:?}"
    );
    Ok(())
}

#[recorded::test(live)]
async fn send_only_rule_cannot_receive(_ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const TEST_NAME: &str = "send_only_rule_cannot_receive";
    let _serial = SERIAL.lock().await;

    let connection_string = env::var("EVENTHUBS_SEND_ONLY_CONNECTION_STRING")?;
    let eventhub = env::var("EVENTHUB_NAME").ok();

    // The open must succeed: it ends at `ensure_connection`, which never
    // authorizes a path.
    let consumer = ConsumerClient::builder()
        .with_application_id(TEST_NAME.to_string())
        .open_with_connection_string(&connection_string, eventhub.as_deref())
        .await?;

    // This must succeed too: it only builds an `EventReceiver`.
    let receiver = consumer
        .open_receiver_on_partition(TEST_PARTITION.to_string(), None)
        .await?;

    // The first read attaches the link, and that is where the broker refuses.
    let first = {
        let mut stream = receiver.stream_events();
        timeout(OP_DEADLINE, stream.next()).await
    };

    receiver.close().await?;
    consumer.close().await?;

    let first = first.expect("the Send-only read did not finish before the deadline");
    match first {
        Some(Err(error)) => assert!(
            looks_unauthorized(&error),
            "the Send-only read failed for another reason: {error:?}"
        ),
        other => panic!("a Send-only rule read from the partition: {other:?}"),
    }
    Ok(())
}

#[recorded::test(live)]
async fn listen_only_rule_can_receive(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const TEST_NAME: &str = "listen_only_rule_can_receive";
    let _serial = SERIAL.lock().await;

    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let connection_string = env::var("EVENTHUBS_LISTEN_ONLY_CONNECTION_STRING")?;

    // A separate Entra producer seeds the partition and reads its tail: the
    // Listen-only client must make no management call at all.
    let producer = ProducerClient::builder()
        .with_application_id(TEST_NAME.to_string())
        .open(
            host.as_str(),
            eventhub.as_str(),
            ctx.recording().credential(),
        )
        .await?;
    let start = producer
        .get_partition_properties(TEST_PARTITION)
        .await?
        .last_enqueued_sequence_number;

    let run_marker = run_marker(TEST_NAME);
    producer
        .send_event(
            marked_event(&run_marker),
            Some(SendEventOptions {
                partition_id: Some(TEST_PARTITION.to_string()),
            }),
        )
        .await?;

    let consumer = ConsumerClient::builder()
        .with_application_id(TEST_NAME.to_string())
        .open_with_connection_string(&connection_string, Some(eventhub.as_str()))
        .await?;
    // The start position is the captured tail: `Latest` would race the attach
    // against the send above.
    let receiver = consumer
        .open_receiver_on_partition(
            TEST_PARTITION.to_string(),
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: StartLocation::SequenceNumber(start),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .await?;

    let read = {
        let mut stream = receiver.stream_events();
        timeout(READ_DEADLINE, async {
            while let Some(event) = stream.next().await {
                if tagged(&event?, &run_marker) {
                    return Ok::<bool, EventHubsError>(true);
                }
            }
            Ok(false)
        })
        .await
    };

    receiver.close().await?;
    consumer.close().await?;
    producer.close().await?;

    assert!(
        matches!(read, Ok(Ok(true))),
        "a Listen-only rule did not read the tagged event: {read:?}"
    );
    Ok(())
}

#[recorded::test(live)]
async fn listen_only_rule_cannot_send(_ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const TEST_NAME: &str = "listen_only_rule_cannot_send";
    let _serial = SERIAL.lock().await;

    let connection_string = env::var("EVENTHUBS_LISTEN_ONLY_CONNECTION_STRING")?;
    let eventhub = env::var("EVENTHUB_NAME").ok();

    // The open must succeed: the send is the first call that authorizes the
    // path, and that is where the broker refuses.
    let producer = ProducerClient::builder()
        .with_application_id(TEST_NAME.to_string())
        .open_with_connection_string(&connection_string, eventhub.as_deref())
        .await?;

    let run_marker = run_marker(TEST_NAME);
    let result = timeout(
        OP_DEADLINE,
        producer.send_event(
            marked_event(&run_marker),
            Some(SendEventOptions {
                partition_id: Some(TEST_PARTITION.to_string()),
            }),
        ),
    )
    .await;

    producer.close().await?;

    let result = result.expect("the Listen-only send did not finish before the deadline");
    let error = result.expect_err("a Listen-only rule sent an event");
    assert!(
        looks_unauthorized(&error),
        "the Listen-only send failed for another reason: {error:?}"
    );
    Ok(())
}

#[recorded::test(live)]
async fn custom_endpoint_completes_a_service_operation(
    _ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    const TEST_NAME: &str = "custom_endpoint_completes_a_service_operation";
    let _serial = SERIAL.lock().await;

    let host = env::var("EVENTHUBS_HOST")?;
    let connection_string = env::var("EVENTHUBS_CONNECTION_STRING")?;
    let eventhub = env::var("EVENTHUB_NAME").ok();

    let producer = ProducerClient::builder()
        .with_application_id(TEST_NAME.to_string())
        // The value goes through `Url::parse`, so the scheme and the port are
        // both required.
        .with_custom_endpoint(format!("amqps://{host}:5671"))
        .open_with_connection_string(&connection_string, eventhub.as_deref())
        .await?;

    let properties = producer.get_eventhub_properties().await?;

    producer.close().await?;

    // The honest limit: when the custom endpoint is the namespace itself, this
    // cannot tell "honored" from "ignored", because the dial target and the
    // open-frame hostname resolve to the same host. The next test closes that.
    assert!(!properties.partition_ids.is_empty());
    Ok(())
}

#[recorded::test(live)]
async fn custom_endpoint_is_used_for_the_dial(_ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const TEST_NAME: &str = "custom_endpoint_is_used_for_the_dial";
    let _serial = SERIAL.lock().await;

    let connection_string = env::var("EVENTHUBS_CONNECTION_STRING")?;
    let eventhub = env::var("EVENTHUB_NAME").ok();

    // The baseline stops a vacuous pass: with the network down, the bogus
    // endpoint would fail for a reason that has nothing to do with the option.
    let baseline = ProducerClient::builder()
        .with_application_id(TEST_NAME.to_string())
        .open_with_connection_string(&connection_string, eventhub.as_deref())
        .await;

    let bogus = timeout(
        OP_DEADLINE,
        ProducerClient::builder()
            .with_application_id(TEST_NAME.to_string())
            .with_custom_endpoint(UNREACHABLE_ENDPOINT.to_string())
            // The defaults are 8 retries over 60 seconds, which is longer than
            // this test should wait for an address that nothing listens on.
            .with_retry_options(RetryOptions {
                max_retries: 1,
                max_total_elapsed: azure_core::time::Duration::seconds(5),
                ..Default::default()
            })
            .open_with_connection_string(&connection_string, eventhub.as_deref()),
    )
    .await;

    let bogus_rendered = match &bogus {
        Ok(Ok(_)) => "the open succeeded".to_string(),
        Ok(Err(error)) => format!("{error:?}"),
        Err(_) => "the open did not finish before the deadline".to_string(),
    };
    let bogus_failed = matches!(bogus, Ok(Err(_)));
    if let Ok(Ok(producer)) = bogus {
        producer.close().await?;
    }

    let baseline_error = baseline.as_ref().err().map(|error| format!("{error:?}"));
    if let Ok(producer) = baseline {
        producer.close().await?;
    }

    assert!(
        baseline_error.is_none(),
        "the baseline open failed, so this run proves nothing about the custom endpoint: {baseline_error:?}"
    );
    assert!(
        bogus_failed,
        "an unreachable custom endpoint did not fail the open: {bogus_rendered}"
    );
    Ok(())
}
