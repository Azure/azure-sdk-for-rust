// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Typed throughput headers for database and container creation.

use crate::error::{CosmosErrorCode, CosmosStatusCode};
use azure_data_cosmos_driver::models::{
    AutoscaleAutoUpgradePolicy, AutoscaleThroughputPolicy, CosmosRequestHeaders,
    OfferAutoscaleSettings,
};
use std::mem::size_of;

const INVALID: CosmosErrorCode = CosmosErrorCode::CosmosErrorCodeInvalidOptionValue;

/// Typed throughput headers for database and container creation.
///
/// Zero-initialize the payload fields and set the size/version fields below.
/// Unset leaves inherited custom headers unchanged. Set values take precedence
/// over conflicting custom headers without replacing their map. All inputs are
/// copied during submit.
#[repr(C)]
pub struct CosmosThroughputOptions {
    /// Readable allocation size, at least the known record size, including any
    /// zero-filled extension bytes. Must not exceed the actual readable storage.
    pub struct_size_bytes: u32,
    /// Must be 1.
    pub abi_version: u32,
    /// 0 unset, 1 manual, 2 autoscale. Inactive payloads must be zero.
    pub throughput_kind: u32,
    /// Positive RU/s when `throughput_kind` is 1; otherwise zero.
    pub manual_throughput: u64,
    /// Positive maximum RU/s when `throughput_kind` is 2; otherwise zero.
    pub autoscale_max_throughput: u64,
    /// 0 absent, 1 present. Only valid for autoscale.
    pub has_auto_upgrade_policy: u8,
    /// 0 absent, 1 present. Requires `has_auto_upgrade_policy` to be 1.
    pub has_throughput_policy: u8,
    /// Full unsigned percentage; zero when the throughput policy is absent.
    pub increment_percent: u32,
}

unsafe fn record<'a, T>(input: *const T) -> Result<&'a T, CosmosStatusCode> {
    if input.is_null() || !input.is_aligned() {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code());
    }
    // SAFETY: all versioned records begin with two readable u32 fields.
    let size = unsafe { input.cast::<u32>().read() } as usize;
    if size < size_of::<T>() || size > isize::MAX as usize {
        return Err(INVALID.as_status_code());
    }
    // SAFETY: declared size covers the prefix, known record, and extension.
    unsafe {
        if input.cast::<u32>().add(1).read() != 1
            || std::slice::from_raw_parts(
                input.cast::<u8>().add(size_of::<T>()),
                size - size_of::<T>(),
            )
            .iter()
            .any(|byte| *byte != 0)
        {
            return Err(INVALID.as_status_code());
        }
        Ok(&*input)
    }
}

pub(crate) unsafe fn throughput_headers(
    input: *const CosmosThroughputOptions,
) -> Result<Option<CosmosRequestHeaders>, CosmosStatusCode> {
    if input.is_null() {
        return Ok(None);
    }
    // SAFETY: caller supplies readable versioned storage for the duration of submit.
    let input = unsafe { record(input)? };
    if input.has_auto_upgrade_policy > 1
        || input.has_throughput_policy > input.has_auto_upgrade_policy
        || (input.has_throughput_policy == 0 && input.increment_percent != 0)
    {
        return Err(INVALID.as_status_code());
    }
    let no_policy = input.has_auto_upgrade_policy == 0;
    let mut headers = CosmosRequestHeaders::default();
    match input.throughput_kind {
        0 if input.manual_throughput == 0 && input.autoscale_max_throughput == 0 && no_policy => {
            return Ok(None);
        }
        1 if input.manual_throughput > 0 && input.autoscale_max_throughput == 0 && no_policy => {
            headers.offer_throughput = Some(input.manual_throughput);
        }
        2 if input.manual_throughput == 0 && input.autoscale_max_throughput > 0 => {
            let mut settings = OfferAutoscaleSettings::new(input.autoscale_max_throughput);
            if input.has_auto_upgrade_policy == 1 {
                let mut policy = AutoscaleAutoUpgradePolicy::default();
                if input.has_throughput_policy == 1 {
                    let mut throughput = AutoscaleThroughputPolicy::default();
                    throughput.increment_percent = input.increment_percent;
                    policy.throughput_policy = Some(throughput);
                }
                settings.auto_upgrade_policy = Some(policy);
            }
            headers.offer_autopilot_settings = Some(settings);
        }
        _ => return Err(INVALID.as_status_code()),
    }
    Ok(Some(headers))
}

#[cfg(test)]
mod tests;
