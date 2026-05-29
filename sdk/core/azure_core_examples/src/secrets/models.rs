// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Models for the example secrets client.

use async_trait::async_trait;
use azure_core::{
    http::{
        pager::{Page, PagerOptions},
        RequestContent,
    },
    json::to_json,
    Result,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A secret value and its properties.
#[derive(Clone, Default, Deserialize, Serialize, Debug)]
pub struct Secret {
    /// The secret id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The secret value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Properties of a secret (no value).
#[derive(Clone, Default, Deserialize, Serialize, Debug)]
pub struct SecretProperties {
    /// The secret id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// A page of secret properties.
#[derive(Clone, Default, Deserialize, Serialize, Debug)]
pub struct ListSecretPropertiesResult {
    /// A list of secret properties.
    #[serde(default)]
    pub value: Vec<SecretProperties>,

    /// The URL to get the next set of secrets.
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

#[async_trait]
impl Page for ListSecretPropertiesResult {
    type Item = SecretProperties;
    type IntoIter = <Vec<SecretProperties> as IntoIterator>::IntoIter;
    async fn into_items(self) -> Result<Self::IntoIter> {
        Ok(self.value.into_iter())
    }
}

/// Parameters for setting a secret.
#[derive(Clone, Default, Deserialize, Serialize, Debug)]
pub struct SetSecretParameters {
    /// The value of the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// The content type of the secret.
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    /// Application-specific metadata as key-value pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, String>>,
}

impl TryFrom<SetSecretParameters> for RequestContent<SetSecretParameters> {
    type Error = azure_core::Error;
    fn try_from(value: SetSecretParameters) -> Result<Self> {
        Ok(to_json(&value)?.into())
    }
}

/// Parameters for updating secret properties.
#[derive(Clone, Default, Deserialize, Serialize, Debug)]
pub struct UpdateSecretPropertiesParameters {
    /// The content type of the secret.
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    /// The secret management attributes.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub secret_attributes: Option<serde_json::Value>,

    /// Application-specific metadata as key-value pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, String>>,
}

impl TryFrom<UpdateSecretPropertiesParameters>
    for RequestContent<UpdateSecretPropertiesParameters>
{
    type Error = azure_core::Error;
    fn try_from(value: UpdateSecretPropertiesParameters) -> Result<Self> {
        Ok(to_json(&value)?.into())
    }
}

/// Options for `SecretClient::list_secret_properties`.
#[derive(Clone, Default, Debug)]
pub struct SecretClientListSecretPropertiesOptions<'a> {
    /// Allows customization of the method call.
    pub method_options: PagerOptions<'a>,
}
