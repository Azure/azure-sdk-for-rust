use crate::{client::API_VERSION_PARAM, DeviceUpdateClient};
use azure_core::{
    error::{Error, ErrorKind, ResultExt},
    sleep,
};
use getset::Getters;
use log::debug;
use serde::Deserialize;
use serde_json::{Map, Value};
use std::fmt::Debug;
use std::time::Duration;
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct ImportManifestMetadata {
    hashes: Map<String, Value>,
    size_in_bytes: u64,
    url: String,
}

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct FileImportMetadata {
    filename: String,
    url: String,
}

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct InputUpdateInputItem {
    files: Vec<FileImportMetadata>,
    friendly_name: String,
    import_manifest: ImportManifestMetadata,
}

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct ImportUpdateInput {
    #[serde(flatten)]
    update_to_import: Vec<InputUpdateInputItem>,
}

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct UpdateId {
    name: String,
    provider: String,
    version: String,
}

#[derive(Debug, Deserialize)]
pub enum StepType {
    #[serde(rename = "inline")]
    Inline,
    #[serde(rename = "reference")]
    Reference,
}

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct Step {
    description: Option<String>,
    files: Vec<String>,
    handler: String,
    handler_properties: Map<String, Value>,
    #[serde(rename = "type")]
    step_type: Option<StepType>,
    update_id: Option<UpdateId>,
}

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct Instructions {
    steps: Vec<Step>,
}

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct Update {
    compatibility: Vec<Map<String, Value>>,
    #[serde(with = "azure_core::date::rfc3339")]
    created_date_time: OffsetDateTime,
    description: Option<String>,
    etag: String,
    friendly_name: Option<String>,
    #[serde(with = "azure_core::date::rfc3339")]
    imported_date_time: OffsetDateTime,
    installed_criteria: Option<String>,
    instructions: Option<Instructions>,
    is_deployable: bool,
    manifest_version: String,
    referenced_by: Option<Vec<UpdateId>>,
    scan_result: String,
    update_id: UpdateId,
    update_type: Option<String>,
}

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct UpdateFile {
    etag: String,
    file_id: String,
    file_name: String,
    hashes: Map<String, Value>,
    mime_type: String,
    scan_details: String,
    scan_result: String,
    size_in_bytes: u64,
}

#[derive(Debug, Deserialize)]
pub enum OperationStatus {
    Failed,
    NotStarted,
    Running,
    Succeeded,
    Undefined,
}

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct UpdateOperation {
    #[serde(with = "azure_core::date::rfc3339")]
    pub created_date_time: OffsetDateTime,
    pub error: Option<Value>,
    pub etag: String,
    #[serde(with = "azure_core::date::rfc3339")]
    pub last_action_date_time: OffsetDateTime,
    pub operation_id: String,
    pub resource_location: Option<String>,
    pub status: OperationStatus,
    pub trace_id: String,
    pub update_id: Option<UpdateId>,
}

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct StringsList {
    next_link: Option<String>,
    value: Vec<String>,
}

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct UpdateOperationsList {
    next_link: Option<String>,
    value: Vec<UpdateOperation>,
}

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
#[serde(rename_all = "camelCase")]
pub struct UpdateList {
    next_link: Option<String>,
    value: Vec<Update>,
}

impl DeviceUpdateClient {
    /// Import new update version.
    /// POST https://{endpoint}/deviceupdate/{instanceId}/updates?action=import&api-version=2021-06-01-preview
    pub async fn import_update(
        &self,
        instance_id: &str,
        import_json: String,
    ) -> azure_core::Result<UpdateOperation> {
        let mut uri = self.device_update_url.clone();
        let path = format!("deviceupdate/{instance_id}/updates");
        uri.set_path(&path);
        uri.set_query(Some(API_VERSION_PARAM));
        uri.query_pairs_mut().append_pair("action", "import");

        debug!("Import request: {}", &uri);
        let resp_body = self.post(uri.to_string(), Some(import_json)).await?;
        debug!("Import response: {}", &resp_body);

        loop {
            sleep(Duration::from_secs(5)).await;
            let mut uri = self.device_update_url.clone();
            uri.set_path(&resp_body);
            debug!("Requesting operational status: {}", &uri);
            let update_operation: UpdateOperation = self.get(uri.to_string()).await?;

            match update_operation.status {
                OperationStatus::Failed => {
                    return Err(Error::with_message(ErrorKind::Other, {
                        || {
                            format!(
                                "import unsuccessful with status failed. status: {:?}",
                                update_operation.status
                            )
                        }
                    }))
                }
                OperationStatus::Succeeded => return Ok(update_operation),
                OperationStatus::NotStarted => continue,
                OperationStatus::Running => continue,
                OperationStatus::Undefined => {
                    return Err(Error::with_message(ErrorKind::Other, || {
                        format!(
                            "import unsuccessful with status undefined. status: {:?}",
                            update_operation.status
                        )
                    }))
                }
            }
        }
    }

    /// Delete a specific update version.
    /// DELETE https://{endpoint}/deviceupdate/{instanceId}/updates/providers/{provider}/names/{name}/versions/{version}?api-version=2021-06-01-preview
    pub async fn delete_update(
        &self,
        instance_id: &str,
        provider: &str,
        name: &str,
        version: &str,
    ) -> azure_core::Result<String> {
        let mut uri = self.device_update_url.clone();
        let path = format!("deviceupdate/{instance_id}/updates/providers/{provider}/names/{name}/versions/{version}");
        uri.set_path(&path);
        uri.set_query(Some(API_VERSION_PARAM));

        self.delete(uri.to_string()).await
    }

    /// Get a specific update file from the version.
    /// GET https://{endpoint}/deviceupdate/{instanceId}/updates/providers/{provider}/names/{name}/versions/{version}/files/{fileId}?api-version=2021-06-01-previe
    pub async fn get_file(
        &self,
        instance_id: &str,
        provider: &str,
        name: &str,
        version: &str,
        file_id: &str,
    ) -> azure_core::Result<UpdateFile> {
        let mut uri = self.device_update_url.clone();
        let path = format!("deviceupdate/{instance_id}/updates/providers/{provider}/names/{name}/versions/{version}/files/{file_id}");
        uri.set_path(&path);
        uri.set_query(Some(API_VERSION_PARAM));

        self.get(uri.to_string()).await
    }

    /// Retrieve operation status.
    /// GET https://{endpoint}/deviceupdate/{instanceId}/updates/operations/{operationId}?api-version=2021-06-01-preview
    pub async fn get_operation(
        &self,
        instance_id: &str,
        operation_id: &str,
    ) -> azure_core::Result<UpdateOperation> {
        let mut uri = self.device_update_url.clone();
        let path = format!("deviceupdate/{instance_id}/updates/operations/{operation_id}");
        uri.set_path(&path);
        uri.set_query(Some(API_VERSION_PARAM));

        self.get(uri.to_string()).await
    }

    /// Get a specific update version.
    /// GET https://{endpoint}/deviceupdate/{instanceId}/updates/providers/{provider}/names/{name}/versions/{version}?api-version=2021-06-01-preview
    pub async fn get_update(
        &self,
        instance_id: &str,
        provider: &str,
        name: &str,
        version: &str,
    ) -> azure_core::Result<Update> {
        let mut uri = self.device_update_url.clone();
        let path = format!("deviceupdate/{instance_id}/updates/providers/{provider}/names/{name}/versions/{version}");
        uri.set_path(&path);
        uri.set_query(Some(API_VERSION_PARAM));

        self.get(uri.to_string()).await
    }

    /// Get a list of all update file identifiers for the specified version.
    /// GET https://{endpoint}/deviceupdate/{instanceId}/updates/providers/{provider}/names/{name}/versions/{version}/files?api-version=2021-06-01-preview
    pub async fn list_files(
        &self,
        instance_id: &str,
        provider: &str,
        name: &str,
        version: &str,
    ) -> azure_core::Result<Vec<String>> {
        let mut uri = self.device_update_url.clone();
        let path = format!("deviceupdate/{instance_id}/updates/providers/{provider}/names/{name}/versions/{version}/files");
        uri.set_path(&path);
        uri.set_query(Some(API_VERSION_PARAM));

        let mut all_results: Vec<String> = Vec::new();

        loop {
            let mut response: StringsList = self.get(uri.to_string()).await?;
            all_results.append(&mut response.value);

            match response.next_link {
                None => break,
                Some(url) => {
                    uri = self.device_update_url.clone();
                    uri.set_path("");
                    uri.set_query(None);
                    uri = uri.join(&url).with_context(ErrorKind::DataConversion, || {
                        format!("failed to parse url. url: {url}")
                    })?
                }
            }
        }
        Ok(all_results)
    }

    /// Get a list of all update names that match the specified provider.
    /// GET https://{endpoint}/deviceupdate/{instanceId}/updates/providers/{provider}/names?api-version=2021-06-01-preview
    pub async fn list_names(
        &self,
        instance_id: &str,
        provider: &str,
    ) -> azure_core::Result<Vec<String>> {
        let mut uri = self.device_update_url.clone();
        let path = format!("deviceupdate/{instance_id}/updates/providers/{provider}/names");
        uri.set_path(&path);
        uri.set_query(Some(API_VERSION_PARAM));

        let mut all_results: Vec<String> = Vec::new();

        loop {
            let mut response: StringsList = self.get(uri.to_string()).await?;
            all_results.append(&mut response.value);

            match response.next_link {
                None => break,
                Some(url) => {
                    uri = self.device_update_url.clone();
                    uri.set_path("");
                    uri.set_query(None);
                    uri = uri.join(&url).with_context(ErrorKind::DataConversion, || {
                        format!("failed to parse url. url: {url}")
                    })?
                }
            }
        }
        Ok(all_results)
    }

    /// Get a list of all import update operations.
    /// Completed operations are kept for 7 days before auto-deleted. Delete operations are not returned by this API version.
    /// GET https://{endpoint}/deviceupdate/{instanceId}/updates/operations?$filter={$filter}&$top={$top}&api-version=2021-06-01-preview
    pub async fn list_operations(
        &self,
        instance_id: &str,
        filter: Option<&str>,
        top: Option<&str>,
    ) -> azure_core::Result<Vec<UpdateOperation>> {
        let mut uri = self.device_update_url.clone();
        let path = format!("deviceupdate/{instance_id}/updates/operations");

        uri.set_path(&path);
        uri.set_query(Some(API_VERSION_PARAM));
        if let Some(filter) = filter {
            uri.query_pairs_mut().append_pair("$filter", filter);
        }
        if let Some(top) = top {
            uri.query_pairs_mut().append_pair("$top", top);
        }

        let mut all_results: Vec<UpdateOperation> = Vec::new();

        loop {
            let mut response: UpdateOperationsList = self.get(uri.to_string()).await?;
            all_results.append(&mut response.value);

            match response.next_link {
                None => break,
                Some(url) => {
                    uri = self.device_update_url.clone();
                    uri.set_path("");
                    uri.set_query(None);
                    uri = uri.join(&url).with_context(ErrorKind::DataConversion, || {
                        format!("failed to parse url. url: {url}")
                    })?
                }
            }
        }
        Ok(all_results)
    }

    /// Get a list of all update providers that have been imported to Device Update for IoT Hub.
    /// GET https://{endpoint}/deviceupdate/{instanceId}/updates/providers?api-version=2021-06-01-preview
    pub async fn list_providers(&self, instance_id: &str) -> azure_core::Result<Vec<String>> {
        let mut uri = self.device_update_url.clone();
        let path = format!("deviceupdate/{instance_id}/updates/providers");
        uri.set_path(&path);
        uri.set_query(Some(API_VERSION_PARAM));

        let mut all_results: Vec<String> = Vec::new();

        loop {
            let mut response: StringsList = self.get(uri.to_string()).await?;
            all_results.append(&mut response.value);

            match response.next_link {
                None => break,
                Some(url) => {
                    uri = self.device_update_url.clone();
                    uri.set_path("");
                    uri.set_query(None);
                    uri = uri.join(&url).with_context(ErrorKind::DataConversion, || {
                        format!("failed to parse url. url: {url}")
                    })?
                }
            }
        }
        Ok(all_results)
    }

    /// Get a list of all updates that have been imported to Device Update for IoT Hub.
    /// GET https://{endpoint}/deviceupdate/{instanceId}/updates?api-version=2021-06-01-preview&$search={$search}&$filter={$filter}
    pub async fn list_updates(
        &self,
        instance_id: &str,
        filter: Option<&str>,
        search: Option<&str>,
    ) -> azure_core::Result<Vec<Update>> {
        let mut uri = self.device_update_url.clone();
        let path = format!("deviceupdate/{instance_id}/updates");
        uri.set_path(&path);

        uri.set_query(Some(API_VERSION_PARAM));
        if let Some(search) = search {
            uri.query_pairs_mut().append_pair("$search", search);
        }

        if let Some(filter) = filter {
            uri.query_pairs_mut().append_pair("$filter", filter);
        }

        let mut all_results: Vec<Update> = Vec::new();

        loop {
            let mut response: UpdateList = self.get(uri.to_string()).await?;
            all_results.append(&mut response.value);

            match response.next_link {
                None => break,
                Some(url) => {
                    uri = self.device_update_url.clone();
                    uri.set_path("");
                    uri.set_query(None);
                    uri = uri.join(&url).with_context(ErrorKind::DataConversion, || {
                        format!("failed to parse url. url: {url}")
                    })?
                }
            }
        }
        Ok(all_results)
    }

    /// Get a list of all update versions that match the specified provider and name.
    /// GET https://{endpoint}/deviceupdate/{instanceId}/updates/providers/{provider}/names/{name}/versions?api-version=2021-06-01-preview&$filter={$filter}
    pub async fn list_versions(
        &self,
        instance_id: &str,
        provider: &str,
        name: &str,
        filter: Option<&str>,
    ) -> azure_core::Result<Vec<String>> {
        let mut uri = self.device_update_url.clone();
        let path = format!(
            "deviceupdate/{instance_id}/updates/providers/{provider}/names/{name}/versions"
        );
        uri.set_path(&path);

        uri.set_query(Some(API_VERSION_PARAM));
        if let Some(filter) = filter {
            uri.query_pairs_mut().append_pair("$filter", filter);
        }

        let mut all_results: Vec<String> = Vec::new();

        loop {
            let mut response: StringsList = self.get(uri.to_string()).await?;
            all_results.append(&mut response.value);

            match response.next_link {
                None => break,
                Some(url) => {
                    uri = self.device_update_url.clone();
                    uri.set_path("");
                    uri.set_query(None);
                    uri = uri.join(&url).with_context(ErrorKind::DataConversion, || {
                        format!("failed to parse url. url: {url}")
                    })?
                }
            }
        }
        Ok(all_results)
    }
}

#[cfg(test)]
mod tests {
    use azure_core::date;
    use mockito::{mock, Matcher};
    use serde_json::json;

    use crate::{client::API_VERSION, tests::mock_client};

    #[tokio::test]
    async fn can_import_update() -> azure_core::Result<()> {
        let _m = mock("POST", "/deviceupdate/test-instance/updates")
            .match_query(Matcher::UrlEncoded(
                "api-version".into(),
                API_VERSION.into(),
            ))
            .with_header("operation-location", "/op_location")
            .with_status(202)
            .create();
        let _op = mock("GET", "/op_location")
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "createdDateTime":"1999-09-10T21:59:22Z",
                    "lastActionDateTime":"1999-09-10T03:05:07.3845533+01:00",
                    "etag": "\"some_tag\"",
                    "operationId": "some_op_id",
                    "resourceLocation": "/deviceupdate/instance/updates/providers/xxx/names/yyy/versions/x.y.z?api-version=2021-06-01-preview",
                    "status": "Succeeded",
                    "traceId": "zzzzzzzzzzzzzzzz",
                    "updateId": {
                        "name": "somename",
                        "provider": "someprov",
                        "version": "x.y.z",
                    }
                })
                .to_string(),
            )
            .with_status(200)
            .create();

        let client = mock_client();

        let update = client
            .import_update(
                "test-instance",
                r#"{"some":"json","...":"fields"}"#.to_owned(),
            )
            .await?;
        assert_eq!(update.etag, "\"some_tag\"");
        assert_eq!(update.operation_id, "some_op_id");
        assert_eq!(
            update.created_date_time,
            date::parse_rfc3339("1999-09-10T21:59:22Z").unwrap()
        );
        assert_eq!(
            update.last_action_date_time,
            date::parse_rfc3339("1999-09-10T02:05:07.3845533Z").unwrap()
        );

        Ok(())
    }
}
