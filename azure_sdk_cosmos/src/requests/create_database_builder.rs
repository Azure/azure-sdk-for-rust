use crate::clients::{Client, CosmosUriBuilder, ResourceType};
use crate::database::DatabaseName;
use crate::prelude::*;
use crate::responses::CreateDatabaseResponse;
use crate::ClientRequired;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::convert::TryFrom;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateDatabaseBuilder<'a, CUB, DB, DatabaseNameSet>
where
    DatabaseNameSet: ToAssign,
    CUB: CosmosUriBuilder,
    DB: DatabaseName,
{
    client: &'a Client<CUB>,
    p_database_name: PhantomData<DatabaseNameSet>,
    database_name: Option<&'a DB>,
}

impl<'a, CUB, DB> CreateDatabaseBuilder<'a, CUB, DB, No>
where
    CUB: CosmosUriBuilder,
    DB: DatabaseName,
{
    pub(crate) fn new(client: &'a Client<CUB>) -> CreateDatabaseBuilder<'a, CUB, DB, No> {
        CreateDatabaseBuilder {
            client,
            p_database_name: PhantomData {},
            database_name: None,
        }
    }
}

impl<'a, CUB, DB, DatabaseNameSet> ClientRequired<'a, CUB>
    for CreateDatabaseBuilder<'a, CUB, DB, DatabaseNameSet>
where
    DatabaseNameSet: ToAssign,
    CUB: CosmosUriBuilder,
    DB: DatabaseName,
{
    fn client(&self) -> &'a Client<CUB> {
        self.client
    }
}

impl<'a, CUB, DB> DatabaseNameRequired<'a, DB> for CreateDatabaseBuilder<'a, CUB, DB, Yes>
where
    CUB: CosmosUriBuilder,
    DB: DatabaseName,
{
    fn database_name(&self) -> &'a DB {
        self.database_name.unwrap()
    }
}

impl<'a, CUB, DB> DatabaseNameSupport<'a, DB> for CreateDatabaseBuilder<'a, CUB, DB, No>
where
    CUB: CosmosUriBuilder,
    DB: DatabaseName,
{
    type O = CreateDatabaseBuilder<'a, CUB, DB, Yes>;

    fn with_database_name(self, database_name: &'a DB) -> Self::O {
        CreateDatabaseBuilder {
            client: self.client,
            p_database_name: PhantomData {},
            database_name: Some(database_name),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB, DB> CreateDatabaseBuilder<'a, CUB, DB, Yes>
where
    CUB: CosmosUriBuilder,
    DB: DatabaseName,
{
    pub async fn execute(&self) -> Result<CreateDatabaseResponse, AzureError> {
        trace!("CreateDatabaseBuilder::execute called");

        #[derive(Serialize, Debug)]
        pub struct CreateDatabaseRequest<'a> {
            pub id: &'a str,
        }

        let req = serde_json::to_string(&CreateDatabaseRequest {
            id: self.database_name().name(),
        })?;

        let request = self
            .client()
            .prepare_request("dbs", hyper::Method::POST, ResourceType::Databases)
            .body(hyper::Body::from(req))?; // todo: set content-length here and elsewhere without builders

        debug!("create database request prepared == {:?}", request);

        let future_response = self.client().hyper_client().request(request);
        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::CREATED).await?;

        Ok(CreateDatabaseResponse::try_from((
            &headers,
            &body as &[u8],
        ))?)
    }
}
