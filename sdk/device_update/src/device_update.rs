use crate::client::API_VERSION_PARAM;
use crate::DeviceUpdateClient;
use crate::Error;
use azure_core::auth::TokenCredential;
use chrono::{DateTime, Utc};
use getset::Getters;
use log::debug;
use serde::Deserialize;
use serde_json::{Map, Value};
use std::fmt::Debug;

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
    created_date_time: DateTime<Utc>,
    description: Option<String>,
    etag: String,
    friendly_name: Option<String>,
    imported_date_time: DateTime<Utc>,
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
    created_date_time: DateTime<Utc>,
    error: Option<Value>,
    etag: String,
    last_action_date_time: DateTime<Utc>,
    operation_id: String,
    resource_location: Option<String>,
    status: OperationStatus,
    trace_id: String,
    update_id: Option<UpdateId>,
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

impl<'a, T: TokenCredential> DeviceUpdateClient<'a, T> {
    /// Import new update version.
    /// POST https://{endpoint}/deviceupdate/{instanceId}/updates?action=import&api-version=2021-06-01-preview
    pub async fn import_update(
        &mut self,
        instance_id: &str,
        import_json: String,
    ) -> Result<UpdateOperation, Error> {
        let mut uri = self.device_update_url.clone();
        let path = format!("deviceupdate/{instance_id}/updates");
        uri.set_path(&path);
        let params = format!("action=import&{}", API_VERSION_PARAM);
        uri.set_query(Some(&params));

        debug!("Import request: {}", &uri);
        let resp_body = self.post_authed(uri.to_string(), Some(import_json)).await?;
        debug!("Import response: {}", &resp_body);

        loop {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            let mut uri = self.device_update_url.clone();
            uri.set_path(&resp_body);
            debug!("Requesting operational status: {}", &uri);
            let resp_body = self.get_authed(uri.to_string()).await?;
            debug!("Operational status response: {}", &resp_body);
            match serde_json::from_str::<UpdateOperation>(&resp_body) {
                Ok(status) => {
                    let error: String = match status.error.clone() {
                        None => "not present".to_owned(),
                        Some(v) => v.to_string(),
                    };
                    match status.status {
                        OperationStatus::Failed => return Err(Error::ImportFailed(error)),
                        OperationStatus::Succeeded => return Ok(status),
                        OperationStatus::NotStarted => continue,
                        OperationStatus::Running => continue,
                        OperationStatus::Undefined => return Err(Error::ImportUndefined(error)),
                    }
                }
                Err(_e) => {
                    return Err(Error::SerdeParse(_e));
                }
            }
        }
    }

    /// Delete a specific update version.
    /// DELETE https://{endpoint}/deviceupdate/{instanceId}/updates/providers/{provider}/names/{name}/versions/{version}?api-version=2021-06-01-preview
    pub async fn delete_update(
        &mut self,
        instance_id: &str,
        provider: &str,
        name: &str,
        version: &str,
    ) -> Result<String, Error> {
        let mut uri = self.device_update_url.clone();
        let path = format!("deviceupdate/{instance_id}/updates/providers/{provider}/names/{name}/versions/{version}");
        uri.set_path(&path);
        uri.set_query(Some(API_VERSION_PARAM));

        let resp_body = self.delete_authed(uri.to_string()).await?;
        Ok(resp_body)
    }

    /// Get a specific update file from the version.
    /// GET https://{endpoint}/deviceupdate/{instanceId}/updates/providers/{provider}/names/{name}/versions/{version}/files/{fileId}?api-version=2021-06-01-previe
    pub async fn get_file(
        &mut self,
        instance_id: &str,
        provider: &str,
        name: &str,
        version: &str,
        file_id: &str,
    ) -> Result<UpdateFile, Error> {
        let mut uri = self.device_update_url.clone();
        let path = format!("deviceupdate/{instance_id}/updates/providers/{provider}/names/{name}/versions/{version}/files/{file_id}");
        uri.set_path(&path);
        uri.set_query(Some(API_VERSION_PARAM));

        let resp_body = self.get_authed(uri.to_string()).await?;
        let response = serde_json::from_str::<UpdateFile>(&resp_body)?;
        Ok(response)
    }

    /// Retrieve operation status.
    /// GET https://{endpoint}/deviceupdate/{instanceId}/updates/operations/{operationId}?api-version=2021-06-01-preview
    pub async fn get_operation(
        &mut self,
        instance_id: &str,
        operation_id: &str,
    ) -> Result<UpdateOperation, Error> {
        let mut uri = self.device_update_url.clone();
        let path = format!("deviceupdate/{instance_id}/updates/operations/{operation_id}");
        uri.set_path(&path);
        uri.set_query(Some(API_VERSION_PARAM));

        let resp_body = self.get_authed(uri.to_string()).await?;
        let response = serde_json::from_str::<UpdateOperation>(&resp_body)?;
        Ok(response)
    }

    /// Get a specific update version.
    /// GET https://{endpoint}/deviceupdate/{instanceId}/updates/providers/{provider}/names/{name}/versions/{version}?api-version=2021-06-01-preview
    pub async fn get_update(
        &mut self,
        instance_id: &str,
        provider: &str,
        name: &str,
        version: &str,
    ) -> Result<Update, Error> {
        let mut uri = self.device_update_url.clone();
        let path = format!("deviceupdate/{instance_id}/updates/providers/{provider}/names/{name}/versions/{version}");
        uri.set_path(&path);
        uri.set_query(Some(API_VERSION_PARAM));

        let resp_body = self.get_authed(uri.to_string()).await?;
        let response = serde_json::from_str::<Update>(&resp_body)?;
        Ok(response)
    }

    /// Get a list of all update file identifiers for the specified version.
    /// GET https://{endpoint}/deviceupdate/{instanceId}/updates/providers/{provider}/names/{name}/versions/{version}/files?api-version=2021-06-01-preview
    pub async fn list_files(
        &mut self,
        instance_id: &str,
        provider: &str,
        name: &str,
        version: &str,
    ) -> Result<Vec<String>, Error> {
        let mut uri = self.device_update_url.clone();
        let path = format!("deviceupdate/{instance_id}/updates/providers/{provider}/names/{name}/versions/{version}/files");
        uri.set_path(&path);
        uri.set_query(Some(API_VERSION_PARAM));

        let mut all_results: Vec<String> = Vec::new();

        loop {
            let resp_body = self.get_authed(uri.to_string()).await?;
            let mut response = serde_json::from_str::<StringsList>(&resp_body)?;
            all_results.append(&mut response.value);

            match response.next_link {
                None => break,
                Some(url) => {
                    uri = self.device_update_url.clone();
                    uri.set_path("");
                    uri.set_query(None);
                    uri = uri.join(&url)?
                }
            }
        }
        Ok(all_results)
    }

    /// Get a list of all update names that match the specified provider.
    /// GET https://{endpoint}/deviceupdate/{instanceId}/updates/providers/{provider}/names?api-version=2021-06-01-preview
    pub async fn list_names(
        &mut self,
        instance_id: &str,
        provider: &str,
    ) -> Result<Vec<String>, Error> {
        let mut uri = self.device_update_url.clone();
        let path = format!("deviceupdate/{instance_id}/updates/providers/{provider}/names");
        uri.set_path(&path);
        uri.set_query(Some(API_VERSION_PARAM));

        let mut all_results: Vec<String> = Vec::new();

        loop {
            let resp_body = self.get_authed(uri.to_string()).await?;
            let mut response = serde_json::from_str::<StringsList>(&resp_body)?;
            all_results.append(&mut response.value);

            match response.next_link {
                None => break,
                Some(url) => {
                    uri = self.device_update_url.clone();
                    uri.set_path("");
                    uri.set_query(None);
                    uri = uri.join(&url)?
                }
            }
        }
        Ok(all_results)
    }

    /// Get a list of all import update operations.
    /// Completed operations are kept for 7 days before auto-deleted. Delete operations are not returned by this API version.
    /// GET https://{endpoint}/deviceupdate/{instanceId}/updates/operations?$filter={$filter}&$top={$top}&api-version=2021-06-01-preview
    pub async fn list_operations(
        &mut self,
        instance_id: &str,
        filter: Option<&str>,
        top: Option<&str>,
    ) -> Result<Vec<UpdateOperation>, Error> {
        let mut uri = self.device_update_url.clone();
        let path = format!("deviceupdate/{instance_id}/updates/operations");
        let mut params = API_VERSION_PARAM.to_owned();
        match top {
            None => {}
            Some(t) => {
                params = format!("$top={t}&{params}");
            }
        }
        match filter {
            None => {}
            Some(f) => {
                params = format!("$filter={f}&{params}");
            }
        }
        uri.set_path(&path);
        uri.set_query(Some(&params));

        let mut all_results: Vec<UpdateOperation> = Vec::new();

        loop {
            let resp_body = self.get_authed(uri.to_string()).await?;
            let mut response = serde_json::from_str::<UpdateOperationsList>(&resp_body)?;
            all_results.append(&mut response.value);

            match response.next_link {
                None => break,
                Some(url) => {
                    uri = self.device_update_url.clone();
                    uri.set_path("");
                    uri.set_query(None);
                    uri = uri.join(&url)?
                }
            }
        }
        Ok(all_results)
    }

    /// Get a list of all update providers that have been imported to Device Update for IoT Hub.
    /// GET https://{endpoint}/deviceupdate/{instanceId}/updates/providers?api-version=2021-06-01-preview
    pub async fn list_providers(&mut self, instance_id: &str) -> Result<Vec<String>, Error> {
        let mut uri = self.device_update_url.clone();
        let path = format!("deviceupdate/{instance_id}/updates/providers");
        uri.set_path(&path);
        uri.set_query(Some(API_VERSION_PARAM));

        let mut all_results: Vec<String> = Vec::new();

        loop {
            let resp_body = self.get_authed(uri.to_string()).await?;
            let mut response = serde_json::from_str::<StringsList>(&resp_body)?;
            all_results.append(&mut response.value);

            match response.next_link {
                None => break,
                Some(url) => {
                    uri = self.device_update_url.clone();
                    uri.set_path("");
                    uri.set_query(None);
                    uri = uri.join(&url)?
                }
            }
        }
        Ok(all_results)
    }

    /// Get a list of all updates that have been imported to Device Update for IoT Hub.
    /// GET https://{endpoint}/deviceupdate/{instanceId}/updates?api-version=2021-06-01-preview&$search={$search}&$filter={$filter}
    pub async fn list_updates(
        &mut self,
        instance_id: &str,
        filter: Option<&str>,
        search: Option<&str>,
    ) -> Result<Vec<Update>, Error> {
        let mut uri = self.device_update_url.clone();
        let path = format!("deviceupdate/{instance_id}/updates");
        uri.set_path(&path);
        let mut params = API_VERSION_PARAM.to_owned();
        match search {
            None => {}
            Some(s) => {
                params = format!("{params}&$search={s}");
            }
        }
        match filter {
            None => {}
            Some(f) => {
                params = format!("{params}&$filter={f}");
            }
        }
        uri.set_query(Some(&params));

        let mut all_results: Vec<Update> = Vec::new();

        loop {
            let resp_body = self.get_authed(uri.to_string()).await?;
            let mut response = serde_json::from_str::<UpdateList>(&resp_body)?;
            all_results.append(&mut response.value);

            match response.next_link {
                None => break,
                Some(url) => {
                    uri = self.device_update_url.clone();
                    uri.set_path("");
                    uri.set_query(None);
                    uri = uri.join(&url)?
                }
            }
        }
        Ok(all_results)
    }

    /// Get a list of all update versions that match the specified provider and name.
    /// GET https://{endpoint}/deviceupdate/{instanceId}/updates/providers/{provider}/names/{name}/versions?api-version=2021-06-01-preview&$filter={$filter}
    pub async fn list_versions(
        &mut self,
        instance_id: &str,
        provider: &str,
        name: &str,
        filter: Option<&str>,
    ) -> Result<Vec<String>, Error> {
        let mut uri = self.device_update_url.clone();
        let path = format!(
            "deviceupdate/{instance_id}/updates/providers/{provider}/names/{name}/versions"
        );
        uri.set_path(&path);
        let mut params = API_VERSION_PARAM.to_owned();
        match filter {
            None => {}
            Some(f) => {
                params = format!("{params}&$filter={f}");
            }
        }
        uri.set_query(Some(&params));

        let mut all_results: Vec<String> = Vec::new();

        loop {
            let resp_body = self.get_authed(uri.to_string()).await?;
            let mut response = serde_json::from_str::<StringsList>(&resp_body)?;
            all_results.append(&mut response.value);

            match response.next_link {
                None => break,
                Some(url) => {
                    uri = self.device_update_url.clone();
                    uri.set_path("");
                    uri.set_query(None);
                    uri = uri.join(&url)?
                }
            }
        }
        Ok(all_results)
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use mockito::{mock, Matcher};
    use serde_json::json;

    use crate::client::API_VERSION;
    use crate::mock_key_client;
    use crate::tests::MockCredential;

    #[tokio::test]
    async fn can_import_update() {
        let mut op_url = mockito::server_url();
        op_url += "/op_location";
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

        let creds = MockCredential;
        let mut client = mock_key_client!(&"test-du", &creds,);

        let update = client
            .import_update(
                "test-instance",
                r#"{"some":"json","...":"fields"}"#.to_owned(),
            )
            .await
            .unwrap();
        assert_eq!(update.etag, "\"some_tag\"");
        assert_eq!(update.operation_id, "some_op_id");
        assert_eq!(
            update.created_date_time,
            DateTime::parse_from_rfc3339("1999-09-10T21:59:22Z").unwrap()
        );
        assert_eq!(
            update.last_action_date_time,
            DateTime::parse_from_rfc3339("1999-09-10T02:05:07.3845533Z").unwrap()
        );
    }
}
