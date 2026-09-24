// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB error types re-exported from the driver.

use std::sync::Arc;

use azure_data_cosmos_driver::{binary_json::BinaryError, error::CosmosErrorBuilder};

use crate::diagnostics::DiagnosticsContext;
#[cfg(feature = "preview_patch")]
use crate::models::PatchTrackingId;

pub use azure_data_cosmos_driver::error::{CosmosError, CosmosStatus, SubStatusCode};

/// Converts a binary-JSON encode error into a [`CosmosError`].
pub(crate) fn convert_binary_encode_error(error: BinaryError) -> CosmosError {
    CosmosError::builder()
        .with_status(
            azure_data_cosmos_driver::error::status_codes::SERIALIZATION_REQUEST_BODY_INVALID,
        )
        .with_message("failed to serialize item to Cosmos binary JSON")
        .with_source(error)
        .build()
}

/// Maps a text-JSON encode failure to a request-body serialization error.
pub(crate) fn convert_json_encode_error(error: serde_json::Error) -> CosmosError {
    CosmosError::builder()
        .with_status(
            azure_data_cosmos_driver::error::status_codes::SERIALIZATION_REQUEST_BODY_INVALID,
        )
        .with_message("failed to serialize item to JSON")
        .with_source(error)
        .build()
}

pub(crate) fn with_diagnostics(
    error: CosmosError,
    diagnostics: Arc<DiagnosticsContext>,
) -> CosmosError {
    CosmosErrorBuilder::from_error(error)
        .with_diagnostics(diagnostics)
        .build()
}

#[cfg(feature = "preview_patch")]
pub(crate) fn with_patch_tracking_id(
    error: CosmosError,
    tracking_id: PatchTrackingId,
) -> CosmosError {
    CosmosErrorBuilder::from_error(error)
        .with_patch_tracking_id(tracking_id.into_driver())
        .build()
}

/// `azure_data_cosmos` crate-wide [`Result`] alias.
pub type Result<T> = std::result::Result<T, CosmosError>;

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::error::ErrorKind as CoreErrorKind;
    use std::error::Error as _;

    #[cfg(feature = "preview_patch")]
    #[test]
    fn patch_tracking_id_converts_to_sdk_model() {
        let id = crate::models::PatchTrackingId::from(uuid::Uuid::from_u128(42));
        let cosmos: CosmosError = CosmosError::builder()
            .with_status(azure_data_cosmos_driver::error::status_codes::TRANSPORT_IO_FAILED)
            .with_patch_tracking_id(id.into_driver())
            .build();

        assert_eq!(
            cosmos.patch_tracking_id().map(PatchTrackingId::from_driver),
            Some(id)
        );
    }

    #[cfg(feature = "preview_patch")]
    #[test]
    fn patch_tracking_id_can_decorate_existing_error() {
        let id = crate::models::PatchTrackingId::from(uuid::Uuid::from_u128(42));
        let error: CosmosError = serde_json::from_slice::<serde_json::Value>(b"{")
            .unwrap_err()
            .into();

        let error = with_patch_tracking_id(error, id);

        assert_eq!(
            error.patch_tracking_id().map(PatchTrackingId::from_driver),
            Some(id)
        );
        assert!(error.source().is_some());
    }

    #[test]
    fn from_cosmos_error_for_azure_core_error_preserves_chain_and_kind() {
        let inner_io = std::io::Error::new(std::io::ErrorKind::Other, "io fail");
        let cosmos: CosmosError = CosmosError::builder()
            .with_status(azure_data_cosmos_driver::error::status_codes::TRANSPORT_IO_FAILED)
            .with_message("transport blew up")
            .with_source(inner_io)
            .build();
        let core_err: azure_core::Error = cosmos.into();
        // TRANSPORT_IO_FAILED maps to Io.
        assert!(matches!(core_err.kind(), CoreErrorKind::Io));
        // Message + source chain preserved (the `CosmosError` becomes the
        // azure_core::Error's source so callers can downcast).
        let rendered = format!("{core_err}");
        assert!(
            rendered.contains("transport blew up") || rendered.contains("io fail"),
            "azure_core::Error rendering must surface the cosmos message or chain: {rendered}",
        );
    }

    #[test]
    fn from_cosmos_error_for_azure_core_error_maps_dns_failure_to_connection() {
        // DNS / connect-refused / H2-incompatibility never sent any bytes
        // on the wire — these map to `Connection`, which `azure_core`
        // documents as safe-to-retry for non-idempotent writes.
        let cosmos: CosmosError = CosmosError::builder()
            .with_status(azure_data_cosmos_driver::error::status_codes::TRANSPORT_DNS_FAILED)
            .with_message("dns lookup failed")
            .build();
        let core_err: azure_core::Error = cosmos.into();
        assert!(
            matches!(core_err.kind(), CoreErrorKind::Connection),
            "TRANSPORT_DNS_FAILED must map to Connection, got {:?}",
            core_err.kind()
        );
    }

    #[test]
    fn from_cosmos_error_for_azure_core_error_maps_auth_to_credential() {
        let cosmos: CosmosError = CosmosError::builder()
            .with_status(azure_data_cosmos_driver::error::status_codes::AUTHENTICATION_TOKEN_ACQUISITION_FAILED)
            .with_message("token acquisition failed")
            .build();
        let core_err: azure_core::Error = cosmos.into();
        assert!(matches!(core_err.kind(), CoreErrorKind::Credential));
    }

    #[test]
    fn from_cosmos_error_for_azure_core_error_maps_serialization_to_data_conversion() {
        let cosmos: CosmosError = CosmosError::builder()
            .with_status(
                azure_data_cosmos_driver::error::status_codes::SERIALIZATION_RESPONSE_BODY_INVALID,
            )
            .with_message("bad json")
            .build();
        let core_err: azure_core::Error = cosmos.into();
        assert!(matches!(core_err.kind(), CoreErrorKind::DataConversion));
    }

    #[test]
    fn from_cosmos_error_for_azure_core_error_synthetic_without_substatus_is_other() {
        // Pure client-validation error: status BadRequest, no sub_status,
        // no wire response. Maps to `Other` — more honest than fabricating
        // an `HttpResponse` from a placeholder status code.
        let cosmos: CosmosError = CosmosError::builder()
            .with_status(CosmosStatus::new(azure_core::http::StatusCode::BadRequest))
            .with_message("bad arg")
            .build();
        let core_err: azure_core::Error = cosmos.into();
        assert!(matches!(core_err.kind(), CoreErrorKind::Other));
    }

    #[test]
    fn from_cosmos_error_for_azure_core_error_downcast_recovers_cosmos_error() {
        let cosmos: CosmosError = CosmosError::builder()
            .with_status(CosmosStatus::new(azure_core::http::StatusCode::BadRequest))
            .with_message("bad arg")
            .build();
        let core_err: azure_core::Error = cosmos.into();
        let chain: &(dyn std::error::Error + 'static) = &core_err;
        let mut cur = chain.source();
        let mut found = false;
        while let Some(s) = cur {
            if s.downcast_ref::<CosmosError>().is_some() {
                found = true;
                break;
            }
            cur = s.source();
        }
        assert!(
            found,
            "azure_core::Error source chain must let callers downcast back to CosmosError"
        );
    }

    /// Asserts the sibling `Connection` mappings: alongside the
    /// already-tested `TRANSPORT_DNS_FAILED`, `TRANSPORT_CONNECTION_FAILED`
    /// and `TRANSPORT_HTTP2_INCOMPATIBLE` are the other two sub-statuses
    /// that provably never put bytes on the wire and are therefore
    /// safe-to-retry for non-idempotent writes per
    /// `azure_core::ErrorKind::Connection`.
    #[test]
    fn from_cosmos_error_for_azure_core_error_connection_siblings_all_map_to_connection() {
        for status in [
            azure_data_cosmos_driver::error::status_codes::TRANSPORT_CONNECTION_FAILED,
            azure_data_cosmos_driver::error::status_codes::TRANSPORT_HTTP2_INCOMPATIBLE,
        ] {
            let cosmos: CosmosError = CosmosError::builder()
                .with_status(status)
                .with_message("never sent")
                .build();
            let core_err: azure_core::Error = cosmos.into();
            assert!(
                matches!(core_err.kind(), CoreErrorKind::Connection),
                "{:?} must map to Connection, got {:?}",
                status.sub_status(),
                core_err.kind()
            );
        }
    }

    /// Asserts the sibling `Io` mappings: alongside the already-tested
    /// `TRANSPORT_IO_FAILED`, both `TRANSPORT_BODY_READ_FAILED` and
    /// `TRANSPORT_GENERATED_503` map to `Io` (retry safety is `Unknown`
    /// — bytes may have left the socket mid-stream). `CLIENT_OPERATION_TIMEOUT`
    /// is in the same Io bucket; it has no public `CosmosStatus` constant
    /// yet so it is not covered here.
    #[test]
    fn from_cosmos_error_for_azure_core_error_io_siblings_all_map_to_io() {
        for status in [
            azure_data_cosmos_driver::error::status_codes::TRANSPORT_BODY_READ_FAILED,
            azure_data_cosmos_driver::error::status_codes::TRANSPORT_GENERATED_503,
        ] {
            let cosmos: CosmosError = CosmosError::builder()
                .with_status(status)
                .with_message("mid-stream")
                .build();
            let core_err: azure_core::Error = cosmos.into();
            assert!(
                matches!(core_err.kind(), CoreErrorKind::Io),
                "{:?} must map to Io, got {:?}",
                status.sub_status(),
                core_err.kind()
            );
        }
    }

    /// Sibling `Credential` mapping: alongside
    /// `AUTHENTICATION_TOKEN_ACQUISITION_FAILED`, a client-generated 401
    /// (signing / authorization failure prior to the wire) also maps to
    /// `Credential`.
    #[test]
    fn from_cosmos_error_for_azure_core_error_client_generated_401_maps_to_credential() {
        let cosmos: CosmosError = CosmosError::builder()
            .with_status(azure_data_cosmos_driver::error::status_codes::CLIENT_GENERATED_401)
            .with_message("client-side auth failure")
            .build();
        let core_err: azure_core::Error = cosmos.into();
        assert!(
            matches!(core_err.kind(), CoreErrorKind::Credential),
            "CLIENT_GENERATED_401 must map to Credential, got {:?}",
            core_err.kind()
        );
    }
}
