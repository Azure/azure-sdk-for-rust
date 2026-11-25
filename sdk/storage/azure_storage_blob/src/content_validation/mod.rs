// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    error::ErrorKind,
    http::{Body, RequestContent},
    Error, Result,
};

use crate::models::UploadContentValidation;

pub(crate) mod structured_message;

/// Tags a [Body] as either the raw content to be transferred or as a structured message containing the content.
pub(crate) enum StorageUploadBody {
    RawContent(Body),
    StructuredMessage(Body),
}

impl From<StorageUploadBody> for Body {
    fn from(value: StorageUploadBody) -> Self {
        match value {
            StorageUploadBody::RawContent(body) => body,
            StorageUploadBody::StructuredMessage(body) => body,
        }
    }
}

impl<T, F> From<StorageUploadBody> for RequestContent<T, F> {
    fn from(value: StorageUploadBody) -> Self {
        let body: Body = value.into();
        body.into()
    }
}

/// Applies content validation to a request based on the provided options. It determines how
/// content validation should be performed based on provided options and headers, and then applies
/// that strategy to the body and/or headers.
///
/// This function **may** modify the provided body by encoding it as a structured message. The returned
/// body will be tagged as such.
///
/// This function **may** modify the referenced transactional headers if the function chooses to use
/// headers rather than structured message.
///
/// # Arguments
///
/// * `body` - The raw content to be validated on upload.
/// * `content_validation` - The content validation options specified for the upload.
/// * `transactional_crc_header` - A mutable reference to the raw transactional CRC64 header specified
///                                for the upload. This is used both to check the options combination,
///                                as well as to set a value for the final request if needed.
/// * `transactional_md5_header` - A mutable reference to the raw transactional MD5 header specified for
///                                the upload. This is used both to check the options combination, as well
///                                as to set a value for the final request if needed.
/// * `structured_body_type_header` - A mutable reference to the raw header for structured body type.
///                                   This will be set if the body is converted to a structured message.
/// * `structured_body_length_header` - A mutable reference to the raw header for structured body length.
///                                     This will be set if the body is converted to a structured message.
///
/// # Returns
///
/// Returns a `StorageUploadBody` tagged as either raw content or a structured message.
pub(crate) fn apply_upload_content_validation(
    body: Body,
    content_validation: Option<UploadContentValidation>,
    transactional_crc_header: &mut Option<Vec<u8>>,
    transactional_md5_header: &mut Option<Vec<u8>>,
    structured_body_type_header: &mut Option<String>,
    structured_body_length_header: &mut Option<u64>,
) -> Result<StorageUploadBody> {
    if structured_body_type_header.is_some() || structured_body_length_header.is_some() {
        return Err(Error::with_message(
            ErrorKind::Other,
            "Structured body headers must be empty for this operation.",
        ));
    }
    let content_validation = match (
        content_validation,
        &transactional_crc_header,
        &transactional_md5_header,
    ) {
        (Some(validation), None, None) => validation,
        (None, None, None) => Default::default(),
        (None, Some(_), _) => UploadContentValidation::Disabled,
        (None, _, Some(_)) => UploadContentValidation::Disabled,
        (Some(_), Some(_), _) => {
            return Err(Error::with_message(
                ErrorKind::Other,
                "Cannot specify both content validation and a transactional CRC header.",
            ))
        }
        (Some(_), _, Some(_)) => {
            return Err(Error::with_message(
                ErrorKind::Other,
                "Cannot specify both content validation and a transactional MD5 header.",
            ))
        }
    };

    Ok(match (content_validation, body) {
        (
            UploadContentValidation::Auto | UploadContentValidation::Crc64Nvme(None),
            Body::Bytes(bytes),
        ) => {
            let mut digest = crc_fast::Digest::new(crc_fast::CrcAlgorithm::Crc64Nvme);
            digest.update(&bytes);
            *transactional_crc_header = Some(digest.finalize().to_le_bytes().into());
            StorageUploadBody::RawContent(Body::Bytes(bytes))
        }
        (
            UploadContentValidation::Auto | UploadContentValidation::Crc64Nvme(None),
            Body::SeekableStream(stream),
        ) => {
            structured_body_type_header.replace("XSM/1.0; properties=crc64".to_string());
            structured_body_length_header.replace(stream.len().ok_or_else(|| {
                Error::with_message(
                    ErrorKind::Other,
                    "Body must have length to encode for content validation.",
                )
            })?);
            StorageUploadBody::StructuredMessage(structured_message::encode_with_checksum(
                stream.into(),
                None,
            )?)
        }
        (UploadContentValidation::Crc64Nvme(Some(precalculated_checksum)), body) => {
            *transactional_crc_header = Some(precalculated_checksum.into());
            StorageUploadBody::RawContent(body)
        }
        (UploadContentValidation::Disabled, body) => StorageUploadBody::RawContent(body),
    })
}

#[cfg(test)]
mod tests {
    use std::num::NonZero;

    use super::{structured_message::encode_in_place::encode_bytes_in_structured_message, *};
    use azure_core::stream::BytesStream;
    use bytes::Bytes;
    use futures::{AsyncRead, AsyncReadExt};

    const DATA: &str = "Hello, world!";
    const DATA_CRC: [u8; 8] = [52, 61, 26, 0, 105, 85, 120, 159];
    const DATA_MD5: [u8; 16] = 0x6cd3556deb0da54bca060b4c39479839u128.to_be_bytes();

    const FAKE_CRC: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    const FAKE_MD5: [u8; 16] = 0x0102030405060708090a0b0c0d0e0f10u128.to_be_bytes();

    const DATA_PLAINTEXT_LEN: usize = DATA.len();
    const DATA_STRUCTURED_LEN: usize = 13 + 10 + DATA.len() + 8 + 8;

    #[tokio::test]
    async fn test_bytes_w_no_options_sets_crc_header() {
        let mut crc_header = None;
        let mut md5_header = None;
        let mut structured_body_type_header = None;
        let mut structured_body_length_header = None;

        let content_validation_options = None;
        let input_body = Body::Bytes(Bytes::from(DATA));

        let output_body = apply_upload_content_validation(
            input_body,
            content_validation_options,
            &mut crc_header,
            &mut md5_header,
            &mut structured_body_type_header,
            &mut structured_body_length_header,
        )
        .unwrap();

        assert!(matches!(
            output_body,
            StorageUploadBody::RawContent(Body::Bytes(bytes))
            if bytes == DATA
        ));
        assert!(structured_body_type_header.is_none());
        assert!(structured_body_length_header.is_none());
        assert_eq!(crc_header, Some(DATA_CRC.to_vec()));
        assert!(md5_header.is_none());
    }

    #[tokio::test]
    async fn test_bytes_w_auto_options_sets_crc_header() {
        let mut crc_header = None;
        let mut md5_header = None;
        let mut structured_body_type_header = None;
        let mut structured_body_length_header = None;

        let content_validation_options = Some(UploadContentValidation::Auto);
        let input_body = Body::Bytes(Bytes::from(DATA));

        let output_body = apply_upload_content_validation(
            input_body,
            content_validation_options,
            &mut crc_header,
            &mut md5_header,
            &mut structured_body_type_header,
            &mut structured_body_length_header,
        )
        .unwrap();

        assert!(matches!(
            output_body,
            StorageUploadBody::RawContent(Body::Bytes(bytes))
            if bytes == DATA
        ));
        assert!(structured_body_type_header.is_none());
        assert!(structured_body_length_header.is_none());
        assert_eq!(crc_header, Some(DATA_CRC.to_vec()));
        assert!(md5_header.is_none());
    }

    #[tokio::test]
    async fn test_bytes_w_crc_options_sets_crc_header() {
        let mut crc_header = None;
        let mut md5_header = None;
        let mut structured_body_type_header = None;
        let mut structured_body_length_header = None;

        let content_validation_options = Some(UploadContentValidation::Crc64Nvme(None));
        let input_body = Body::Bytes(Bytes::from(DATA));

        let output_body = apply_upload_content_validation(
            input_body,
            content_validation_options,
            &mut crc_header,
            &mut md5_header,
            &mut structured_body_type_header,
            &mut structured_body_length_header,
        )
        .unwrap();

        assert!(matches!(
            output_body,
            StorageUploadBody::RawContent(Body::Bytes(bytes))
            if bytes == DATA
        ));
        assert!(structured_body_type_header.is_none());
        assert!(structured_body_length_header.is_none());
        assert_eq!(crc_header, Some(DATA_CRC.to_vec()));
        assert!(md5_header.is_none());
    }

    #[tokio::test]
    async fn test_bytes_w_precalculated_crc_sets_crc_header() {
        let mut crc_header = None;
        let mut md5_header = None;
        let mut structured_body_type_header = None;
        let mut structured_body_length_header = None;

        let content_validation_options = Some(UploadContentValidation::Crc64Nvme(Some(FAKE_CRC)));
        let input_body = Body::Bytes(Bytes::from(DATA));

        let output_body = apply_upload_content_validation(
            input_body,
            content_validation_options,
            &mut crc_header,
            &mut md5_header,
            &mut structured_body_type_header,
            &mut structured_body_length_header,
        )
        .unwrap();

        assert!(matches!(
            output_body,
            StorageUploadBody::RawContent(Body::Bytes(bytes))
            if bytes == DATA
        ));
        assert!(structured_body_type_header.is_none());
        assert!(structured_body_length_header.is_none());
        assert_eq!(crc_header, Some(FAKE_CRC.to_vec()));
        assert!(md5_header.is_none());
    }

    #[tokio::test]
    async fn test_bytes_w_disabled_sets_nothing() {
        let mut crc_header = None;
        let mut md5_header = None;
        let mut structured_body_type_header = None;
        let mut structured_body_length_header = None;

        let content_validation_options = Some(UploadContentValidation::Disabled);
        let input_body = Body::Bytes(Bytes::from(DATA));

        let output_body = apply_upload_content_validation(
            input_body,
            content_validation_options,
            &mut crc_header,
            &mut md5_header,
            &mut structured_body_type_header,
            &mut structured_body_length_header,
        )
        .unwrap();

        assert!(matches!(
            output_body,
            StorageUploadBody::RawContent(Body::Bytes(bytes))
            if bytes == DATA
        ));
        assert!(structured_body_type_header.is_none());
        assert!(structured_body_length_header.is_none());
        assert!(crc_header.is_none());
        assert!(md5_header.is_none());
    }

    #[tokio::test]
    async fn test_bytes_w_raw_crc_header_remains_unchanged() {
        let mut crc_header = Some(FAKE_CRC.to_vec());
        let mut md5_header = None;
        let mut structured_body_type_header = None;
        let mut structured_body_length_header = None;

        let content_validation_options = None;
        let input_body = Body::Bytes(Bytes::from(DATA));

        let output_body = apply_upload_content_validation(
            input_body,
            content_validation_options,
            &mut crc_header,
            &mut md5_header,
            &mut structured_body_type_header,
            &mut structured_body_length_header,
        )
        .unwrap();

        assert!(matches!(
            output_body,
            StorageUploadBody::RawContent(Body::Bytes(bytes))
            if bytes == DATA
        ));
        assert!(structured_body_type_header.is_none());
        assert!(structured_body_length_header.is_none());
        assert_eq!(crc_header, Some(FAKE_CRC.to_vec()));
        assert!(md5_header.is_none());
    }

    #[tokio::test]
    async fn test_bytes_w_raw_md5_header_remains_unchanged() {
        let mut crc_header = None;
        let mut md5_header = Some(FAKE_MD5.to_vec());
        let mut structured_body_type_header = None;
        let mut structured_body_length_header = None;

        let content_validation_options = None;
        let input_body = Body::Bytes(Bytes::from(DATA));

        let output_body = apply_upload_content_validation(
            input_body,
            content_validation_options,
            &mut crc_header,
            &mut md5_header,
            &mut structured_body_type_header,
            &mut structured_body_length_header,
        )
        .unwrap();

        assert!(matches!(
            output_body,
            StorageUploadBody::RawContent(Body::Bytes(bytes))
            if bytes == DATA
        ));
        assert!(structured_body_type_header.is_none());
        assert!(structured_body_length_header.is_none());
        assert!(crc_header.is_none());
        assert_eq!(md5_header, Some(FAKE_MD5.to_vec()));
    }

    #[tokio::test]
    async fn test_stream_w_no_options_encodes_body() {
        const SEG_LEN: NonZero<usize> = NonZero::new(4 * 1024 * 1024).unwrap();
        let mut crc_header = None;
        let mut md5_header = None;
        let mut structured_body_type_header = None;
        let mut structured_body_length_header = None;

        let content_validation_options = None;
        let input_body = Body::SeekableStream(Box::new(BytesStream::new(DATA.as_bytes())));

        let output_body = apply_upload_content_validation(
            input_body,
            content_validation_options,
            &mut crc_header,
            &mut md5_header,
            &mut structured_body_type_header,
            &mut structured_body_length_header,
        )
        .unwrap();

        let StorageUploadBody::StructuredMessage(Body::SeekableStream(mut stream)) = output_body
        else {
            panic!("Expected stream of structured message");
        };
        assert_eq!(
            collect_data(&mut stream).await,
            encode_bytes_in_structured_message(DATA.as_bytes().into(), SEG_LEN).concat()
        );
        assert!(structured_body_type_header.is_some());
        assert!(structured_body_length_header.is_some());
        assert!(crc_header.is_none());
        assert!(md5_header.is_none());
    }

    #[tokio::test]
    async fn test_stream_w_auto_options_encodes_body() {
        const SEG_LEN: NonZero<usize> = NonZero::new(4 * 1024 * 1024).unwrap();
        let mut crc_header = None;
        let mut md5_header = None;
        let mut structured_body_type_header = None;
        let mut structured_body_length_header = None;

        let content_validation_options = Some(UploadContentValidation::Auto);
        let input_body = Body::SeekableStream(Box::new(BytesStream::new(DATA.as_bytes())));

        let output_body = apply_upload_content_validation(
            input_body,
            content_validation_options,
            &mut crc_header,
            &mut md5_header,
            &mut structured_body_type_header,
            &mut structured_body_length_header,
        )
        .unwrap();

        let StorageUploadBody::StructuredMessage(Body::SeekableStream(mut stream)) = output_body
        else {
            panic!("Expected stream of structured message");
        };
        assert_eq!(
            collect_data(&mut stream).await,
            encode_bytes_in_structured_message(DATA.as_bytes().into(), SEG_LEN).concat()
        );
        assert!(structured_body_type_header.is_some());
        assert!(structured_body_length_header.is_some());
        assert!(crc_header.is_none());
        assert!(md5_header.is_none());
    }

    #[tokio::test]
    async fn test_stream_w_crc_options_encodes_body() {
        const SEG_LEN: NonZero<usize> = NonZero::new(4 * 1024 * 1024).unwrap();
        let mut crc_header = None;
        let mut md5_header = None;
        let mut structured_body_type_header = None;
        let mut structured_body_length_header = None;

        let content_validation_options = Some(UploadContentValidation::Crc64Nvme(None));
        let input_body = Body::SeekableStream(Box::new(BytesStream::new(DATA.as_bytes())));

        let output_body = apply_upload_content_validation(
            input_body,
            content_validation_options,
            &mut crc_header,
            &mut md5_header,
            &mut structured_body_type_header,
            &mut structured_body_length_header,
        )
        .unwrap();

        let StorageUploadBody::StructuredMessage(Body::SeekableStream(mut stream)) = output_body
        else {
            panic!("Expected stream of structured message");
        };
        assert_eq!(
            collect_data(&mut stream).await,
            encode_bytes_in_structured_message(DATA.as_bytes().into(), SEG_LEN).concat()
        );
        assert!(structured_body_type_header.is_some());
        assert!(structured_body_length_header.is_some());
        assert!(crc_header.is_none());
        assert!(md5_header.is_none());
    }

    #[tokio::test]
    async fn test_stream_w_precalculated_crc_sets_crc_header() {
        let mut crc_header = None;
        let mut md5_header = None;
        let mut structured_body_type_header = None;
        let mut structured_body_length_header = None;

        let content_validation_options = Some(UploadContentValidation::Crc64Nvme(Some(FAKE_CRC)));
        let input_body = Body::SeekableStream(Box::new(BytesStream::new(DATA.as_bytes())));

        let output_body = apply_upload_content_validation(
            input_body,
            content_validation_options,
            &mut crc_header,
            &mut md5_header,
            &mut structured_body_type_header,
            &mut structured_body_length_header,
        )
        .unwrap();

        let StorageUploadBody::RawContent(Body::SeekableStream(mut stream)) = output_body else {
            panic!("Expected stream of raw content");
        };
        assert!(collect_data(&mut stream).await == DATA.as_bytes().to_vec());
        assert!(structured_body_type_header.is_none());
        assert!(structured_body_length_header.is_none());
        assert_eq!(crc_header, Some(FAKE_CRC.to_vec()));
        assert!(md5_header.is_none());
    }

    #[tokio::test]
    async fn test_stream_w_disabled_options_sets_nothing() {
        let mut crc_header = None;
        let mut md5_header = None;
        let mut structured_body_type_header = None;
        let mut structured_body_length_header = None;

        let content_validation_options = Some(UploadContentValidation::Disabled);
        let input_body = Body::SeekableStream(Box::new(BytesStream::new(DATA.as_bytes())));

        let output_body = apply_upload_content_validation(
            input_body,
            content_validation_options,
            &mut crc_header,
            &mut md5_header,
            &mut structured_body_type_header,
            &mut structured_body_length_header,
        )
        .unwrap();

        let StorageUploadBody::RawContent(Body::SeekableStream(mut stream)) = output_body else {
            panic!("Expected stream of raw content");
        };
        assert!(collect_data(&mut stream).await == DATA.as_bytes().to_vec());
        assert!(structured_body_type_header.is_none());
        assert!(structured_body_length_header.is_none());
        assert!(crc_header.is_none());
        assert!(md5_header.is_none());
    }

    #[tokio::test]
    async fn test_stream_w_raw_crc_header_remains_unchanged() {
        let mut crc_header = Some(FAKE_CRC.to_vec());
        let mut md5_header = None;
        let mut structured_body_type_header = None;
        let mut structured_body_length_header = None;

        let content_validation_options = None;
        let input_body = Body::SeekableStream(Box::new(BytesStream::new(DATA.as_bytes())));

        let output_body = apply_upload_content_validation(
            input_body,
            content_validation_options,
            &mut crc_header,
            &mut md5_header,
            &mut structured_body_type_header,
            &mut structured_body_length_header,
        )
        .unwrap();

        let StorageUploadBody::RawContent(Body::SeekableStream(mut stream)) = output_body else {
            panic!("Expected stream of raw content");
        };
        assert!(collect_data(&mut stream).await == DATA.as_bytes().to_vec());
        assert!(structured_body_type_header.is_none());
        assert!(structured_body_length_header.is_none());
        assert_eq!(crc_header, Some(FAKE_CRC.to_vec()));
        assert!(md5_header.is_none());
    }

    #[tokio::test]
    async fn test_stream_w_raw_md5_header_remains_unchanged() {
        let mut crc_header = None;
        let mut md5_header = Some(FAKE_MD5.to_vec());
        let mut structured_body_type_header = None;
        let mut structured_body_length_header = None;

        let content_validation_options = None;
        let input_body = Body::SeekableStream(Box::new(BytesStream::new(DATA.as_bytes())));

        let output_body = apply_upload_content_validation(
            input_body,
            content_validation_options,
            &mut crc_header,
            &mut md5_header,
            &mut structured_body_type_header,
            &mut structured_body_length_header,
        )
        .unwrap();

        let StorageUploadBody::RawContent(Body::SeekableStream(mut stream)) = output_body else {
            panic!("Expected stream of raw content");
        };
        assert!(collect_data(&mut stream).await == DATA.as_bytes().to_vec());
        assert!(structured_body_type_header.is_none());
        assert!(structured_body_length_header.is_none());
        assert!(crc_header.is_none());
        assert_eq!(md5_header, Some(FAKE_MD5.to_vec()));
    }

    fn crc_inline(bytes: &[u8]) -> u64 {
        let mut digest = crc_fast::Digest::new(crc_fast::CrcAlgorithm::Crc64Nvme);
        digest.update(bytes);
        digest.finalize()
    }

    async fn collect_data(stream: &mut (impl AsyncRead + Unpin)) -> Vec<u8> {
        let mut dst = Vec::new();
        stream.read_to_end(&mut dst).await.unwrap();
        dst
    }
}
