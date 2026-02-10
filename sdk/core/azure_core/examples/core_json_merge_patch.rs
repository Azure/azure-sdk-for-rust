// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{http::Transport, Value};
use example::{setup, ExampleClient, ExampleClientOptions};

/// This sample demonstrates how to update a resource and send it back in a [JSON merge patch](https://www.rfc-editor.org/rfc/rfc7396).
///
/// Basically,
///
/// * Property values are updated to new values in the request body.
/// * Property values set to an explicit `null` are deleted or unset.
/// * Missing properties are not changed.
/// * Arrays are replaced entirely by the contents of the array in the request body.
///
/// You can deserialize a response payload into a [`Value`] and change values or even set to an explicit [`Value::Null`] as shown below.
async fn example_json_merge_patch() -> Result<(), Box<dyn std::error::Error>> {
    let mut options = ExampleClientOptions::default();

    // Ignore: this is only set up for testing.
    // You normally would create credentials from `azure_identity` and
    // use the default transport in production.
    let transport = setup()?;
    options.client_options.transport = Some(Transport::new(transport));

    let client = ExampleClient::new("https://api.contoso.com", Some(options))?;

    // Azure SDK for Rust does not implement support for JSON merge patch directly,
    // but does allow you to de/serialize your own models including a generic JSON `Value`.
    let mut resource: Value = client.get_resource("foo", None).await?.body().json()?;

    // Change the description and update tags.
    resource["description"] = "an updated foo".into();
    resource["optional"] = Value::Null;
    if let Some(tags) = resource["tags"].as_object_mut() {
        tags["test"] = true.into();
        tags.insert("version".into(), 1.into());
    }

    // Update the resource and assert expected properties.
    let resource = client
        .update_resource("foo", resource.try_into()?, None)
        .await?
        .into_model()?;

    assert_eq!(resource.id.as_deref(), Some("foo"));
    assert_eq!(resource.description.as_deref(), Some("an updated foo"));
    assert_eq!(resource.optional, None);

    let tags = resource.tags.expect("expected tags");
    assert_eq!(tags["test"], Value::Bool(true));
    assert_eq!(tags["version"], Value::Number(1.into()));

    Ok(())
}

// ----- BEGIN TEST SETUP -----
#[tokio::test]
async fn test_core_json_merge_patch() -> Result<(), Box<dyn std::error::Error>> {
    example_json_merge_patch().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    example_json_merge_patch().await
}

mod example {
    use azure_core::{
        fmt::SafeDebug,
        http::{
            headers::Headers, AsyncRawResponse, ClientMethodOptions, ClientOptions, HttpClient,
            Method, Pipeline, Request, RequestContent, Response, StatusCode, Url,
        },
        Bytes, Value,
    };
    use azure_core_test::http::MockHttpClient;
    use futures::FutureExt;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::{collections::HashMap, sync::Arc};

    #[allow(clippy::type_complexity)]
    pub fn setup() -> Result<Arc<dyn HttpClient>, Box<dyn std::error::Error>> {
        let client = MockHttpClient::new(|req| {
            let mut resource = json!({
                "id": "foo",
                "description": "just a foo",
                "optional": "set",
                "tags": {
                    "test": false
                }
            });
            async move {
            match req.url().path() {
                "/foo" if req.method() == Method::Get => Ok(AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    resource.to_string(),
                )),
                "/foo" if req.method() == Method::Patch => {
                    let Ok(patch) = serde_json::from_slice::<Value>(&Bytes::from(req.body()))
                    else {
                        return Ok(AsyncRawResponse::from_bytes(
                            StatusCode::BadRequest,
                            Headers::new(),
                            r#"{"error":{"code":"BadParameter","message":"Invalid JSON merge patch"}}"#,
                        ));
                    };
                    json_patch::merge(&mut resource, &patch);
                    Ok(AsyncRawResponse::from_bytes(
                        StatusCode::Ok,
                        Headers::new(),
                        resource.to_string(),
                    ))
                }
                _ => panic!("unexpected request"),
            }
        }
        .boxed()
        });

        Ok(Arc::new(client))
    }

    #[derive(Debug)]
    pub struct ExampleClient {
        endpoint: Url,
        pipeline: Pipeline,
    }

    #[derive(Debug, Default, Clone)]
    pub struct ExampleClientOptions {
        pub client_options: ClientOptions,
    }

    #[derive(Debug, Default, Clone)]
    pub struct ExampleClientGetResourceOptions<'a> {
        pub method_options: ClientMethodOptions<'a>,
    }

    #[derive(Debug, Default, Clone)]
    pub struct ExampleClientUpdateResourceOptions<'a> {
        pub method_options: ClientMethodOptions<'a>,
    }

    #[derive(Clone, Default, Deserialize, SafeDebug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Resource {
        pub id: Option<String>,
        pub description: Option<String>,
        // We want to serialize an explicit null.
        pub optional: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<HashMap<String, Value>>,
    }

    impl ExampleClient {
        pub fn new(
            endpoint: &str,
            options: Option<ExampleClientOptions>,
        ) -> azure_core::Result<Self> {
            let endpoint: Url = endpoint.parse()?;
            let options = options.unwrap_or_default();

            Ok(Self {
                endpoint,
                pipeline: Pipeline::new(
                    option_env!("CARGO_PKG_NAME"),
                    option_env!("CARGO_PKG_VERSION"),
                    options.client_options,
                    Vec::new(),
                    Vec::new(),
                    None,
                ),
            })
        }

        pub async fn get_resource(
            &self,
            name: &str,
            options: Option<ExampleClientGetResourceOptions<'_>>,
        ) -> azure_core::Result<Response<Resource>> {
            if name.is_empty() {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "parameter name cannot be empty",
                ));
            }
            let options = options.unwrap_or_default();
            let ctx = options.method_options.context.to_borrowed();
            let mut url = self.endpoint.clone();
            let path = name.to_string();
            url = url.join(&path)?;
            let mut req = Request::new(url, Method::Get);
            req.insert_header("accept", "application/json");
            let resp = self.pipeline.send(&ctx, &mut req, None).await?;
            Ok(resp.into())
        }

        pub async fn update_resource(
            &self,
            name: &str,
            resource: RequestContent<Resource>,
            options: Option<ExampleClientUpdateResourceOptions<'_>>,
        ) -> azure_core::Result<Response<Resource>> {
            if name.is_empty() {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "parameter name cannot be empty",
                ));
            }
            let options = options.unwrap_or_default();
            let ctx = options.method_options.context.to_borrowed();
            let mut url = self.endpoint.clone();
            let path = name.to_string();
            url = url.join(&path)?;
            let mut req = Request::new(url, Method::Patch);
            req.insert_header("accept", "application/json");
            req.set_body(resource);
            let resp = self.pipeline.send(&ctx, &mut req, None).await?;
            Ok(resp.into())
        }
    }
}
// ----- END TEST SETUP -----
