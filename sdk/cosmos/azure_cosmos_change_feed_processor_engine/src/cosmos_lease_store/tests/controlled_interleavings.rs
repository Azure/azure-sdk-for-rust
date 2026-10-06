// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{finish_fixture, Fixture};
use crate::{ChangeFeedReadOptions, CheckpointError};
use azure_core::http::{headers::HeaderName, AsyncRawResponse, Body, Method, Request};
use azure_data_cosmos_driver::{
    diagnostics::RequestSentStatus,
    in_memory_emulator::{
        ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, VirtualAccountConfig,
        VirtualRegion,
    },
    models::{AccountReference, ChangeFeedStartFrom, FeedRange},
    options::DriverOptions,
    test::{
        ConnectionPoolOptions, HttpClientConfig, HttpClientFactory, HttpRequest, HttpResponse,
        TransportClient, TransportError,
    },
    CosmosError,
};
use serde_json::{json, Value};
use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use tokio::{sync::oneshot, task::JoinHandle, time::timeout};

mod coordinator_ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    RequestReceived,
    Committed,
    ResponseDelivered,
    ResponseLost,
    CallbackEntered,
    CallbackCompleted,
    RenewConfirmed,
    CheckpointStarted,
    CheckpointConfirmed,
    TaskExited,
}

#[derive(Clone, Debug)]
struct Event {
    worker: &'static str,
    lease_id: String,
    generation: u64,
    batch_id: u64,
    operation_id: u64,
    phase: Phase,
    if_match: String,
}

#[derive(Clone, Copy, Debug)]
enum Boundary {
    BeforeCommit,
    AfterCommit,
}

#[derive(Debug)]
struct Pause {
    entered: oneshot::Sender<Event>,
    resume: oneshot::Receiver<()>,
}
#[derive(Debug)]
struct Script {
    boundary: Boundary,
    pause: Pause,
    lose_response: bool,
    batch_id: u64,
    checkpoint: Option<String>,
    operation_id: u64,
}
struct Gate {
    entered: oneshot::Receiver<Event>,
    resume: oneshot::Sender<()>,
    operation_id: u64,
}

#[derive(Debug)]
struct ControlledService {
    worker: &'static str,
    inner: Arc<InMemoryEmulatorHttpClient>,
    scripts: Mutex<VecDeque<Script>>,
    events: Arc<Mutex<Vec<Event>>>,
    operations: Mutex<OperationIds>,
    next_operation: AtomicU64,
    remote_requests: Mutex<Vec<JoinHandle<()>>>,
}
type OperationIds = HashMap<(Vec<u8>, String), (u64, u64)>;

impl ControlledService {
    fn new(worker: &'static str, inner: Arc<InMemoryEmulatorHttpClient>) -> Arc<Self> {
        Arc::new(Self {
            worker,
            inner,
            scripts: Mutex::new(VecDeque::new()),
            events: Arc::default(),
            operations: Mutex::new(HashMap::new()),
            next_operation: AtomicU64::new(1),
            remote_requests: Mutex::new(Vec::new()),
        })
    }
    fn gate(&self, boundary: Boundary, batch_id: u64, lose_response: bool) -> Gate {
        let operation_id = self.next_operation.fetch_add(1, Ordering::SeqCst);
        let (entered, arrival) = oneshot::channel();
        let (resume, released) = oneshot::channel();
        self.scripts.lock().unwrap().push_back(Script {
            boundary,
            pause: Pause {
                entered,
                resume: released,
            },
            lose_response,
            batch_id,
            checkpoint: None,
            operation_id,
        });
        Gate {
            entered: arrival,
            resume,
            operation_id,
        }
    }
    fn checkpoint_gate(
        &self,
        boundary: Boundary,
        batch_id: u64,
        checkpoint: &str,
        lose_response: bool,
    ) -> Gate {
        let gate = self.gate(boundary, batch_id, lose_response);
        self.scripts.lock().unwrap().back_mut().unwrap().checkpoint = Some(checkpoint.to_owned());
        gate
    }
    async fn join_requests(&self) {
        let tasks = std::mem::take(&mut *self.remote_requests.lock().unwrap());
        for task in tasks {
            task.await.unwrap();
        }
    }
}
impl Drop for ControlledService {
    fn drop(&mut self) {
        for task in self.remote_requests.get_mut().unwrap().drain(..) {
            task.abort();
        }
    }
}

async fn pause(pause: Pause, event: &Event) {
    pause
        .entered
        .send(event.clone())
        .expect("test observes operation boundary");
    pause
        .resume
        .await
        .expect("test releases operation boundary");
}

impl ControlledService {
    async fn execute_controlled(
        &self,
        request: &Request,
    ) -> azure_data_cosmos_driver::Result<AsyncRawResponse> {
        if request.method() != Method::Put || !request.url().path().ends_with("/docs/lease") {
            return self.inner.execute_request(request).await;
        }
        let bytes = match request.body() {
            Body::Bytes(bytes) => bytes.to_vec(),
            Body::SeekableStream(_) => panic!("lease requests are buffered"),
        };
        let body: Value = if azure_data_cosmos_driver::binary_json::is_binary(&bytes) {
            azure_data_cosmos_driver::binary_json::from_slice(&bytes).unwrap()
        } else {
            serde_json::from_slice(&bytes).unwrap()
        };
        let script = {
            let mut scripts = self.scripts.lock().unwrap();
            if scripts.front().is_some_and(|script| {
                script
                    .checkpoint
                    .as_deref()
                    .is_none_or(|checkpoint| body["checkpoint"].as_str() == Some(checkpoint))
            }) {
                scripts.pop_front()
            } else {
                None
            }
        };
        let etag = request
            .headers()
            .get_optional_str(&HeaderName::from_static("if-match"))
            .unwrap()
            .to_owned();
        let (operation_id, batch_id) = *self
            .operations
            .lock()
            .unwrap()
            .entry((bytes, etag.clone()))
            .or_insert_with(|| {
                (
                    script.as_ref().map_or_else(
                        || self.next_operation.fetch_add(1, Ordering::SeqCst),
                        |script| script.operation_id,
                    ),
                    script.as_ref().map_or(0, |script| script.batch_id),
                )
            });
        let mut event = Event {
            worker: self.worker,
            lease_id: body["id"].as_str().unwrap().to_owned(),
            generation: body["ownership"]["generation"].as_u64().unwrap(),
            batch_id,
            operation_id,
            phase: Phase::RequestReceived,
            if_match: etag,
        };
        let request = request.clone();
        let inner = self.inner.clone();
        let events = self.events.clone();
        let (returned, result) = oneshot::channel();
        let remote = tokio::spawn(async move {
            events.lock().unwrap().push(event.clone());
            let mut script = script;
            if script
                .as_ref()
                .is_some_and(|script| matches!(script.boundary, Boundary::BeforeCommit))
            {
                let gate = script.take().unwrap();
                pause(gate.pause, &event).await;
                script = None;
            }
            let response = inner.execute_request(&request).await;
            if response
                .as_ref()
                .is_ok_and(|response| (200..300).contains(&u16::from(response.status())))
            {
                event.phase = Phase::Committed;
                events.lock().unwrap().push(event.clone());
            }
            if let Some(script) = script {
                pause(script.pause, &event).await;
                if script.lose_response {
                    event.phase = Phase::ResponseLost;
                    events.lock().unwrap().push(event);
                    let _caller_cancelled = returned.send(Err(CosmosError::builder()
                        .with_status(azure_data_cosmos_driver::error::status_codes::TRANSPORT_BODY_READ_FAILED)
                        .with_message("response lost after commit").build()));
                    return;
                }
            }
            event.phase = Phase::ResponseDelivered;
            events.lock().unwrap().push(event);
            let _caller_cancelled = returned.send(response);
        });
        self.remote_requests.lock().unwrap().push(remote);
        result.await.map_err(|_| {
            CosmosError::builder()
                .with_status(
                    azure_data_cosmos_driver::error::status_codes::TRANSPORT_BODY_READ_FAILED,
                )
                .with_message("remote request task failed")
                .build()
        })?
    }
}

#[async_trait::async_trait]
impl TransportClient for ControlledService {
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        let mut core = Request::new(request.url.clone(), request.method);
        for (name, value) in request.headers.iter() {
            core.headers_mut().insert(name.clone(), value.clone());
        }
        if let Some(body) = &request.body {
            core.set_body(body.to_vec());
        }
        let response = self
            .execute_controlled(&core)
            .await
            .map_err(|error| TransportError::new(error, RequestSentStatus::Sent))?;
        let raw = response.try_into_raw_response().await.map_err(|error| {
            TransportError::new(
                CosmosError::builder()
                    .with_status(
                        azure_data_cosmos_driver::error::status_codes::TRANSPORT_BODY_READ_FAILED,
                    )
                    .with_source(error)
                    .with_message("test response buffering failed")
                    .build(),
                RequestSentStatus::Sent,
            )
        })?;
        Ok(HttpResponse {
            status: u16::from(raw.status()),
            headers: raw.headers().clone(),
            body: raw.body().as_ref().to_vec(),
        })
    }
}

#[derive(Debug)]
struct ControlledFactory(Arc<ControlledService>);
impl HttpClientFactory for ControlledFactory {
    fn build(
        &self,
        _: &ConnectionPoolOptions,
        _: HttpClientConfig,
    ) -> azure_data_cosmos_driver::Result<Arc<dyn TransportClient>> {
        Ok(self.0.clone())
    }
}

async fn fixture() -> Result<(Fixture, Arc<ControlledService>), Box<dyn Error>> {
    let endpoint = "https://eastus.emulator.local".parse()?;
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new("East US", endpoint)])?
        .with_consistency(ConsistencyLevel::Session);
    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
    emulator.store().create_database("db");
    for container in ["source", "leases"] {
        emulator.store().create_container_with_config(
            "db",
            container,
            serde_json::from_value(json!({"paths":["/pk"],"kind":"Hash","version":2}))?,
            ContainerConfig::new().with_partition_count(1).build()?,
        );
    }
    let a = ControlledService::new("A", emulator.clone());
    let b = ControlledService::new("B", emulator.clone());
    let mut drivers = Vec::new();
    for service in [&a, &b] {
        let runtime = emulator
            .runtime_builder()
            .with_mock_http_client_factory(Arc::new(ControlledFactory(service.clone())))
            .build()
            .await?;
        drivers.push(
            runtime
                .create_driver(
                    DriverOptions::builder(AccountReference::with_master_key(
                        "https://eastus.emulator.local".parse()?,
                        "dGVzdGtleQ==",
                    ))
                    .build(),
                )
                .await?,
        );
    }
    Ok((finish_fixture(drivers).await?, a))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn committed_checkpoint_response_loss_retries_to_412_then_reconciles_without_replaying(
) -> Result<(), Box<dyn Error>> {
    timeout(Duration::from_secs(10), async {
        let (f, service) = fixture().await?;
        let session = f.store_a.try_acquire("A").await?.unwrap();
        let expected = session.lease().await;
        let mut reader = f
            .worker_a
            .open_reader(
                ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning)
                    .with_continuation(expected.checkpoint().clone()),
            )
            .await?;
        let candidate = reader.read_page().await?.continuation().clone();
        let gate = service.gate(Boundary::AfterCommit, 1, true);
        let writing = session.clone();
        let prior = expected.clone();
        let pending = candidate.clone();
        let write = tokio::spawn(async move { writing.checkpoint(&prior, &pending).await });
        let committed = gate.entered.await?;
        assert_eq!(committed.phase, Phase::Committed);
        assert_eq!(committed.worker, "A");
        assert_eq!(committed.lease_id, "lease");
        assert_eq!(committed.generation, expected.epoch().get());
        assert_eq!(committed.batch_id, 1);
        assert_eq!(f.store_b.observe().await?.checkpoint(), candidate.as_str());
        gate.resume.send(()).unwrap();
        let error = write.await?.unwrap_err();
        match error {
            CheckpointError::Ambiguous(error) => assert_eq!(
                error.status().status_code(),
                azure_core::http::StatusCode::PreconditionFailed
            ),
            other => {
                panic!("committed-but-unacknowledged checkpoint must remain ambiguous: {other:?}")
            }
        }
        let recovered = session
            .reconcile_checkpoint(&expected, &candidate)
            .await?
            .unwrap();
        assert_eq!(recovered.lease().await.checkpoint(), &candidate);
        let trace = service.events.lock().unwrap().clone();
        assert!(trace
            .iter()
            .any(|event| event.operation_id == committed.operation_id
                && event.phase == Phase::ResponseLost));
        assert_eq!(
            trace
                .iter()
                .filter(|event| event.operation_id == committed.operation_id
                    && event.phase == Phase::Committed)
                .count(),
            1
        );
        assert!(
            trace
                .iter()
                .filter(|event| event.operation_id == committed.operation_id
                    && event.phase == Phase::RequestReceived)
                .count()
                >= 2
        );
        recovered.release().await?;
        service.join_requests().await;
        Ok::<_, Box<dyn Error>>(())
    })
    .await??;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn caller_cancellation_before_commit_does_not_cancel_the_remote_conditional_write(
) -> Result<(), Box<dyn Error>> {
    timeout(Duration::from_secs(10), async {
        let (f, service) = fixture().await?;
        let session = f.store_a.try_acquire("A").await?.unwrap();
        let initial = session.lease().await;
        let gate = service.gate(Boundary::BeforeCommit, 0, false);
        let writer = session.clone();
        let writing = tokio::spawn(async move { writer.renew().await });
        let request = gate.entered.await?;
        assert_eq!(request.phase, Phase::RequestReceived);
        writing.abort();
        assert!(writing.await.unwrap_err().is_cancelled());
        assert!(session.control().is_lost());
        assert_eq!(f.store_b.observe().await?.revision(), initial.revision());
        gate.resume.send(()).unwrap();
        service.join_requests().await;
        assert_ne!(f.store_b.observe().await?.revision(), initial.revision());
        assert_eq!(
            f.store_b.observe().await?.checkpoint(),
            initial.checkpoint().as_str()
        );
        Ok::<_, Box<dyn Error>>(())
    })
    .await??;
    Ok(())
}
