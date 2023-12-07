use crate::prelude::BlobServiceClient;
use azure_core::{
    date::iso8601,
    headers::Headers,
    xml::{read_xml_str, to_xml},
    Method,
};
use azure_storage::{
    headers::CommonStorageResponseHeaders, shared_access_signature::service_sas::UserDeligationKey,
};
use bytes::{Bytes, BytesMut};
use time::OffsetDateTime;

operation! {
    GetUserDelegationKey,
    client: BlobServiceClient,
    start_time: OffsetDateTime,
    expiry_time: OffsetDateTime,
}

impl GetUserDelegationKeyBuilder {
    pub fn into_future(mut self) -> GetUserDelegationKey {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut()
                .extend_pairs([("restype", "service"), ("comp", "userdelegationkey")]);

            let body = GetUserDelegationKeyRequest {
                start: self.start_time,
                expiry: self.expiry_time,
            }
            .encode()?;

            let mut request = BlobServiceClient::finalize_request(
                url,
                Method::Post,
                Headers::new(),
                Some(body.into()),
            )?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            let (_, headers, body) = response.deconstruct();
            let body = body.collect_string().await?;
            GetUserDelegationKeyResponse::try_from(&headers, &body)
        })
    }
}

#[derive(Serialize)]
#[serde(rename = "KeyInfo")]
struct GetUserDelegationKeyRequest {
    #[serde(rename = "Start", with = "iso8601")]
    start: OffsetDateTime,
    #[serde(rename = "Expiry", with = "iso8601")]
    expiry: OffsetDateTime,
}

impl GetUserDelegationKeyRequest {
    pub fn encode(&self) -> azure_core::Result<Bytes> {
        let mut body = BytesMut::from("<?xml version=\"1.0\" encoding=\"utf-8\"?>");
        body.extend(to_xml(self)?);
        Ok(body.freeze())
    }
}

#[derive(Debug)]
pub struct GetUserDelegationKeyResponse {
    pub common: CommonStorageResponseHeaders,
    pub user_deligation_key: UserDeligationKey,
}

impl GetUserDelegationKeyResponse {
    pub(crate) fn try_from(headers: &Headers, body: &str) -> azure_core::Result<Self> {
        let common = CommonStorageResponseHeaders::try_from(headers)?;
        let user_deligation_key: UserDeligationKey = read_xml_str(body)?;

        Ok(Self {
            common,
            user_deligation_key,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use azure_core::auth::Secret;
    use uuid::Uuid;

    const BASIC_REQUEST: &[u8] = b"<?xml version=\"1.0\" encoding=\"utf-8\"?><KeyInfo><Start>1970-01-01T00:00:00Z</Start><Expiry>1970-01-01T00:00:01Z</Expiry></KeyInfo>";
    const BASIC_RESPONSE: &str = "
        <UserDeligationKey>
            <SignedOid>00000000-0000-0000-0000-000000000000</SignedOid>
            <SignedTid>00000000-0000-0000-0000-000000000001</SignedTid>
            <SignedStart>1970-01-01T00:00:00Z</SignedStart>
            <SignedExpiry>1970-01-01T00:00:01Z</SignedExpiry>
            <SignedService>b</SignedService>
            <SignedVersion>c</SignedVersion>
            <Value>d</Value>
        </UserDeligationKey>
    ";

    #[test]
    fn request_xml() -> azure_core::Result<()> {
        let request = GetUserDelegationKeyRequest {
            start: OffsetDateTime::from_unix_timestamp(0).unwrap(),
            expiry: OffsetDateTime::from_unix_timestamp(1).unwrap(),
        }
        .encode()?;
        assert_eq!(BASIC_REQUEST, request);
        Ok(())
    }

    #[test]
    fn parse_response() -> azure_core::Result<()> {
        let expected = UserDeligationKey {
            signed_oid: Uuid::from_u128(0),
            signed_tid: Uuid::from_u128(1),
            signed_start: OffsetDateTime::from_unix_timestamp(0).unwrap(),
            signed_expiry: OffsetDateTime::from_unix_timestamp(1).unwrap(),
            signed_service: "b".to_owned(),
            signed_version: "c".to_owned(),
            value: Secret::new("d"),
        };

        let deserialized: UserDeligationKey = read_xml_str(BASIC_RESPONSE)?;
        assert_eq!(deserialized, expected);

        Ok(())
    }
}
