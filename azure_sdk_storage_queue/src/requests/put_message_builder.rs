use crate::prelude::*;
use crate::responses::*;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, ToAssign, Yes};
use azure_sdk_storage_core::prelude::*;
use hyper::StatusCode;
use std::borrow::Cow;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct PutMessageBuilder<'a, 'b, C, MessageBodySet>
where
    MessageBodySet: ToAssign,
    C: Client,
{
    queue_name_service: &'a dyn QueueNameService<StorageClient = C>,
    p_message_body: PhantomData<MessageBodySet>,
    message_body: Option<Cow<'b, str>>,
    visibility_timeout_seconds: u64,
    message_ttl_seconds: u64,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
}

impl<'a, 'b, C> PutMessageBuilder<'a, 'b, C, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(
        queue_name_service: &'a dyn QueueNameService<StorageClient = C>,
    ) -> PutMessageBuilder<'a, 'b, C, No> {
        PutMessageBuilder {
            queue_name_service,
            p_message_body: PhantomData {},
            message_body: None,
            visibility_timeout_seconds: 0,
            message_ttl_seconds: 25200,
            timeout: None,
            client_request_id: None,
        }
    }
}

//set mandatory no traits methods
impl<'a, 'b, C> MessageBodyRequired for PutMessageBuilder<'a, 'b, C, Yes>
where
    C: Client,
{
    #[inline]
    fn message_body(&self) -> &str {
        self.message_body.as_ref().unwrap()
    }
}

impl<'a, 'b, C, MessageBodySet> VisibilityTimeoutRequired
    for PutMessageBuilder<'a, 'b, C, MessageBodySet>
where
    MessageBodySet: ToAssign,
    C: Client,
{
    #[inline]
    fn visibility_timeout_seconds(&self) -> u64 {
        self.visibility_timeout_seconds
    }
}

impl<'a, 'b, C, MessageBodySet> MessageTTLRequired for PutMessageBuilder<'a, 'b, C, MessageBodySet>
where
    MessageBodySet: ToAssign,
    C: Client,
{
    #[inline]
    fn message_ttl_seconds(&self) -> u64 {
        self.message_ttl_seconds
    }
}

impl<'a, 'b, C, MessageBodySet> TimeoutOption for PutMessageBuilder<'a, 'b, C, MessageBodySet>
where
    MessageBodySet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, 'b, C, MessageBodySet> ClientRequestIdOption<'a>
    for PutMessageBuilder<'a, 'b, C, MessageBodySet>
where
    MessageBodySet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, 'b, C> MessageBodySupport<'b> for PutMessageBuilder<'a, 'b, C, No>
where
    C: Client,
{
    type O = PutMessageBuilder<'a, 'b, C, Yes>;

    #[inline]
    fn with_message_body<BODY: Into<Cow<'b, str>>>(self, message_body: BODY) -> Self::O {
        PutMessageBuilder {
            queue_name_service: self.queue_name_service,
            p_message_body: PhantomData {},
            message_body: Some(message_body.into()),
            visibility_timeout_seconds: self.visibility_timeout_seconds,
            message_ttl_seconds: self.message_ttl_seconds,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, 'b, C, MessageBodySet> VisibilityTimeoutSupport
    for PutMessageBuilder<'a, 'b, C, MessageBodySet>
where
    MessageBodySet: ToAssign,
    C: Client,
{
    type O = PutMessageBuilder<'a, 'b, C, MessageBodySet>;

    #[inline]
    fn with_visibility_timeout_seconds(self, visibility_timeout_seconds: u64) -> Self::O {
        PutMessageBuilder {
            queue_name_service: self.queue_name_service,
            p_message_body: PhantomData {},
            message_body: self.message_body,
            visibility_timeout_seconds,
            message_ttl_seconds: self.message_ttl_seconds,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, 'b, C, MessageBodySet> MessageTTLSupport for PutMessageBuilder<'a, 'b, C, MessageBodySet>
where
    MessageBodySet: ToAssign,
    C: Client,
{
    type O = PutMessageBuilder<'a, 'b, C, MessageBodySet>;

    #[inline]
    fn with_message_ttl_seconds(self, message_ttl_seconds: u64) -> Self::O {
        PutMessageBuilder {
            queue_name_service: self.queue_name_service,
            p_message_body: PhantomData {},
            message_body: self.message_body,
            visibility_timeout_seconds: self.visibility_timeout_seconds,
            message_ttl_seconds,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, 'b, C, MessageBodySet> TimeoutSupport for PutMessageBuilder<'a, 'b, C, MessageBodySet>
where
    MessageBodySet: ToAssign,
    C: Client,
{
    type O = PutMessageBuilder<'a, 'b, C, MessageBodySet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        PutMessageBuilder {
            queue_name_service: self.queue_name_service,
            p_message_body: PhantomData {},
            message_body: self.message_body,
            visibility_timeout_seconds: self.visibility_timeout_seconds,
            message_ttl_seconds: self.message_ttl_seconds,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, 'b, C, MessageBodySet> ClientRequestIdSupport<'a>
    for PutMessageBuilder<'a, 'b, C, MessageBodySet>
where
    MessageBodySet: ToAssign,
    C: Client,
{
    type O = PutMessageBuilder<'a, 'b, C, MessageBodySet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        PutMessageBuilder {
            queue_name_service: self.queue_name_service,
            p_message_body: PhantomData {},
            message_body: self.message_body,
            visibility_timeout_seconds: self.visibility_timeout_seconds,
            message_ttl_seconds: self.message_ttl_seconds,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable regardless
impl<'a, 'b, C, MessageBodySet> PutMessageBuilder<'a, 'b, C, MessageBodySet>
where
    MessageBodySet: ToAssign,
    C: Client,
{
    pub fn queue_name_service(&self) -> &'a dyn QueueNameService<StorageClient = C> {
        self.queue_name_service
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, C> PutMessageBuilder<'a, 'b, C, Yes>
where
    C: Client,
{
    pub async fn execute(self) -> Result<PutMessageResponse, AzureError> {
        let mut uri = format!(
            "{}/{}/messages",
            self.queue_name_service.storage_client().queue_uri(),
            self.queue_name_service.queue_name()
        );

        uri = format!(
            "{}?{}",
            uri,
            VisibilityTimeoutRequired::to_uri_parameter(&self)
        );
        uri = format!("{}&{}", uri, MessageTTLRequired::to_uri_parameter(&self));
        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        debug!("uri == {}", uri);

        // since the format is fixed we just decorate the message with the tags.
        // This could be made optional in the future and/or more
        // stringent.
        let message = format!(
            "<QueueMessage><MessageText>{}</MessageText></QueueMessage>",
            self.message_body()
        );

        let future_response = self.queue_name_service.storage_client().perform_request(
            &uri,
            &http::Method::POST,
            &|mut request| {
                request = ClientRequestIdOption::add_header(&self, request);
                request
            },
            Some(message.as_bytes()),
        )?;

        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::CREATED).await?;

        (&headers, &body as &[u8]).try_into()
    }
}
