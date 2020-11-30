use crate::queue::*;
use crate::responses::*;
use azure_core::errors::AzureError;
use azure_core::prelude::*;
use hyper::StatusCode;
use std::borrow::Cow;
use std::convert::TryInto;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct PutMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    queue_name_client: &'a QueueNameClient<C>,
    message_body: Cow<'a, str>,
    visibility_timeout: Option<Duration>,
    message_ttl_seconds: u64,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> PutMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    #[inline]
    pub(crate) fn new<MB: Into<Cow<'a, str>>>(
        queue_name_client: &'a QueueNameClient<C>,
        message_body: MB,
    ) -> PutMessageBuilder<'a, C> {
        PutMessageBuilder {
            queue_name_client,
            message_body: message_body.into(),
            visibility_timeout: None,
            message_ttl_seconds: 25200,
            timeout: None,
            client_request_id: None,
        }
    }
}

//set mandatory no traits methods
impl<'a, C> MessageBodyRequired for PutMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    #[inline]
    fn message_body(&self) -> &str {
        self.message_body.as_ref()
    }
}

impl<'a, C> VisibilityTimeoutOption for PutMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    #[inline]
    fn visibility_timeout(&self) -> Option<Duration> {
        self.visibility_timeout
    }
}

impl<'a, C> MessageTTLRequired for PutMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    #[inline]
    fn message_ttl_seconds(&self) -> u64 {
        self.message_ttl_seconds
    }
}

impl<'a, C> TimeoutOption for PutMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C> ClientRequestIdOption<'a> for PutMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C> VisibilityTimeoutSupport for PutMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = PutMessageBuilder<'a, C>;

    #[inline]
    fn with_visibility_timeout(self, visibility_timeout: Duration) -> Self::O {
        PutMessageBuilder {
            queue_name_client: self.queue_name_client,
            message_body: self.message_body,
            visibility_timeout: Some(visibility_timeout),
            message_ttl_seconds: self.message_ttl_seconds,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> MessageTTLSupport for PutMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = PutMessageBuilder<'a, C>;

    #[inline]
    fn with_message_ttl_seconds(self, message_ttl_seconds: u64) -> Self::O {
        PutMessageBuilder {
            queue_name_client: self.queue_name_client,
            message_body: self.message_body,
            visibility_timeout: self.visibility_timeout,
            message_ttl_seconds,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> TimeoutSupport for PutMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = PutMessageBuilder<'a, C>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        PutMessageBuilder {
            queue_name_client: self.queue_name_client,
            message_body: self.message_body,
            visibility_timeout: self.visibility_timeout,
            message_ttl_seconds: self.message_ttl_seconds,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> ClientRequestIdSupport<'a> for PutMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = PutMessageBuilder<'a, C>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        PutMessageBuilder {
            queue_name_client: self.queue_name_client,
            message_body: self.message_body,
            visibility_timeout: self.visibility_timeout,
            message_ttl_seconds: self.message_ttl_seconds,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable regardless
impl<'a, C> PutMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    pub fn queue_name_client(&self) -> &'a QueueNameClient<C> {
        self.queue_name_client
    }

    pub async fn execute(self) -> Result<PutMessageResponse, AzureError> {
        let mut url = url::Url::parse(&format!(
            "{}/{}/messages",
            self.queue_name_client.storage_client().queue_uri(),
            self.queue_name_client.queue_name()
        ))?;

        MessageTTLRequired::append_pair(&self, &mut url);
        VisibilityTimeoutOption::append_pair(&self, &mut url);
        TimeoutOption::append_pair(&self, &mut url);

        debug!("url == {:?}", url);

        // since the format is fixed we just decorate the message with the tags.
        // This could be made optional in the future and/or more
        // stringent.
        let message = format!(
            "<QueueMessage><MessageText>{}</MessageText></QueueMessage>",
            self.message_body()
        );

        debug!("message about to be posted == {}", message);

        let perform_request_response = self.queue_name_client.storage_client().perform_request(
            url.as_str(),
            &http::Method::POST,
            &|mut request| {
                request = ClientRequestIdOption::add_header(&self, request);
                request
            },
            Some(message.as_bytes()),
        )?;

        let (headers, body) = perform_request_response
            .check_status_extract_headers_and_body(StatusCode::CREATED)
            .await?;

        (&headers, &body as &[u8]).try_into()
    }
}
