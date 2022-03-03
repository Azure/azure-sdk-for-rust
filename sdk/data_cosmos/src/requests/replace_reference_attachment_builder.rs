use crate::prelude::*;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ReplaceReferenceAttachmentBuilder<'a, 'b> {
    attachment: &'a AttachmentClient,
    if_match_condition: Option<IfMatchCondition>,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> ReplaceReferenceAttachmentBuilder<'a, 'b> {
    pub(crate) fn new(attachment: &'a AttachmentClient) -> Self {
        Self {
            attachment,
            if_match_condition: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b> ReplaceReferenceAttachmentBuilder<'a, 'b> {
    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition => Some(if_match_condition),
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> ReplaceReferenceAttachmentBuilder<'a, 'b> {
    pub async fn execute<M, C>(
        &self,
        media: M,
        content_type: C,
    ) -> crate::Result<crate::responses::ReplaceReferenceAttachmentResponse>
    where
        M: AsRef<str>,
        C: Into<ContentType<'b>>,
    {
        let mut req = self
            .attachment
            .prepare_request_with_attachment_name(http::Method::PUT);

        // add trait headers
        req = azure_core::headers::add_optional_header(&self.if_match_condition, req);
        req = azure_core::headers::add_optional_header(&self.user_agent, req);
        req = azure_core::headers::add_optional_header(&self.activity_id, req);
        req = azure_core::headers::add_optional_header(&self.consistency_level, req);

        req = crate::cosmos_entity::add_as_partition_key_header_serialized(
            self.attachment.document().partition_key_serialized(),
            req,
        );

        // create serialized request
        #[derive(Debug, Clone, Serialize)]
        struct _Request<'r> {
            pub id: &'r str,
            #[serde(rename = "contentType")]
            pub content_type: &'r str,
            pub media: &'r str,
        }

        let request = azure_core::to_json(&_Request {
            id: self.attachment.attachment_name(),
            content_type: content_type.into().as_str(),
            media: media.as_ref(),
        })?;

        req = req.header(http::header::CONTENT_TYPE, "application/json");
        req = req.header(http::header::CONTENT_LENGTH, request.len());
        let req = req.body(request)?;
        debug!("req == {:#?}", req);

        Ok(self
            .attachment
            .http_client()
            .execute_request_check_status(req, StatusCode::OK)
            .await?
            .try_into()?)
    }
}
