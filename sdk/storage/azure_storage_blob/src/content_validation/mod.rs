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
        (UploadContentValidation::None, body) => StorageUploadBody::RawContent(body),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::stream::BytesStream;
    use bytes::Bytes;
    use test_case::test_case;

    const DATA: &str = "Hello, world!";
    const DATA_CRC: [u8; 8] = [52, 61, 26, 0, 105, 85, 120, 159];
    const DATA_MD5: [u8; 16] = 0x6cd3556deb0da54bca060b4c39479839u128.to_be_bytes();

    const FAKE_CRC: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    const FAKE_MD5: [u8; 16] = 0x0102030405060708090a0b0c0d0e0f10u128.to_be_bytes();

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Format {
        Bytes,
        Stream,
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Encoding {
        Plain,
        Structured,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum RawHeader {
        Crc([u8; 8]),
        Md5([u8; 16]),
    }

    /// Test options combinations for Body::Bytes input.
    #[test_case(Format::Bytes, None, None,                                                        Format::Bytes, Encoding::Plain, Some(RawHeader::Crc(DATA_CRC)) ; "Bytes + no options sets crc header")]
    #[test_case(Format::Bytes, Some(UploadContentValidation::Auto), None,                         Format::Bytes, Encoding::Plain, Some(RawHeader::Crc(DATA_CRC)) ; "Bytes + auto sets crc header")]
    #[test_case(Format::Bytes, Some(UploadContentValidation::Crc64Nvme(None)), None,              Format::Bytes, Encoding::Plain, Some(RawHeader::Crc(DATA_CRC)) ; "Bytes + crc sets crc header")]
    #[test_case(Format::Bytes, Some(UploadContentValidation::Crc64Nvme(Some(FAKE_CRC))), None,    Format::Bytes, Encoding::Plain, Some(RawHeader::Crc(FAKE_CRC)) ; "Bytes + crc64nvme precalculated sets crc header")]
    #[test_case(Format::Bytes, Some(UploadContentValidation::None), None,                         Format::Bytes, Encoding::Plain, None ; "Bytes + explicit none sets nothing")]
    #[test_case(Format::Bytes, None, Some(RawHeader::Crc(FAKE_CRC)),                              Format::Bytes, Encoding::Plain, Some(RawHeader::Crc(FAKE_CRC)) ; "Bytes + raw crc header remains unchanged")]
    #[test_case(Format::Bytes, None, Some(RawHeader::Md5(FAKE_MD5)),                              Format::Bytes, Encoding::Plain, Some(RawHeader::Md5(FAKE_MD5)) ; "Bytes + raw md5 header remains unchanged")]
    #[test_case(Format::Stream, None, None,                                                       Format::Stream, Encoding::Structured, None ; "Stream + no options encodes body")]
    #[test_case(Format::Stream, Some(UploadContentValidation::Auto), None,                        Format::Stream, Encoding::Structured, None ; "Stream + auto encodes body")]
    #[test_case(Format::Stream, Some(UploadContentValidation::Crc64Nvme(None)), None,             Format::Stream, Encoding::Structured, None ; "Stream + crc encodes body")]
    #[test_case(Format::Stream, Some(UploadContentValidation::Crc64Nvme(Some(FAKE_CRC))), None,   Format::Stream, Encoding::Plain, Some(RawHeader::Crc(FAKE_CRC)) ; "Stream + crc64nvme precalculated sets crc header")]
    #[test_case(Format::Stream, Some(UploadContentValidation::None), None,                        Format::Stream, Encoding::Plain, None ; "Stream + explicit none sets nothing")]
    #[test_case(Format::Stream, None, Some(RawHeader::Crc(FAKE_CRC)),                             Format::Stream, Encoding::Plain, Some(RawHeader::Crc(FAKE_CRC)) ; "Stream + raw crc header remains unchanged")]
    #[test_case(Format::Stream, None, Some(RawHeader::Md5(FAKE_MD5)),                             Format::Stream, Encoding::Plain, Some(RawHeader::Md5(FAKE_MD5)) ; "Stream + raw md5 header remains unchanged")]
    #[tokio::test]
    async fn test_apply_validation(
        input_format: Format,
        input_options: Option<UploadContentValidation>,
        input_raw_header: Option<RawHeader>,
        expect_format: Format,
        expect_encoding: Encoding,
        expect_raw_header: Option<RawHeader>,
    ) {
        let mut crc_header = match &input_raw_header {
            Some(RawHeader::Crc(crc)) => Some(crc.to_vec()),
            _ => None,
        };
        let mut md5_header = match &input_raw_header {
            Some(RawHeader::Md5(md5)) => Some(md5.to_vec()),
            _ => None,
        };
        let mut structured_body_type_header = None;
        let mut structured_body_length_header = None;

        let output_body = apply_upload_content_validation(
            match input_format {
                Format::Bytes => Body::Bytes(Bytes::from(DATA)),
                Format::Stream => Body::SeekableStream(Box::new(BytesStream::new(DATA.as_bytes()))),
            },
            input_options,
            &mut crc_header,
            &mut md5_header,
            &mut structured_body_type_header,
            &mut structured_body_length_header,
        )
        .unwrap();

        // assert output encoding matches expectation
        match (expect_encoding, &output_body) {
            (Encoding::Plain, StorageUploadBody::RawContent(_)) => {
                assert!(structured_body_type_header.is_none());
                assert!(structured_body_length_header.is_none());
            }
            (Encoding::Structured, StorageUploadBody::StructuredMessage(_)) => {
                assert!(structured_body_type_header.is_some());
                assert!(structured_body_length_header.is_some());
            }
            _ => panic!("Unexpected encoding"),
        };

        // assert output data structure matches expectation
        let output_body: Body = output_body.into();
        match (expect_format, &output_body) {
            (Format::Bytes, Body::Bytes(_)) => {}
            (Format::Stream, Body::SeekableStream(_)) => {}
            _ => panic!("Unexpected body structure (Bytes vs. SeekableStream)"),
        }

        // assert raw headers match expectation
        match expect_raw_header {
            Some(RawHeader::Crc(expected_crc)) => {
                assert_eq!(crc_header, Some(expected_crc.to_vec()));
                assert!(md5_header.is_none());
            }
            Some(RawHeader::Md5(expected_md5)) => {
                assert!(crc_header.is_none());
                assert_eq!(md5_header, Some(expected_md5.to_vec()));
            }
            None => {
                assert!(crc_header.is_none());
                assert!(md5_header.is_none());
            }
        };
    }

    fn crc_inline(bytes: &[u8]) -> u64 {
        let mut digest = crc_fast::Digest::new(crc_fast::CrcAlgorithm::Crc64Nvme);
        digest.update(bytes);
        digest.finalize()
    }
}
