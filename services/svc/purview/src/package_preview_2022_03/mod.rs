#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
pub mod models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
    options: azure_core::ClientOptions,
}
pub const DEFAULT_ENDPOINT: &str = azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD;
impl ClientBuilder {
    #[doc = "Create a new instance of `ClientBuilder`."]
    #[must_use]
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
            options: azure_core::ClientOptions::default(),
        }
    }
    #[doc = "Set the endpoint."]
    #[must_use]
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    #[doc = "Set the scopes."]
    #[must_use]
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    #[doc = "Set the retry options."]
    #[must_use]
    pub fn retry(mut self, retry: impl Into<azure_core::RetryOptions>) -> Self {
        self.options = self.options.retry(retry);
        self
    }
    #[doc = "Set the transport options."]
    #[must_use]
    pub fn transport(mut self, transport: impl Into<azure_core::TransportOptions>) -> Self {
        self.options = self.options.transport(transport);
        self
    }
    #[doc = "Convert the builder into a `Client` instance."]
    #[must_use]
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes, self.options)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: &mut azure_core::Request) -> azure_core::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        self.pipeline.send(&mut context, request).await
    }
    #[doc = "Create a new `ClientBuilder`."]
    #[must_use]
    pub fn builder(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> ClientBuilder {
        ClientBuilder::new(credential)
    }
    #[doc = "Create a new `Client`."]
    #[must_use]
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
        options: azure_core::ClientOptions,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options,
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn collection_client(&self) -> collection::Client {
        collection::Client(self.clone())
    }
    pub fn discovery_client(&self) -> discovery::Client {
        discovery::Client(self.clone())
    }
    pub fn entity_client(&self) -> entity::Client {
        entity::Client(self.clone())
    }
    pub fn glossary_client(&self) -> glossary::Client {
        glossary::Client(self.clone())
    }
    pub fn lineage_client(&self) -> lineage::Client {
        lineage::Client(self.clone())
    }
    pub fn relationship_client(&self) -> relationship::Client {
        relationship::Client(self.clone())
    }
    pub fn types_client(&self) -> types::Client {
        types::Client(self.clone())
    }
}
pub mod entity {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Create or update an entity in Atlas.\nExisting entity is matched using its unique guid if supplied or by its unique attributes eg: qualifiedName.\nMap and array of collections are not well supported. E.g., array<array<int>>, array<map<string, int>>."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `entity`: Atlas entity with extended information."]
        pub fn create_or_update(&self, entity: impl Into<models::AtlasEntityWithExtInfo>) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                entity: entity.into(),
            }
        }
        #[doc = "List entities in bulk identified by its GUIDs."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: An array of GUIDs of entities to list."]
        pub fn list_by_guids(&self, guid: Vec<String>) -> list_by_guids::Builder {
            list_by_guids::Builder {
                client: self.0.clone(),
                guid,
                min_ext_info: None,
                ignore_relationships: None,
                exclude_relationship_types: Vec::new(),
            }
        }
        #[doc = "Create or update entities in Atlas in bulk.\nExisting entity is matched using its unique guid if supplied or by its unique attributes eg: qualifiedName.\nMap and array of collections are not well supported. E.g., array<array<int>>, array<map<string, int>>."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `entities`: An array of entities to create or update."]
        pub fn create_or_update_entities(
            &self,
            entities: impl Into<models::AtlasEntitiesWithExtInfo>,
        ) -> create_or_update_entities::Builder {
            create_or_update_entities::Builder {
                client: self.0.clone(),
                entities: entities.into(),
            }
        }
        #[doc = "Delete a list of entities in bulk identified by their GUIDs or unique attributes."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: An array of GUIDs of entities to delete."]
        pub fn delete_by_guids(&self, guid: Vec<String>) -> delete_by_guids::Builder {
            delete_by_guids::Builder {
                client: self.0.clone(),
                guid,
            }
        }
        #[doc = "Associate a classification to multiple entities in bulk."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `request`: The request to associate a classification to multiple entities."]
        pub fn add_classification(&self, request: impl Into<models::ClassificationAssociateRequest>) -> add_classification::Builder {
            add_classification::Builder {
                client: self.0.clone(),
                request: request.into(),
            }
        }
        #[doc = "Get complete definition of an entity given its GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the entity."]
        pub fn get_by_guid(&self, guid: impl Into<String>) -> get_by_guid::Builder {
            get_by_guid::Builder {
                client: self.0.clone(),
                guid: guid.into(),
                min_ext_info: None,
                ignore_relationships: None,
            }
        }
        #[doc = "Update entity partially - create or update entity attribute identified by its GUID.\nSupports only primitive attribute type and entity references.\nIt does not support updating complex types like arrays, and maps.\nNull updates are not possible."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the entity."]
        #[doc = "* `name`: The name of the attribute."]
        #[doc = "* `body`: The value of the attribute."]
        pub fn partial_update_entity_attribute_by_guid(
            &self,
            guid: impl Into<String>,
            name: impl Into<String>,
            body: impl Into<serde_json::Value>,
        ) -> partial_update_entity_attribute_by_guid::Builder {
            partial_update_entity_attribute_by_guid::Builder {
                client: self.0.clone(),
                guid: guid.into(),
                name: name.into(),
                body: body.into(),
            }
        }
        #[doc = "Delete an entity identified by its GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the entity."]
        pub fn delete_by_guid(&self, guid: impl Into<String>) -> delete_by_guid::Builder {
            delete_by_guid::Builder {
                client: self.0.clone(),
                guid: guid.into(),
            }
        }
        #[doc = "List classifications for a given entity represented by a GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the entity."]
        #[doc = "* `classification_name`: The name of the classification."]
        pub fn get_classification(&self, guid: impl Into<String>, classification_name: impl Into<String>) -> get_classification::Builder {
            get_classification::Builder {
                client: self.0.clone(),
                guid: guid.into(),
                classification_name: classification_name.into(),
            }
        }
        #[doc = "Delete a given classification from an existing entity represented by a GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the entity."]
        #[doc = "* `classification_name`: The name of the classification."]
        pub fn delete_classification(
            &self,
            guid: impl Into<String>,
            classification_name: impl Into<String>,
        ) -> delete_classification::Builder {
            delete_classification::Builder {
                client: self.0.clone(),
                guid: guid.into(),
                classification_name: classification_name.into(),
            }
        }
        #[doc = "List classifications for a given entity represented by a GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the entity."]
        pub fn get_classifications(&self, guid: impl Into<String>) -> get_classifications::Builder {
            get_classifications::Builder {
                client: self.0.clone(),
                guid: guid.into(),
            }
        }
        #[doc = "Add classifications to an existing entity represented by a GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the entity."]
        #[doc = "* `classifications`: An array of classifications to be added."]
        pub fn add_classifications(
            &self,
            guid: impl Into<String>,
            classifications: Vec<models::AtlasClassification>,
        ) -> add_classifications::Builder {
            add_classifications::Builder {
                client: self.0.clone(),
                guid: guid.into(),
                classifications,
            }
        }
        #[doc = "Update classifications to an existing entity represented by a guid."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the entity."]
        #[doc = "* `classifications`: An array of classifications to be updated."]
        pub fn update_classifications(
            &self,
            guid: impl Into<String>,
            classifications: Vec<models::AtlasClassification>,
        ) -> update_classifications::Builder {
            update_classifications::Builder {
                client: self.0.clone(),
                guid: guid.into(),
                classifications,
            }
        }
        #[doc = "Get complete definition of an entity given its type and unique attribute.\nIn addition to the typeName path parameter, attribute key-value pair(s) can be provided in the following format:\nattr:\\<attrName>=<attrValue>. \nNOTE: The attrName and attrValue should be unique across entities, eg. qualifiedName.\nThe REST request would look something like this:\nGET /v2/entity/uniqueAttribute/type/aType?attr:aTypeAttribute=someValue."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `type_name`: The name of the type."]
        pub fn get_by_unique_attributes(&self, type_name: impl Into<String>) -> get_by_unique_attributes::Builder {
            get_by_unique_attributes::Builder {
                client: self.0.clone(),
                type_name: type_name.into(),
                min_ext_info: None,
                ignore_relationships: None,
                attr_qualified_name: None,
            }
        }
        #[doc = "Update entity partially - Allow a subset of attributes to be updated on\nan entity which is identified by its type and unique attribute  eg: Referenceable.qualifiedName.\nNull updates are not possible.\nIn addition to the typeName path parameter, attribute key-value pair(s) can be provided in the following format:\nattr:<attrName>=<attrValue>.\nNOTE: The attrName and attrValue should be unique across entities, eg. qualifiedName.\nThe REST request would look something like this:\nPUT /v2/entity/uniqueAttribute/type/aType?attr:aTypeAttribute=someValue."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `type_name`: The name of the type."]
        #[doc = "* `atlas_entity_with_ext_info`: Atlas entity with extended information."]
        pub fn partial_update_entity_by_unique_attributes(
            &self,
            type_name: impl Into<String>,
            atlas_entity_with_ext_info: impl Into<models::AtlasEntityWithExtInfo>,
        ) -> partial_update_entity_by_unique_attributes::Builder {
            partial_update_entity_by_unique_attributes::Builder {
                client: self.0.clone(),
                type_name: type_name.into(),
                atlas_entity_with_ext_info: atlas_entity_with_ext_info.into(),
                attr_qualified_name: None,
            }
        }
        #[doc = "Delete an entity identified by its type and unique attributes.\nIn addition to the typeName path parameter, attribute key-value pair(s) can be provided in the following format:\nattr:\\<attrName>=\\<attrValue>.\nNOTE: The attrName and attrValue should be unique across entities, eg. qualifiedName.\nThe REST request would look something like this:\nDELETE /v2/entity/uniqueAttribute/type/aType?attr:aTypeAttribute=someValue."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `type_name`: The name of the type."]
        pub fn delete_by_unique_attribute(&self, type_name: impl Into<String>) -> delete_by_unique_attribute::Builder {
            delete_by_unique_attribute::Builder {
                client: self.0.clone(),
                type_name: type_name.into(),
                attr_qualified_name: None,
            }
        }
        #[doc = "Delete a given classification from an entity identified by its type and unique attributes."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `type_name`: The name of the type."]
        #[doc = "* `classification_name`: The name of the classification."]
        pub fn delete_classification_by_unique_attribute(
            &self,
            type_name: impl Into<String>,
            classification_name: impl Into<String>,
        ) -> delete_classification_by_unique_attribute::Builder {
            delete_classification_by_unique_attribute::Builder {
                client: self.0.clone(),
                type_name: type_name.into(),
                classification_name: classification_name.into(),
                attr_qualified_name: None,
            }
        }
        #[doc = "Add classification to the entity identified by its type and unique attributes."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `type_name`: The name of the type."]
        #[doc = "* `atlas_classification_array`: An array of classification to be added."]
        pub fn add_classifications_by_unique_attribute(
            &self,
            type_name: impl Into<String>,
            atlas_classification_array: Vec<models::AtlasClassification>,
        ) -> add_classifications_by_unique_attribute::Builder {
            add_classifications_by_unique_attribute::Builder {
                client: self.0.clone(),
                type_name: type_name.into(),
                atlas_classification_array,
                attr_qualified_name: None,
            }
        }
        #[doc = "Update classification on an entity identified by its type and unique attributes."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `type_name`: The name of the type."]
        #[doc = "* `atlas_classification_array`: An array of classification to be updated."]
        pub fn update_classifications_by_unique_attribute(
            &self,
            type_name: impl Into<String>,
            atlas_classification_array: Vec<models::AtlasClassification>,
        ) -> update_classifications_by_unique_attribute::Builder {
            update_classifications_by_unique_attribute::Builder {
                client: self.0.clone(),
                type_name: type_name.into(),
                atlas_classification_array,
                attr_qualified_name: None,
            }
        }
        #[doc = "Set classifications on entities in bulk."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `entity_headers`: Atlas entity headers."]
        pub fn set_classifications(&self, entity_headers: impl Into<models::AtlasEntityHeaders>) -> set_classifications::Builder {
            set_classifications::Builder {
                client: self.0.clone(),
                entity_headers: entity_headers.into(),
            }
        }
        #[doc = "Bulk API to retrieve list of entities identified by its unique attributes.\n\nIn addition to the typeName path parameter, attribute key-value pair(s) can be provided in the following format\n\ntypeName=\\<typeName>&attr_1:\\<attrName>=\\<attrValue>&attr_2:\\<attrName>=\\<attrValue>&attr_3:\\<attrName>=\\<attrValue>\n\nNOTE: The attrName should be an unique attribute for the given entity-type\n\nThe REST request would look something like this\n\nGET /v2/entity/bulk/uniqueAttribute/type/hive_db?attr_0:qualifiedName=db1@cl1&attr_2:qualifiedName=db2@cl1"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `type_name`: The name of the type."]
        pub fn get_entities_by_unique_attributes(&self, type_name: impl Into<String>) -> get_entities_by_unique_attributes::Builder {
            get_entities_by_unique_attributes::Builder {
                client: self.0.clone(),
                type_name: type_name.into(),
                min_ext_info: None,
                ignore_relationships: None,
                attr_n_qualified_name: None,
            }
        }
        #[doc = "Get entity header given its GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the entity."]
        pub fn get_header(&self, guid: impl Into<String>) -> get_header::Builder {
            get_header::Builder {
                client: self.0.clone(),
                guid: guid.into(),
            }
        }
        #[doc = "Add business metadata to an entity."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the entity."]
        pub fn add_or_update_business_metadata(&self, guid: impl Into<String>) -> add_or_update_business_metadata::Builder {
            add_or_update_business_metadata::Builder {
                client: self.0.clone(),
                guid: guid.into(),
                is_overwrite: None,
                body: None,
            }
        }
        #[doc = "Remove business metadata from an entity."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the entity."]
        pub fn delete_business_metadata(&self, guid: impl Into<String>) -> delete_business_metadata::Builder {
            delete_business_metadata::Builder {
                client: self.0.clone(),
                guid: guid.into(),
                body: None,
            }
        }
        #[doc = "Add or update business metadata attributes"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `bm_name`: BusinessMetadata name"]
        #[doc = "* `guid`: The globally unique identifier of the entity."]
        pub fn add_or_update_business_metadata_attributes(
            &self,
            bm_name: impl Into<String>,
            guid: impl Into<String>,
        ) -> add_or_update_business_metadata_attributes::Builder {
            add_or_update_business_metadata_attributes::Builder {
                client: self.0.clone(),
                bm_name: bm_name.into(),
                guid: guid.into(),
                body: None,
            }
        }
        #[doc = "Delete business metadata attributes from an entity."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `bm_name`: BusinessMetadata name"]
        #[doc = "* `guid`: The globally unique identifier of the entity."]
        pub fn delete_business_metadata_attributes(
            &self,
            bm_name: impl Into<String>,
            guid: impl Into<String>,
        ) -> delete_business_metadata_attributes::Builder {
            delete_business_metadata_attributes::Builder {
                client: self.0.clone(),
                bm_name: bm_name.into(),
                guid: guid.into(),
                body: None,
            }
        }
        #[doc = "Get the sample Template for uploading/creating bulk BusinessMetaData"]
        pub fn get_sample_business_metadata_template(&self) -> get_sample_business_metadata_template::Builder {
            get_sample_business_metadata_template::Builder { client: self.0.clone() }
        }
        #[doc = "Upload the file for creating Business Metadata in BULK"]
        pub fn import_business_metadata(&self) -> import_business_metadata::Builder {
            import_business_metadata::Builder {
                client: self.0.clone(),
                uploaded_input_stream: None,
            }
        }
        #[doc = "Set labels to a given entity"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the entity."]
        pub fn set_labels(&self, guid: impl Into<String>) -> set_labels::Builder {
            set_labels::Builder {
                client: self.0.clone(),
                guid: guid.into(),
                body: Vec::new(),
            }
        }
        #[doc = "add given labels to a given entity"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the entity."]
        pub fn add_label(&self, guid: impl Into<String>) -> add_label::Builder {
            add_label::Builder {
                client: self.0.clone(),
                guid: guid.into(),
                body: Vec::new(),
            }
        }
        #[doc = "delete given labels to a given entity"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the entity."]
        pub fn delete_labels(&self, guid: impl Into<String>) -> delete_labels::Builder {
            delete_labels::Builder {
                client: self.0.clone(),
                guid: guid.into(),
                body: Vec::new(),
            }
        }
        #[doc = "Set labels to a given entity identified by its type and unique attributes, if labels is null/empty, existing labels will all be removed. In addition to the typeName path parameter, attribute key-value pair(s) can be provided in the following format: attr:<attrName>=<attrValue>. NOTE: The attrName and attrValue should be unique across entities, eg. qualifiedName. The REST request would look something like this: POST /v2/entity/uniqueAttribute/type/aType?attr:aTypeAttribute=someValue."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `type_name`: The name of the type."]
        pub fn set_labels_by_unique_attribute(&self, type_name: impl Into<String>) -> set_labels_by_unique_attribute::Builder {
            set_labels_by_unique_attribute::Builder {
                client: self.0.clone(),
                type_name: type_name.into(),
                attr_qualified_name: None,
                body: Vec::new(),
            }
        }
        #[doc = "Add given labels to a given entity identified by its type and unique attributes, if labels is null/empty, no labels will be added. In addition to the typeName path parameter, attribute key-value pair(s) can be provided in the following format: attr:<attrName>=<attrValue>. NOTE: The attrName and attrValue should be unique across entities, eg. qualifiedName. The REST request would look something like this: PUT /v2/entity/uniqueAttribute/type/aType?attr:aTypeAttribute=someValue."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `type_name`: The name of the type."]
        pub fn add_labels_by_unique_attribute(&self, type_name: impl Into<String>) -> add_labels_by_unique_attribute::Builder {
            add_labels_by_unique_attribute::Builder {
                client: self.0.clone(),
                type_name: type_name.into(),
                attr_qualified_name: None,
                body: Vec::new(),
            }
        }
        #[doc = "Delete given labels to a given entity identified by its type and unique attributes, if labels is null/empty, no labels will be removed. If any labels in labels set are non-existing labels, they will be ignored, only existing labels will be removed. In addition to the typeName path parameter, attribute key-value pair(s) can be provided in the following format: attr:<attrName>=<attrValue>. NOTE: The attrName and attrValue should be unique across entities, eg. qualifiedName. The REST request would look something like this: DELETE /v2/entity/uniqueAttribute/type/aType?attr:aTypeAttribute=someValue. "]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `type_name`: The name of the type."]
        pub fn delete_labels_by_unique_attribute(&self, type_name: impl Into<String>) -> delete_labels_by_unique_attribute::Builder {
            delete_labels_by_unique_attribute::Builder {
                client: self.0.clone(),
                type_name: type_name.into(),
                attr_qualified_name: None,
                body: Vec::new(),
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        type Response = models::EntityMutationResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) entity: models::AtlasEntityWithExtInfo,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/entity", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.entity)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EntityMutationResponse = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_by_guids {
        use super::models;
        type Response = models::AtlasEntitiesWithExtInfo;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: Vec<String>,
            pub(crate) min_ext_info: Option<bool>,
            pub(crate) ignore_relationships: Option<bool>,
            pub(crate) exclude_relationship_types: Vec<String>,
        }
        impl Builder {
            #[doc = "Whether to return minimal information for referred entities."]
            pub fn min_ext_info(mut self, min_ext_info: bool) -> Self {
                self.min_ext_info = Some(min_ext_info);
                self
            }
            #[doc = "Whether to ignore relationship attributes."]
            pub fn ignore_relationships(mut self, ignore_relationships: bool) -> Self {
                self.ignore_relationships = Some(ignore_relationships);
                self
            }
            #[doc = "An array of the relationship types need to be excluded from the response."]
            pub fn exclude_relationship_types(mut self, exclude_relationship_types: Vec<String>) -> Self {
                self.exclude_relationship_types = exclude_relationship_types;
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/entity/bulk", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let guid = &this.guid;
                        for value in &this.guid {
                            req.url_mut().query_pairs_mut().append_pair("guid", &value.to_string());
                        }
                        if let Some(min_ext_info) = &this.min_ext_info {
                            req.url_mut().query_pairs_mut().append_pair("minExtInfo", &min_ext_info.to_string());
                        }
                        if let Some(ignore_relationships) = &this.ignore_relationships {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("ignoreRelationships", &ignore_relationships.to_string());
                        }
                        let exclude_relationship_types = &this.exclude_relationship_types;
                        for value in &this.exclude_relationship_types {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("excludeRelationshipTypes", &value.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasEntitiesWithExtInfo = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod create_or_update_entities {
        use super::models;
        type Response = models::EntityMutationResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) entities: models::AtlasEntitiesWithExtInfo,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/entity/bulk", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.entities)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EntityMutationResponse = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_by_guids {
        use super::models;
        type Response = models::EntityMutationResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: Vec<String>,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/entity/bulk", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let guid = &this.guid;
                        for value in &this.guid {
                            req.url_mut().query_pairs_mut().append_pair("guid", &value.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EntityMutationResponse = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod add_classification {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) request: models::ClassificationAssociateRequest,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/entity/bulk/classification", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.request)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_by_guid {
        use super::models;
        type Response = models::AtlasEntityWithExtInfo;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
            pub(crate) min_ext_info: Option<bool>,
            pub(crate) ignore_relationships: Option<bool>,
        }
        impl Builder {
            #[doc = "Whether to return minimal information for referred entities."]
            pub fn min_ext_info(mut self, min_ext_info: bool) -> Self {
                self.min_ext_info = Some(min_ext_info);
                self
            }
            #[doc = "Whether to ignore relationship attributes."]
            pub fn ignore_relationships(mut self, ignore_relationships: bool) -> Self {
                self.ignore_relationships = Some(ignore_relationships);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/entity/guid/{}", this.client.endpoint(), &this.guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(min_ext_info) = &this.min_ext_info {
                            req.url_mut().query_pairs_mut().append_pair("minExtInfo", &min_ext_info.to_string());
                        }
                        if let Some(ignore_relationships) = &this.ignore_relationships {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("ignoreRelationships", &ignore_relationships.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasEntityWithExtInfo = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod partial_update_entity_attribute_by_guid {
        use super::models;
        type Response = models::EntityMutationResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
            pub(crate) name: String,
            pub(crate) body: serde_json::Value,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/entity/guid/{}", this.client.endpoint(), &this.guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let name = &this.name;
                        req.url_mut().query_pairs_mut().append_pair("name", name);
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EntityMutationResponse = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_by_guid {
        use super::models;
        type Response = models::EntityMutationResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/entity/guid/{}", this.client.endpoint(), &this.guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EntityMutationResponse = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_classification {
        use super::models;
        type Response = models::AtlasClassification;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
            pub(crate) classification_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/entity/guid/{}/classification/{}",
                            this.client.endpoint(),
                            &this.guid,
                            &this.classification_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasClassification = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_classification {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
            pub(crate) classification_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/entity/guid/{}/classification/{}",
                            this.client.endpoint(),
                            &this.guid,
                            &this.classification_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_classifications {
        use super::models;
        type Response = models::AtlasClassifications;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/entity/guid/{}/classifications",
                            this.client.endpoint(),
                            &this.guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasClassifications = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod add_classifications {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
            pub(crate) classifications: Vec<models::AtlasClassification>,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/entity/guid/{}/classifications",
                            this.client.endpoint(),
                            &this.guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.classifications)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod update_classifications {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
            pub(crate) classifications: Vec<models::AtlasClassification>,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/entity/guid/{}/classifications",
                            this.client.endpoint(),
                            &this.guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.classifications)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_by_unique_attributes {
        use super::models;
        type Response = models::AtlasEntityWithExtInfo;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) type_name: String,
            pub(crate) min_ext_info: Option<bool>,
            pub(crate) ignore_relationships: Option<bool>,
            pub(crate) attr_qualified_name: Option<String>,
        }
        impl Builder {
            #[doc = "Whether to return minimal information for referred entities."]
            pub fn min_ext_info(mut self, min_ext_info: bool) -> Self {
                self.min_ext_info = Some(min_ext_info);
                self
            }
            #[doc = "Whether to ignore relationship attributes."]
            pub fn ignore_relationships(mut self, ignore_relationships: bool) -> Self {
                self.ignore_relationships = Some(ignore_relationships);
                self
            }
            #[doc = "The qualified name of the entity."]
            pub fn attr_qualified_name(mut self, attr_qualified_name: impl Into<String>) -> Self {
                self.attr_qualified_name = Some(attr_qualified_name.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/entity/uniqueAttribute/type/{}",
                            this.client.endpoint(),
                            &this.type_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(min_ext_info) = &this.min_ext_info {
                            req.url_mut().query_pairs_mut().append_pair("minExtInfo", &min_ext_info.to_string());
                        }
                        if let Some(ignore_relationships) = &this.ignore_relationships {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("ignoreRelationships", &ignore_relationships.to_string());
                        }
                        if let Some(attr_qualified_name) = &this.attr_qualified_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("attr:qualifiedName", attr_qualified_name);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasEntityWithExtInfo = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod partial_update_entity_by_unique_attributes {
        use super::models;
        type Response = models::EntityMutationResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) type_name: String,
            pub(crate) atlas_entity_with_ext_info: models::AtlasEntityWithExtInfo,
            pub(crate) attr_qualified_name: Option<String>,
        }
        impl Builder {
            #[doc = "The qualified name of the entity."]
            pub fn attr_qualified_name(mut self, attr_qualified_name: impl Into<String>) -> Self {
                self.attr_qualified_name = Some(attr_qualified_name.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/entity/uniqueAttribute/type/{}",
                            this.client.endpoint(),
                            &this.type_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(attr_qualified_name) = &this.attr_qualified_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("attr:qualifiedName", attr_qualified_name);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.atlas_entity_with_ext_info)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EntityMutationResponse = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_by_unique_attribute {
        use super::models;
        type Response = models::EntityMutationResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) type_name: String,
            pub(crate) attr_qualified_name: Option<String>,
        }
        impl Builder {
            #[doc = "The qualified name of the entity."]
            pub fn attr_qualified_name(mut self, attr_qualified_name: impl Into<String>) -> Self {
                self.attr_qualified_name = Some(attr_qualified_name.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/entity/uniqueAttribute/type/{}",
                            this.client.endpoint(),
                            &this.type_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(attr_qualified_name) = &this.attr_qualified_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("attr:qualifiedName", attr_qualified_name);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EntityMutationResponse = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_classification_by_unique_attribute {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) type_name: String,
            pub(crate) classification_name: String,
            pub(crate) attr_qualified_name: Option<String>,
        }
        impl Builder {
            #[doc = "The qualified name of the entity."]
            pub fn attr_qualified_name(mut self, attr_qualified_name: impl Into<String>) -> Self {
                self.attr_qualified_name = Some(attr_qualified_name.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/entity/uniqueAttribute/type/{}/classification/{}",
                            this.client.endpoint(),
                            &this.type_name,
                            &this.classification_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(attr_qualified_name) = &this.attr_qualified_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("attr:qualifiedName", attr_qualified_name);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod add_classifications_by_unique_attribute {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) type_name: String,
            pub(crate) atlas_classification_array: Vec<models::AtlasClassification>,
            pub(crate) attr_qualified_name: Option<String>,
        }
        impl Builder {
            #[doc = "The qualified name of the entity."]
            pub fn attr_qualified_name(mut self, attr_qualified_name: impl Into<String>) -> Self {
                self.attr_qualified_name = Some(attr_qualified_name.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/entity/uniqueAttribute/type/{}/classifications",
                            this.client.endpoint(),
                            &this.type_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(attr_qualified_name) = &this.attr_qualified_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("attr:qualifiedName", attr_qualified_name);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.atlas_classification_array)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod update_classifications_by_unique_attribute {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) type_name: String,
            pub(crate) atlas_classification_array: Vec<models::AtlasClassification>,
            pub(crate) attr_qualified_name: Option<String>,
        }
        impl Builder {
            #[doc = "The qualified name of the entity."]
            pub fn attr_qualified_name(mut self, attr_qualified_name: impl Into<String>) -> Self {
                self.attr_qualified_name = Some(attr_qualified_name.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/entity/uniqueAttribute/type/{}/classifications",
                            this.client.endpoint(),
                            &this.type_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(attr_qualified_name) = &this.attr_qualified_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("attr:qualifiedName", attr_qualified_name);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.atlas_classification_array)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod set_classifications {
        use super::models;
        type Response = Vec<String>;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) entity_headers: models::AtlasEntityHeaders,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/entity/bulk/setClassifications", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.entity_headers)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: Vec<String> = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_entities_by_unique_attributes {
        use super::models;
        type Response = models::AtlasEntitiesWithExtInfo;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) type_name: String,
            pub(crate) min_ext_info: Option<bool>,
            pub(crate) ignore_relationships: Option<bool>,
            pub(crate) attr_n_qualified_name: Option<String>,
        }
        impl Builder {
            #[doc = "Whether to return minimal information for referred entities."]
            pub fn min_ext_info(mut self, min_ext_info: bool) -> Self {
                self.min_ext_info = Some(min_ext_info);
                self
            }
            #[doc = "Whether to ignore relationship attributes."]
            pub fn ignore_relationships(mut self, ignore_relationships: bool) -> Self {
                self.ignore_relationships = Some(ignore_relationships);
                self
            }
            #[doc = "Qualified name of an entity. E.g. to find 2 entities you can set attrs_0:qualifiedName=db1@cl1&attrs_2:qualifiedName=db2@cl1"]
            pub fn attr_n_qualified_name(mut self, attr_n_qualified_name: impl Into<String>) -> Self {
                self.attr_n_qualified_name = Some(attr_n_qualified_name.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/entity/bulk/uniqueAttribute/type/{}",
                            this.client.endpoint(),
                            &this.type_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(min_ext_info) = &this.min_ext_info {
                            req.url_mut().query_pairs_mut().append_pair("minExtInfo", &min_ext_info.to_string());
                        }
                        if let Some(ignore_relationships) = &this.ignore_relationships {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("ignoreRelationships", &ignore_relationships.to_string());
                        }
                        if let Some(attr_n_qualified_name) = &this.attr_n_qualified_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("attr_N:qualifiedName", attr_n_qualified_name);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasEntitiesWithExtInfo = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_header {
        use super::models;
        type Response = models::AtlasEntityHeader;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/atlas/v2/entity/guid/{}/header", this.client.endpoint(), &this.guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasEntityHeader = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod add_or_update_business_metadata {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
            pub(crate) is_overwrite: Option<bool>,
            pub(crate) body: Option<serde_json::Value>,
        }
        impl Builder {
            #[doc = "Whether to overwrite the existing business metadata on the entity or not, default is false."]
            pub fn is_overwrite(mut self, is_overwrite: bool) -> Self {
                self.is_overwrite = Some(is_overwrite);
                self
            }
            #[doc = "Business Metadata"]
            pub fn body(mut self, body: impl Into<serde_json::Value>) -> Self {
                self.body = Some(body.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/entity/guid/{}/businessmetadata",
                            this.client.endpoint(),
                            &this.guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(is_overwrite) = &this.is_overwrite {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("isOverwrite", &is_overwrite.to_string());
                        }
                        let req_body = if let Some(body) = &this.body {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(body)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_business_metadata {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
            pub(crate) body: Option<serde_json::Value>,
        }
        impl Builder {
            #[doc = "BusinessMetadata"]
            pub fn body(mut self, body: impl Into<serde_json::Value>) -> Self {
                self.body = Some(body.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/entity/guid/{}/businessmetadata",
                            this.client.endpoint(),
                            &this.guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = if let Some(body) = &this.body {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(body)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod add_or_update_business_metadata_attributes {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) bm_name: String,
            pub(crate) guid: String,
            pub(crate) body: Option<serde_json::Value>,
        }
        impl Builder {
            #[doc = "BusinessMetadataAttributes"]
            pub fn body(mut self, body: impl Into<serde_json::Value>) -> Self {
                self.body = Some(body.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/entity/guid/{}/businessmetadata/{}",
                            this.client.endpoint(),
                            &this.guid,
                            &this.bm_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = if let Some(body) = &this.body {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(body)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_business_metadata_attributes {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) bm_name: String,
            pub(crate) guid: String,
            pub(crate) body: Option<serde_json::Value>,
        }
        impl Builder {
            #[doc = "BusinessMetadataAttributes"]
            pub fn body(mut self, body: impl Into<serde_json::Value>) -> Self {
                self.body = Some(body.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/entity/guid/{}/businessmetadata/{}",
                            this.client.endpoint(),
                            &this.guid,
                            &this.bm_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = if let Some(body) = &this.body {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(body)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_sample_business_metadata_template {
        use super::models;
        type Response = bytes::Bytes;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/entity/businessmetadata/import/template",
                            this.client.endpoint(),
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value = rsp_body;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod import_business_metadata {
        use super::models;
        type Response = models::BulkImportResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) uploaded_input_stream: Option<bytes::Bytes>,
        }
        impl Builder {
            #[doc = "InputStream of file"]
            pub fn uploaded_input_stream(mut self, uploaded_input_stream: impl Into<bytes::Bytes>) -> Self {
                self.uploaded_input_stream = Some(uploaded_input_stream.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/entity/businessmetadata/import", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        unimplemented!("form data not yet supported");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BulkImportResponse = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod set_labels {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
            pub(crate) body: Vec<String>,
        }
        impl Builder {
            #[doc = "set of labels to be set to the entity"]
            pub fn body(mut self, body: Vec<String>) -> Self {
                self.body = body;
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/atlas/v2/entity/guid/{}/labels", this.client.endpoint(), &this.guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod add_label {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
            pub(crate) body: Vec<String>,
        }
        impl Builder {
            #[doc = "set of labels to be added"]
            pub fn body(mut self, body: Vec<String>) -> Self {
                self.body = body;
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/atlas/v2/entity/guid/{}/labels", this.client.endpoint(), &this.guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_labels {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
            pub(crate) body: Vec<String>,
        }
        impl Builder {
            #[doc = "set of labels to be deleted"]
            pub fn body(mut self, body: Vec<String>) -> Self {
                self.body = body;
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/atlas/v2/entity/guid/{}/labels", this.client.endpoint(), &this.guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod set_labels_by_unique_attribute {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) type_name: String,
            pub(crate) attr_qualified_name: Option<String>,
            pub(crate) body: Vec<String>,
        }
        impl Builder {
            #[doc = "The qualified name of the entity"]
            pub fn attr_qualified_name(mut self, attr_qualified_name: impl Into<String>) -> Self {
                self.attr_qualified_name = Some(attr_qualified_name.into());
                self
            }
            #[doc = "set of labels to be set"]
            pub fn body(mut self, body: Vec<String>) -> Self {
                self.body = body;
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/entity/uniqueAttribute/type/{}/labels",
                            this.client.endpoint(),
                            &this.type_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(attr_qualified_name) = &this.attr_qualified_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("attr:qualifiedName", attr_qualified_name);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod add_labels_by_unique_attribute {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) type_name: String,
            pub(crate) attr_qualified_name: Option<String>,
            pub(crate) body: Vec<String>,
        }
        impl Builder {
            #[doc = "The qualified name of the entity"]
            pub fn attr_qualified_name(mut self, attr_qualified_name: impl Into<String>) -> Self {
                self.attr_qualified_name = Some(attr_qualified_name.into());
                self
            }
            #[doc = "set of labels to be added"]
            pub fn body(mut self, body: Vec<String>) -> Self {
                self.body = body;
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/entity/uniqueAttribute/type/{}/labels",
                            this.client.endpoint(),
                            &this.type_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(attr_qualified_name) = &this.attr_qualified_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("attr:qualifiedName", attr_qualified_name);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_labels_by_unique_attribute {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) type_name: String,
            pub(crate) attr_qualified_name: Option<String>,
            pub(crate) body: Vec<String>,
        }
        impl Builder {
            #[doc = "The qualified name of the entity"]
            pub fn attr_qualified_name(mut self, attr_qualified_name: impl Into<String>) -> Self {
                self.attr_qualified_name = Some(attr_qualified_name.into());
                self
            }
            #[doc = "set of labels to be deleted"]
            pub fn body(mut self, body: Vec<String>) -> Self {
                self.body = body;
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/entity/uniqueAttribute/type/{}/labels",
                            this.client.endpoint(),
                            &this.type_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(attr_qualified_name) = &this.attr_qualified_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("attr:qualifiedName", attr_qualified_name);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
}
pub mod glossary {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get all glossaries registered with Atlas."]
        pub fn list_glossaries(&self) -> list_glossaries::Builder {
            list_glossaries::Builder {
                client: self.0.clone(),
                limit: None,
                offset: None,
                sort: None,
                ignore_terms_and_categories: None,
            }
        }
        #[doc = "Create a glossary."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `atlas_glossary`: Glossary definition, terms & categories can be anchored to a glossary.\nUsing the anchor attribute when creating the Term/Category."]
        pub fn create_glossary(&self, atlas_glossary: impl Into<models::AtlasGlossary>) -> create_glossary::Builder {
            create_glossary::Builder {
                client: self.0.clone(),
                atlas_glossary: atlas_glossary.into(),
            }
        }
        #[doc = "Create glossary category in bulk."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `glossary_category`: An array of glossary category definitions to be created."]
        pub fn create_glossary_categories(
            &self,
            glossary_category: Vec<models::AtlasGlossaryCategory>,
        ) -> create_glossary_categories::Builder {
            create_glossary_categories::Builder {
                client: self.0.clone(),
                glossary_category,
            }
        }
        #[doc = "Create a glossary category."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `glossary_category`: The glossary category definition. A category must be anchored to a Glossary when creating.\nOptionally, terms belonging to the category and the hierarchy can also be defined during creation."]
        pub fn create_glossary_category(
            &self,
            glossary_category: impl Into<models::AtlasGlossaryCategory>,
        ) -> create_glossary_category::Builder {
            create_glossary_category::Builder {
                client: self.0.clone(),
                glossary_category: glossary_category.into(),
            }
        }
        #[doc = "Get specific glossary category by its GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `category_guid`: The globally unique identifier of the category."]
        pub fn get_glossary_category(&self, category_guid: impl Into<String>) -> get_glossary_category::Builder {
            get_glossary_category::Builder {
                client: self.0.clone(),
                category_guid: category_guid.into(),
            }
        }
        #[doc = "Update the given glossary category by its GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `category_guid`: The globally unique identifier of the category."]
        #[doc = "* `glossary_category`: The glossary category to be updated."]
        pub fn update_glossary_category(
            &self,
            category_guid: impl Into<String>,
            glossary_category: impl Into<models::AtlasGlossaryCategory>,
        ) -> update_glossary_category::Builder {
            update_glossary_category::Builder {
                client: self.0.clone(),
                category_guid: category_guid.into(),
                glossary_category: glossary_category.into(),
            }
        }
        #[doc = "Delete a glossary category."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `category_guid`: The globally unique identifier of the category."]
        pub fn delete_glossary_category(&self, category_guid: impl Into<String>) -> delete_glossary_category::Builder {
            delete_glossary_category::Builder {
                client: self.0.clone(),
                category_guid: category_guid.into(),
            }
        }
        #[doc = "Update the glossary category partially."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `category_guid`: The globally unique identifier of the category."]
        #[doc = "* `partial_updates`: A map containing keys as attribute names and values as corresponding attribute values for partial update."]
        pub fn partial_update_glossary_category(
            &self,
            category_guid: impl Into<String>,
            partial_updates: impl Into<serde_json::Value>,
        ) -> partial_update_glossary_category::Builder {
            partial_update_glossary_category::Builder {
                client: self.0.clone(),
                category_guid: category_guid.into(),
                partial_updates: partial_updates.into(),
            }
        }
        #[doc = "Get all related categories (parent and children). Limit, offset, and sort parameters are currently not being enabled and won't work even they are passed."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `category_guid`: The globally unique identifier of the category."]
        pub fn list_related_categories(&self, category_guid: impl Into<String>) -> list_related_categories::Builder {
            list_related_categories::Builder {
                client: self.0.clone(),
                category_guid: category_guid.into(),
                limit: None,
                offset: None,
                sort: None,
            }
        }
        #[doc = "Get all terms associated with the specific category."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `category_guid`: The globally unique identifier of the category."]
        pub fn list_category_terms(&self, category_guid: impl Into<String>) -> list_category_terms::Builder {
            list_category_terms::Builder {
                client: self.0.clone(),
                category_guid: category_guid.into(),
                limit: None,
                offset: None,
                sort: None,
            }
        }
        #[doc = "Create a glossary term."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `glossary_term`: The glossary term definition. A term must be anchored to a Glossary at the time of creation.\nOptionally it can be categorized as well."]
        pub fn create_glossary_term(&self, glossary_term: impl Into<models::AtlasGlossaryTerm>) -> create_glossary_term::Builder {
            create_glossary_term::Builder {
                client: self.0.clone(),
                glossary_term: glossary_term.into(),
                include_term_hierarchy: None,
            }
        }
        #[doc = "Get a specific glossary term by its GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `term_guid`: The globally unique identifier for glossary term."]
        pub fn get_glossary_term(&self, term_guid: impl Into<String>) -> get_glossary_term::Builder {
            get_glossary_term::Builder {
                client: self.0.clone(),
                term_guid: term_guid.into(),
                include_term_hierarchy: None,
                exclude_relationship_types: Vec::new(),
            }
        }
        #[doc = "Update the given glossary term by its GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `term_guid`: The globally unique identifier for glossary term."]
        #[doc = "* `glossary_term`: The glossary term to be updated."]
        pub fn update_glossary_term(
            &self,
            term_guid: impl Into<String>,
            glossary_term: impl Into<models::AtlasGlossaryTerm>,
        ) -> update_glossary_term::Builder {
            update_glossary_term::Builder {
                client: self.0.clone(),
                term_guid: term_guid.into(),
                glossary_term: glossary_term.into(),
                include_term_hierarchy: None,
            }
        }
        #[doc = "Delete a glossary term."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `term_guid`: The globally unique identifier for glossary term."]
        pub fn delete_glossary_term(&self, term_guid: impl Into<String>) -> delete_glossary_term::Builder {
            delete_glossary_term::Builder {
                client: self.0.clone(),
                term_guid: term_guid.into(),
            }
        }
        #[doc = "Update the glossary term partially."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `term_guid`: The globally unique identifier for glossary term."]
        #[doc = "* `partial_updates`: A map containing keys as attribute names and values as corresponding attribute values to be updated."]
        pub fn partial_update_glossary_term(
            &self,
            term_guid: impl Into<String>,
            partial_updates: impl Into<serde_json::Value>,
        ) -> partial_update_glossary_term::Builder {
            partial_update_glossary_term::Builder {
                client: self.0.clone(),
                term_guid: term_guid.into(),
                partial_updates: partial_updates.into(),
                include_term_hierarchy: None,
            }
        }
        #[doc = "Create glossary terms in bulk."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `glossary_term`: An array of glossary term definitions to be created in bulk."]
        pub fn create_glossary_terms(&self, glossary_term: Vec<models::AtlasGlossaryTerm>) -> create_glossary_terms::Builder {
            create_glossary_terms::Builder {
                client: self.0.clone(),
                glossary_term,
                include_term_hierarchy: None,
            }
        }
        #[doc = "Get all related objects assigned with the specified term."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `term_guid`: The globally unique identifier for glossary term."]
        pub fn get_entities_assigned_with_term(&self, term_guid: impl Into<String>) -> get_entities_assigned_with_term::Builder {
            get_entities_assigned_with_term::Builder {
                client: self.0.clone(),
                term_guid: term_guid.into(),
                limit: None,
                offset: None,
                sort: None,
            }
        }
        #[doc = "Assign the given term to the provided list of related objects."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `term_guid`: The globally unique identifier for glossary term."]
        #[doc = "* `related_object_ids`: An array of related object IDs to which the term has to be associated."]
        pub fn assign_term_to_entities(
            &self,
            term_guid: impl Into<String>,
            related_object_ids: Vec<models::AtlasRelatedObjectId>,
        ) -> assign_term_to_entities::Builder {
            assign_term_to_entities::Builder {
                client: self.0.clone(),
                term_guid: term_guid.into(),
                related_object_ids,
            }
        }
        #[doc = "Delete the term assignment for the given list of related objects."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `term_guid`: The globally unique identifier for glossary term."]
        #[doc = "* `related_object_ids`: An array of related object IDs from which the term has to be dissociated."]
        pub fn remove_term_assignment_from_entities(
            &self,
            term_guid: impl Into<String>,
            related_object_ids: Vec<models::AtlasRelatedObjectId>,
        ) -> remove_term_assignment_from_entities::Builder {
            remove_term_assignment_from_entities::Builder {
                client: self.0.clone(),
                term_guid: term_guid.into(),
                related_object_ids,
            }
        }
        #[doc = "Delete the term assignment for the given list of related objects."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `term_guid`: The globally unique identifier for glossary term."]
        #[doc = "* `related_object_ids`: An array of related object IDs from which the term has to be dissociated."]
        pub fn delete_term_assignment_from_entities(
            &self,
            term_guid: impl Into<String>,
            related_object_ids: Vec<models::AtlasRelatedObjectId>,
        ) -> delete_term_assignment_from_entities::Builder {
            delete_term_assignment_from_entities::Builder {
                client: self.0.clone(),
                term_guid: term_guid.into(),
                related_object_ids,
            }
        }
        #[doc = "Get all related terms for a specific term by its GUID. Limit, offset, and sort parameters are currently not being enabled and won't work even they are passed."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `term_guid`: The globally unique identifier for glossary term."]
        pub fn list_related_terms(&self, term_guid: impl Into<String>) -> list_related_terms::Builder {
            list_related_terms::Builder {
                client: self.0.clone(),
                term_guid: term_guid.into(),
                limit: None,
                offset: None,
                sort: None,
            }
        }
        #[doc = "Get a specific Glossary by its GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `glossary_guid`: The globally unique identifier for glossary."]
        pub fn get_glossary(&self, glossary_guid: impl Into<String>) -> get_glossary::Builder {
            get_glossary::Builder {
                client: self.0.clone(),
                glossary_guid: glossary_guid.into(),
            }
        }
        #[doc = "Update the given glossary."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `glossary_guid`: The globally unique identifier for glossary."]
        #[doc = "* `updated_glossary`: The glossary definition to be updated."]
        pub fn update_glossary(
            &self,
            glossary_guid: impl Into<String>,
            updated_glossary: impl Into<models::AtlasGlossary>,
        ) -> update_glossary::Builder {
            update_glossary::Builder {
                client: self.0.clone(),
                glossary_guid: glossary_guid.into(),
                updated_glossary: updated_glossary.into(),
            }
        }
        #[doc = "Delete a glossary."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `glossary_guid`: The globally unique identifier for glossary."]
        pub fn delete_glossary(&self, glossary_guid: impl Into<String>) -> delete_glossary::Builder {
            delete_glossary::Builder {
                client: self.0.clone(),
                glossary_guid: glossary_guid.into(),
            }
        }
        #[doc = "Get the categories belonging to a specific glossary."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `glossary_guid`: The globally unique identifier for glossary."]
        pub fn list_glossary_categories(&self, glossary_guid: impl Into<String>) -> list_glossary_categories::Builder {
            list_glossary_categories::Builder {
                client: self.0.clone(),
                glossary_guid: glossary_guid.into(),
                limit: None,
                offset: None,
                sort: None,
            }
        }
        #[doc = "Get the category headers belonging to a specific glossary."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `glossary_guid`: The globally unique identifier for glossary."]
        pub fn list_glossary_categories_headers(&self, glossary_guid: impl Into<String>) -> list_glossary_categories_headers::Builder {
            list_glossary_categories_headers::Builder {
                client: self.0.clone(),
                glossary_guid: glossary_guid.into(),
                limit: None,
                offset: None,
                sort: None,
            }
        }
        #[doc = "Get a specific glossary with detailed information."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `glossary_guid`: The globally unique identifier for glossary."]
        pub fn get_detailed_glossary(&self, glossary_guid: impl Into<String>) -> get_detailed_glossary::Builder {
            get_detailed_glossary::Builder {
                client: self.0.clone(),
                glossary_guid: glossary_guid.into(),
                include_term_hierarchy: None,
            }
        }
        #[doc = "Update the glossary partially. Some properties such as qualifiedName are not allowed to be updated."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `glossary_guid`: The globally unique identifier for glossary."]
        #[doc = "* `partial_updates`: A map containing keys as attribute names and values as corresponding attribute values."]
        pub fn partial_update_glossary(
            &self,
            glossary_guid: impl Into<String>,
            partial_updates: impl Into<serde_json::Value>,
        ) -> partial_update_glossary::Builder {
            partial_update_glossary::Builder {
                client: self.0.clone(),
                glossary_guid: glossary_guid.into(),
                partial_updates: partial_updates.into(),
                include_term_hierarchy: None,
            }
        }
        #[doc = "Get terms belonging to a specific glossary."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `glossary_guid`: The globally unique identifier for glossary."]
        pub fn list_glossary_terms(&self, glossary_guid: impl Into<String>) -> list_glossary_terms::Builder {
            list_glossary_terms::Builder {
                client: self.0.clone(),
                glossary_guid: glossary_guid.into(),
                include_term_hierarchy: None,
                limit: None,
                offset: None,
                sort: None,
            }
        }
        #[doc = "Get term headers belonging to a specific glossary."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `glossary_guid`: The globally unique identifier for glossary."]
        pub fn list_glossary_term_headers(&self, glossary_guid: impl Into<String>) -> list_glossary_term_headers::Builder {
            list_glossary_term_headers::Builder {
                client: self.0.clone(),
                glossary_guid: glossary_guid.into(),
                limit: None,
                offset: None,
                sort: None,
            }
        }
        #[doc = "Import Glossary Terms from local csv file"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `glossary_guid`: The globally unique identifier for glossary."]
        #[doc = "* `file`: The csv file to import glossary terms from."]
        pub fn import_glossary_terms_via_csv(
            &self,
            glossary_guid: impl Into<String>,
            file: impl Into<bytes::Bytes>,
        ) -> import_glossary_terms_via_csv::Builder {
            import_glossary_terms_via_csv::Builder {
                client: self.0.clone(),
                glossary_guid: glossary_guid.into(),
                file: file.into(),
                include_term_hierarchy: None,
            }
        }
        #[doc = "Import Glossary Terms from local csv file by glossaryName"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `glossary_name`: The name of the glossary."]
        #[doc = "* `file`: The csv file to import glossary terms from."]
        pub fn import_glossary_terms_via_csv_by_glossary_name(
            &self,
            glossary_name: impl Into<String>,
            file: impl Into<bytes::Bytes>,
        ) -> import_glossary_terms_via_csv_by_glossary_name::Builder {
            import_glossary_terms_via_csv_by_glossary_name::Builder {
                client: self.0.clone(),
                glossary_name: glossary_name.into(),
                file: file.into(),
                include_term_hierarchy: None,
            }
        }
        #[doc = "Get the status of import csv operation"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `operation_guid`: The globally unique identifier for async operation/job."]
        pub fn get_import_csv_operation_status(&self, operation_guid: impl Into<String>) -> get_import_csv_operation_status::Builder {
            get_import_csv_operation_status::Builder {
                client: self.0.clone(),
                operation_guid: operation_guid.into(),
            }
        }
        #[doc = "Export Glossary Terms as csv file"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `glossary_guid`: The globally unique identifier for glossary."]
        #[doc = "* `term_guids`: An array of term guids."]
        pub fn export_glossary_terms_as_csv(
            &self,
            glossary_guid: impl Into<String>,
            term_guids: Vec<models::TermGuid>,
        ) -> export_glossary_terms_as_csv::Builder {
            export_glossary_terms_as_csv::Builder {
                client: self.0.clone(),
                glossary_guid: glossary_guid.into(),
                term_guids,
                include_term_hierarchy: None,
            }
        }
        #[doc = "Get terms by glossary name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `glossary_name`: The name of the glossary."]
        pub fn list_terms_by_glossary_name(&self, glossary_name: impl Into<String>) -> list_terms_by_glossary_name::Builder {
            list_terms_by_glossary_name::Builder {
                client: self.0.clone(),
                glossary_name: glossary_name.into(),
                limit: None,
                offset: None,
                include_term_hierarchy: None,
            }
        }
    }
    pub mod list_glossaries {
        use super::models;
        type Response = Vec<models::AtlasGlossary>;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) limit: Option<i32>,
            pub(crate) offset: Option<i32>,
            pub(crate) sort: Option<String>,
            pub(crate) ignore_terms_and_categories: Option<bool>,
        }
        impl Builder {
            #[doc = "The page size - by default there is no paging."]
            pub fn limit(mut self, limit: i32) -> Self {
                self.limit = Some(limit);
                self
            }
            #[doc = "The offset for pagination purpose."]
            pub fn offset(mut self, offset: i32) -> Self {
                self.offset = Some(offset);
                self
            }
            #[doc = "The sort order, ASC (default) or DESC."]
            pub fn sort(mut self, sort: impl Into<String>) -> Self {
                self.sort = Some(sort.into());
                self
            }
            #[doc = "Whether ignore terms and categories"]
            pub fn ignore_terms_and_categories(mut self, ignore_terms_and_categories: bool) -> Self {
                self.ignore_terms_and_categories = Some(ignore_terms_and_categories);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/glossary", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(limit) = &this.limit {
                            req.url_mut().query_pairs_mut().append_pair("limit", &limit.to_string());
                        }
                        if let Some(offset) = &this.offset {
                            req.url_mut().query_pairs_mut().append_pair("offset", &offset.to_string());
                        }
                        if let Some(sort) = &this.sort {
                            req.url_mut().query_pairs_mut().append_pair("sort", sort);
                        }
                        if let Some(ignore_terms_and_categories) = &this.ignore_terms_and_categories {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("ignoreTermsAndCategories", &ignore_terms_and_categories.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: Vec<models::AtlasGlossary> = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod create_glossary {
        use super::models;
        type Response = models::AtlasGlossary;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) atlas_glossary: models::AtlasGlossary,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/glossary", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.atlas_glossary)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasGlossary = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod create_glossary_categories {
        use super::models;
        type Response = Vec<models::AtlasGlossaryCategory>;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) glossary_category: Vec<models::AtlasGlossaryCategory>,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/glossary/categories", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.glossary_category)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: Vec<models::AtlasGlossaryCategory> = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod create_glossary_category {
        use super::models;
        type Response = models::AtlasGlossaryCategory;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) glossary_category: models::AtlasGlossaryCategory,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/glossary/category", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.glossary_category)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasGlossaryCategory = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_glossary_category {
        use super::models;
        type Response = models::AtlasGlossaryCategory;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) category_guid: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/glossary/category/{}",
                            this.client.endpoint(),
                            &this.category_guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasGlossaryCategory = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod update_glossary_category {
        use super::models;
        type Response = models::AtlasGlossaryCategory;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) category_guid: String,
            pub(crate) glossary_category: models::AtlasGlossaryCategory,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/glossary/category/{}",
                            this.client.endpoint(),
                            &this.category_guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.glossary_category)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasGlossaryCategory = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_glossary_category {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) category_guid: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/glossary/category/{}",
                            this.client.endpoint(),
                            &this.category_guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod partial_update_glossary_category {
        use super::models;
        type Response = models::AtlasGlossaryCategory;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) category_guid: String,
            pub(crate) partial_updates: serde_json::Value,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/glossary/category/{}/partial",
                            this.client.endpoint(),
                            &this.category_guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.partial_updates)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasGlossaryCategory = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_related_categories {
        use super::models;
        type Response = serde_json::Value;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) category_guid: String,
            pub(crate) limit: Option<i32>,
            pub(crate) offset: Option<i32>,
            pub(crate) sort: Option<String>,
        }
        impl Builder {
            #[doc = "The page size - by default there is no paging."]
            pub fn limit(mut self, limit: i32) -> Self {
                self.limit = Some(limit);
                self
            }
            #[doc = "The offset for pagination purpose."]
            pub fn offset(mut self, offset: i32) -> Self {
                self.offset = Some(offset);
                self
            }
            #[doc = "The sort order, ASC (default) or DESC."]
            pub fn sort(mut self, sort: impl Into<String>) -> Self {
                self.sort = Some(sort.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/glossary/category/{}/related",
                            this.client.endpoint(),
                            &this.category_guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(limit) = &this.limit {
                            req.url_mut().query_pairs_mut().append_pair("limit", &limit.to_string());
                        }
                        if let Some(offset) = &this.offset {
                            req.url_mut().query_pairs_mut().append_pair("offset", &offset.to_string());
                        }
                        if let Some(sort) = &this.sort {
                            req.url_mut().query_pairs_mut().append_pair("sort", sort);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: serde_json::Value = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_category_terms {
        use super::models;
        type Response = Vec<models::AtlasRelatedTermHeader>;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) category_guid: String,
            pub(crate) limit: Option<i32>,
            pub(crate) offset: Option<i32>,
            pub(crate) sort: Option<String>,
        }
        impl Builder {
            #[doc = "The page size - by default there is no paging."]
            pub fn limit(mut self, limit: i32) -> Self {
                self.limit = Some(limit);
                self
            }
            #[doc = "The offset for pagination purpose."]
            pub fn offset(mut self, offset: i32) -> Self {
                self.offset = Some(offset);
                self
            }
            #[doc = "The sort order, ASC (default) or DESC."]
            pub fn sort(mut self, sort: impl Into<String>) -> Self {
                self.sort = Some(sort.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/glossary/category/{}/terms",
                            this.client.endpoint(),
                            &this.category_guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(limit) = &this.limit {
                            req.url_mut().query_pairs_mut().append_pair("limit", &limit.to_string());
                        }
                        if let Some(offset) = &this.offset {
                            req.url_mut().query_pairs_mut().append_pair("offset", &offset.to_string());
                        }
                        if let Some(sort) = &this.sort {
                            req.url_mut().query_pairs_mut().append_pair("sort", sort);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: Vec<models::AtlasRelatedTermHeader> = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod create_glossary_term {
        use super::models;
        type Response = models::AtlasGlossaryTerm;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) glossary_term: models::AtlasGlossaryTerm,
            pub(crate) include_term_hierarchy: Option<bool>,
        }
        impl Builder {
            #[doc = "Whether include term hierarchy"]
            pub fn include_term_hierarchy(mut self, include_term_hierarchy: bool) -> Self {
                self.include_term_hierarchy = Some(include_term_hierarchy);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/glossary/term", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.glossary_term)?;
                        if let Some(include_term_hierarchy) = &this.include_term_hierarchy {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeTermHierarchy", &include_term_hierarchy.to_string());
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasGlossaryTerm = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_glossary_term {
        use super::models;
        type Response = models::AtlasGlossaryTerm;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) term_guid: String,
            pub(crate) include_term_hierarchy: Option<bool>,
            pub(crate) exclude_relationship_types: Vec<String>,
        }
        impl Builder {
            #[doc = "Whether include term hierarchy"]
            pub fn include_term_hierarchy(mut self, include_term_hierarchy: bool) -> Self {
                self.include_term_hierarchy = Some(include_term_hierarchy);
                self
            }
            #[doc = "An array of relationship types which need to be excluded."]
            pub fn exclude_relationship_types(mut self, exclude_relationship_types: Vec<String>) -> Self {
                self.exclude_relationship_types = exclude_relationship_types;
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/atlas/v2/glossary/term/{}", this.client.endpoint(), &this.term_guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(include_term_hierarchy) = &this.include_term_hierarchy {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeTermHierarchy", &include_term_hierarchy.to_string());
                        }
                        let exclude_relationship_types = &this.exclude_relationship_types;
                        for value in &this.exclude_relationship_types {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("excludeRelationshipTypes", &value.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasGlossaryTerm = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod update_glossary_term {
        use super::models;
        type Response = models::AtlasGlossaryTerm;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) term_guid: String,
            pub(crate) glossary_term: models::AtlasGlossaryTerm,
            pub(crate) include_term_hierarchy: Option<bool>,
        }
        impl Builder {
            #[doc = "Whether include term hierarchy"]
            pub fn include_term_hierarchy(mut self, include_term_hierarchy: bool) -> Self {
                self.include_term_hierarchy = Some(include_term_hierarchy);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/atlas/v2/glossary/term/{}", this.client.endpoint(), &this.term_guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.glossary_term)?;
                        if let Some(include_term_hierarchy) = &this.include_term_hierarchy {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeTermHierarchy", &include_term_hierarchy.to_string());
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasGlossaryTerm = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_glossary_term {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) term_guid: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/atlas/v2/glossary/term/{}", this.client.endpoint(), &this.term_guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod partial_update_glossary_term {
        use super::models;
        type Response = models::AtlasGlossaryTerm;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) term_guid: String,
            pub(crate) partial_updates: serde_json::Value,
            pub(crate) include_term_hierarchy: Option<bool>,
        }
        impl Builder {
            #[doc = "Whether include term hierarchy"]
            pub fn include_term_hierarchy(mut self, include_term_hierarchy: bool) -> Self {
                self.include_term_hierarchy = Some(include_term_hierarchy);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/glossary/term/{}/partial",
                            this.client.endpoint(),
                            &this.term_guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(include_term_hierarchy) = &this.include_term_hierarchy {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeTermHierarchy", &include_term_hierarchy.to_string());
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.partial_updates)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasGlossaryTerm = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod create_glossary_terms {
        use super::models;
        type Response = Vec<models::AtlasGlossaryTerm>;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) glossary_term: Vec<models::AtlasGlossaryTerm>,
            pub(crate) include_term_hierarchy: Option<bool>,
        }
        impl Builder {
            #[doc = "Whether include term hierarchy"]
            pub fn include_term_hierarchy(mut self, include_term_hierarchy: bool) -> Self {
                self.include_term_hierarchy = Some(include_term_hierarchy);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/glossary/terms", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.glossary_term)?;
                        if let Some(include_term_hierarchy) = &this.include_term_hierarchy {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeTermHierarchy", &include_term_hierarchy.to_string());
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: Vec<models::AtlasGlossaryTerm> = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_entities_assigned_with_term {
        use super::models;
        type Response = Vec<models::AtlasRelatedObjectId>;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) term_guid: String,
            pub(crate) limit: Option<i32>,
            pub(crate) offset: Option<i32>,
            pub(crate) sort: Option<String>,
        }
        impl Builder {
            #[doc = "The page size - by default there is no paging."]
            pub fn limit(mut self, limit: i32) -> Self {
                self.limit = Some(limit);
                self
            }
            #[doc = "The offset for pagination purpose."]
            pub fn offset(mut self, offset: i32) -> Self {
                self.offset = Some(offset);
                self
            }
            #[doc = "The sort order, ASC (default) or DESC."]
            pub fn sort(mut self, sort: impl Into<String>) -> Self {
                self.sort = Some(sort.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/glossary/terms/{}/assignedEntities",
                            this.client.endpoint(),
                            &this.term_guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(limit) = &this.limit {
                            req.url_mut().query_pairs_mut().append_pair("limit", &limit.to_string());
                        }
                        if let Some(offset) = &this.offset {
                            req.url_mut().query_pairs_mut().append_pair("offset", &offset.to_string());
                        }
                        if let Some(sort) = &this.sort {
                            req.url_mut().query_pairs_mut().append_pair("sort", sort);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: Vec<models::AtlasRelatedObjectId> = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod assign_term_to_entities {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) term_guid: String,
            pub(crate) related_object_ids: Vec<models::AtlasRelatedObjectId>,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/glossary/terms/{}/assignedEntities",
                            this.client.endpoint(),
                            &this.term_guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.related_object_ids)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod remove_term_assignment_from_entities {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) term_guid: String,
            pub(crate) related_object_ids: Vec<models::AtlasRelatedObjectId>,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/glossary/terms/{}/assignedEntities",
                            this.client.endpoint(),
                            &this.term_guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.related_object_ids)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_term_assignment_from_entities {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) term_guid: String,
            pub(crate) related_object_ids: Vec<models::AtlasRelatedObjectId>,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/glossary/terms/{}/assignedEntities",
                            this.client.endpoint(),
                            &this.term_guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.related_object_ids)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_related_terms {
        use super::models;
        type Response = serde_json::Value;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) term_guid: String,
            pub(crate) limit: Option<i32>,
            pub(crate) offset: Option<i32>,
            pub(crate) sort: Option<String>,
        }
        impl Builder {
            #[doc = "The page size - by default there is no paging."]
            pub fn limit(mut self, limit: i32) -> Self {
                self.limit = Some(limit);
                self
            }
            #[doc = "The offset for pagination purpose."]
            pub fn offset(mut self, offset: i32) -> Self {
                self.offset = Some(offset);
                self
            }
            #[doc = "The sort order, ASC (default) or DESC."]
            pub fn sort(mut self, sort: impl Into<String>) -> Self {
                self.sort = Some(sort.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/glossary/terms/{}/related",
                            this.client.endpoint(),
                            &this.term_guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(limit) = &this.limit {
                            req.url_mut().query_pairs_mut().append_pair("limit", &limit.to_string());
                        }
                        if let Some(offset) = &this.offset {
                            req.url_mut().query_pairs_mut().append_pair("offset", &offset.to_string());
                        }
                        if let Some(sort) = &this.sort {
                            req.url_mut().query_pairs_mut().append_pair("sort", sort);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: serde_json::Value = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_glossary {
        use super::models;
        type Response = models::AtlasGlossary;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) glossary_guid: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/glossary/{}", this.client.endpoint(), &this.glossary_guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasGlossary = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod update_glossary {
        use super::models;
        type Response = models::AtlasGlossary;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) glossary_guid: String,
            pub(crate) updated_glossary: models::AtlasGlossary,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/glossary/{}", this.client.endpoint(), &this.glossary_guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.updated_glossary)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasGlossary = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_glossary {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) glossary_guid: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/glossary/{}", this.client.endpoint(), &this.glossary_guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_glossary_categories {
        use super::models;
        type Response = Vec<models::AtlasGlossaryCategory>;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) glossary_guid: String,
            pub(crate) limit: Option<i32>,
            pub(crate) offset: Option<i32>,
            pub(crate) sort: Option<String>,
        }
        impl Builder {
            #[doc = "The page size - by default there is no paging."]
            pub fn limit(mut self, limit: i32) -> Self {
                self.limit = Some(limit);
                self
            }
            #[doc = "The offset for pagination purpose."]
            pub fn offset(mut self, offset: i32) -> Self {
                self.offset = Some(offset);
                self
            }
            #[doc = "The sort order, ASC (default) or DESC."]
            pub fn sort(mut self, sort: impl Into<String>) -> Self {
                self.sort = Some(sort.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/glossary/{}/categories",
                            this.client.endpoint(),
                            &this.glossary_guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(limit) = &this.limit {
                            req.url_mut().query_pairs_mut().append_pair("limit", &limit.to_string());
                        }
                        if let Some(offset) = &this.offset {
                            req.url_mut().query_pairs_mut().append_pair("offset", &offset.to_string());
                        }
                        if let Some(sort) = &this.sort {
                            req.url_mut().query_pairs_mut().append_pair("sort", sort);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: Vec<models::AtlasGlossaryCategory> = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_glossary_categories_headers {
        use super::models;
        type Response = Vec<models::AtlasRelatedCategoryHeader>;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) glossary_guid: String,
            pub(crate) limit: Option<i32>,
            pub(crate) offset: Option<i32>,
            pub(crate) sort: Option<String>,
        }
        impl Builder {
            #[doc = "The page size - by default there is no paging."]
            pub fn limit(mut self, limit: i32) -> Self {
                self.limit = Some(limit);
                self
            }
            #[doc = "The offset for pagination purpose."]
            pub fn offset(mut self, offset: i32) -> Self {
                self.offset = Some(offset);
                self
            }
            #[doc = "The sort order, ASC (default) or DESC."]
            pub fn sort(mut self, sort: impl Into<String>) -> Self {
                self.sort = Some(sort.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/glossary/{}/categories/headers",
                            this.client.endpoint(),
                            &this.glossary_guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(limit) = &this.limit {
                            req.url_mut().query_pairs_mut().append_pair("limit", &limit.to_string());
                        }
                        if let Some(offset) = &this.offset {
                            req.url_mut().query_pairs_mut().append_pair("offset", &offset.to_string());
                        }
                        if let Some(sort) = &this.sort {
                            req.url_mut().query_pairs_mut().append_pair("sort", sort);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: Vec<models::AtlasRelatedCategoryHeader> = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_detailed_glossary {
        use super::models;
        type Response = models::AtlasGlossaryExtInfo;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) glossary_guid: String,
            pub(crate) include_term_hierarchy: Option<bool>,
        }
        impl Builder {
            #[doc = "Whether include term hierarchy"]
            pub fn include_term_hierarchy(mut self, include_term_hierarchy: bool) -> Self {
                self.include_term_hierarchy = Some(include_term_hierarchy);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/glossary/{}/detailed",
                            this.client.endpoint(),
                            &this.glossary_guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(include_term_hierarchy) = &this.include_term_hierarchy {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeTermHierarchy", &include_term_hierarchy.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasGlossaryExtInfo = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod partial_update_glossary {
        use super::models;
        type Response = models::AtlasGlossary;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) glossary_guid: String,
            pub(crate) partial_updates: serde_json::Value,
            pub(crate) include_term_hierarchy: Option<bool>,
        }
        impl Builder {
            #[doc = "Whether include term hierarchy"]
            pub fn include_term_hierarchy(mut self, include_term_hierarchy: bool) -> Self {
                self.include_term_hierarchy = Some(include_term_hierarchy);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/glossary/{}/partial",
                            this.client.endpoint(),
                            &this.glossary_guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(include_term_hierarchy) = &this.include_term_hierarchy {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeTermHierarchy", &include_term_hierarchy.to_string());
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.partial_updates)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasGlossary = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_glossary_terms {
        use super::models;
        type Response = Vec<models::AtlasGlossaryTerm>;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) glossary_guid: String,
            pub(crate) include_term_hierarchy: Option<bool>,
            pub(crate) limit: Option<i32>,
            pub(crate) offset: Option<i32>,
            pub(crate) sort: Option<String>,
        }
        impl Builder {
            #[doc = "Whether include term hierarchy"]
            pub fn include_term_hierarchy(mut self, include_term_hierarchy: bool) -> Self {
                self.include_term_hierarchy = Some(include_term_hierarchy);
                self
            }
            #[doc = "The page size - by default there is no paging."]
            pub fn limit(mut self, limit: i32) -> Self {
                self.limit = Some(limit);
                self
            }
            #[doc = "The offset for pagination purpose."]
            pub fn offset(mut self, offset: i32) -> Self {
                self.offset = Some(offset);
                self
            }
            #[doc = "The sort order, ASC (default) or DESC."]
            pub fn sort(mut self, sort: impl Into<String>) -> Self {
                self.sort = Some(sort.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/glossary/{}/terms",
                            this.client.endpoint(),
                            &this.glossary_guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(include_term_hierarchy) = &this.include_term_hierarchy {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeTermHierarchy", &include_term_hierarchy.to_string());
                        }
                        if let Some(limit) = &this.limit {
                            req.url_mut().query_pairs_mut().append_pair("limit", &limit.to_string());
                        }
                        if let Some(offset) = &this.offset {
                            req.url_mut().query_pairs_mut().append_pair("offset", &offset.to_string());
                        }
                        if let Some(sort) = &this.sort {
                            req.url_mut().query_pairs_mut().append_pair("sort", sort);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: Vec<models::AtlasGlossaryTerm> = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_glossary_term_headers {
        use super::models;
        type Response = Vec<models::AtlasRelatedTermHeader>;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) glossary_guid: String,
            pub(crate) limit: Option<i32>,
            pub(crate) offset: Option<i32>,
            pub(crate) sort: Option<String>,
        }
        impl Builder {
            #[doc = "The page size - by default there is no paging."]
            pub fn limit(mut self, limit: i32) -> Self {
                self.limit = Some(limit);
                self
            }
            #[doc = "The offset for pagination purpose."]
            pub fn offset(mut self, offset: i32) -> Self {
                self.offset = Some(offset);
                self
            }
            #[doc = "The sort order, ASC (default) or DESC."]
            pub fn sort(mut self, sort: impl Into<String>) -> Self {
                self.sort = Some(sort.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/glossary/{}/terms/headers",
                            this.client.endpoint(),
                            &this.glossary_guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(limit) = &this.limit {
                            req.url_mut().query_pairs_mut().append_pair("limit", &limit.to_string());
                        }
                        if let Some(offset) = &this.offset {
                            req.url_mut().query_pairs_mut().append_pair("offset", &offset.to_string());
                        }
                        if let Some(sort) = &this.sort {
                            req.url_mut().query_pairs_mut().append_pair("sort", sort);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: Vec<models::AtlasRelatedTermHeader> = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod import_glossary_terms_via_csv {
        use super::models;
        type Response = models::ImportCsvOperation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) glossary_guid: String,
            pub(crate) file: bytes::Bytes,
            pub(crate) include_term_hierarchy: Option<bool>,
        }
        impl Builder {
            #[doc = "Whether include term hierarchy"]
            pub fn include_term_hierarchy(mut self, include_term_hierarchy: bool) -> Self {
                self.include_term_hierarchy = Some(include_term_hierarchy);
                self
            }
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/glossary/{}/terms/import", this.client.endpoint(), &this.glossary_guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01-preview");
                        if let Some(include_term_hierarchy) = &this.include_term_hierarchy {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeTermHierarchy", &include_term_hierarchy.to_string());
                        }
                        unimplemented!("form data not yet supported");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ImportCsvOperation = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod import_glossary_terms_via_csv_by_glossary_name {
        use super::models;
        type Response = models::ImportCsvOperation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) glossary_name: String,
            pub(crate) file: bytes::Bytes,
            pub(crate) include_term_hierarchy: Option<bool>,
        }
        impl Builder {
            #[doc = "Whether include term hierarchy"]
            pub fn include_term_hierarchy(mut self, include_term_hierarchy: bool) -> Self {
                self.include_term_hierarchy = Some(include_term_hierarchy);
                self
            }
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/glossary/name/{}/terms/import",
                            this.client.endpoint(),
                            &this.glossary_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01-preview");
                        if let Some(include_term_hierarchy) = &this.include_term_hierarchy {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeTermHierarchy", &include_term_hierarchy.to_string());
                        }
                        unimplemented!("form data not yet supported");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ImportCsvOperation = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_import_csv_operation_status {
        use super::models;
        type Response = models::ImportCsvOperation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) operation_guid: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/glossary/terms/import/{}",
                            this.client.endpoint(),
                            &this.operation_guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ImportCsvOperation = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod export_glossary_terms_as_csv {
        use super::models;
        type Response = bytes::Bytes;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) glossary_guid: String,
            pub(crate) term_guids: Vec<models::TermGuid>,
            pub(crate) include_term_hierarchy: Option<bool>,
        }
        impl Builder {
            #[doc = "Whether include term hierarchy"]
            pub fn include_term_hierarchy(mut self, include_term_hierarchy: bool) -> Self {
                self.include_term_hierarchy = Some(include_term_hierarchy);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/glossary/{}/terms/export", this.client.endpoint(), &this.glossary_guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01-preview");
                        if let Some(include_term_hierarchy) = &this.include_term_hierarchy {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeTermHierarchy", &include_term_hierarchy.to_string());
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.term_guids)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value = rsp_body;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_terms_by_glossary_name {
        use super::models;
        type Response = Vec<models::AtlasGlossaryTerm>;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) glossary_name: String,
            pub(crate) limit: Option<i32>,
            pub(crate) offset: Option<i32>,
            pub(crate) include_term_hierarchy: Option<bool>,
        }
        impl Builder {
            #[doc = "The page size - by default there is no paging."]
            pub fn limit(mut self, limit: i32) -> Self {
                self.limit = Some(limit);
                self
            }
            #[doc = "The offset for pagination purpose."]
            pub fn offset(mut self, offset: i32) -> Self {
                self.offset = Some(offset);
                self
            }
            #[doc = "Whether include term hierarchy"]
            pub fn include_term_hierarchy(mut self, include_term_hierarchy: bool) -> Self {
                self.include_term_hierarchy = Some(include_term_hierarchy);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/glossary/name/{}/terms", this.client.endpoint(), &this.glossary_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01-preview");
                        if let Some(limit) = &this.limit {
                            req.url_mut().query_pairs_mut().append_pair("limit", &limit.to_string());
                        }
                        if let Some(offset) = &this.offset {
                            req.url_mut().query_pairs_mut().append_pair("offset", &offset.to_string());
                        }
                        if let Some(include_term_hierarchy) = &this.include_term_hierarchy {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeTermHierarchy", &include_term_hierarchy.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: Vec<models::AtlasGlossaryTerm> = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
}
pub mod discovery {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets data using search."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `search_request`: An object specifying the search criteria."]
        pub fn query(&self, search_request: impl Into<models::SearchRequest>) -> query::Builder {
            query::Builder {
                client: self.0.clone(),
                search_request: search_request.into(),
            }
        }
        #[doc = "Get search suggestions by query criteria."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `suggest_request`: An object specifying the suggest criteria."]
        pub fn suggest(&self, suggest_request: impl Into<models::SuggestRequest>) -> suggest::Builder {
            suggest::Builder {
                client: self.0.clone(),
                suggest_request: suggest_request.into(),
            }
        }
        #[doc = "Browse entities by path or entity type."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `browse_request`: An object specifying the browse criteria."]
        pub fn browse(&self, browse_request: impl Into<models::BrowseRequest>) -> browse::Builder {
            browse::Builder {
                client: self.0.clone(),
                browse_request: browse_request.into(),
            }
        }
        #[doc = "Get auto complete options."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `auto_complete_request`: An object specifying the autocomplete criteria."]
        pub fn auto_complete(&self, auto_complete_request: impl Into<models::AutoCompleteRequest>) -> auto_complete::Builder {
            auto_complete::Builder {
                client: self.0.clone(),
                auto_complete_request: auto_complete_request.into(),
            }
        }
    }
    pub mod query {
        use super::models;
        type Response = models::SearchResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) search_request: models::SearchRequest,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/search/query", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.search_request)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SearchResult = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod suggest {
        use super::models;
        type Response = models::SuggestResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) suggest_request: models::SuggestRequest,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/search/suggest", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.suggest_request)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SuggestResult = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod browse {
        use super::models;
        type Response = models::BrowseResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) browse_request: models::BrowseRequest,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/browse", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.browse_request)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BrowseResult = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod auto_complete {
        use super::models;
        type Response = models::AutoCompleteResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) auto_complete_request: models::AutoCompleteRequest,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/search/autocomplete", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.auto_complete_request)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AutoCompleteResult = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
}
pub mod lineage {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get lineage info of the entity specified by GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the entity."]
        #[doc = "* `direction`: The direction of the lineage, which could be INPUT, OUTPUT or BOTH."]
        pub fn get_lineage_graph(&self, guid: impl Into<String>, direction: impl Into<String>) -> get_lineage_graph::Builder {
            get_lineage_graph::Builder {
                client: self.0.clone(),
                guid: guid.into(),
                direction: direction.into(),
                depth: None,
                width: None,
                include_parent: None,
                get_derived_lineage: None,
            }
        }
        #[doc = "Return immediate next page lineage info about entity with pagination"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the entity."]
        #[doc = "* `direction`: The direction of the lineage, which could be INPUT, OUTPUT or BOTH."]
        pub fn next_page_lineage(&self, guid: impl Into<String>, direction: impl Into<String>) -> next_page_lineage::Builder {
            next_page_lineage::Builder {
                client: self.0.clone(),
                guid: guid.into(),
                direction: direction.into(),
                get_derived_lineage: None,
                offset: None,
                limit: None,
            }
        }
        #[doc = "Returns lineage info about entity.\n\nIn addition to the typeName path parameter, attribute key-value pair(s) can be provided in the following format\n\nattr:[attrName]=[attrValue]\n\nNOTE: The attrName and attrValue should be unique across entities, eg. qualifiedName"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `type_name`: The name of the type."]
        #[doc = "* `direction`: The direction of the lineage, which could be INPUT, OUTPUT or BOTH."]
        pub fn get_lineage_by_unique_attribute(
            &self,
            type_name: impl Into<String>,
            direction: impl Into<String>,
        ) -> get_lineage_by_unique_attribute::Builder {
            get_lineage_by_unique_attribute::Builder {
                client: self.0.clone(),
                type_name: type_name.into(),
                direction: direction.into(),
                depth: None,
                width: None,
                include_parent: None,
                get_derived_lineage: None,
            }
        }
    }
    pub mod get_lineage_graph {
        use super::models;
        type Response = models::AtlasLineageInfo;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
            pub(crate) direction: String,
            pub(crate) depth: Option<i32>,
            pub(crate) width: Option<i32>,
            pub(crate) include_parent: Option<bool>,
            pub(crate) get_derived_lineage: Option<bool>,
        }
        impl Builder {
            #[doc = "The number of hops for lineage."]
            pub fn depth(mut self, depth: i32) -> Self {
                self.depth = Some(depth);
                self
            }
            #[doc = "The number of max expanding width in lineage."]
            pub fn width(mut self, width: i32) -> Self {
                self.width = Some(width);
                self
            }
            #[doc = "True to include the parent chain in the response."]
            pub fn include_parent(mut self, include_parent: bool) -> Self {
                self.include_parent = Some(include_parent);
                self
            }
            #[doc = "True to include derived lineage in the response"]
            pub fn get_derived_lineage(mut self, get_derived_lineage: bool) -> Self {
                self.get_derived_lineage = Some(get_derived_lineage);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/lineage/{}", this.client.endpoint(), &this.guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(depth) = &this.depth {
                            req.url_mut().query_pairs_mut().append_pair("depth", &depth.to_string());
                        }
                        if let Some(width) = &this.width {
                            req.url_mut().query_pairs_mut().append_pair("width", &width.to_string());
                        }
                        let direction = &this.direction;
                        req.url_mut().query_pairs_mut().append_pair("direction", direction);
                        if let Some(include_parent) = &this.include_parent {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeParent", &include_parent.to_string());
                        }
                        if let Some(get_derived_lineage) = &this.get_derived_lineage {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("getDerivedLineage", &get_derived_lineage.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasLineageInfo = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod next_page_lineage {
        use super::models;
        type Response = models::AtlasLineageInfo;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
            pub(crate) direction: String,
            pub(crate) get_derived_lineage: Option<bool>,
            pub(crate) offset: Option<i32>,
            pub(crate) limit: Option<i32>,
        }
        impl Builder {
            #[doc = "True to include derived lineage in the response"]
            pub fn get_derived_lineage(mut self, get_derived_lineage: bool) -> Self {
                self.get_derived_lineage = Some(get_derived_lineage);
                self
            }
            #[doc = "The offset for pagination purpose."]
            pub fn offset(mut self, offset: i32) -> Self {
                self.offset = Some(offset);
                self
            }
            #[doc = "The page size - by default there is no paging."]
            pub fn limit(mut self, limit: i32) -> Self {
                self.limit = Some(limit);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/lineage/{}/next/", this.client.endpoint(), &this.guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01-preview");
                        let direction = &this.direction;
                        req.url_mut().query_pairs_mut().append_pair("direction", direction);
                        if let Some(get_derived_lineage) = &this.get_derived_lineage {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("getDerivedLineage", &get_derived_lineage.to_string());
                        }
                        if let Some(offset) = &this.offset {
                            req.url_mut().query_pairs_mut().append_pair("offset", &offset.to_string());
                        }
                        if let Some(limit) = &this.limit {
                            req.url_mut().query_pairs_mut().append_pair("limit", &limit.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasLineageInfo = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_lineage_by_unique_attribute {
        use super::models;
        type Response = models::AtlasLineageInfo;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) type_name: String,
            pub(crate) direction: String,
            pub(crate) depth: Option<i32>,
            pub(crate) width: Option<i32>,
            pub(crate) include_parent: Option<bool>,
            pub(crate) get_derived_lineage: Option<bool>,
        }
        impl Builder {
            #[doc = "The number of hops for lineage."]
            pub fn depth(mut self, depth: i32) -> Self {
                self.depth = Some(depth);
                self
            }
            #[doc = "The number of max expanding width in lineage."]
            pub fn width(mut self, width: i32) -> Self {
                self.width = Some(width);
                self
            }
            #[doc = "True to include the parent chain in the response."]
            pub fn include_parent(mut self, include_parent: bool) -> Self {
                self.include_parent = Some(include_parent);
                self
            }
            #[doc = "True to include derived lineage in the response"]
            pub fn get_derived_lineage(mut self, get_derived_lineage: bool) -> Self {
                self.get_derived_lineage = Some(get_derived_lineage);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/lineage/uniqueAttribute/type/{}",
                            this.client.endpoint(),
                            &this.type_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(depth) = &this.depth {
                            req.url_mut().query_pairs_mut().append_pair("depth", &depth.to_string());
                        }
                        if let Some(width) = &this.width {
                            req.url_mut().query_pairs_mut().append_pair("width", &width.to_string());
                        }
                        let direction = &this.direction;
                        req.url_mut().query_pairs_mut().append_pair("direction", direction);
                        if let Some(include_parent) = &this.include_parent {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeParent", &include_parent.to_string());
                        }
                        if let Some(get_derived_lineage) = &this.get_derived_lineage {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("getDerivedLineage", &get_derived_lineage.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasLineageInfo = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
}
pub mod relationship {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Create a new relationship between entities."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `relationship`: The AtlasRelationship object containing the information for the relationship to be created."]
        pub fn create(&self, relationship: impl Into<models::AtlasRelationship>) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                relationship: relationship.into(),
            }
        }
        #[doc = "Update an existing relationship between entities."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `relationship`: The AtlasRelationship object containing the information for the relationship to be created."]
        pub fn update(&self, relationship: impl Into<models::AtlasRelationship>) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                relationship: relationship.into(),
            }
        }
        #[doc = "Get relationship information between entities by its GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the relationship."]
        pub fn get(&self, guid: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                guid: guid.into(),
                extended_info: None,
            }
        }
        #[doc = "Delete a relationship between entities by its GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the relationship."]
        pub fn delete(&self, guid: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                guid: guid.into(),
            }
        }
    }
    pub mod create {
        use super::models;
        type Response = models::AtlasRelationship;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) relationship: models::AtlasRelationship,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/relationship", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.relationship)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasRelationship = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod update {
        use super::models;
        type Response = models::AtlasRelationship;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) relationship: models::AtlasRelationship,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/relationship", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.relationship)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasRelationship = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::AtlasRelationshipWithExtInfo;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
            pub(crate) extended_info: Option<bool>,
        }
        impl Builder {
            #[doc = "Limits whether includes extended information."]
            pub fn extended_info(mut self, extended_info: bool) -> Self {
                self.extended_info = Some(extended_info);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/relationship/guid/{}", this.client.endpoint(), &this.guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(extended_info) = &this.extended_info {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("extendedInfo", &extended_info.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasRelationshipWithExtInfo = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/relationship/guid/{}", this.client.endpoint(), &this.guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
}
pub mod types {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get the businessMetadata definition for the given guid"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: businessMetadata guid"]
        pub fn get_business_metadata_def_by_guid(&self, guid: impl Into<String>) -> get_business_metadata_def_by_guid::Builder {
            get_business_metadata_def_by_guid::Builder {
                client: self.0.clone(),
                guid: guid.into(),
            }
        }
        #[doc = "Get the businessMetadata definition by it's name (unique)"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: businessMetadata name"]
        pub fn get_business_metadata_def_by_name(&self, name: impl Into<String>) -> get_business_metadata_def_by_name::Builder {
            get_business_metadata_def_by_name::Builder {
                client: self.0.clone(),
                name: name.into(),
            }
        }
        #[doc = "Get the classification definition for the given GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the classification."]
        pub fn get_classification_def_by_guid(&self, guid: impl Into<String>) -> get_classification_def_by_guid::Builder {
            get_classification_def_by_guid::Builder {
                client: self.0.clone(),
                guid: guid.into(),
            }
        }
        #[doc = "Get the classification definition by its name (unique)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: The name of the classification."]
        pub fn get_classification_def_by_name(&self, name: impl Into<String>) -> get_classification_def_by_name::Builder {
            get_classification_def_by_name::Builder {
                client: self.0.clone(),
                name: name.into(),
            }
        }
        #[doc = "Get the Entity definition for the given GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the entity."]
        pub fn get_entity_definition_by_guid(&self, guid: impl Into<String>) -> get_entity_definition_by_guid::Builder {
            get_entity_definition_by_guid::Builder {
                client: self.0.clone(),
                guid: guid.into(),
            }
        }
        #[doc = "Get the entity definition by its name (unique)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: The name of the entity."]
        pub fn get_entity_definition_by_name(&self, name: impl Into<String>) -> get_entity_definition_by_name::Builder {
            get_entity_definition_by_name::Builder {
                client: self.0.clone(),
                name: name.into(),
            }
        }
        #[doc = "Get the enum definition for the given GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the enum."]
        pub fn get_enum_def_by_guid(&self, guid: impl Into<String>) -> get_enum_def_by_guid::Builder {
            get_enum_def_by_guid::Builder {
                client: self.0.clone(),
                guid: guid.into(),
            }
        }
        #[doc = "Get the enum definition by its name (unique)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: The name of the enum."]
        pub fn get_enum_def_by_name(&self, name: impl Into<String>) -> get_enum_def_by_name::Builder {
            get_enum_def_by_name::Builder {
                client: self.0.clone(),
                name: name.into(),
            }
        }
        #[doc = "Get the relationship definition for the given GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the relationship."]
        pub fn get_relationship_def_by_guid(&self, guid: impl Into<String>) -> get_relationship_def_by_guid::Builder {
            get_relationship_def_by_guid::Builder {
                client: self.0.clone(),
                guid: guid.into(),
            }
        }
        #[doc = "Get the relationship definition by its name (unique)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: The name of the relationship."]
        pub fn get_relationship_def_by_name(&self, name: impl Into<String>) -> get_relationship_def_by_name::Builder {
            get_relationship_def_by_name::Builder {
                client: self.0.clone(),
                name: name.into(),
            }
        }
        #[doc = "Get the struct definition for the given GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the struct."]
        pub fn get_struct_def_by_guid(&self, guid: impl Into<String>) -> get_struct_def_by_guid::Builder {
            get_struct_def_by_guid::Builder {
                client: self.0.clone(),
                guid: guid.into(),
            }
        }
        #[doc = "Get the struct definition by its name (unique)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: The name of the struct."]
        pub fn get_struct_def_by_name(&self, name: impl Into<String>) -> get_struct_def_by_name::Builder {
            get_struct_def_by_name::Builder {
                client: self.0.clone(),
                name: name.into(),
            }
        }
        #[doc = "Get the type definition for the given GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the type."]
        pub fn get_type_definition_by_guid(&self, guid: impl Into<String>) -> get_type_definition_by_guid::Builder {
            get_type_definition_by_guid::Builder {
                client: self.0.clone(),
                guid: guid.into(),
            }
        }
        #[doc = "Get the type definition by its name (unique)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: The name of the type."]
        pub fn get_type_definition_by_name(&self, name: impl Into<String>) -> get_type_definition_by_name::Builder {
            get_type_definition_by_name::Builder {
                client: self.0.clone(),
                name: name.into(),
            }
        }
        #[doc = "Delete API for type identified by its name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: The name of the type."]
        pub fn delete_type_by_name(&self, name: impl Into<String>) -> delete_type_by_name::Builder {
            delete_type_by_name::Builder {
                client: self.0.clone(),
                name: name.into(),
            }
        }
        #[doc = "Get all type definitions in Atlas in bulk."]
        pub fn get_all_type_definitions(&self) -> get_all_type_definitions::Builder {
            get_all_type_definitions::Builder {
                client: self.0.clone(),
                include_term_template: None,
                type_: None,
            }
        }
        #[doc = "Create all atlas type definitions in bulk, only new definitions will be created.\nAny changes to the existing definitions will be discarded."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `types_def`: A composite wrapper object with corresponding lists of the type definition."]
        pub fn create_type_definitions(&self, types_def: impl Into<models::AtlasTypesDef>) -> create_type_definitions::Builder {
            create_type_definitions::Builder {
                client: self.0.clone(),
                types_def: types_def.into(),
            }
        }
        #[doc = "Update all types in bulk, changes detected in the type definitions would be persisted."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `types_def`: A composite object that captures all type definition changes."]
        pub fn update_atlas_type_definitions(&self, types_def: impl Into<models::AtlasTypesDef>) -> update_atlas_type_definitions::Builder {
            update_atlas_type_definitions::Builder {
                client: self.0.clone(),
                types_def: types_def.into(),
            }
        }
        #[doc = "Delete API for all types in bulk."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `types_def`: A composite object that captures all types to be deleted"]
        pub fn delete_type_definitions(&self, types_def: impl Into<models::AtlasTypesDef>) -> delete_type_definitions::Builder {
            delete_type_definitions::Builder {
                client: self.0.clone(),
                types_def: types_def.into(),
            }
        }
        #[doc = "List all type definitions returned as a list of minimal information header."]
        pub fn list_type_definition_headers(&self) -> list_type_definition_headers::Builder {
            list_type_definition_headers::Builder {
                client: self.0.clone(),
                include_term_template: None,
                type_: None,
            }
        }
        #[doc = "Get the term template definition for the given GUID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `guid`: The globally unique identifier of the term template."]
        pub fn get_term_template_def_by_guid(&self, guid: impl Into<String>) -> get_term_template_def_by_guid::Builder {
            get_term_template_def_by_guid::Builder {
                client: self.0.clone(),
                guid: guid.into(),
            }
        }
        #[doc = "Get the term template definition by its name (unique)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `name`: The name of the term template."]
        pub fn get_term_template_def_by_name(&self, name: impl Into<String>) -> get_term_template_def_by_name::Builder {
            get_term_template_def_by_name::Builder {
                client: self.0.clone(),
                name: name.into(),
            }
        }
    }
    pub mod get_business_metadata_def_by_guid {
        use super::models;
        type Response = models::AtlasBusinessMetadataDef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/types/businessmetadatadef/guid/{}",
                            this.client.endpoint(),
                            &this.guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasBusinessMetadataDef = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_business_metadata_def_by_name {
        use super::models;
        type Response = models::AtlasBusinessMetadataDef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/types/businessmetadatadef/name/{}",
                            this.client.endpoint(),
                            &this.name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasBusinessMetadataDef = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_classification_def_by_guid {
        use super::models;
        type Response = models::AtlasClassificationDef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/types/classificationdef/guid/{}",
                            this.client.endpoint(),
                            &this.guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasClassificationDef = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_classification_def_by_name {
        use super::models;
        type Response = models::AtlasClassificationDef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/types/classificationdef/name/{}",
                            this.client.endpoint(),
                            &this.name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasClassificationDef = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_entity_definition_by_guid {
        use super::models;
        type Response = models::AtlasEntityDef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/atlas/v2/types/entitydef/guid/{}", this.client.endpoint(), &this.guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasEntityDef = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_entity_definition_by_name {
        use super::models;
        type Response = models::AtlasEntityDef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/atlas/v2/types/entitydef/name/{}", this.client.endpoint(), &this.name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasEntityDef = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_enum_def_by_guid {
        use super::models;
        type Response = models::AtlasEnumDef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/atlas/v2/types/enumdef/guid/{}", this.client.endpoint(), &this.guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasEnumDef = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_enum_def_by_name {
        use super::models;
        type Response = models::AtlasEnumDef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/atlas/v2/types/enumdef/name/{}", this.client.endpoint(), &this.name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasEnumDef = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_relationship_def_by_guid {
        use super::models;
        type Response = models::AtlasRelationshipDef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/types/relationshipdef/guid/{}",
                            this.client.endpoint(),
                            &this.guid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasRelationshipDef = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_relationship_def_by_name {
        use super::models;
        type Response = models::AtlasRelationshipDef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/atlas/v2/types/relationshipdef/name/{}",
                            this.client.endpoint(),
                            &this.name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasRelationshipDef = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_struct_def_by_guid {
        use super::models;
        type Response = models::AtlasStructDef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/atlas/v2/types/structdef/guid/{}", this.client.endpoint(), &this.guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasStructDef = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_struct_def_by_name {
        use super::models;
        type Response = models::AtlasStructDef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/atlas/v2/types/structdef/name/{}", this.client.endpoint(), &this.name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasStructDef = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_type_definition_by_guid {
        use super::models;
        type Response = models::AtlasTypeDef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/atlas/v2/types/typedef/guid/{}", this.client.endpoint(), &this.guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasTypeDef = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_type_definition_by_name {
        use super::models;
        type Response = models::AtlasTypeDef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/atlas/v2/types/typedef/name/{}", this.client.endpoint(), &this.name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasTypeDef = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_type_by_name {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/atlas/v2/types/typedef/name/{}", this.client.endpoint(), &this.name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_all_type_definitions {
        use super::models;
        type Response = models::AtlasTypesDef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) include_term_template: Option<bool>,
            pub(crate) type_: Option<String>,
        }
        impl Builder {
            #[doc = "Whether include termtemplatedef when return all typedefs.\nThis is always true when search filter type=term_template"]
            pub fn include_term_template(mut self, include_term_template: bool) -> Self {
                self.include_term_template = Some(include_term_template);
                self
            }
            #[doc = "Typedef name as search filter when get typedefs."]
            pub fn type_(mut self, type_: impl Into<String>) -> Self {
                self.type_ = Some(type_.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/types/typedefs", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(include_term_template) = &this.include_term_template {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeTermTemplate", &include_term_template.to_string());
                        }
                        if let Some(type_) = &this.type_ {
                            req.url_mut().query_pairs_mut().append_pair("type", type_);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasTypesDef = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod create_type_definitions {
        use super::models;
        type Response = models::AtlasTypesDef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) types_def: models::AtlasTypesDef,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/types/typedefs", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.types_def)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasTypesDef = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod update_atlas_type_definitions {
        use super::models;
        type Response = models::AtlasTypesDef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) types_def: models::AtlasTypesDef,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/types/typedefs", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.types_def)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AtlasTypesDef = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_type_definitions {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) types_def: models::AtlasTypesDef,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/types/typedefs", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.types_def)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list_type_definition_headers {
        use super::models;
        type Response = Vec<models::AtlasTypeDefHeader>;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) include_term_template: Option<bool>,
            pub(crate) type_: Option<String>,
        }
        impl Builder {
            #[doc = "Whether include termtemplatedef when return all typedefs.\nThis is always true when search filter type=term_template"]
            pub fn include_term_template(mut self, include_term_template: bool) -> Self {
                self.include_term_template = Some(include_term_template);
                self
            }
            #[doc = "Typedef name as search filter when get typedefs."]
            pub fn type_(mut self, type_: impl Into<String>) -> Self {
                self.type_ = Some(type_.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/atlas/v2/types/typedefs/headers", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(include_term_template) = &this.include_term_template {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeTermTemplate", &include_term_template.to_string());
                        }
                        if let Some(type_) = &this.type_ {
                            req.url_mut().query_pairs_mut().append_pair("type", type_);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: Vec<models::AtlasTypeDefHeader> = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_term_template_def_by_guid {
        use super::models;
        type Response = models::TermTemplateDef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) guid: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/types/termtemplatedef/guid/{}", this.client.endpoint(), &this.guid))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TermTemplateDef = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod get_term_template_def_by_name {
        use super::models;
        type Response = models::TermTemplateDef;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/types/termtemplatedef/name/{}", this.client.endpoint(), &this.name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TermTemplateDef = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
}
pub mod collection {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Creates or updates an entity to a collection.\nExisting entity is matched using its unique guid if supplied or by its unique attributes eg: qualifiedName.\nMap and array of collections are not well supported. E.g., array<array<int>>, array<map<string, int>>."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `collection`: the collection unique name"]
        #[doc = "* `entity`: Atlas entity with extended information."]
        pub fn create_or_update(
            &self,
            collection: impl Into<String>,
            entity: impl Into<models::AtlasEntityWithExtInfo>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                collection: collection.into(),
                entity: entity.into(),
            }
        }
        #[doc = "Creates or updates entities in bulk to a collection.\nExisting entity is matched using its unique guid if supplied or by its unique attributes eg: qualifiedName.\nMap and array of collections are not well supported. E.g., array<array<int>>, array<map<string, int>>."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `collection`: the collection unique name"]
        #[doc = "* `entities`: Atlas entities with extended information."]
        pub fn create_or_update_bulk(
            &self,
            collection: impl Into<String>,
            entities: impl Into<models::AtlasEntitiesWithExtInfo>,
        ) -> create_or_update_bulk::Builder {
            create_or_update_bulk::Builder {
                client: self.0.clone(),
                collection: collection.into(),
                entities: entities.into(),
            }
        }
        #[doc = "Move existing entities to the target collection."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `collection`: the collection unique name"]
        #[doc = "* `move_entities_request`: Entity guids to be moved to target collection."]
        pub fn move_entities_to_collection(
            &self,
            collection: impl Into<String>,
            move_entities_request: impl Into<models::MoveEntitiesRequest>,
        ) -> move_entities_to_collection::Builder {
            move_entities_to_collection::Builder {
                client: self.0.clone(),
                collection: collection.into(),
                move_entities_request: move_entities_request.into(),
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        type Response = models::EntityMutationResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) collection: String,
            pub(crate) entity: models::AtlasEntityWithExtInfo,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/collections/{}/entity", this.client.endpoint(), &this.collection))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.entity)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EntityMutationResponse = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod create_or_update_bulk {
        use super::models;
        type Response = models::EntityMutationResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) collection: String,
            pub(crate) entities: models::AtlasEntitiesWithExtInfo,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/collections/{}/entity/bulk", this.client.endpoint(), &this.collection))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.entities)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EntityMutationResponse = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod move_entities_to_collection {
        use super::models;
        type Response = models::EntityMutationResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) collection: String,
            pub(crate) move_entities_request: models::MoveEntitiesRequest,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/collections/{}/entity/moveHere",
                            this.client.endpoint(),
                            &this.collection
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2022-03-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.move_entities_request)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EntityMutationResponse = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
}
