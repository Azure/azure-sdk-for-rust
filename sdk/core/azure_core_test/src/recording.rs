// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The [`Recording`] and other types used in recorded tests.

mod policy;

// cspell:ignore csprng seedable tpbwhbkhckmk
use crate::{
    credentials::{self, MockCredential},
    proxy::{
        client::{
            ClientAddSanitizerOptions, ClientRemoveSanitizersOptions, ClientSetMatcherOptions,
        },
        models::{SanitizerList, StartPayload, VariablePayload},
        policy::RecordingPolicy,
        Proxy, ProxyExt, RecordingId,
    },
    recording::policy::RecordingModePolicy,
    Matcher, Sanitizer,
};
use azure_core::{
    base64,
    credentials::TokenCredential,
    error::ErrorKind,
    http::{
        headers::{AsHeaders, Header, HeaderName, HeaderValue},
        ClientOptions,
    },
    test::TestMode,
};
use rand::{
    distr::{Alphanumeric, Distribution, SampleString, StandardUniform},
    Rng, SeedableRng,
};
use rand_chacha::ChaCha20Rng;
use std::{
    borrow::Cow,
    cell::OnceCell,
    collections::HashMap,
    env,
    sync::{Arc, Mutex, OnceLock, RwLock},
};
use tracing::span::EnteredSpan;

/// Represents a playback or recording session using the [`Proxy`].
#[derive(Debug)]
pub struct Recording {
    test_mode: TestMode,
    // Keep the span open for our lifetime.
    #[allow(dead_code)]
    span: EnteredSpan,
    proxy: Option<Arc<Proxy>>,
    test_mode_policy: OnceCell<Arc<RecordingModePolicy>>,
    recording_policy: OnceCell<Arc<RecordingPolicy>>,
    service_directory: String,
    recording_file: String,
    recording_assets_file: Option<String>,
    id: Option<RecordingId>,
    variables: RwLock<HashMap<String, String>>,
    rand: OnceLock<Mutex<ChaCha20Rng>>,
}

impl Recording {
    /// Adds a [`Sanitizer`] to sanitize PII for the current test.
    pub async fn add_sanitizer<S>(&self, sanitizer: S) -> azure_core::Result<()>
    where
        S: Sanitizer,
        azure_core::Error: From<<S as AsHeaders>::Error>,
    {
        let Some(client) = self.proxy.client() else {
            return Ok(());
        };

        let options = ClientAddSanitizerOptions {
            recording_id: self.id.as_ref(),
            ..Default::default()
        };
        client.add_sanitizer(sanitizer, Some(options)).await
    }

    /// Gets a [`TokenCredential`] you can use for testing.
    ///
    /// # Panics
    ///
    /// Panics if the [`TokenCredential`] could not be created.
    pub fn credential(&self) -> Arc<dyn TokenCredential> {
        match self.test_mode {
            TestMode::Playback => Arc::new(MockCredential) as Arc<dyn TokenCredential>,
            _ => credentials::from_env(None).map_or_else(
                |err| panic!("failed to create DeveloperToolsCredential: {err}"),
                |cred| cred as Arc<dyn TokenCredential>,
            ),
        }
    }

    /// Instruments the [`ClientOptions`] to support recording and playing back of session records.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use azure_core_test::{recorded, TestContext};
    ///
    /// # struct MyClient;
    /// # #[derive(Default)]
    /// # struct MyClientOptions { client_options: azure_core::http::ClientOptions };
    /// # impl MyClient {
    /// #   fn new(endpoint: impl AsRef<str>, options: Option<MyClientOptions>) -> Self { todo!() }
    /// #   async fn invoke(&self) -> azure_core::Result<()> { todo!() }
    /// # }
    /// #[recorded::test]
    /// async fn test_invoke(ctx: TestContext) -> azure_core::Result<()> {
    ///     let recording = ctx.recording();
    ///
    ///     let mut options = MyClientOptions::default();
    ///     ctx.instrument(&mut options.client_options);
    ///
    ///     let client = MyClient::new("https://azure.net", Some(options));
    ///     client.invoke().await
    /// }
    /// ```
    pub fn instrument(&self, options: &mut ClientOptions) {
        let Some(client) = self.proxy.client() else {
            return;
        };

        if self.test_mode == TestMode::Playback || self.test_mode == TestMode::Record {
            let test_mode_policy = self
                .test_mode_policy
                .get_or_init(|| {
                    Arc::new(RecordingModePolicy::new(
                        self.test_mode
                            .try_into()
                            .expect("supports only `Playback` and `Record`"),
                    ))
                })
                .clone();

            options.per_call_policies.push(test_mode_policy);
        }

        let recording_policy = self
            .recording_policy
            .get_or_init(|| {
                Arc::new(RecordingPolicy {
                    test_mode: self.test_mode,
                    host: Some(client.endpoint().clone()),
                    recording_id: self.id.clone(),
                    ..Default::default()
                })
            })
            .clone();

        options.per_try_policies.push(recording_policy);
    }

    /// Get random data from the OS or recording.
    ///
    /// This will always be the OS cryptographically secure pseudo-random number generator (CSPRNG) when running live.
    /// When recording, it will initialize from the OS CSPRNG but save the seed value to the recording file.
    /// When playing back, the saved seed value is read from the recording to reproduce the same sequence of random data.
    ///
    /// # Examples
    ///
    /// Generate a random integer.
    ///
    /// ```
    /// # let recording = azure_core_test::Recording::with_seed();
    /// let i: i32 = recording.random();
    /// # assert_eq!(i, 1054672670);
    /// ```
    ///
    /// Generate a symmetric data encryption key (DEK).
    ///
    /// ```
    /// # let recording = azure_core_test::Recording::with_seed();
    /// let dek: [u8; 32] = recording.random();
    /// # assert_eq!(typespec_client_core::base64::encode(dek), "HumPRAN6RqKWf0YhFV2CAFWu/8L/pwh0LRzeam5VlGo=");
    /// ```
    ///
    /// Generate a UUID.
    ///
    /// ```
    /// use azure_core::Uuid;
    /// # let recording = azure_core_test::Recording::with_seed();
    /// let uuid: Uuid = Uuid::from_u128(recording.random());
    /// # assert_eq!(uuid.to_string(), "fe906b44-5838-cc8f-05e3-c7e93edd071e");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the recording variables cannot be locked for reading or writing,
    /// or if the random seed cannot be encoded or decoded properly.
    ///
    pub fn random<T>(&self) -> T
    where
        StandardUniform: Distribution<T>,
    {
        let rng = self.rng();
        let Ok(mut rng) = rng.lock() else {
            panic!("failed to lock RNG");
        };

        rng.random()
    }

    /// Generate a random string with optional prefix.
    ///
    /// This will always be the OS cryptographically secure pseudo-random number generator (CSPRNG) when running live.
    /// When recording, it will initialize from the OS CSPRNG but save the seed value to the recording file.
    /// When playing back, the saved seed value is read from the recording to reproduce the same sequence of random data.
    ///
    /// # Examples
    ///
    /// Generate a random string.
    ///
    /// ```
    /// # let recording = azure_core_test::Recording::with_seed();
    /// let id = recording.random_string::<12>(Some("t")).to_ascii_lowercase();
    /// # assert_eq!(id, "tpbwhbkhckmk");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the recording variables cannot be locked for reading or writing,
    /// if the random seed cannot be encoded or decoded properly,
    /// if `LEN` is 0,
    /// or if the length of `prefix` is greater than or equal to `LEN`.
    ///
    /// ```should_panic
    /// # let recording = azure_core_test::Recording::with_seed();
    /// let vault_name = recording.random_string::<8>(Some("keyvault"));
    /// ```
    ///
    pub fn random_string<const LEN: usize>(&self, prefix: Option<&str>) -> String {
        struct NonZero<const N: usize>;
        impl<const N: usize> NonZero<N> {
            const ASSERT: () = assert!(N > 0, "LEN must be greater than 0");
        }
        #[allow(clippy::let_unit_value)]
        let _ = NonZero::<LEN>::ASSERT;
        let len = match prefix {
            Some(p) => {
                assert!(p.len() < LEN, "prefix length must be less than LEN");
                LEN - p.len()
            }
            None => LEN,
        };

        let rng = self.rng();
        let Ok(mut rng) = rng.lock() else {
            panic!("failed to lock RNG");
        };

        let value = Alphanumeric.sample_string(&mut *rng, len);
        match prefix {
            Some(prefix) => prefix.to_string() + &value,
            None => value,
        }
    }
    /// Removes the list of sanitizers from the recording.
    ///
    /// You can find a list of default sanitizers in [source code](https://github.com/Azure/azure-sdk-tools/blob/main/tools/test-proxy/Azure.Sdk.Tools.TestProxy/Common/SanitizerDictionary.cs).
    pub async fn remove_sanitizers(&self, sanitizers: &[&str]) -> azure_core::Result<()> {
        let Some(client) = self.proxy.client() else {
            return Ok(());
        };

        let body = SanitizerList {
            sanitizers: Vec::from_iter(sanitizers.iter().map(|s| String::from(*s))),
        };
        let options = ClientRemoveSanitizersOptions {
            recording_id: self.id.as_ref(),
            ..Default::default()
        };
        client
            .remove_sanitizers(body.try_into()?, Some(options))
            .await?;

        Ok(())
    }

    /// Sets a [`Matcher`] to compare requests and/or responses.
    pub async fn set_matcher(&self, matcher: Matcher) -> azure_core::Result<()> {
        let Some(client) = self.proxy.client() else {
            return Ok(());
        };

        let options = ClientSetMatcherOptions {
            recording_id: self.id.as_ref(),
            ..Default::default()
        };
        client.set_matcher(matcher, Some(options)).await
    }

    /// Skip recording the request body, or the entire request and response until the [`SkipGuard`] is dropped.
    ///
    /// This only affects [`TestMode::Record`] mode and is intended for cleanup.
    /// When [`Recording::test_mode()`] is [`TestMode::Playback`] you should avoid sending those requests.
    pub fn skip(&self, skip: Skip) -> azure_core::Result<SkipGuard<'_>> {
        self.set_skip(Some(skip))?;
        Ok(SkipGuard(self))
    }

    /// Gets the current [`TestMode`].
    pub fn test_mode(&self) -> TestMode {
        self.test_mode
    }

    /// Gets a required variable from the environment or recording.
    pub fn var<K>(&self, key: K, options: Option<VarOptions>) -> String
    where
        K: AsRef<str>,
    {
        let key = key.as_ref();
        self.var_opt(key, options)
            .unwrap_or_else(|| panic!("{key} is not set"))
    }

    /// Gets an optional variable from the environment or recording.
    pub fn var_opt<K>(&self, key: K, options: Option<VarOptions>) -> Option<String>
    where
        K: AsRef<str>,
    {
        let key = key.as_ref();
        if self.test_mode == TestMode::Playback {
            let variables = self.variables.read().map_err(read_lock_error).ok()?;
            return variables.get(key).cloned();
        }

        // Get the environment variable or, if unset (None), the optional VarOptions::default_value.
        let options = options.unwrap_or_default();
        let (value, sanitized) = options.apply(self.env(key));

        if self.test_mode == TestMode::Live {
            return value;
        }

        // Do not record unset (None) environment variables.
        if let Some(sanitized) = sanitized {
            let mut variables = self.variables.write().map_err(write_lock_error).ok()?;
            variables.insert(key.into(), sanitized);
        }

        value
    }
}

const RANDOM_SEED_NAME: &str = "RandomSeed";

impl Recording {
    pub(crate) fn new(
        test_mode: TestMode,
        span: EnteredSpan,
        proxy: Option<Arc<Proxy>>,
        service_directory: &'static str,
        recording_file: String,
        recording_assets_file: Option<String>,
    ) -> Self {
        Self {
            test_mode,
            span,
            proxy,
            test_mode_policy: OnceCell::new(),
            recording_policy: OnceCell::new(),
            service_directory: service_directory.into(),
            recording_file,
            recording_assets_file,
            id: None,
            variables: RwLock::new(HashMap::new()),
            rand: OnceLock::new(),
        }
    }

    // #[cfg(any(test, doctest))] // BUGBUG: https://github.com/rust-lang/rust/issues/67295
    #[doc(hidden)]
    pub fn with_seed() -> Self {
        let span = tracing::trace_span!("Recording::with_seed");
        Self {
            test_mode: TestMode::Playback,
            span: span.entered(),
            proxy: None,
            test_mode_policy: OnceCell::new(),
            recording_policy: OnceCell::new(),
            service_directory: String::from("sdk/core"),
            recording_file: String::from("none"),
            recording_assets_file: None,
            id: None,
            variables: RwLock::new(HashMap::from([(
                RANDOM_SEED_NAME.into(),
                "8S9UCR2yV8LU01tq+VNEwGssAXVUbL0Hd488GAYVosM=".into(),
            )])),
            rand: OnceLock::new(),
        }
    }

    fn env<K>(&self, key: K) -> Option<String>
    where
        K: AsRef<str>,
    {
        const AZURE_PREFIX: &str = "AZURE_";

        env::var_os(self.service_directory.clone() + "_" + key.as_ref())
            .or_else(|| env::var_os(key.as_ref()))
            .or_else(|| env::var_os(String::from(AZURE_PREFIX) + key.as_ref()))
            .and_then(|value| value.into_string().ok())
    }

    fn rng(&self) -> &Mutex<ChaCha20Rng> {
        // Use ChaCha20 for a deterministic, portable CSPRNG.
        self.rand.get_or_init(|| match self.test_mode {
            TestMode::Live => ChaCha20Rng::from_os_rng().into(),
            TestMode::Playback => {
                let variables = self
                    .variables
                    .read()
                    .map_err(read_lock_error)
                    .unwrap_or_else(|err| panic!("{err}"));
                let seed = variables
                    .get(RANDOM_SEED_NAME)
                    .unwrap_or_else(|| panic!("random seed variable not set"));
                let seed = base64::decode(seed)
                    .unwrap_or_else(|err| panic!("failed to decode random seed: {err}"));
                let seed = seed
                    .first_chunk::<32>()
                    .unwrap_or_else(|| panic!("insufficient random seed variable"));

                ChaCha20Rng::from_seed(*seed).into()
            }
            TestMode::Record => {
                let rng = ChaCha20Rng::from_os_rng();
                let seed = rng.get_seed();
                let seed = base64::encode(seed);

                let mut variables = self
                    .variables
                    .write()
                    .map_err(write_lock_error)
                    .unwrap_or_else(|err| panic!("{err}"));
                variables.insert(RANDOM_SEED_NAME.to_string(), seed);

                rng.into()
            }
        })
    }

    fn set_skip(&self, skip: Option<Skip>) -> azure_core::Result<()> {
        let Some(policy) = self.recording_policy.get() else {
            return Ok(());
        };

        let mut options = policy
            .options
            .write()
            .map_err(|err| azure_core::Error::with_message(ErrorKind::Other, err.to_string()))?;
        options.skip = skip;

        Ok(())
    }

    /// Starts recording or playback.
    ///
    /// If playing back a recording, environment variable that were recorded will be reloaded.
    pub(crate) async fn start(&mut self) -> azure_core::Result<()> {
        let Some(client) = self.proxy.client() else {
            // Assumes running live test.
            return Ok(());
        };

        let payload = StartPayload {
            recording_file: self.recording_file.clone(),
            recording_assets_file: self.recording_assets_file.clone(),
        };

        // TODO: Should RecordingId be used everywhere and models implement AsHeaders and FromHeaders?
        let recording_id = match self.test_mode {
            TestMode::Playback => {
                let result = client.playback_start(payload.try_into()?, None).await?;
                let mut variables = self.variables.write().map_err(write_lock_error)?;
                variables.extend(result.variables.into_iter());

                result.recording_id
            }
            TestMode::Record => {
                client
                    .record_start(payload.try_into()?, None)
                    .await?
                    .recording_id
            }
            mode => panic!("{mode:?} not supported"),
        };
        self.id = Some(recording_id.parse()?);

        Ok(())
    }

    /// Stops the recording or playback.
    ///
    /// If recording, environment variables that were retrieved will be recorded.
    pub(crate) async fn stop(&self) -> azure_core::Result<()> {
        let Some(client) = self.proxy.client() else {
            // Assumes running live test.
            return Ok(());
        };

        let Some(recording_id) = self.id.as_ref() else {
            // Do not return an error or we hide any test-proxy client or client under test error.
            return Ok(());
        };

        match self.test_mode {
            TestMode::Playback => client.playback_stop(recording_id.as_ref(), None).await,
            TestMode::Record => {
                let payload = {
                    let variables = self.variables.read().map_err(read_lock_error)?;
                    VariablePayload {
                        variables: HashMap::from_iter(
                            variables
                                .iter()
                                .map(|(k, value)| (k.clone(), value.clone())),
                        ),
                    }
                };
                client
                    .record_stop(recording_id.as_ref(), payload.try_into()?, None)
                    .await
            }
            mode => panic!("{mode:?} not supported"),
        }
    }
}

impl Drop for Recording {
    /// Stops the recording or playback.
    fn drop(&mut self) {
        futures::executor::block_on(self.stop()).unwrap_or_else(|err| panic!("{err}"));
    }
}

fn read_lock_error(_: impl std::error::Error) -> azure_core::Error {
    azure_core::Error::with_message(ErrorKind::Other, "failed to lock variables for read")
}

fn write_lock_error(_: impl std::error::Error) -> azure_core::Error {
    azure_core::Error::with_message(ErrorKind::Other, "failed to lock variables for write")
}

/// What to skip when recording to a file.
///
/// This only affects [`TestMode::Record`] mode and is intended for cleanup.
/// When [`Recording::test_mode()`] is [`TestMode::Playback`] you should avoid sending those requests.
#[derive(Debug)]
pub enum Skip {
    /// Skip recording only the request body.
    RequestBody,

    /// Skip recording both the request and response entirely.
    RequestResponse,
}

impl Header for Skip {
    fn name(&self) -> HeaderName {
        HeaderName::from_static("x-recording-skip")
    }

    fn value(&self) -> HeaderValue {
        match self {
            Self::RequestBody => HeaderValue::from_static("request-body"),
            Self::RequestResponse => HeaderValue::from_static("request-response"),
        }
    }
}

/// When the `SkipGuard` is dropped, recording requests and responses will begin again.
///
/// Returned from [`Recording::skip()`].
pub struct SkipGuard<'a>(&'a Recording);

impl Drop for SkipGuard<'_> {
    fn drop(&mut self) {
        if self.0.test_mode == TestMode::Record {
            let _ = self.0.set_skip(None);
        }
    }
}

/// Options for getting variables from a [`Recording`].
#[derive(Clone, Debug)]
pub struct VarOptions {
    /// The value to return if not already recorded.
    pub default_value: Option<Cow<'static, str>>,

    /// Whether to sanitize the variable value with [`VarOptions::sanitize_value`].
    pub sanitize: bool,

    /// The value to use for sanitized variables.
    ///
    /// The default is "Sanitized".
    pub sanitize_value: Cow<'static, str>,
}

impl VarOptions {
    /// Returns a tuple of the `value` or [`VarOptions::default_value`], and the sanitized value.
    ///
    /// The `value` is only replaced with the `VarOptions::default_value` if `None`. This is returned as the first tuple field.
    ///
    /// The [`VarOptions::sanitize_value`] is only `Some` if [`VarOptions::sanitize`] is `true`. This is returned as the second tuple field.
    fn apply<S: Into<String>>(self, value: Option<S>) -> (Option<String>, Option<String>) {
        let value = value.map_or_else(
            || self.default_value.as_deref().map(ToString::to_string),
            |value| Some(value.into()),
        );
        let sanitized = match value.as_deref() {
            None => None,
            Some(_) if self.sanitize => Some(self.sanitize_value.to_string()),
            Some(v) => Some(v.to_string()),
        };
        (value, sanitized)
    }
}

impl Default for VarOptions {
    fn default() -> Self {
        Self {
            default_value: None,
            sanitize: false,
            sanitize_value: Cow::Borrowed(crate::DEFAULT_SANITIZED_VALUE),
        }
    }
}

#[test]
fn test_var_options_apply() {
    let (value, ..) = VarOptions::default().apply(None::<String>);
    assert_eq!(value, None);

    let (value, ..) = VarOptions::default().apply(Some("".to_string()));
    assert_eq!(value, Some(String::new()));

    let (value, ..) = VarOptions::default().apply(Some("test".to_string()));
    assert_eq!(value, Some("test".into()));

    let (value, ..) = VarOptions {
        default_value: None,
        ..Default::default()
    }
    .apply(None::<String>);
    assert_eq!(value, None);

    let (value, ..) = VarOptions {
        default_value: Some("".into()),
        ..Default::default()
    }
    .apply(None::<String>);
    assert_eq!(value, Some("".into()));

    let (value, ..) = VarOptions {
        default_value: Some("test".into()),
        ..Default::default()
    }
    .apply(None::<String>);
    assert_eq!(value, Some("test".into()));

    let (value, ..) = VarOptions {
        default_value: Some("default".into()),
        ..Default::default()
    }
    .apply(Some("".to_string()));
    assert_eq!(value, Some("".into()));

    let (value, ..) = VarOptions {
        default_value: Some("default".into()),
        ..Default::default()
    }
    .apply(Some("test".to_string()));
    assert_eq!(value, Some("test".into()));
}

#[test]
fn test_var_options_apply_sanitized() {
    let (value, sanitized) = VarOptions::default().apply(None::<String>);
    assert_eq!(value, None);
    assert_eq!(sanitized, None);

    let (value, sanitized) = VarOptions {
        sanitize: true,
        ..Default::default()
    }
    .apply(None::<String>);
    assert_eq!(value, None);
    assert_eq!(sanitized, None);

    let (value, sanitized) = VarOptions {
        sanitize: true,
        ..Default::default()
    }
    .apply(Some("".to_string()));
    assert_eq!(value, Some("".to_string()));
    assert_eq!(sanitized, Some("Sanitized".into()));

    let (value, sanitized) = VarOptions {
        sanitize: true,
        ..Default::default()
    }
    .apply(Some("test".to_string()));
    assert_eq!(value, Some("test".to_string()));
    assert_eq!(sanitized, Some("Sanitized".into()));

    let (value, sanitized) = VarOptions {
        sanitize: true,
        sanitize_value: "*****".into(),
        ..Default::default()
    }
    .apply(None::<String>);
    assert_eq!(value, None);
    assert_eq!(sanitized, None);

    let (value, sanitized) = VarOptions {
        sanitize: true,
        sanitize_value: "*****".into(),
        ..Default::default()
    }
    .apply(Some("".to_string()));
    assert_eq!(value, Some("".to_string()));
    assert_eq!(sanitized, Some("*****".into()));

    let (value, sanitized) = VarOptions {
        sanitize: true,
        sanitize_value: "*****".into(),
        ..Default::default()
    }
    .apply(Some("test".to_string()));
    assert_eq!(value, Some("test".to_string()));
    assert_eq!(sanitized, Some("*****".into()));
}
