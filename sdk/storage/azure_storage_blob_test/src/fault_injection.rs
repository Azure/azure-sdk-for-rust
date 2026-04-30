// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;
use azure_core::http::{
    headers::HeaderName,
    policies::{Policy, PolicyResult},
    Context, Request, Url,
};
use futures::lock::Mutex;
use rand::{distr::StandardUniform, RngExt};

const FAULT_INJECTION_HEADER: HeaderName =
    HeaderName::from_static("x-ms-faultinjector-response-option");
const UPSTREAM_HEADER: HeaderName = HeaderName::from_static("x-upstream-base-uri");

const FULL_RESPONSE: &str = "f";
const PARTIAL_RESPONSE_HANG: &str = "p";
const PARTIAL_RESPONSE_CLOSE: &str = "pc";
const PARTIAL_RESPONSE_ABORT: &str = "pa";
const PARTIAL_RESPONSE_NORMAL: &str = "pn";
const NO_RESPONSE_HANG: &str = "n";
const NO_RESPONSE_CLOSE: &str = "nc";
const NO_RESPONSE_ABORT: &str = "na";

#[derive(Debug)]
pub struct FaultInjectionPolicy<Rng> {
    fault_injector_endpoint: Url,
    injector: Arc<Mutex<ProbabilityHeaderInjector<Rng>>>,
}

impl<Rng: rand::SeedableRng + Debug + Send + Sync> FaultInjectionPolicy<Rng> {
    pub fn new(
        fault_injector_endpoint: Url,
        probabilities: FaultInjectionProbabilities,
    ) -> azure_core::Result<Self> {
        Ok(Self {
            fault_injector_endpoint,
            injector: Arc::new(Mutex::new(ProbabilityHeaderInjector {
                header_name: FAULT_INJECTION_HEADER,
                header_value_probabilities: probabilities.try_into()?,
                rng: rand::make_rng(),
            })),
        })
    }
}

#[async_trait]
impl<Rng: rand::Rng + Debug + Send + Sync> Policy for FaultInjectionPolicy<Rng> {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        if request.headers().get_str(&UPSTREAM_HEADER).is_err() {
            let f = Url::parse(&format!(
                "{}://{}{}",
                request.url().scheme(),
                request.url().host_str().unwrap(),
                request
                    .url()
                    .port()
                    .map(|port| format!(":{port}"))
                    .unwrap_or_default(),
            ))?;
            request.headers_mut().insert(UPSTREAM_HEADER, f.to_string());
        }

        request
            .url_mut()
            .set_scheme(self.fault_injector_endpoint.scheme())
            .unwrap();
        request
            .url_mut()
            .set_host(self.fault_injector_endpoint.host_str())
            .unwrap();
        request
            .url_mut()
            .set_port(self.fault_injector_endpoint.port())
            .unwrap();

        self.injector.lock().await.inject(request);

        next[0].send(ctx, request, &next[1..]).await
    }
}

#[derive(Clone, Debug, Default)]
pub struct FaultInjectionProbabilities {
    pub partial_response_hang: f32,
    pub partial_response_close: f32,
    pub partial_response_abort: f32,
    pub partial_response_normal: f32,
    pub no_response_hang: f32,
    pub no_response_close: f32,
    pub no_response_abort: f32,
}

impl FaultInjectionProbabilities {
    pub fn is_zero(&self) -> bool {
        self.partial_response_hang == 0.0
            && self.partial_response_close == 0.0
            && self.partial_response_abort == 0.0
            && self.partial_response_normal == 0.0
            && self.no_response_hang == 0.0
            && self.no_response_close == 0.0
            && self.no_response_abort == 0.0
    }
}

impl TryFrom<FaultInjectionProbabilities> for Vec<(f32, &'static str)> {
    type Error = azure_core::Error;
    fn try_from(value: FaultInjectionProbabilities) -> Result<Self, Self::Error> {
        let mut vec = Vec::new();
        let non_negative = |value: f32, name: &str| {
            if value < 0.0 {
                Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{name} must be a non-negative value."),
                ))
            } else {
                Ok(value)
            }
        };

        for (prob, name, header_value) in [
            (
                value.partial_response_hang,
                "partial_response_hang",
                PARTIAL_RESPONSE_HANG,
            ),
            (
                value.partial_response_close,
                "partial_response_close",
                PARTIAL_RESPONSE_CLOSE,
            ),
            (
                value.partial_response_abort,
                "partial_response_abort",
                PARTIAL_RESPONSE_ABORT,
            ),
            (
                value.partial_response_normal,
                "partial_response_normal",
                PARTIAL_RESPONSE_NORMAL,
            ),
            (value.no_response_hang, "no_response_hang", NO_RESPONSE_HANG),
            (
                value.no_response_close,
                "no_response_close",
                NO_RESPONSE_CLOSE,
            ),
            (
                value.no_response_abort,
                "no_response_abort",
                NO_RESPONSE_ABORT,
            ),
        ] {
            if non_negative(prob, name)? > 0.0 {
                vec.push((prob, header_value));
            }
        }

        if vec.iter().map(|(prob, _)| *prob).sum::<f32>() > 1.0 {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "Fault probabilities exceeded 1.0.",
            ));
        }

        Ok(vec)
    }
}

/// Struct that applies up to one of a list of header values to a request
/// under a predefined header key.
#[derive(Debug)]
struct ProbabilityHeaderInjector<Rng> {
    header_name: HeaderName,
    header_value_probabilities: Vec<(f32, &'static str)>,
    rng: Rng,
}

impl<Rng: rand::Rng> ProbabilityHeaderInjector<Rng> {
    fn inject(&mut self, request: &mut Request) {
        let f: f32 = self.rng.sample(StandardUniform);
        let mut prob_sum = 0.0;
        for (prob, value) in self.header_value_probabilities.iter() {
            prob_sum += *prob;
            if f < prob_sum {
                request
                    .headers_mut()
                    .insert(self.header_name.clone(), *value);
                return;
            }
        }
        request
            .headers_mut()
            .insert(self.header_name.clone(), FULL_RESPONSE);
    }
}
