use crate::clients::{Client, CosmosUriBuilder, DatabaseClient};
use crate::database::DatabaseName;
use crate::{requests, DatabaseTrait, UserName, UserTrait};
use azure_sdk_core::No;

#[derive(Debug, Clone)]
pub struct UserClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    database_client: &'a DatabaseClient<'a, CUB>,
    user_name: &'a dyn UserName,
}

impl<'a, CUB> UserClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(
        database_client: &'a DatabaseClient<'a, CUB>,
        user_name: &'a dyn UserName,
    ) -> Self {
        Self {
            database_client,
            user_name,
        }
    }

    pub(crate) fn main_client(&self) -> &Client<CUB> {
        self.database_client.main_client()
    }

    pub(crate) fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.main_client().hyper_client()
    }
}

impl<'a, CUB> UserTrait<'a, CUB> for UserClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName {
        self.database_client.database_name()
    }

    fn user_name(&self) -> &'a dyn UserName {
        self.user_name
    }

    fn create_user(&self) -> requests::CreateUserBuilder<'_, CUB> {
        requests::CreateUserBuilder::new(self)
    }

    fn get_user(&self) -> requests::GetUserBuilder<'_, CUB> {
        requests::GetUserBuilder::new(self)
    }

    fn replace_user(&self) -> requests::ReplaceUserBuilder<'_, CUB, No> {
        requests::ReplaceUserBuilder::new(self)
    }
}
