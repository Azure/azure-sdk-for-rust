// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! A/B harness that measures what Cosmos DB binary encoding actually buys.
//!
//! The harness answers two questions with numbers instead of intuition:
//!
//! 1. **Does binary encoding shrink the payload on the wire?** Every request
//!    and response body is measured at the transport boundary — after the
//!    driver has transcoded the request to binary, and before it transcodes
//!    the response back to text — so the byte counts are the bytes that
//!    actually travel over the socket.
//! 2. **What does it cost in latency?** Each operation is timed end to end
//!    (including client-side transcoding), and each HTTP round trip is timed
//!    separately, so transcode overhead shows up as the gap between them.
//!
//! ## How the measurement works
//!
//! The driver's HTTP client factory is replaced with [`MeasuringFactory`],
//! which builds a [`MeasuringTransport`] wrapping a plain `reqwest::Client`.
//! That transport records, per request: request body bytes, response body
//! bytes, round-trip time, request charge, and whether the response body
//! started with the binary JSON preamble (`0x80`).
//!
//! Only body bytes are counted — headers are excluded, and no content
//! encoding (gzip/br) is negotiated, so body bytes are a faithful proxy for
//! wire bytes.
//!
//! Text and binary modes are interleaved round by round so that network
//! drift, throttling, and cache warm-up affect both arms equally, and the arm
//! order rotates each round so no arm consistently benefits from running last.
//!
//! `point_delete` doubles as the noise floor: it sends no request body and
//! receives no response body, so its arms are byte-identical on the wire and
//! any delta it reports is measurement bias rather than a real effect. Read the
//! other workloads' latency deltas against it.
//!
//! ## Document shapes
//!
//! Binary encoding compresses structure — property names, numbers, booleans —
//! but not string contents, so the measured savings depend entirely on what the
//! documents look like. `--profile` selects between four shapes:
//!
//! - `simple`: a small flat document, the best case for text JSON and so a
//!   conservative lower bound on the savings.
//! - `rich`: a realistic nested business document with mixed types.
//! - `huge`: deeply nested, wide arrays, and numeric edge cases.
//! - `corpus`: real documents sampled from the local `testdata/*.json` corpus,
//!   which is the most defensible option because the shapes were not chosen by
//!   the harness author.
//!
//! All profiles are seeded from `--seed`, so every mode sees byte-identical
//! documents and a run reproduces exactly.
//!
//! ## Running
//!
//! ```text
//! # PowerShell: keep the key out of shell history and process listings.
//! $env:AZURE_COSMOS_CONNECTION_STRING = "AccountEndpoint=...;AccountKey=...;"
//!
//! cargo run --release -p azure_data_cosmos_perf --features binary-ab --bin binary_payload_ab -- \
//!     --application-region "West US 2" \
//!     --docs 200 --rounds 3 --iterations 20 --include-text-response-mode
//! ```
//!
//! The connection string may also be passed with `--connection-string`, but the
//! environment variable is strongly preferred.

use std::{
    collections::BTreeMap,
    error::Error,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock},
    time::{Duration, Instant},
};

use async_trait::async_trait;
use azure_core::http::StatusCode;
use azure_data_cosmos::{
    clients::ContainerClient,
    feed::FeedScope,
    models::{ContainerProperties, ThroughputProperties},
    options::{BinaryEncodingOptions, CreateContainerOptions, QueryOptions},
    AccountEndpoint, AccountReference, CosmosClient, CosmosRuntimeBuilder, Query, RoutingStrategy,
};
use azure_data_cosmos_driver::{
    diagnostics::RequestSentStatus,
    error::{CosmosError, CosmosStatus},
    models::ConnectionString,
    testing::{
        ConnectionPoolOptions, HttpClientConfig, HttpClientFactory, HttpRequest, HttpResponse,
        TransportClient, TransportError,
    },
    CosmosDriverRuntimeBuilder,
};
use clap::Parser;
use futures::TryStreamExt;
use serde_json::{Map, Value};

/// First byte of a Cosmos binary JSON payload.
const BINARY_PREAMBLE: u8 = 0x80;

/// The partition key path used by the harness container.
const PARTITION_KEY_PATH: &str = "/partition_key";

/// The partition key property name, derived from [`PARTITION_KEY_PATH`].
const PARTITION_KEY_FIELD: &str = "partition_key";

/// Attempts to draw an acceptably-sized corpus document before giving up, so a
/// corpus made entirely of oversized objects fails loudly instead of looping.
const MAX_CORPUS_SAMPLE_ATTEMPTS: usize = 64;

/// Real-world documents harvested from the local `testdata/*.json` corpus.
///
/// Loaded once at startup when `--profile corpus` is selected, and shared by
/// every mode so all arms see byte-identical documents.
static CORPUS: OnceLock<Vec<Map<String, Value>>> = OnceLock::new();

/// Command line configuration for the harness.
#[derive(Parser, Debug)]
#[command(name = "binary_payload_ab")]
struct Args {
    /// Cosmos DB connection string (`AccountEndpoint=...;AccountKey=...;`).
    ///
    /// Prefer setting `AZURE_COSMOS_CONNECTION_STRING` so the account key never
    /// lands in shell history or process listings.
    #[arg(long, env = "AZURE_COSMOS_CONNECTION_STRING", hide_env_values = true)]
    connection_string: String,

    /// Azure region where this process is running (e.g. "West US 2").
    #[arg(long)]
    application_region: String,

    /// Database name. Created if it does not exist.
    #[arg(long, default_value = "binaryab")]
    database: String,

    /// Container name. Created if it does not exist.
    #[arg(long, default_value = "binaryab")]
    container: String,

    /// Provisioned throughput used when the container has to be created.
    #[arg(long, default_value_t = 10000)]
    throughput: usize,

    /// Number of documents to seed before measuring.
    #[arg(long, default_value_t = 200)]
    docs: usize,

    /// Number of distinct partition keys to spread the seeded documents over.
    #[arg(long, default_value_t = 20)]
    partitions: usize,

    /// Shape and size of the documents under test.
    #[arg(long, value_enum, default_value_t = Profile::Rich)]
    profile: Profile,

    /// Length of the free-text field in each seeded document.
    ///
    /// Binary encoding compresses structure (property names, numbers, booleans)
    /// but not string contents, so a larger value dilutes the measured savings.
    #[arg(long, default_value_t = 64)]
    text_len: usize,

    /// Number of numeric entries in each seeded document.
    ///
    /// Numbers are where binary encoding wins the most, so this is the main
    /// knob for shaping how binary-friendly the workload is.
    #[arg(long, default_value_t = 20)]
    numbers: usize,

    /// Maximum nesting depth of the generated documents.
    ///
    /// Ignored by `--profile simple`. Deeper documents have proportionally more
    /// structural overhead, which is what binary encoding removes.
    #[arg(long, default_value_t = 6)]
    depth: u32,

    /// Number of fields at each level of the generated documents.
    ///
    /// Ignored by `--profile simple`.
    #[arg(long, default_value_t = 8)]
    breadth: usize,

    /// Maximum length of the generated arrays.
    #[arg(long, default_value_t = 12)]
    array_len: usize,

    /// Include non-ASCII strings in the generated documents.
    ///
    /// Multi-byte UTF-8 costs the same in both encodings, so this makes the
    /// measured savings more conservative and more realistic.
    #[arg(long, default_value_t = false)]
    unicode: bool,

    /// Seed for the document generator, so a run is exactly reproducible.
    #[arg(long, default_value_t = 0x5EED_1234_ABCD_9876)]
    seed: u64,

    /// Directory holding the `*.json` corpus used by `--profile corpus`.
    ///
    /// Defaults to `testdata/` inside this crate.
    #[arg(long)]
    corpus_dir: Option<PathBuf>,

    /// Skip corpus documents larger than this many bytes when serialized.
    ///
    /// Cosmos rejects items over 2 MB and adds system properties on top of what
    /// the client sends, so the default leaves generous headroom.
    #[arg(long, default_value_t = 400_000)]
    corpus_max_bytes: usize,

    /// Maximum number of documents retained from each corpus file.
    ///
    /// Several corpus files hold hundreds of thousands of small objects;
    /// retaining all of them would cost gigabytes for no extra variety.
    #[arg(long, default_value_t = 2000)]
    corpus_per_file: usize,

    /// Operations per workload, per mode, per round.
    #[arg(long, default_value_t = 20)]
    iterations: usize,

    /// Number of interleaved text/binary rounds.
    #[arg(long, default_value_t = 3)]
    rounds: usize,

    /// Page size hint for query workloads.
    #[arg(long, default_value_t = 100)]
    page_size: u32,

    /// Also measure binary encoding with `request_text_response = true`.
    ///
    /// That combination asks the service to answer in text, so the harness
    /// shows how much of the benefit is forfeited by opting into it.
    #[arg(long, default_value_t = false)]
    include_text_response_mode: bool,

    /// Accept invalid TLS certificates (for the Cosmos DB emulator).
    #[arg(long, default_value_t = false)]
    allow_invalid_cert: bool,
}

/// Shape and size of the documents under test.
#[derive(Copy, Clone, Debug, PartialEq, Eq, clap::ValueEnum)]
enum Profile {
    /// A small flat document: a handful of scalars and one number array.
    ///
    /// Represents the best case for text JSON, so the savings measured here are
    /// a conservative lower bound.
    Simple,
    /// A realistic nested business document with mixed types.
    Rich,
    /// A large, deeply nested document that exercises every JSON type,
    /// including the numeric edge cases binary encoding compresses hardest.
    Huge,
    /// Real documents sampled from the local `testdata/*.json` corpus.
    ///
    /// This is the most defensible profile: the shapes are not chosen by the
    /// harness author, so the measured savings cannot be accused of being
    /// tuned to favor binary encoding.
    Corpus,
}

/// One of the encoding configurations under test.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Mode {
    /// Binary encoding disabled — plain text JSON in both directions.
    Text,
    /// Binary encoding enabled, responses requested in binary.
    Binary,
    /// Binary encoding enabled, but responses explicitly requested as text.
    BinaryTextResponse,
}

impl Mode {
    /// The label used for this mode in the report.
    fn label(self) -> &'static str {
        match self {
            Mode::Text => "text",
            Mode::Binary => "binary",
            Mode::BinaryTextResponse => "binary+text_resp",
        }
    }

    /// A fixed-width tag used inside generated document ids.
    ///
    /// Ids travel inside the request body, so if modes had different-length
    /// tags the request-byte comparison would be contaminated by the id length
    /// rather than measuring the encoding. All slugs are the same width.
    fn slug(self) -> &'static str {
        match self {
            Mode::Text => "m0",
            Mode::Binary => "m1",
            Mode::BinaryTextResponse => "m2",
        }
    }

    /// The client options that select this mode.
    fn options(self) -> BinaryEncodingOptions {
        match self {
            Mode::Text => BinaryEncodingOptions::new().with_enabled(false),
            Mode::Binary => BinaryEncodingOptions::new()
                .with_enabled(true)
                .with_request_text_response(false),
            Mode::BinaryTextResponse => BinaryEncodingOptions::new()
                .with_enabled(true)
                .with_request_text_response(true),
        }
    }
}

/// A single HTTP round trip observed at the transport boundary.
#[derive(Clone, Debug)]
struct Sample {
    /// Bytes in the request body as sent.
    request_bytes: usize,
    /// Bytes in the response body as received.
    response_bytes: usize,
    /// Time spent inside the HTTP round trip.
    round_trip: Duration,
    /// Request units charged, when the service reported them.
    request_charge: f64,
    /// Whether the response body arrived binary encoded.
    binary_response: bool,
}

/// Everything recorded for one workload running in one mode.
#[derive(Default, Debug)]
struct Bucket {
    /// Transport-level samples.
    samples: Vec<Sample>,
    /// End-to-end durations of the SDK calls, including transcoding.
    operations: Vec<Duration>,
    /// Number of items returned to the application.
    items: usize,
}

/// Collects samples and attributes them to the currently active phase.
#[derive(Default, Debug)]
struct Recorder {
    state: Mutex<RecorderState>,
}

/// Interior state of the [`Recorder`].
#[derive(Default, Debug)]
struct RecorderState {
    /// The phase that in-flight requests are attributed to, if any.
    phase: Option<String>,
    /// Samples grouped by phase.
    buckets: BTreeMap<String, Bucket>,
}

impl Recorder {
    /// Starts attributing transport samples to `phase`.
    fn begin(&self, phase: impl Into<String>) {
        let mut state = self.state.lock().expect("recorder poisoned");
        let phase = phase.into();
        state.buckets.entry(phase.clone()).or_default();
        state.phase = Some(phase);
    }

    /// Stops attributing transport samples to any phase.
    ///
    /// Requests issued outside a phase (metadata refreshes, address resolution,
    /// warm-up) are dropped so they cannot skew the comparison.
    fn end(&self) {
        self.state.lock().expect("recorder poisoned").phase = None;
    }

    /// Records one HTTP round trip against the active phase, if any.
    fn record(&self, sample: Sample) {
        let mut state = self.state.lock().expect("recorder poisoned");
        let Some(phase) = state.phase.clone() else {
            return;
        };
        state.buckets.entry(phase).or_default().samples.push(sample);
    }

    /// Records the end-to-end duration and item count of one SDK operation.
    fn record_operation(&self, phase: &str, elapsed: Duration, items: usize) {
        let mut state = self.state.lock().expect("recorder poisoned");
        let bucket = state.buckets.entry(phase.to_string()).or_default();
        bucket.operations.push(elapsed);
        bucket.items += items;
    }

    /// Takes a snapshot of everything recorded so far.
    fn snapshot(&self) -> BTreeMap<String, Summary> {
        let state = self.state.lock().expect("recorder poisoned");
        state
            .buckets
            .iter()
            .map(|(phase, bucket)| (phase.clone(), Summary::from_bucket(bucket)))
            .collect()
    }
}

/// Aggregated statistics for one phase.
#[derive(Clone, Debug, Default)]
struct Summary {
    /// Number of SDK operations.
    operations: usize,
    /// Number of HTTP round trips.
    requests: usize,
    /// Total request body bytes.
    request_bytes: usize,
    /// Total response body bytes.
    response_bytes: usize,
    /// Total request charge.
    request_charge: f64,
    /// Number of responses that arrived binary encoded.
    binary_responses: usize,
    /// Total items returned to the application.
    items: usize,
    /// Median end-to-end operation latency.
    op_p50: Duration,
    /// 95th percentile end-to-end operation latency.
    op_p95: Duration,
    /// Median HTTP round-trip latency.
    rt_p50: Duration,
}

impl Summary {
    /// Aggregates a [`Bucket`] into a [`Summary`].
    fn from_bucket(bucket: &Bucket) -> Self {
        let mut ops: Vec<Duration> = bucket.operations.clone();
        ops.sort_unstable();
        let mut round_trips: Vec<Duration> = bucket.samples.iter().map(|s| s.round_trip).collect();
        round_trips.sort_unstable();

        Self {
            operations: bucket.operations.len(),
            requests: bucket.samples.len(),
            request_bytes: bucket.samples.iter().map(|s| s.request_bytes).sum(),
            response_bytes: bucket.samples.iter().map(|s| s.response_bytes).sum(),
            request_charge: bucket.samples.iter().map(|s| s.request_charge).sum(),
            binary_responses: bucket.samples.iter().filter(|s| s.binary_response).count(),
            items: bucket.items,
            op_p50: percentile(&ops, 0.50),
            op_p95: percentile(&ops, 0.95),
            rt_p50: percentile(&round_trips, 0.50),
        }
    }

    /// Response bytes per SDK operation.
    fn response_bytes_per_op(&self) -> f64 {
        ratio(self.response_bytes as f64, self.operations)
    }

    /// Request bytes per SDK operation.
    fn request_bytes_per_op(&self) -> f64 {
        ratio(self.request_bytes as f64, self.operations)
    }

    /// Request charge per SDK operation.
    fn charge_per_op(&self) -> f64 {
        ratio(self.request_charge, self.operations)
    }

    /// Response bytes per item returned.
    ///
    /// This is the comparison that survives small differences in result-set
    /// size between modes, so it is the number to trust for query workloads.
    fn response_bytes_per_item(&self) -> f64 {
        ratio(self.response_bytes as f64, self.items)
    }

    /// Request charge per item returned.
    fn charge_per_item(&self) -> f64 {
        ratio(self.request_charge, self.items)
    }
}

/// Divides `total` by `count`, yielding zero when there is nothing to divide.
fn ratio(total: f64, count: usize) -> f64 {
    if count == 0 {
        0.0
    } else {
        total / count as f64
    }
}

/// Returns the `p`th percentile of an already sorted slice.
fn percentile(sorted: &[Duration], p: f64) -> Duration {
    if sorted.is_empty() {
        return Duration::ZERO;
    }
    let index = ((sorted.len() as f64 - 1.0) * p).round() as usize;
    sorted[index.min(sorted.len() - 1)]
}

/// Builds [`MeasuringTransport`] instances for the driver.
#[derive(Debug)]
struct MeasuringFactory {
    /// Where measurements are sent.
    recorder: Arc<Recorder>,
    /// Whether to accept invalid TLS certificates.
    allow_invalid_cert: bool,
}

impl HttpClientFactory for MeasuringFactory {
    fn build(
        &self,
        _connection_pool: &ConnectionPoolOptions,
        _config: HttpClientConfig,
    ) -> azure_data_cosmos_driver::error::Result<Arc<dyn TransportClient>> {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(self.allow_invalid_cert)
            .build()
            .map_err(|err| {
                CosmosError::builder()
                    .with_status(CosmosStatus::TRANSPORT_IO_FAILED)
                    .with_message(format!("failed to build measuring HTTP client: {err}"))
                    .build()
            })?;

        Ok(Arc::new(MeasuringTransport {
            client,
            recorder: Arc::clone(&self.recorder),
        }))
    }
}

/// A pass-through HTTP transport that measures every round trip.
#[derive(Debug)]
struct MeasuringTransport {
    /// The real HTTP client.
    client: reqwest::Client,
    /// Where measurements are sent.
    recorder: Arc<Recorder>,
}

#[async_trait]
impl TransportClient for MeasuringTransport {
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        let method = reqwest::Method::from_bytes(request.method.as_str().as_bytes())
            .expect("azure_core::http::Method is always a valid HTTP method");
        let mut builder = self.client.request(method, request.url.clone());

        for (name, value) in request.headers.iter() {
            builder = builder.header(name.as_str(), value.as_str());
        }

        let request_bytes = request.body.as_ref().map(|b| b.len()).unwrap_or(0);
        if let Some(body) = request.body.as_ref() {
            builder = builder.body(body.clone());
        }
        if let Some(timeout) = request.timeout {
            builder = builder.timeout(timeout);
        }

        let started = Instant::now();
        let response = builder.send().await.map_err(|err| {
            TransportError::new(
                CosmosError::builder()
                    .with_status(CosmosStatus::TRANSPORT_IO_FAILED)
                    .with_message(format!("request failed: {err}"))
                    .build(),
                RequestSentStatus::Unknown,
            )
        })?;

        let status = response.status().as_u16();
        let mut headers = azure_core::http::headers::Headers::new();
        for (name, value) in response.headers().iter() {
            if let Ok(value) = value.to_str() {
                headers.insert(
                    azure_core::http::headers::HeaderName::from(name.as_str().to_lowercase()),
                    value.to_string(),
                );
            }
        }

        let body = response.bytes().await.map_err(|err| {
            TransportError::new(
                CosmosError::builder()
                    .with_status(CosmosStatus::TRANSPORT_IO_FAILED)
                    .with_message(format!("failed to read response body: {err}"))
                    .build(),
                RequestSentStatus::Sent,
            )
        })?;
        let round_trip = started.elapsed();

        self.recorder.record(Sample {
            request_bytes,
            response_bytes: body.len(),
            round_trip,
            request_charge: request_charge(&headers),
            binary_response: body.first() == Some(&BINARY_PREAMBLE),
        });

        Ok(HttpResponse {
            status,
            headers,
            body: body.to_vec(),
        })
    }
}

/// Reads the request charge from response headers, defaulting to zero.
fn request_charge(headers: &azure_core::http::headers::Headers) -> f64 {
    headers
        .iter()
        .find(|(name, _)| name.as_str() == "x-ms-request-charge")
        .and_then(|(_, value)| value.as_str().parse::<f64>().ok())
        .unwrap_or(0.0)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let recorder = Arc::new(Recorder::default());

    let mut modes = vec![Mode::Text, Mode::Binary];
    if args.include_text_response_mode {
        modes.push(Mode::BinaryTextResponse);
    }

    if args.profile == Profile::Corpus {
        let dir = args
            .corpus_dir
            .clone()
            .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("testdata"));
        let _ = CORPUS.set(load_corpus(&dir, args.corpus_per_file)?);
    }

    // The seeding client never records: it exists only to create the
    // database, container, and documents under test.
    let seed_client = build_client(&args, Arc::clone(&recorder), Mode::Text).await?;
    let container = ensure_container(&seed_client, &args).await?;
    let docs = seed(&container, &args).await?;
    report_document_sizes(&args, docs.len());

    let mut clients = Vec::with_capacity(modes.len());
    for mode in &modes {
        let client = build_client(&args, Arc::clone(&recorder), *mode).await?;
        let container = client
            .database_client(&args.database)
            .container_client(&args.container, None)
            .await?;
        // Warm up connections, routing caches, and query plans outside of any
        // recording phase so the first measured round is not penalized.
        warmup(&container).await?;
        clients.push((*mode, container));
    }

    for round in 1..=args.rounds {
        // Rotate the arm order each round. With a fixed order the arm that
        // runs last benefits from whatever warms up over a round (connection
        // reuse, service-side caching), which shows up as a latency edge it
        // did not earn — visible as a nonzero `point_delete` delta, a workload
        // whose arms send byte-identical HTTP. Rotation spreads that bias
        // evenly, and is deterministic (unlike shuffling) so runs stay
        // comparable. Byte and RU columns are unaffected either way.
        let offset = (round - 1) % clients.len();
        for (mode, container) in clients.iter().cycle().skip(offset).take(clients.len()) {
            println!("round {round}: {}", mode.label());

            // Point writes run first and are size-neutral: every document
            // created is deleted again before the queries run, so each mode
            // sees exactly the same container contents.
            run_point_creates(container, &recorder, *mode, round, &args).await?;
            run_point_replaces(container, &recorder, *mode, &args).await?;
            run_writes(container, &recorder, *mode, &args).await?;
            run_point_deletes(container, &recorder, *mode, round, &args).await?;

            run_point_reads(container, &recorder, *mode, &docs, &args).await?;
            run_query(
                container,
                &recorder,
                *mode,
                "query_select_all",
                Query::from("SELECT * FROM c"),
                &args,
            )
            .await?;
            run_query(
                container,
                &recorder,
                *mode,
                "query_order_by",
                Query::from("SELECT * FROM c ORDER BY c.seq DESC"),
                &args,
            )
            .await?;
            run_query(
                container,
                &recorder,
                *mode,
                "query_projection",
                Query::from("SELECT c.id, c.seq FROM c"),
                &args,
            )
            .await?;
            // The two `SkipTake` shapes. Both synthesize pages client-side
            // rather than passing the service's page through, so they are the
            // workloads where the emitted item encoding is chosen by the
            // driver rather than by the wire.
            run_query(
                container,
                &recorder,
                *mode,
                "query_order_by_offset_limit",
                Query::from("SELECT * FROM c ORDER BY c.seq DESC OFFSET 5 LIMIT 50"),
                &args,
            )
            .await?;
            run_query(
                container,
                &recorder,
                *mode,
                "query_top",
                Query::from("SELECT TOP 50 * FROM c"),
                &args,
            )
            .await?;
        }
    }

    report(&recorder.snapshot(), &modes);
    Ok(())
}

/// Prints the text-JSON size distribution of the seeded documents.
///
/// Corpus documents vary by orders of magnitude, so a single mean would be
/// misleading; the percentiles put the harness's headline byte counts in
/// context.
fn report_document_sizes(args: &Args, seeded: usize) {
    let mut sizes: Vec<usize> = (0..seeded)
        .map(|index| {
            let document = make_document(&format!("doc-{index}"), "pk-0", index, args);
            serde_json::to_vec(&document).map(|b| b.len()).unwrap_or(0)
        })
        .collect();
    sizes.sort_unstable();

    let at = |p: f64| -> usize {
        sizes
            .get(((sizes.len() as f64 - 1.0) * p).round() as usize)
            .copied()
            .unwrap_or(0)
    };
    let total: usize = sizes.iter().sum();

    println!(
        "seeded {seeded} document(s) as text JSON: min {} B, p50 {} B, p95 {} B, max {} B, total {:.1} KiB\n",
        sizes.first().copied().unwrap_or(0),
        at(0.50),
        at(0.95),
        sizes.last().copied().unwrap_or(0),
        total as f64 / 1024.0
    );
}

/// Builds a client whose transport reports to `recorder`, configured for `mode`.
async fn build_client(
    args: &Args,
    recorder: Arc<Recorder>,
    mode: Mode,
) -> Result<CosmosClient, Box<dyn Error>> {
    let driver_builder = CosmosDriverRuntimeBuilder::new().with_mock_http_client_factory(Arc::new(
        MeasuringFactory {
            recorder,
            allow_invalid_cert: args.allow_invalid_cert,
        },
    ));
    let runtime = CosmosRuntimeBuilder::from(driver_builder).build().await?;

    let connection_string: ConnectionString = args.connection_string.parse()?;
    let endpoint: AccountEndpoint = connection_string.account_endpoint().parse()?;
    let account = AccountReference::with_authentication_key(
        endpoint,
        connection_string.account_key().clone(),
    );

    let client = CosmosClient::builder()
        .with_runtime(runtime)
        .with_binary_encoding_options(mode.options())
        .build(
            account,
            RoutingStrategy::ProximityTo(args.application_region.clone().into()),
        )
        .await?;
    Ok(client)
}

/// Creates the database and container if they do not already exist.
async fn ensure_container(
    client: &CosmosClient,
    args: &Args,
) -> Result<ContainerClient, Box<dyn Error>> {
    match client
        .create_database(&args.database, None)
        .await
        .map(|_| ())
    {
        Ok(()) => println!("created database '{}'", args.database),
        Err(err) if err.status().status_code() == StatusCode::Conflict => {}
        Err(err) => return Err(err.into()),
    }

    let database = client.database_client(&args.database);
    let properties = ContainerProperties::new(args.container.clone(), PARTITION_KEY_PATH.into());
    let options = CreateContainerOptions::default()
        .with_throughput(ThroughputProperties::manual(args.throughput));

    match database
        .create_container(properties, Some(options))
        .await
        .map(|_| ())
    {
        Ok(()) => println!("created container '{}'", args.container),
        Err(err) if err.status().status_code() == StatusCode::Conflict => {}
        Err(err) => return Err(err.into()),
    }

    Ok(database.container_client(&args.container, None).await?)
}

/// Identifies one seeded document.
#[derive(Clone, Debug)]
struct SeededDoc {
    /// The document id.
    id: String,
    /// The document's partition key value.
    partition_key: String,
}

/// Writes the documents under test, replacing any previous run's contents.
///
/// The write workload's targets are pre-seeded here so the container size stays
/// constant from the first measured operation; creating them lazily would let
/// later modes query a larger container and skew the byte comparison.
async fn seed(container: &ContainerClient, args: &Args) -> Result<Vec<SeededDoc>, Box<dyn Error>> {
    let partitions = args.partitions.max(1);
    let mut docs = Vec::with_capacity(args.docs);

    for index in 0..args.docs {
        let partition_key = format!("pk-{}", index % partitions);
        let id = format!("doc-{index}");
        let document = make_document(&id, &partition_key, index, args);
        container
            .upsert_item(&partition_key, &id, &document, None)
            .await?;
        docs.push(SeededDoc { id, partition_key });
    }

    for (partition_key, id, index) in write_targets(args) {
        let document = make_document(&id, &partition_key, index, args);
        container
            .upsert_item(&partition_key, &id, &document, None)
            .await?;
    }

    Ok(docs)
}

/// Enumerates every document the write workloads will target, across all modes.
fn write_targets(args: &Args) -> Vec<(String, String, usize)> {
    [Mode::Text, Mode::Binary, Mode::BinaryTextResponse]
        .into_iter()
        .flat_map(|mode| write_targets_for(mode, args))
        .collect()
}

/// Enumerates the documents the upsert and replace workloads target in `mode`,
/// paired with the document index whose contents each one carries.
///
/// The index is derived from the iteration and is identical across modes, so
/// every arm writes byte-identical document bodies.
fn write_targets_for(mode: Mode, args: &Args) -> Vec<(String, String, usize)> {
    let partitions = args.partitions.max(1);
    (0..args.iterations)
        .map(|iteration| {
            (
                format!("pk-write-{}", iteration % partitions),
                format!("write-{}-{iteration:06}", mode.slug()),
                document_index(iteration, args),
            )
        })
        .collect()
}

/// Maps an iteration onto a seeded document index, striding across the whole
/// seeded set.
///
/// `corpus` document sizes span orders of magnitude, so sampling only the head
/// would report a size distribution unlike the one the queries read back.
fn document_index(iteration: usize, args: &Args) -> usize {
    let docs = args.docs.max(1);
    let iterations = args.iterations.max(1);
    (iteration * docs / iterations) % docs
}

/// Builds a document whose shape exercises the parts of JSON that binary
/// encoding actually compresses: property names, numbers, and booleans.
///
/// The generator is seeded per document from `(args.seed, index)`, so a run is
/// exactly reproducible and every mode sees byte-identical documents.
fn make_document(id: &str, partition_key: &str, index: usize, args: &Args) -> Value {
    let mut rng = SplitMix64::new(args.seed ^ (index as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));

    let mut map = Map::new();
    map.insert("id".into(), Value::String(id.to_string()));
    map.insert(
        PARTITION_KEY_FIELD.into(),
        Value::String(partition_key.to_string()),
    );
    // `seq` gives ORDER BY a stable, selective sort key.
    map.insert("seq".into(), Value::from(index));

    match args.profile {
        Profile::Simple => {
            map.insert("active".into(), Value::Bool(index.is_multiple_of(2)));
            map.insert("score".into(), Value::from(index as f64 * 1.5));
            map.insert(
                "measurements".into(),
                Value::Array(
                    (0..args.numbers)
                        .map(|n| Value::from((index * 31 + n * 7) as f64 / 3.0))
                        .collect(),
                ),
            );
            map.insert("text".into(), Value::String("x".repeat(args.text_len)));
        }
        Profile::Rich | Profile::Huge => {
            for (key, value) in business_fields(&mut rng, args) {
                map.insert(key, value);
            }
            if args.profile == Profile::Huge {
                map.insert("numeric_edges".into(), numeric_edge_cases(&mut rng));
                map.insert("deep".into(), gen_nested(&mut rng, args, args.depth.max(1)));
                map.insert(
                    "matrix".into(),
                    Value::Array(
                        (0..args.array_len.max(1))
                            .map(|_| {
                                Value::Array(
                                    (0..args.array_len.max(1))
                                        .map(|_| gen_number(&mut rng))
                                        .collect(),
                                )
                            })
                            .collect(),
                    ),
                );
            }
        }
        Profile::Corpus => {
            for (key, value) in sample_corpus_document(&mut rng, args) {
                map.insert(key, value);
            }
        }
    }

    Value::Object(map)
}

/// Draws one real document out of the loaded corpus, retrying past oversized
/// ones so a single multi-megabyte object cannot fail the seed with a 413.
///
/// Reserved keys are dropped: `id` and the partition key are owned by the
/// harness, and `_`-prefixed keys are Cosmos system properties left behind by
/// whatever export produced the corpus file.
fn sample_corpus_document(rng: &mut SplitMix64, args: &Args) -> Vec<(String, Value)> {
    let pool = CORPUS
        .get()
        .expect("corpus must be loaded before documents are generated");

    for _ in 0..MAX_CORPUS_SAMPLE_ATTEMPTS {
        let candidate = &pool[rng.below(pool.len() as u64) as usize];
        let fields: Vec<(String, Value)> = candidate
            .iter()
            .filter(|(key, _)| {
                !key.starts_with('_')
                    && key.as_str() != "id"
                    && key.as_str() != PARTITION_KEY_FIELD
                    && key.as_str() != "seq"
            })
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();

        let size = fields
            .iter()
            .filter_map(|(key, value)| serde_json::to_vec(value).ok().map(|b| b.len() + key.len()))
            .sum::<usize>();
        if size <= args.corpus_max_bytes {
            return fields;
        }
    }

    panic!(
        "no corpus document under {} bytes found in {MAX_CORPUS_SAMPLE_ATTEMPTS} attempts; \
         raise --corpus-max-bytes",
        args.corpus_max_bytes
    );
}

/// Loads every `*.json` file under `dir` and flattens them into a pool of
/// candidate JSON objects.
///
/// The corpus is a local, untracked copy of real service documents, so a
/// missing directory is reported with an explanation rather than a bare IO
/// error. Files that fail to parse are skipped instead of failing the run.
fn load_corpus(dir: &Path, per_file: usize) -> Result<Vec<Map<String, Value>>, Box<dyn Error>> {
    let entries = std::fs::read_dir(dir).map_err(|e| {
        format!(
            "failed to read the corpus under {} ({e}). The corpus is a local copy \
             that is not tracked in source control; restore the `*.json` files, or \
             pick a different `--profile`.",
            dir.display()
        )
    })?;

    let mut pool = Vec::new();
    let mut files = 0usize;
    for entry in entries {
        let path = entry?.path();
        if !path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| e.eq_ignore_ascii_case("json"))
        {
            continue;
        }
        files += 1;
        let Ok(bytes) = std::fs::read(&path) else {
            continue;
        };
        let Ok(value) = serde_json::from_slice::<Value>(&bytes) else {
            continue;
        };

        // Cap per file so one huge export cannot crowd out the other 28 shapes.
        let mut from_file = Vec::new();
        collect_objects(value, &mut from_file, per_file.max(1));
        pool.append(&mut from_file);
    }

    if pool.is_empty() {
        return Err(format!(
            "no JSON objects found under {} (scanned {files} file(s))",
            dir.display()
        )
        .into());
    }

    println!(
        "loaded {} document(s) from {files} corpus file(s) in {}",
        pool.len(),
        dir.display()
    );
    Ok(pool)
}

/// Recursively harvests up to `limit` JSON objects from a value, descending
/// into arrays and into the array-valued fields of objects.
///
/// Corpus files come in several shapes — a single object, a bare array, or an
/// envelope object whose payload is an array (e.g. GeoJSON `features`) — so all
/// three are unwrapped into individual documents.
fn collect_objects(value: Value, pool: &mut Vec<Map<String, Value>>, limit: usize) {
    if pool.len() >= limit {
        return;
    }

    match value {
        Value::Array(items) => {
            for item in items {
                collect_objects(item, pool, limit);
                if pool.len() >= limit {
                    return;
                }
            }
        }
        Value::Object(map) => {
            let nested: Vec<Value> = map
                .values()
                .filter(|v| matches!(v, Value::Array(items) if items.iter().any(Value::is_object)))
                .cloned()
                .collect();
            if nested.is_empty() {
                pool.push(map);
            } else {
                for value in nested {
                    collect_objects(value, pool, limit);
                    if pool.len() >= limit {
                        return;
                    }
                }
            }
        }
        _ => {}
    }
}

/// The realistic business-document fields shared by the `rich` and `huge`
/// profiles: identifiers, timestamps, money, geo, tags, and line items.
fn business_fields(rng: &mut SplitMix64, args: &Args) -> Vec<(String, Value)> {
    let mut fields = Vec::new();

    fields.push(("uuid".into(), Value::String(gen_uuid(rng))));
    fields.push(("etag_like".into(), Value::String(gen_hex(rng, 32))));
    fields.push((
        "created_at".into(),
        Value::String(format!(
            "2026-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
            1 + rng.below(12),
            1 + rng.below(28),
            rng.below(24),
            rng.below(60),
            rng.below(60),
            rng.below(1000)
        )),
    ));
    fields.push(("version".into(), Value::from(rng.below(1_000_000))));
    fields.push(("active".into(), Value::Bool(rng.below(2) == 0)));
    fields.push(("deleted".into(), Value::Null));

    // Money and geo: high-precision floats, the single most binary-friendly type.
    let mut money = Map::new();
    money.insert("currency".into(), Value::String("USD".into()));
    money.insert("amount".into(), gen_number(rng));
    money.insert("tax".into(), gen_number(rng));
    money.insert("discount".into(), gen_number(rng));
    fields.push(("total".into(), Value::Object(money)));

    let mut geo = Map::new();
    geo.insert("type".into(), Value::String("Point".into()));
    geo.insert(
        "coordinates".into(),
        Value::Array(vec![gen_number(rng), gen_number(rng)]),
    );
    fields.push(("location".into(), Value::Object(geo)));

    // Tags: short repeated strings, where binary's string table helps least.
    fields.push((
        "tags".into(),
        Value::Array(
            (0..1 + rng.below(args.array_len.max(1) as u64))
                .map(|_| Value::String(gen_word(rng, 8)))
                .collect(),
        ),
    ));

    // Metrics: a dense numeric array, the clearest binary win.
    fields.push((
        "metrics".into(),
        Value::Array((0..args.numbers).map(|_| gen_number(rng)).collect()),
    ));

    // Line items: an array of uniform objects, so property names repeat many
    // times per document — exactly what binary encoding deduplicates.
    fields.push((
        "line_items".into(),
        Value::Array(
            (0..1 + rng.below(args.array_len.max(1) as u64))
                .map(|_| {
                    let mut item = Map::new();
                    item.insert("sku".into(), Value::String(gen_hex_upper(rng, 12)));
                    item.insert("description".into(), Value::String(gen_word(rng, 24)));
                    item.insert("quantity".into(), Value::from(1 + rng.below(100)));
                    item.insert("unit_price".into(), gen_number(rng));
                    item.insert("taxable".into(), Value::Bool(rng.below(2) == 0));
                    Value::Object(item)
                })
                .collect(),
        ),
    ));

    fields.push((
        "notes".into(),
        Value::String(gen_text(rng, args.text_len, args.unicode)),
    ));

    fields
}

/// Values that stress the numeric encoder: integer width boundaries, negative
/// zero, subnormals, and high-precision decimals.
///
/// These are where binary encoding's variable-width integers pay off most, and
/// where a text encoder is forced into its longest representations.
fn numeric_edge_cases(rng: &mut SplitMix64) -> Value {
    let mut map = Map::new();
    map.insert("zero".into(), Value::from(0));
    map.insert("neg_zero".into(), Value::from(-0.0));
    map.insert("i8_min".into(), Value::from(i8::MIN));
    map.insert("i16_min".into(), Value::from(i16::MIN));
    map.insert("i32_min".into(), Value::from(i32::MIN));
    map.insert("i64_min".into(), Value::from(i64::MIN));
    map.insert("i64_max".into(), Value::from(i64::MAX));
    map.insert("u32_max".into(), Value::from(u32::MAX));
    map.insert("small_float".into(), Value::from(f64::MIN_POSITIVE));
    map.insert("precise".into(), Value::from(std::f64::consts::PI * 1e12));
    map.insert(
        "mixed_widths".into(),
        Value::Array(
            (0..16)
                .map(|shift| Value::from(1i64 << (shift * 4 % 63)))
                .collect(),
        ),
    );
    map.insert(
        "decimals".into(),
        Value::Array((0..16).map(|_| gen_number(rng)).collect()),
    );
    Value::Object(map)
}

/// Builds a nested object `depth` levels deep, with `args.breadth` fields per
/// level, alternating objects and arrays.
fn gen_nested(rng: &mut SplitMix64, args: &Args, depth: u32) -> Value {
    if depth == 0 {
        return gen_number(rng);
    }

    let mut map = Map::new();
    map.insert("level".into(), Value::from(depth));
    for field in 0..args.breadth.max(1) {
        let key = format!("field_{field}");
        let value = match field % 4 {
            0 => gen_number(rng),
            1 => Value::String(gen_word(rng, 12)),
            2 => Value::Bool(rng.below(2) == 0),
            _ => Value::Array((0..3).map(|_| gen_number(rng)).collect()),
        };
        map.insert(key, value);
    }
    map.insert("child".into(), gen_nested(rng, args, depth - 1));
    Value::Object(map)
}

/// A number drawn from a mix of small integers, wide integers, and
/// high-precision floats, in roughly the proportions seen in real documents.
fn gen_number(rng: &mut SplitMix64) -> Value {
    match rng.below(4) {
        0 => Value::from(rng.below(256) as i64 - 128),
        1 => Value::from(rng.next_u64() as i64 / 4096),
        2 => Value::from(rng.below(100_000) as f64 / 100.0),
        _ => Value::from(rng.next_u64() as f64 / 1e9),
    }
}

/// A lowercase alphabetic word of `len` characters.
fn gen_word(rng: &mut SplitMix64, len: usize) -> String {
    const ALPHA: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    (0..len)
        .map(|_| ALPHA[rng.below(ALPHA.len() as u64) as usize] as char)
        .collect()
}

/// A lowercase hex string of `len` nibbles.
fn gen_hex(rng: &mut SplitMix64, len: usize) -> String {
    const HEX: &[u8] = b"0123456789abcdef";
    (0..len)
        .map(|_| HEX[rng.below(16) as usize] as char)
        .collect()
}

/// An uppercase hex string of `len` nibbles.
fn gen_hex_upper(rng: &mut SplitMix64, len: usize) -> String {
    gen_hex(rng, len).to_ascii_uppercase()
}

/// A canonical `8-4-4-4-12` UUID string.
fn gen_uuid(rng: &mut SplitMix64) -> String {
    format!(
        "{}-{}-{}-{}-{}",
        gen_hex(rng, 8),
        gen_hex(rng, 4),
        gen_hex(rng, 4),
        gen_hex(rng, 4),
        gen_hex(rng, 12)
    )
}

/// Free text of roughly `len` characters, optionally including non-ASCII.
fn gen_text(rng: &mut SplitMix64, len: usize, unicode: bool) -> String {
    const MULTIBYTE: &[char] = &['é', 'ü', 'ñ', 'π', 'λ', '日', '本', '语', '😀', '🚀'];

    let mut out = String::with_capacity(len);
    while out.chars().count() < len {
        if unicode && rng.below(8) == 0 {
            out.push(MULTIBYTE[rng.below(MULTIBYTE.len() as u64) as usize]);
        } else {
            let word_len = 1 + rng.below(9) as usize;
            out.push_str(&gen_word(rng, word_len));
            out.push(' ');
        }
    }
    out.truncate(
        out.char_indices()
            .nth(len)
            .map(|(i, _)| i)
            .unwrap_or(out.len()),
    );
    out
}

/// A small deterministic PRNG (SplitMix64) so runs are exactly reproducible.
///
/// Matches the generator used by the binary encoding round-trip fuzzer, so a
/// document shape can be moved between the two tools by seed.
struct SplitMix64 {
    /// The current generator state.
    state: u64,
}

impl SplitMix64 {
    /// Creates a generator from `seed`.
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Returns the next 64 bits of the stream.
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Returns a uniform integer in `[0, n)`. `n` must be non-zero.
    fn below(&mut self, n: u64) -> u64 {
        self.next_u64() % n.max(1)
    }
}

/// Issues a few unrecorded operations so caches and connections are warm.
async fn warmup(container: &ContainerClient) -> Result<(), Box<dyn Error>> {
    let mut pages = container
        .query_items::<Value>(
            Query::from("SELECT * FROM c"),
            FeedScope::full_container(),
            None,
        )
        .await?
        .into_pages();
    while pages.try_next().await?.is_some() {}
    Ok(())
}

/// Runs the point-read workload.
async fn run_point_reads(
    container: &ContainerClient,
    recorder: &Recorder,
    mode: Mode,
    docs: &[SeededDoc],
    args: &Args,
) -> Result<(), Box<dyn Error>> {
    if docs.is_empty() {
        return Ok(());
    }

    let phase = phase_name("point_read", mode);
    recorder.begin(&phase);
    for iteration in 0..args.iterations {
        let doc = &docs[document_index(iteration, args) % docs.len()];
        let started = Instant::now();
        let response = container
            .read_item(&doc.partition_key, &doc.id, None)
            .await?;
        let _: Value = response.into_model()?;
        recorder.record_operation(&phase, started.elapsed(), 1);
    }
    recorder.end();
    Ok(())
}

/// Runs the point-create workload.
///
/// Each round creates a fresh set of documents that [`run_point_deletes`]
/// removes again, so the container size is unchanged by the time the next mode
/// runs its queries.
async fn run_point_creates(
    container: &ContainerClient,
    recorder: &Recorder,
    mode: Mode,
    round: usize,
    args: &Args,
) -> Result<(), Box<dyn Error>> {
    let phase = phase_name("point_create", mode);
    recorder.begin(&phase);
    for (partition_key, id, index) in transient_docs(mode, round, args) {
        let document = make_document(&id, &partition_key, index, args);
        let started = Instant::now();
        container
            .create_item(&partition_key, &id, &document, None)
            .await?;
        recorder.record_operation(&phase, started.elapsed(), 1);
    }
    recorder.end();
    Ok(())
}

/// Runs the point-delete workload, removing what [`run_point_creates`] added.
async fn run_point_deletes(
    container: &ContainerClient,
    recorder: &Recorder,
    mode: Mode,
    round: usize,
    args: &Args,
) -> Result<(), Box<dyn Error>> {
    let phase = phase_name("point_delete", mode);
    recorder.begin(&phase);
    for (partition_key, id, _) in transient_docs(mode, round, args) {
        let started = Instant::now();
        container.delete_item(&partition_key, &id, None).await?;
        recorder.record_operation(&phase, started.elapsed(), 1);
    }
    recorder.end();
    Ok(())
}

/// Runs the point-replace workload against pre-seeded documents.
///
/// Replace is the operation where request-side binary encoding matters most:
/// the full document body travels to the service on every call.
async fn run_point_replaces(
    container: &ContainerClient,
    recorder: &Recorder,
    mode: Mode,
    args: &Args,
) -> Result<(), Box<dyn Error>> {
    let phase = phase_name("point_replace", mode);
    recorder.begin(&phase);
    for (partition_key, id, index) in write_targets_for(mode, args) {
        let document = make_document(&id, &partition_key, index, args);
        let started = Instant::now();
        container
            .replace_item(&partition_key, &id, &document, None)
            .await?;
        recorder.record_operation(&phase, started.elapsed(), 1);
    }
    recorder.end();
    Ok(())
}

/// The documents created and then deleted within one mode's turn in one round,
/// paired with the document index whose contents each one carries.
fn transient_docs(mode: Mode, round: usize, args: &Args) -> Vec<(String, String, usize)> {
    let partitions = args.partitions.max(1);
    (0..args.iterations)
        .map(|iteration| {
            (
                format!("pk-tmp-{}", iteration % partitions),
                format!("tmp-{}-{round:03}-{iteration:06}", mode.slug()),
                document_index(iteration, args),
            )
        })
        .collect()
}

/// Runs a query workload, draining every page each iteration.
async fn run_query(
    container: &ContainerClient,
    recorder: &Recorder,
    mode: Mode,
    workload: &str,
    query: Query,
    args: &Args,
) -> Result<(), Box<dyn Error>> {
    let phase = phase_name(workload, mode);
    recorder.begin(&phase);
    for _ in 0..args.iterations {
        let options = QueryOptions::default().with_max_item_count(max_item_count(args.page_size));
        let started = Instant::now();
        let mut pages = container
            .query_items::<Value>(
                query.clone(),
                FeedScope::full_container(),
                Some(options.clone()),
            )
            .await?
            .into_pages();

        let mut items = 0;
        while let Some(page) = pages.try_next().await? {
            items += page.items().len();
        }
        recorder.record_operation(&phase, started.elapsed(), items);
    }
    recorder.end();
    Ok(())
}

/// Runs the write workload, which is where request-side savings show up.
async fn run_writes(
    container: &ContainerClient,
    recorder: &Recorder,
    mode: Mode,
    args: &Args,
) -> Result<(), Box<dyn Error>> {
    let phase = phase_name("point_upsert", mode);
    recorder.begin(&phase);
    for (partition_key, id, index) in write_targets_for(mode, args) {
        let document = make_document(&id, &partition_key, index, args);
        let started = Instant::now();
        container
            .upsert_item(&partition_key, &id, &document, None)
            .await?;
        recorder.record_operation(&phase, started.elapsed(), 1);
    }
    recorder.end();
    Ok(())
}

/// Builds the phase key for a workload running in a mode.
fn phase_name(workload: &str, mode: Mode) -> String {
    format!("{workload}|{}", mode.label())
}

/// Converts a page size into the SDK's page size hint.
fn max_item_count(page_size: u32) -> azure_data_cosmos::options::MaxItemCountHint {
    use azure_data_cosmos::options::MaxItemCountHint;
    match std::num::NonZeroU32::new(page_size) {
        Some(limit) => MaxItemCountHint::Limit(limit),
        None => MaxItemCountHint::ServerDecides,
    }
}

/// Prints the per-phase table followed by the binary-versus-text deltas.
fn report(summaries: &BTreeMap<String, Summary>, modes: &[Mode]) {
    println!("\n=== raw measurements (body bytes only, headers excluded) ===");
    println!(
        "{:<18} {:<16} {:>5} {:>6} {:>7} {:>11} {:>10} {:>8} {:>8} {:>8} {:>8} {:>8}",
        "workload",
        "mode",
        "ops",
        "reqs",
        "items",
        "resp B/op",
        "req B/op",
        "RU/op",
        "p50 ms",
        "p95 ms",
        "http p50",
        "binary%"
    );

    for (phase, summary) in summaries {
        let (workload, mode) = split_phase(phase);
        println!(
            "{workload:<18} {mode:<16} {:>5} {:>6} {:>7} {:>11.0} {:>10.0} {:>8.2} {:>8.2} {:>8.2} {:>8.2} {:>7.0}%",
            summary.operations,
            summary.requests,
            summary.items,
            summary.response_bytes_per_op(),
            summary.request_bytes_per_op(),
            summary.charge_per_op(),
            summary.op_p50.as_secs_f64() * 1000.0,
            summary.op_p95.as_secs_f64() * 1000.0,
            summary.rt_p50.as_secs_f64() * 1000.0,
            ratio(summary.binary_responses as f64 * 100.0, summary.requests),
        );
    }

    println!("\n=== delta versus text ===");
    println!(
        "{:<18} {:<16} {:>12} {:>13} {:>11} {:>10} {:>11} {:>10}",
        "workload", "mode", "resp B/op", "resp B/item", "req B/op", "RU/op", "RU/item", "p50"
    );

    let workloads: Vec<String> = summaries
        .keys()
        .map(|phase| split_phase(phase).0.to_string())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();

    for workload in workloads {
        let Some(baseline) = summaries.get(&phase_name(&workload, Mode::Text)) else {
            continue;
        };
        for mode in modes.iter().filter(|m| **m != Mode::Text) {
            let Some(candidate) = summaries.get(&phase_name(&workload, *mode)) else {
                continue;
            };
            println!(
                "{workload:<18} {:<16} {:>12} {:>13} {:>11} {:>10} {:>11} {:>10}",
                mode.label(),
                pct(
                    baseline.response_bytes_per_op(),
                    candidate.response_bytes_per_op()
                ),
                pct(
                    baseline.response_bytes_per_item(),
                    candidate.response_bytes_per_item()
                ),
                pct(
                    baseline.request_bytes_per_op(),
                    candidate.request_bytes_per_op()
                ),
                pct(baseline.charge_per_op(), candidate.charge_per_op()),
                pct(baseline.charge_per_item(), candidate.charge_per_item()),
                pct(
                    baseline.op_p50.as_secs_f64(),
                    candidate.op_p50.as_secs_f64()
                ),
            );
        }
    }

    println!(
        "\nNegative percentages are improvements (smaller payload / lower latency).\n\
         `binary%` is the share of HTTP responses whose body actually arrived binary encoded.\n\
         `http p50` is the HTTP round trip alone; the gap to `p50 ms` is client-side work\n\
         (including binary transcoding).\n\
         Prefer the per-item columns for query workloads: they stay valid even if the\n\
         result-set size drifts between modes.\n\
         `point_delete` is the noise floor: its arms send byte-identical HTTP, so its\n\
         deltas are measurement bias. Discount the other latency deltas by that much."
    );
}

/// Formats the relative change from `baseline` to `candidate`.
fn pct(baseline: f64, candidate: f64) -> String {
    if baseline == 0.0 {
        return "n/a".to_string();
    }
    format!("{:+.1}%", (candidate - baseline) / baseline * 100.0)
}

/// Splits a phase key back into its workload and mode parts.
fn split_phase(phase: &str) -> (&str, &str) {
    phase.split_once('|').unwrap_or((phase, ""))
}
