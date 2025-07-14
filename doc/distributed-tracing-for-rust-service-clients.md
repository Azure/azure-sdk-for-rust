<!-- Copyright(C) Microsoft Corp. All Rights Reserved. -->

<!-- cspell: ignore liudmila -->
# Distributed tracing options in Rust service clients

## Distributed tracing fundamentals

There are three core constructs which are used in distributed tracing:

* Tracer Providers
* Tracers
* Spans

### Tracer Provider

The job of a "Tracer Provider" is to be a factory for tracers. It is the "gateway" construct for distributed tracing.

### Tracer

A "tracer" is a factory for "Spans". A `Tracer` is configured with three parameters:

* `namespace` - the "namespace" for the service client. The namespace for all azure services are listed [on this page](https://learn.microsoft.com/azure/azure-resource-manager/management/azure-services-resource-providers).
* `package name` - this is typically the Cargo package name for the service client (`env!("CARGO_PKG_NAME")`)
* `package version` - this is typically the version of the Cargo package for the service client (`env!("CARGO_PKG_VERSION")`)

Tracers have three mechanisms for creating spans:

* Create a new root span.
* Create a new child span from a parent span.
* Create a new child span from the "current" span (where the "current" span is tracer implementation specific).

### Span

A "Span" is a unit of tracing. Each span has the following attributes:

* "name" - the "name" of the span. For public APIs, this is typically the name of the public API, for HTTP request, it is typically the HTTP verb.
* "kind" - HTTP spans come in several flavours:
  * Internal - the "default" for span kinds.
  * Client - represents a client application - HTTP request spans are "Client" spans.
  * Server - represents a server - Azure SDK clients will never use these.
  * Producer - represents a messaging (Event Hubs and Service Bus) message producer.
  * Consumer - represents a message consumer.
* "status" - A span status is either "Unset" or "Error" - OpenTelemetry defines a status of "Ok" in addition to these, but it is reserved for client applications.
* "attributes" - the attributes on a span describe the span. Attributes include:
  * "az.namespace" - the namespace of a request.
  * "url.full" - the full (sanitized) URL for an HTTP request
  * "server.address" - the DNS address of the HTTP server
  * "http.request.method" - the HTTP method used for the request ("GET", "PUT" etc).

## Azure Distributed Tracing requirements

Azure's distributed tracing requirements are laid out in a number of documents:

* [Azure Distributed Tracing Conventions](https://github.com/Azure/azure-sdk/blob/main/docs/tracing/distributed-tracing-conventions.md)
* [Azure Distributed Tracing Implementation](https://github.com/Azure/azure-sdk/blob/main/docs/general/implementation.md#distributed-tracing)
* [Open Telemetry HTTP Span Conventions](https://opentelemetry.io/docs/specs/semconv/http/http-spans/)

Looking at each document, the following two requirements for distributed tracing clients fall out:

1) Each public API (service client function) needs to have a span with the `az.namespace` attribute added - the az.attribute (as defined above). [See this for more information](https://github.com/Azure/azure-sdk/blob/main/docs/tracing/distributed-tracing-conventions.md#public-api-calls).
1) Each HTTP request needs to have a span with the same `az.namespace` attribute and a number of other attributes derived from the HTTP operation. [See this for more information](https://github.com/Azure/azure-sdk/blob/main/docs/tracing/distributed-tracing-conventions.md#http-client-spans). The HTTP request span should be a child of a public API span if possible.

Implementations are allowed to skip adding the `az.namespace` attribute but it is strongly discouraged.

It turns out that in OpenTelemetry, an `OpenTelemetry::Tracer` is constructed with an `InstrumentationScope` which allows arbitrary attributes to be attached to the tracer, which is also attached  to each span constructed from the tracer. As such, it makes sense for each service client to have a `Tracer` attached to the service client, and this `Tracer` can be used to hold the namespace attribute. This architecture is reflected in the distributed tracing wrapper API, the `Tracer` trait contains a `namespace()` function.

## Additional requirements

For public APIs, the rule of thumb is: "If the operation does not take time and cannot fail, it doesn't get a span". For most public APIs, this isn't a huge deal, but for pageable and long running operations, it changes how the code is generated. Specifically, for pageables, the actual service client does not actually interact with the network and cannot fail, but the individual pager returned does interact with the network and can fail - thus the pager's interactions with the service need to be instrumented with a span. Long Running Operations behave similarly - while the original API is instrumented with a span, the same is true for the status monitor - it also needs to be instrumented with a span whose name matches the name of the original API.

In addition, [certain service clients](https://github.com/Azure/azure-sdk/blob/main/docs/tracing/distributed-tracing-conventions.md#library-specific-attributes) (Cosmos DB, KeyVault, etc) have additional client-specific attributes which need to be added to the span.

## Core API design

Given this architecture, it implies that each service client needs the following:

1) A struct field named `tracer` which is an `Arc<dyn Tracing::Tracer>` which holds the tracing implementation specific tracer.
2) Code in the service client's `new` function which instantiates a `tracer` from the `TracerProvider` configured in the service client options. The primary function of this tracer is to provide the value for the `az.namespace` attribute for both the public API spans and the HTTP request spans.
3) Code in each service client public API which instantiates a public API span.

For the Rust implementation, if a tracer provider is configured, ALL http operations will have HTTP request spans created regardless of whether the public API spans are created.

## Implementation details

To provide for requirement #1, if a customer provides a value for the `azure_core::ClientOptions::request_instrumentation` structure, the Azure Core HTTP pipeline will add a `PublicApiInstrumentationPolicy` to the pipeline which is responsible for creating the public API outer span.

To provide for requirement #2, if a customer provides a `azure_core::ClientOptions::request_instrumentation` the `azure_core` HTTP pipeline will add a `RequestInstrumentationPolicy` to the pipeline which is responsible for creating the required HTTP request span to the pipeline.

This implementation means that operations like Long Running Operations (Pollers) and Pageable Operations (Pagers) will all have a Public API span created by the `PublicApiInstrumentationPolicy` and a HTTP Request span created by the `RequestInstrumentationPolicy`.

### Pipeline Construction

When an `azure_core::http::Pipeline` is constructed, if the client options include a tracing provider, then the pipeline will create a tracer from that tracer provider with the crate name and crate version (which are both parameters to the pipeline constructor). This tracer will have a namespace of "None" and acts as a fallback in case the public APIs don't provide a `Tracer` implementation (if, for example public APIs are instrumented, but the service client itself is not instrumented). This tracer will be passed to both of the tracing policies.

### PublicApiInstrumentationPolicy

1) If the pipeline context has a `Arc<dyn Span>` attached to the context, then the public API policy will simply call the next policy in the pipeline, because a span in the pipeline indicates that this API call is  a nested API call.
1) If the context does not have a `PublicApiInstrumentationInformation` attached to it, the policy  will call the next policy in the pipeline, otherwise the policy will:
   1) Look for an `Arc<dyn Tracer>` attached to the context. If one is found, it uses that tracer, otherwise it uses a tracer attached to the policy.
   1) Create a span with a name matching the `name` in the [`PublicApiInstrumentationInformation`] structure and attributes from the attributes attached to the `PublicApiInstrumentationInformation`. It will also add the `az.namespace` attribute to the span if the tracer has a namespace associated with it (this will typically only be the case for tracers attached to the context).
   1) Once the span has been created, the policy will attach the newly created span to the context so other policies in the pipeline (specifically the `RequestInstrumentationPolicy` can use this span).
1) Once the span has been created, the policy calls the next policy in the pipeline.
1) After the remaining policies in the pipeline have run, the policy inspects the `Result` of the pipeline execution and sets the `error.type` attribute and the span status based on the `Result` of the operation.

### RequestInstrumentationPolicy

The `RequestInstrumentationPolicy` will do the following:

1) If the `Context` parameter for the  `RequestInstrumentationPolicy` contains a `Tracer` value, then the `RequestInstrumentationPolicy` will use that `Tracer` value to create the span, otherwise it will use the pre-configured tracer from when the policy was created.
2) If the `Context` parameter for the `RequestInstrumentationPolicy contains a`Span` value, then the policy will use that span as the parent span for the newly created HTTP request span, otherwise it will create a new span.

This design means that even if a service public API is not fully instrumented with a `Tracer` or a `Span`, it will still generate some HTTP request traces.

Since the namespace attribute is service-client wide, it makes sense to capture that inside a per-service client field, that way it can be easily accessed from service clients.

## Convenience Macros

To facilitate the implementation of the three core requirements above, three attribute-like macros are defined for the use of each service.

Those macros are:

* `#[tracing::client]` - applied to each service client `struct` declaration.
* `#[tracing::new]` - applied to each service client "constructor".
* `#[tracing::function]` - applied to each service client "public API".

### `#[tracing::client]`

The `tracing::client` attribute macro does one thing and one thing only: It defines a field named `tracer` which is added to the list of fields in the service client structure.

#### Modification introduced by this macro

From:

```rust
pub struct MyServiceClient {
    endpoint: Url,
}
```

to

```diff
pub struct MyServiceClient {
    endpoint: Url,
+    tracer: std::sync::Arc<dyn azure_core::tracing::Tracer>,
}
```

Arguably this attribute is unnecessary, but it can be incredibly helpful especially if we need to add more elements to each service client in the future.

### `#[tracing::new(<namespace>)]`

Annotates a `new` service client function to initialize the `tracer` field in the structure.

#### Modification introduced by this macro

from:

```diff
pub fn new(
    endpoint: &str,
    credential: Arc<dyn TokenCredential>,
    options: Option<SecretClientOptions>,
) -> Result<Self> {
    let options = options.unwrap_or_default();
    let mut endpoint = Url::parse(endpoint)?;
    if !endpoint.scheme().starts_with("http") {
        return Err(azure_core::Error::message(
            azure_core::error::ErrorKind::Other,
            format!("{endpoint} must use http(s)"),
        ));
    }
    endpoint.set_query(None);
    let auth_policy: Arc<dyn Policy> = Arc::new(BearerTokenCredentialPolicy::new(
        credential,
        vec!["https://vault.azure.net/.default"],
    ));
    Ok(Self {
        endpoint,
        api_version: options.api_version,
        pipeline: Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options.client_options,
            Vec::default(),
            vec![auth_policy],
        ),
     })
}
```

to:

```diff
pub fn new(
    endpoint: &str,
    credential: Arc<dyn TokenCredential>,
    options: Option<SecretClientOptions>
) -> Result<Self> {
    let options = options.unwrap_or_default();
    let mut endpoint = Url::parse(endpoint)?;
    if !endpoint.scheme().starts_with("http") {
        return Err(azure_core::Error::message(
            azure_core::error::ErrorKind::Other,
            format!("{endpoint} must use http(s)"),
            ));
    }
    endpoint.set_query(None);
    let auth_policy: Arc<dyn Policy> = Arc::new(BearerTokenCredentialPolicy::new(
        credential,
        vec!["https://vault.azure.net/.default"],
    ));
+    let tracer = tracing::create_tracer(Some("<Service Namespace>"),
+        option_env!("CARGO_PKG_NAME").unwrap_or("UNKNOWN"),
+        option_env!("CARGO_PKG_VERSION").unwrap_or("UNKNOWN"),
+        &options.client_options);
    Ok(Self {
+            tracer,
           endpoint,
           api_version: options.api_version,
           pipeline: Pipeline::new(
               option_env!("CARGO_PKG_NAME"),
               option_env!("CARGO_PKG_VERSION"),
               options.client_options,
               Vec::default(),
               vec![auth_policy],
           ),
       })
}
```

Note that this implementation takes advantage of a helper function `create_tracer` - without this function, the logic to create the per-client tracer looks like:

```rust
let tracer =
    if let Some(tracer_options) = &options.azure_client_options.request_instrumentation {
        tracer_options
            .tracing_provider
            .as_ref()
            .map(|tracing_provider| {
                tracing_provider.get_tracer(
                    Some("<Service Namespace>"),
                    option_env!("CARGO_PKG_NAME").unwrap_or("UNKNOWN"),
                    option_env!("CARGO_PKG_VERSION").unwrap_or("UNKNOWN"),
                )
            })
    } else {
        None
    };
```

#### Pros

* Simple implementation for clients
* Adds ability for centralized implementation

#### Cons

* Potentially fragile - there are several patterns for implementing the `Self` element within a `new` function, each of which needs to be handled - the current macro handles `Self{}` and `Ok(Self{})` but there are other patterns like `let this = Self {}; this` or similar constructs which would be skipped.
* Does not handle `builder` patterns at all.

### `#[tracing::function(<client_name>::<function_name>)]`

Applied to all public functions in the service client ("public APIs" in distributed tracing terms). This macro creates the client span for each service client method, and updates the client span if appropriate.

#### Modification introduced by this macro

From:

```rust
pub async fn get(
    &self,
    path: &str,
    options: Option<TestServiceClientGetMethodOptions<'_>>,
) -> Result<RawResponse> {
    let options = options.unwrap_or_default();
    let mut url = self.endpoint.clone();
    url.set_path(path);
    url.query_pairs_mut()
        .append_pair("api-version", &self.api_version);

    let mut request = Request::new(url, azure_core::http::Method::Get);

    let response = self
        .pipeline
        .send(&options.method_options.context, &mut request)
        .await?;
    if !response.status().is_success() {
        return Err(azure_core::Error::message(
            azure_core::error::ErrorKind::HttpResponse {
            status: response.status(),
            error_code: None,
        },
        format!("Failed to GET {}: {}", request.url(), response.status())));
    }
    Ok(response)
}
```

To:

```diff
pub async fn get(
    &self,
    path: &str,
    options: Option<TestServiceClientGetMethodOptions<'_>>,
) -> Result<RawResponse> {
+    let mut options = options.unwrap_or_default();

+    let public_api_info = PublicApiInstrumentationInformation {
+        api_name: "get_with_tracing",
+        attributes: vec![],
+    };
+
+    // Add the span to the tracer.
+    let mut ctx = options.method_options.context.with_value(public_api_info);
+    // If the service has a tracer, we add it to the context.
+    if let Some(tracer) = &self.tracer {
+        ctx = ctx.with_value(tracer.clone());
+    }

    let mut url = self.endpoint.clone();
    url.set_path(path);
    url.query_pairs_mut()
        .append_pair("api-version", &self.api_version);

    let mut request = Request::new(url, azure_core::http::Method::Get);

    let response = self
        .pipeline
-        .send(&options.method_options.context, &mut request)
+        .send(&ctx, &mut request)
        .await?;
    if !response.status().is_success() {
        return Err(azure_core::Error::message(
            azure_core::error::ErrorKind::HttpResponse {
            status: response.status(),
            error_code: None,
        },
        format!("Failed to GET {}: {}", request.url(), response.status())));
    }
    response
}
```
