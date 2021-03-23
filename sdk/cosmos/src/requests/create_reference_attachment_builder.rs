use crate::prelude::*;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateReferenceAttachmentBuilder<'a, 'b> {
    attachment_client: &'a AttachmentClient,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> CreateReferenceAttachmentBuilder<'a, 'b> {
    pub(crate) fn new(attachment_client: &'a AttachmentClient) -> Self {
        Self {
            attachment_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b> CreateReferenceAttachmentBuilder<'a, 'b> {
    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }
}

impl<'a, 'b> CreateReferenceAttachmentBuilder<'a, 'b> {
    pub async fn execute<M, C>(
        &self,
        media: M,
        content_type: C,
    ) -> Result<crate::responses::CreateReferenceAttachmentResponse, CosmosError>
    where
        M: AsRef<str>,
        C: AsRef<str>,
    {
        let mut req = self.attachment_client.prepare_request(http::Method::POST);

        req = azure_core::headers::add_optional_header(&self.user_agent, req);
        req = azure_core::headers::add_optional_header(&self.activity_id, req);
        req = azure_core::headers::add_optional_header(&self.consistency_level, req);

        req = crate::cosmos_entity::add_as_partition_key_header_serialized(
            self.attachment_client
                .document_client()
                .partition_key_serialized(),
            req,
        );

        #[derive(Debug, Serialize)]
        struct Request<'r> {
            pub id: &'r str,
            #[serde(rename = "contentType")]
            pub content_type: &'r str,
            pub media: &'r str,
        }

        let request = azure_core::to_json(&Request {
            id: self.attachment_client.attachment_name(),
            content_type: content_type.as_ref(),
            media: media.as_ref(),
        })?;

        req = req.header(http::header::CONTENT_TYPE, "application/json");
        req = req.header(http::header::CONTENT_LENGTH, request.len());
        let req = req.body(request)?;
        debug!("req == {:#?}", req);

        Ok(self
            .attachment_client
            .http_client()
            .execute_request_check_status(req, StatusCode::CREATED)
            .await?
            .try_into()?)
    }
}
