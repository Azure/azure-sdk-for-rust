// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Flat, versioned C ABI records for driver fault injection.

use std::{
    mem::{align_of, size_of},
    sync::Arc,
    time::{Duration, Instant},
};

use azure_core::http::StatusCode;
use azure_data_cosmos_driver::{
    diagnostics::TransportKind,
    fault_injection::{
        CustomResponseBuilder, FaultInjectionConditionBuilder, FaultInjectionErrorType,
        FaultInjectionResultBuilder, FaultInjectionRule, FaultInjectionRuleBuilder,
        FaultOperationType,
    },
    options::Region,
};

use crate::{
    error::{CosmosErrorCode, CosmosStatusCode},
    op_request::{decode_headers, CosmosHeaderKv},
    string::{optional_text, required_text, validate_array, CosmosStringView},
};

/// Version of the native fault-injection record layouts.
pub const COSMOS_FAULT_INJECTION_ABI_VERSION_1: u32 = 1;

/// Operation/resource pair used by a fault-injection condition.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CosmosFaultInjectionOperationType {
    CosmosFaultInjectionOperationTypeUnset = 0,
    CosmosFaultInjectionOperationTypeReadItem = 1,
    CosmosFaultInjectionOperationTypeQueryItem = 2,
    CosmosFaultInjectionOperationTypeCreateItem = 3,
    CosmosFaultInjectionOperationTypeUpsertItem = 4,
    CosmosFaultInjectionOperationTypeReplaceItem = 5,
    CosmosFaultInjectionOperationTypeDeleteItem = 6,
    CosmosFaultInjectionOperationTypePatchItem = 7,
    CosmosFaultInjectionOperationTypeBatchItem = 8,
    CosmosFaultInjectionOperationTypeChangeFeedItem = 9,
    CosmosFaultInjectionOperationTypeMetadataReadContainer = 10,
    CosmosFaultInjectionOperationTypeMetadataReadDatabaseAccount = 11,
    CosmosFaultInjectionOperationTypeMetadataQueryPlan = 12,
    CosmosFaultInjectionOperationTypeMetadataPartitionKeyRanges = 13,
}

impl CosmosFaultInjectionOperationType {
    fn from_i32(value: i32) -> Result<Self, CosmosErrorCode> {
        match value {
            0 => Ok(Self::CosmosFaultInjectionOperationTypeUnset),
            1 => Ok(Self::CosmosFaultInjectionOperationTypeReadItem),
            2 => Ok(Self::CosmosFaultInjectionOperationTypeQueryItem),
            3 => Ok(Self::CosmosFaultInjectionOperationTypeCreateItem),
            4 => Ok(Self::CosmosFaultInjectionOperationTypeUpsertItem),
            5 => Ok(Self::CosmosFaultInjectionOperationTypeReplaceItem),
            6 => Ok(Self::CosmosFaultInjectionOperationTypeDeleteItem),
            7 => Ok(Self::CosmosFaultInjectionOperationTypePatchItem),
            8 => Ok(Self::CosmosFaultInjectionOperationTypeBatchItem),
            9 => Ok(Self::CosmosFaultInjectionOperationTypeChangeFeedItem),
            10 => Ok(Self::CosmosFaultInjectionOperationTypeMetadataReadContainer),
            11 => Ok(Self::CosmosFaultInjectionOperationTypeMetadataReadDatabaseAccount),
            12 => Ok(Self::CosmosFaultInjectionOperationTypeMetadataQueryPlan),
            13 => Ok(Self::CosmosFaultInjectionOperationTypeMetadataPartitionKeyRanges),
            _ => Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue),
        }
    }

    fn to_driver(self) -> Option<FaultOperationType> {
        match self {
            Self::CosmosFaultInjectionOperationTypeUnset => None,
            Self::CosmosFaultInjectionOperationTypeReadItem => Some(FaultOperationType::ReadItem),
            Self::CosmosFaultInjectionOperationTypeQueryItem => Some(FaultOperationType::QueryItem),
            Self::CosmosFaultInjectionOperationTypeCreateItem => {
                Some(FaultOperationType::CreateItem)
            }
            Self::CosmosFaultInjectionOperationTypeUpsertItem => {
                Some(FaultOperationType::UpsertItem)
            }
            Self::CosmosFaultInjectionOperationTypeReplaceItem => {
                Some(FaultOperationType::ReplaceItem)
            }
            Self::CosmosFaultInjectionOperationTypeDeleteItem => {
                Some(FaultOperationType::DeleteItem)
            }
            Self::CosmosFaultInjectionOperationTypePatchItem => Some(FaultOperationType::PatchItem),
            Self::CosmosFaultInjectionOperationTypeBatchItem => Some(FaultOperationType::BatchItem),
            Self::CosmosFaultInjectionOperationTypeChangeFeedItem => {
                Some(FaultOperationType::ChangeFeedItem)
            }
            Self::CosmosFaultInjectionOperationTypeMetadataReadContainer => {
                Some(FaultOperationType::MetadataReadContainer)
            }
            Self::CosmosFaultInjectionOperationTypeMetadataReadDatabaseAccount => {
                Some(FaultOperationType::MetadataReadDatabaseAccount)
            }
            Self::CosmosFaultInjectionOperationTypeMetadataQueryPlan => {
                Some(FaultOperationType::MetadataQueryPlan)
            }
            Self::CosmosFaultInjectionOperationTypeMetadataPartitionKeyRanges => {
                Some(FaultOperationType::MetadataPartitionKeyRanges)
            }
        }
    }
}

/// Fault outcome selected when a rule matches.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CosmosFaultInjectionErrorType {
    CosmosFaultInjectionErrorTypeUnset = 0,
    CosmosFaultInjectionErrorTypeInternalServerError = 1,
    CosmosFaultInjectionErrorTypeTooManyRequests = 2,
    CosmosFaultInjectionErrorTypeRetryWith = 3,
    CosmosFaultInjectionErrorTypeReadSessionNotAvailable = 4,
    CosmosFaultInjectionErrorTypeTimeout = 5,
    CosmosFaultInjectionErrorTypeServiceUnavailable = 6,
    CosmosFaultInjectionErrorTypePartitionIsGone = 7,
    CosmosFaultInjectionErrorTypeWriteForbidden = 8,
    CosmosFaultInjectionErrorTypeDatabaseAccountNotFound = 9,
    CosmosFaultInjectionErrorTypeConnectionError = 10,
    CosmosFaultInjectionErrorTypeResponseTimeout = 11,
    CosmosFaultInjectionErrorTypeResponseTimeoutAfterService = 12,
}

impl CosmosFaultInjectionErrorType {
    fn from_i32(value: i32) -> Result<Self, CosmosErrorCode> {
        match value {
            0 => Ok(Self::CosmosFaultInjectionErrorTypeUnset),
            1 => Ok(Self::CosmosFaultInjectionErrorTypeInternalServerError),
            2 => Ok(Self::CosmosFaultInjectionErrorTypeTooManyRequests),
            3 => Ok(Self::CosmosFaultInjectionErrorTypeRetryWith),
            4 => Ok(Self::CosmosFaultInjectionErrorTypeReadSessionNotAvailable),
            5 => Ok(Self::CosmosFaultInjectionErrorTypeTimeout),
            6 => Ok(Self::CosmosFaultInjectionErrorTypeServiceUnavailable),
            7 => Ok(Self::CosmosFaultInjectionErrorTypePartitionIsGone),
            8 => Ok(Self::CosmosFaultInjectionErrorTypeWriteForbidden),
            9 => Ok(Self::CosmosFaultInjectionErrorTypeDatabaseAccountNotFound),
            10 => Ok(Self::CosmosFaultInjectionErrorTypeConnectionError),
            11 => Ok(Self::CosmosFaultInjectionErrorTypeResponseTimeout),
            12 => Ok(Self::CosmosFaultInjectionErrorTypeResponseTimeoutAfterService),
            _ => Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue),
        }
    }

    fn to_driver(self) -> Option<FaultInjectionErrorType> {
        match self {
            Self::CosmosFaultInjectionErrorTypeUnset => None,
            Self::CosmosFaultInjectionErrorTypeInternalServerError => {
                Some(FaultInjectionErrorType::InternalServerError)
            }
            Self::CosmosFaultInjectionErrorTypeTooManyRequests => {
                Some(FaultInjectionErrorType::TooManyRequests)
            }
            Self::CosmosFaultInjectionErrorTypeRetryWith => {
                Some(FaultInjectionErrorType::RetryWith)
            }
            Self::CosmosFaultInjectionErrorTypeReadSessionNotAvailable => {
                Some(FaultInjectionErrorType::ReadSessionNotAvailable)
            }
            Self::CosmosFaultInjectionErrorTypeTimeout => Some(FaultInjectionErrorType::Timeout),
            Self::CosmosFaultInjectionErrorTypeServiceUnavailable => {
                Some(FaultInjectionErrorType::ServiceUnavailable)
            }
            Self::CosmosFaultInjectionErrorTypePartitionIsGone => {
                Some(FaultInjectionErrorType::PartitionIsGone)
            }
            Self::CosmosFaultInjectionErrorTypeWriteForbidden => {
                Some(FaultInjectionErrorType::WriteForbidden)
            }
            Self::CosmosFaultInjectionErrorTypeDatabaseAccountNotFound => {
                Some(FaultInjectionErrorType::DatabaseAccountNotFound)
            }
            Self::CosmosFaultInjectionErrorTypeConnectionError => {
                Some(FaultInjectionErrorType::ConnectionError)
            }
            Self::CosmosFaultInjectionErrorTypeResponseTimeout => {
                Some(FaultInjectionErrorType::ResponseTimeout)
            }
            Self::CosmosFaultInjectionErrorTypeResponseTimeoutAfterService => {
                Some(FaultInjectionErrorType::ResponseTimeoutAfterService)
            }
        }
    }
}

/// Transport matcher for a fault-injection condition.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CosmosFaultInjectionTransportKind {
    CosmosFaultInjectionTransportKindUnset = 0,
    CosmosFaultInjectionTransportKindGateway = 1,
    CosmosFaultInjectionTransportKindGatewayV2 = 2,
}

impl CosmosFaultInjectionTransportKind {
    fn from_i32(value: i32) -> Result<Self, CosmosErrorCode> {
        match value {
            0 => Ok(Self::CosmosFaultInjectionTransportKindUnset),
            1 => Ok(Self::CosmosFaultInjectionTransportKindGateway),
            2 => Ok(Self::CosmosFaultInjectionTransportKindGatewayV2),
            _ => Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue),
        }
    }

    fn to_driver(self) -> Option<TransportKind> {
        match self {
            Self::CosmosFaultInjectionTransportKindUnset => None,
            Self::CosmosFaultInjectionTransportKindGateway => Some(TransportKind::Gateway),
            Self::CosmosFaultInjectionTransportKindGatewayV2 => Some(TransportKind::GatewayV2),
        }
    }
}

/// Size/version prefix shared by native fault-injection records.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CosmosFaultInjectionRecordHeader {
    pub struct_size: usize,
    pub version: u32,
}

/// Match conditions for a native fault-injection rule.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CosmosFaultInjectionCondition {
    pub header: CosmosFaultInjectionRecordHeader,
    pub operation_type: i32,
    pub region: CosmosStringView,
    pub container_id: CosmosStringView,
    pub transport_kind: i32,
}

/// Injected result for a native fault-injection rule.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CosmosFaultInjectionResult {
    pub header: CosmosFaultInjectionRecordHeader,
    pub error_type: i32,
    /// Delay before completing the injected result, in milliseconds.
    ///
    /// Use `-1` to leave unset. `0` is a configured zero-duration delay.
    pub delay_ms: i64,
    pub probability: f32,
    pub custom_status_code: i32,
    /// Injected Cosmos sub-status for a custom HTTP response.
    ///
    /// Use `-1` to leave unset. `0` is a configured sub-status value.
    pub custom_sub_status: i32,
    /// Injected `x-ms-retry-after-ms` value for a custom HTTP response.
    ///
    /// Use `-1` to leave unset. `0` is a configured zero retry-after value.
    pub retry_after_ms: i64,
    pub custom_headers: *const CosmosHeaderKv,
    pub custom_headers_len: usize,
    pub body: *const u8,
    pub body_len: usize,
}

/// Complete native fault-injection rule.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CosmosFaultInjectionRule {
    pub header: CosmosFaultInjectionRecordHeader,
    pub id: CosmosStringView,
    pub condition: *const CosmosFaultInjectionCondition,
    pub result: *const CosmosFaultInjectionResult,
    /// Maximum number of matching requests to inject.
    ///
    /// Use `-1` to leave unset. `0` disables injection for this rule.
    pub hit_limit: i64,
    /// Delay before the rule becomes active, in milliseconds.
    ///
    /// Use `-1` to leave unset. `0` makes the rule active immediately.
    pub start_delay_ms: i64,
    /// Active duration after the computed start time, in milliseconds.
    ///
    /// Use `-1` to leave unset. `0` expires the rule at its start time. When
    /// `start_delay_ms` is set, expiration is measured from that delayed start,
    /// not from the options build call.
    pub expire_after_ms: i64,
}

#[no_mangle]
pub extern "C" fn cosmos_fault_injection_condition_default() -> CosmosFaultInjectionCondition {
    CosmosFaultInjectionCondition {
        header: record_header::<CosmosFaultInjectionCondition>(),
        operation_type: CosmosFaultInjectionOperationType::CosmosFaultInjectionOperationTypeUnset
            as i32,
        region: CosmosStringView::default(),
        container_id: CosmosStringView::default(),
        transport_kind: CosmosFaultInjectionTransportKind::CosmosFaultInjectionTransportKindUnset
            as i32,
    }
}

#[no_mangle]
pub extern "C" fn cosmos_fault_injection_result_default() -> CosmosFaultInjectionResult {
    CosmosFaultInjectionResult {
        header: record_header::<CosmosFaultInjectionResult>(),
        error_type: CosmosFaultInjectionErrorType::CosmosFaultInjectionErrorTypeUnset as i32,
        delay_ms: -1,
        probability: 1.0,
        custom_status_code: 0,
        custom_sub_status: -1,
        retry_after_ms: -1,
        custom_headers: std::ptr::null(),
        custom_headers_len: 0,
        body: std::ptr::null(),
        body_len: 0,
    }
}

#[no_mangle]
pub extern "C" fn cosmos_fault_injection_rule_default() -> CosmosFaultInjectionRule {
    CosmosFaultInjectionRule {
        header: record_header::<CosmosFaultInjectionRule>(),
        id: CosmosStringView::default(),
        condition: std::ptr::null(),
        result: std::ptr::null(),
        hit_limit: -1,
        start_delay_ms: -1,
        expire_after_ms: -1,
    }
}

const fn record_header<T>() -> CosmosFaultInjectionRecordHeader {
    CosmosFaultInjectionRecordHeader {
        struct_size: size_of::<T>(),
        version: COSMOS_FAULT_INJECTION_ABI_VERSION_1,
    }
}

fn validate_header<T>(header: CosmosFaultInjectionRecordHeader) -> Result<(), CosmosErrorCode> {
    if header.version != COSMOS_FAULT_INJECTION_ABI_VERSION_1 || header.struct_size < size_of::<T>()
    {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue);
    }
    Ok(())
}

/// Reads a record whose first field is a [`CosmosFaultInjectionRecordHeader`].
///
/// # Safety
///
/// `record` must be NULL or point to readable memory matching its declared
/// `struct_size` for the duration of this call.
unsafe fn read_versioned_record<T: Copy>(record: *const T) -> Result<T, CosmosErrorCode> {
    if record.is_null() || !record.is_aligned() {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue);
    }
    // SAFETY: every caller uses a repr(C) record with the header as its first
    // field, and the pointer is aligned and readable for at least that prefix.
    let header = unsafe { record.cast::<CosmosFaultInjectionRecordHeader>().read() };
    validate_header::<T>(header)?;
    // SAFETY: the validated size covers the current record and the caller
    // guarantees that the declared bytes are readable.
    Ok(unsafe { record.read() })
}

impl CosmosFaultInjectionCondition {
    unsafe fn to_driver(
        self,
    ) -> Result<azure_data_cosmos_driver::fault_injection::FaultInjectionCondition, CosmosErrorCode>
    {
        validate_header::<Self>(self.header)?;
        let mut builder = FaultInjectionConditionBuilder::new();
        if let Some(operation) =
            CosmosFaultInjectionOperationType::from_i32(self.operation_type)?.to_driver()
        {
            builder = builder.with_operation_type(operation);
        }
        // SAFETY: the caller keeps the counted strings alive for the build call.
        if let Some(region) = unsafe {
            optional_text(
                self.region,
                CosmosErrorCode::CosmosErrorCodeInvalidOptionValue,
            )
        }? {
            builder = builder.with_region(Region::new(region));
        }
        // SAFETY: the caller keeps the counted strings alive for the build call.
        if let Some(container_id) = unsafe {
            optional_text(
                self.container_id,
                CosmosErrorCode::CosmosErrorCodeInvalidOptionValue,
            )
        }? {
            builder = builder.with_container_id(container_id);
        }
        if let Some(transport) =
            CosmosFaultInjectionTransportKind::from_i32(self.transport_kind)?.to_driver()
        {
            builder = builder.with_transport_kind(transport);
        }
        Ok(builder.build())
    }
}

impl CosmosFaultInjectionResult {
    unsafe fn to_driver(
        self,
    ) -> Result<azure_data_cosmos_driver::fault_injection::FaultInjectionResult, CosmosErrorCode>
    {
        validate_header::<Self>(self.header)?;
        if !self.probability.is_finite() || !(0.0..=1.0).contains(&self.probability) {
            return Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue);
        }
        if self.delay_ms < -1 || self.custom_sub_status < -1 || self.retry_after_ms < -1 {
            return Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue);
        }
        let mut builder = FaultInjectionResultBuilder::new().with_probability(self.probability);
        if let Some(error_type) =
            CosmosFaultInjectionErrorType::from_i32(self.error_type)?.to_driver()
        {
            builder = builder.with_error(error_type);
        }
        if self.delay_ms >= 0 {
            builder = builder.with_delay(Duration::from_millis(self.delay_ms as u64));
        }

        let has_custom_response = self.custom_status_code != 0
            || self.custom_sub_status >= 0
            || self.retry_after_ms >= 0
            || !self.custom_headers.is_null()
            || !self.body.is_null();
        if has_custom_response {
            if !(100..=599).contains(&self.custom_status_code)
                || self.custom_sub_status > u16::MAX as i32
            {
                return Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue);
            }
            let mut response =
                CustomResponseBuilder::new(StatusCode::from(self.custom_status_code as u16));
            // SAFETY: the caller contract covers the header array and views.
            if let Some(headers) =
                unsafe { decode_headers(self.custom_headers, self.custom_headers_len)? }
            {
                for (name, value) in headers {
                    response = response.with_header(name, value);
                }
            }
            if self.custom_sub_status >= 0 {
                response = response.with_sub_status(self.custom_sub_status as u16);
            }
            if self.retry_after_ms >= 0 {
                response =
                    response.with_header("x-ms-retry-after-ms", self.retry_after_ms.to_string());
            }
            // SAFETY: the caller contract covers the body pointer and length.
            validate_array(self.body, self.body_len)?;
            if !self.body.is_null() {
                // SAFETY: validated above; the bytes are copied before return.
                response = response
                    .with_body(unsafe { std::slice::from_raw_parts(self.body, self.body_len) });
            }
            builder = builder.with_custom_response(response.build());
        } else {
            // Keep malformed pointer metadata from being silently ignored.
            validate_array(self.custom_headers, self.custom_headers_len)?;
            validate_array(self.body, self.body_len)?;
        }
        Ok(builder.build())
    }
}

impl CosmosFaultInjectionRule {
    unsafe fn to_driver(self) -> Result<Arc<FaultInjectionRule>, CosmosErrorCode> {
        validate_header::<Self>(self.header)?;
        // SAFETY: the caller keeps the id bytes alive for the build call.
        let id =
            unsafe { required_text(self.id, CosmosErrorCode::CosmosErrorCodeInvalidOptionValue) }?;
        if self.hit_limit < -1
            || self.hit_limit > u32::MAX as i64
            || self.start_delay_ms < -1
            || self.expire_after_ms < -1
        {
            return Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue);
        }
        // SAFETY: nested records follow the borrowed-input contract.
        let condition = unsafe { read_versioned_record(self.condition)? }.to_driver()?;
        // SAFETY: nested records follow the borrowed-input contract.
        let result = unsafe { read_versioned_record(self.result)? }.to_driver()?;
        let now = Instant::now();
        let start = if self.start_delay_ms >= 0 {
            Some(
                now.checked_add(Duration::from_millis(self.start_delay_ms as u64))
                    .ok_or(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue)?,
            )
        } else {
            None
        };
        let mut builder = FaultInjectionRuleBuilder::new(id, result).with_condition(condition);
        if let Some(start) = start {
            builder = builder.with_start_time(start);
        }
        if self.expire_after_ms >= 0 {
            let base = start.unwrap_or(now);
            let end = base
                .checked_add(Duration::from_millis(self.expire_after_ms as u64))
                .ok_or(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue)?;
            builder = builder.with_end_time(end);
        }
        if self.hit_limit >= 0 {
            builder = builder.with_hit_limit(self.hit_limit as u32);
        }
        Ok(Arc::new(builder.build()))
    }
}

/// Decodes a strided array of size/version-prefixed rule records.
///
/// # Safety
///
/// `rules` must be NULL/0 or address `rules_len` records separated by
/// `rule_stride` readable bytes. Every nested pointer follows its record's
/// documented borrowed-input contract.
pub(crate) unsafe fn decode_rules(
    rules: *const CosmosFaultInjectionRule,
    rules_len: usize,
    rule_stride: usize,
) -> Result<Vec<Arc<FaultInjectionRule>>, CosmosStatusCode> {
    if rules_len == 0 {
        return if rules.is_null() {
            Ok(Vec::new())
        } else {
            Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_status_code())
        };
    }
    if rules.is_null()
        || !rules.is_aligned()
        || rule_stride < size_of::<CosmosFaultInjectionRule>()
        || !rule_stride.is_multiple_of(align_of::<CosmosFaultInjectionRule>())
        || rules_len.checked_mul(rule_stride).is_none()
        || rules_len > (isize::MAX as usize) / rule_stride
    {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code());
    }
    let mut decoded = Vec::new();
    decoded
        .try_reserve(rules_len)
        .map_err(|_| CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code())?;
    for index in 0..rules_len {
        // SAFETY: validated stride arithmetic and caller-provided readable records.
        let ptr = unsafe {
            rules
                .cast::<u8>()
                .add(index * rule_stride)
                .cast::<CosmosFaultInjectionRule>()
        };
        // SAFETY: each record follows the borrowed-input contract.
        let rule =
            unsafe { read_versioned_record(ptr) }.map_err(CosmosErrorCode::as_status_code)?;
        // SAFETY: nested pointers remain live for the duration of this call.
        decoded.push(unsafe { rule.to_driver() }.map_err(CosmosErrorCode::as_status_code)?);
    }
    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::string::view;

    #[test]
    fn defaults_are_current_and_unset() {
        let condition = cosmos_fault_injection_condition_default();
        assert_eq!(
            condition.header.struct_size,
            size_of::<CosmosFaultInjectionCondition>()
        );
        assert_eq!(
            condition.header.version,
            COSMOS_FAULT_INJECTION_ABI_VERSION_1
        );
        let result = cosmos_fault_injection_result_default();
        assert_eq!(result.probability, 1.0);
        assert_eq!(result.delay_ms, -1);
        assert_eq!(result.custom_sub_status, -1);
        let rule = cosmos_fault_injection_rule_default();
        assert!(rule.condition.is_null());
        assert!(rule.result.is_null());
        assert_eq!(rule.hit_limit, -1);
        assert_eq!(rule.start_delay_ms, -1);
        assert_eq!(rule.expire_after_ms, -1);
    }

    #[test]
    fn custom_response_copies_all_borrowed_input() {
        let id = b"native-429";
        let region = b"East US";
        let container = b"items";
        let header_name = b"x-ms-request-charge";
        let header_value = b"4.25";
        let headers = [CosmosHeaderKv {
            name: view(header_name),
            value: view(header_value),
        }];
        let body = br#"{"code":"TooManyRequests","message":"injected"}"#;
        let mut condition = cosmos_fault_injection_condition_default();
        condition.operation_type =
            CosmosFaultInjectionOperationType::CosmosFaultInjectionOperationTypeReadItem as i32;
        condition.region = view(region);
        condition.container_id = view(container);
        condition.transport_kind =
            CosmosFaultInjectionTransportKind::CosmosFaultInjectionTransportKindGateway as i32;
        let mut result = cosmos_fault_injection_result_default();
        result.custom_status_code = 429;
        result.custom_sub_status = 3200;
        result.retry_after_ms = 17;
        result.custom_headers = headers.as_ptr();
        result.custom_headers_len = headers.len();
        result.body = body.as_ptr();
        result.body_len = body.len();
        let mut rule = cosmos_fault_injection_rule_default();
        rule.id = view(id);
        rule.condition = &condition;
        rule.result = &result;
        rule.hit_limit = 2;
        rule.start_delay_ms = 0;
        rule.expire_after_ms = 1_000;

        // SAFETY: every borrowed buffer above remains live during conversion.
        let decoded = unsafe { rule.to_driver() }.expect("rule converts");
        assert_eq!(decoded.id(), "native-429");
        assert_eq!(
            decoded.condition().operation_type(),
            Some(FaultOperationType::ReadItem)
        );
        assert_eq!(decoded.condition().region().unwrap().as_str(), "eastus");
        assert_eq!(decoded.condition().container_id(), Some("items"));
        assert_eq!(
            decoded.condition().transport_kind(),
            Some(TransportKind::Gateway)
        );
        assert_eq!(decoded.hit_limit(), Some(2));
        let response = decoded.result().custom_response().unwrap();
        assert_eq!(u16::from(response.status_code()), 429);
        assert_eq!(response.body(), body);
        assert_eq!(
            response
                .headers()
                .get_optional_str(&azure_core::http::headers::HeaderName::from(
                    "x-ms-request-charge"
                )),
            Some("4.25")
        );
    }

    #[test]
    fn invalid_record_and_discriminants_are_rejected() {
        let mut rule = cosmos_fault_injection_rule_default();
        rule.id = view(b"bad");
        let condition = cosmos_fault_injection_condition_default();
        let result = cosmos_fault_injection_result_default();
        rule.condition = &condition;
        rule.result = &result;
        rule.header.version = 99;
        // SAFETY: all pointer fields are valid for this call.
        assert_eq!(
            unsafe { rule.to_driver() }.unwrap_err(),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue
        );
        rule.header = record_header::<CosmosFaultInjectionRule>();
        let mut invalid_condition = cosmos_fault_injection_condition_default();
        invalid_condition.operation_type = 99;
        rule.condition = &invalid_condition;
        // SAFETY: all pointer fields are valid for this call.
        assert_eq!(
            unsafe { rule.to_driver() }.unwrap_err(),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue
        );
    }

    #[test]
    fn result_signed_fields_reject_values_below_unset_sentinel() {
        let mut result = cosmos_fault_injection_result_default();
        result.delay_ms = -2;
        // SAFETY: the default result contains no borrowed pointers.
        assert_eq!(
            unsafe { result.to_driver() }.unwrap_err(),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue
        );

        let mut result = cosmos_fault_injection_result_default();
        result.custom_sub_status = -2;
        // SAFETY: the default result contains no borrowed pointers.
        assert_eq!(
            unsafe { result.to_driver() }.unwrap_err(),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue
        );

        let mut result = cosmos_fault_injection_result_default();
        result.retry_after_ms = -2;
        // SAFETY: the default result contains no borrowed pointers.
        assert_eq!(
            unsafe { result.to_driver() }.unwrap_err(),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue
        );
    }

    #[test]
    fn strided_rules_reject_a_misaligned_base_pointer() {
        let misaligned = std::ptr::NonNull::<CosmosFaultInjectionRule>::dangling()
            .as_ptr()
            .cast::<u8>()
            .wrapping_add(1)
            .cast::<CosmosFaultInjectionRule>();
        // SAFETY: the decoder rejects the pointer before reading from it.
        assert_eq!(
            unsafe { decode_rules(misaligned, 1, size_of::<CosmosFaultInjectionRule>(),) }
                .unwrap_err(),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code()
        );
    }

    #[test]
    fn strided_rules_reject_spans_larger_than_isize() {
        let rules = std::ptr::NonNull::<CosmosFaultInjectionRule>::dangling().as_ptr();
        let stride = size_of::<CosmosFaultInjectionRule>();
        let rules_len = (isize::MAX as usize / stride) + 1;
        // SAFETY: the decoder rejects the metadata before allocation or reads.
        assert_eq!(
            unsafe { decode_rules(rules, rules_len, stride) }.unwrap_err(),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code()
        );
    }

    #[test]
    fn strided_rules_accept_larger_same_version_records() {
        #[repr(C)]
        #[derive(Clone, Copy)]
        struct ExtendedCondition {
            prefix: CosmosFaultInjectionCondition,
            _tail: [u8; 8],
        }

        #[repr(C)]
        #[derive(Clone, Copy)]
        struct ExtendedResult {
            prefix: CosmosFaultInjectionResult,
            _tail: [u8; 8],
        }

        #[repr(C)]
        #[derive(Clone, Copy)]
        struct ExtendedRule {
            prefix: CosmosFaultInjectionRule,
            _tail: [u8; 8],
        }

        let mut condition = ExtendedCondition {
            prefix: cosmos_fault_injection_condition_default(),
            _tail: [0xCD; 8],
        };
        condition.prefix.header.struct_size = size_of::<ExtendedCondition>();
        condition.prefix.operation_type =
            CosmosFaultInjectionOperationType::CosmosFaultInjectionOperationTypeReadItem as i32;

        let mut result = ExtendedResult {
            prefix: cosmos_fault_injection_result_default(),
            _tail: [0xCD; 8],
        };
        result.prefix.header.struct_size = size_of::<ExtendedResult>();
        result.prefix.custom_status_code = 429;
        result.prefix.custom_sub_status = 3200;

        let mut rule = ExtendedRule {
            prefix: cosmos_fault_injection_rule_default(),
            _tail: [0xCD; 8],
        };
        rule.prefix.header.struct_size = size_of::<ExtendedRule>();
        rule.prefix.id = view(b"extended-prefix");
        rule.prefix.condition = (&condition as *const ExtendedCondition).cast();
        rule.prefix.result = (&result as *const ExtendedResult).cast();
        let rules = [rule];

        // SAFETY: all borrowed records and buffers remain live during decoding.
        let decoded = unsafe {
            decode_rules(
                rules.as_ptr().cast::<CosmosFaultInjectionRule>(),
                rules.len(),
                size_of::<ExtendedRule>(),
            )
        }
        .expect("larger same-version records decode");

        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].id(), "extended-prefix");
        assert_eq!(
            decoded[0].condition().operation_type(),
            Some(FaultOperationType::ReadItem)
        );
        let response = decoded[0].result().custom_response().unwrap();
        assert_eq!(u16::from(response.status_code()), 429);
        assert_eq!(
            response
                .headers()
                .get_optional_str(&azure_core::http::headers::HeaderName::from(
                    "x-ms-substatus"
                )),
            Some("3200")
        );
    }
}
