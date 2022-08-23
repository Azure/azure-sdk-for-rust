use crate::clients::policy::AuthorizationPolicy;
use azure_core::{auth::TokenCredential, ClientOptions, Pipeline, TimeoutPolicy};
use std::sync::Arc;

pub(crate) fn new_pipeline_from_options(
    credentials: Arc<dyn TokenCredential>,
    scope: String,
) -> Pipeline {
    let auth_policy: Arc<dyn azure_core::Policy> =
        Arc::new(AuthorizationPolicy::new(credentials, scope));

    // TODO: as we move to the builder pattern for the clients, these should be
    // set there.
    let client_options = ClientOptions::default();
    let timeout_policy = TimeoutPolicy::new(None);

    // The `AuthorizationPolicy` must be the **last** retry policy.
    // Policies can change the url and/or the headers, and the `AuthorizationPolicy`
    // must be able to inspect them or the resulting token will be invalid.
    let per_retry_policies = vec![
        Arc::new(timeout_policy) as Arc<dyn azure_core::Policy>,
        auth_policy,
    ];

    Pipeline::new(
        option_env!("CARGO_PKG_NAME"),
        option_env!("CARGO_PKG_VERSION"),
        client_options,
        Vec::new(),
        per_retry_policies,
    )
}
