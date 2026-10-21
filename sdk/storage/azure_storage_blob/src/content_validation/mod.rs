// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::{Body, RequestContent},
    Result,
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
///
/// # Returns
///
/// Returns a `StorageUploadBody` tagged as either raw content or a structured message.
pub(crate) fn apply_upload_content_validation(
    body: Body,
    content_validation: Option<UploadContentValidation>,
    transactional_crc_header: &mut Option<Vec<u8>>,
    transactional_md5_header: &mut Option<Vec<u8>>,
) -> Result<StorageUploadBody> {
    let content_validation = match (
        content_validation,
        &transactional_crc_header,
        &transactional_md5_header,
    ) {
        (Some(validation), None, None) => validation,
        (None, None, None) => Default::default(),
        (None, Some(_), _) => UploadContentValidation::None,
        (None, _, Some(_)) => UploadContentValidation::None,
        (Some(_), Some(_), _) => todo!("bad option combo"),
        (Some(_), _, Some(_)) => todo!("bad option combo"),
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
            todo!("mark body as structured message using headers");
            #[allow(unreachable_code)]
            StorageUploadBody::StructuredMessage(structured_message::encode_with_checksum(
                stream.into(),
                None,
            )?)
        }
        (UploadContentValidation::Crc64Nvme(Some(precalculated_checksum)), body) => {
            *transactional_crc_header = Some(precalculated_checksum.into());
            StorageUploadBody::RawContent(body)
        }
        (UploadContentValidation::None, body) => StorageUploadBody::RawContent(body),
    })
}
