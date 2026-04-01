// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Authentication pipeline policies.

mod bearer_token_policy;

pub use bearer_token_policy::{
    is_challenge_resource_match, Authorizer, BearerTokenAuthorizationPolicy, OnChallenge, OnRequest,
};
