use crate::prelude::*;
use azure_core::prelude::*;

use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteAttachmentBuilder<'a, 'b> {
    attachment_client: &'a AttachmentClient,
    if_match_condition: Option<IfMatchCondition<'b>>,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> DeleteAttachmentBuilder<'a, 'b> {
    pub(crate) fn new(attachment_client: &'a AttachmentClient) -> Self {
        Self {
            attachment_client,
            if_match_condition: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }

    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition<'b> => Some(if_match_condition),
    }

    pub async fn execute(&self) -> Result<crate::responses::DeleteAttachmentResponse, CosmosError> {
        let mut req = self
            .attachment_client
            .prepare_request_with_attachment_name(http::Method::DELETE);

        // add trait headers
        req = azure_core::headers::add_optional_header(&self.if_match_condition, req);
        req = azure_core::headers::add_optional_header(&self.user_agent, req);
        req = azure_core::headers::add_optional_header(&self.activity_id, req);
        req = azure_core::headers::add_optional_header(&self.consistency_level, req);

        req = crate::cosmos_entity::add_as_partition_key_header_serialized(
            self.attachment_client
                .document_client()
                .partition_key_serialized(),
            req,
        );

        let req = req.body(bytes::Bytes::from_static(EMPTY_BODY))?;

        debug!("req == {:#?}", req);

        Ok(self
            .attachment_client
            .http_client()
            .execute_request_check_status(req, StatusCode::NO_CONTENT)
            .await?
            .try_into()?)
    }
}
