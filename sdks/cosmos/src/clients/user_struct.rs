//use crate::prelude::*;
use crate::requests;
use crate::traits::*;
use crate::PermissionStruct;
use azure_sdk_core::No;
use std::borrow::Cow;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct UserStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    database_client: Cow<'a, D>,
    user_name: Cow<'a, str>,
    p_c: PhantomData<C>,
}

impl<'a, C, D> UserStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    pub(crate) fn new(database_client: Cow<'a, D>, user_name: Cow<'a, str>) -> Self {
        Self {
            database_client,
            user_name,
            p_c: PhantomData {},
        }
    }
}

impl<'a, C, D> HasHyperClient for UserStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    #[inline]
    fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.database_client().hyper_client()
    }
}

impl<'a, C, D> HasCosmosClient<C> for UserStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    #[inline]
    fn cosmos_client(&self) -> &C {
        self.database_client().cosmos_client()
    }
}

impl<'a, C, D> HasDatabaseClient<C, D> for UserStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    #[inline]
    fn database_client(&self) -> &D {
        &self.database_client
    }
}

impl<'a, C, D> UserClient<C, D> for UserStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn user_name(&self) -> &str {
        &self.user_name
    }

    fn create_user(&self) -> requests::CreateUserBuilder<'_, '_, C, D> {
        requests::CreateUserBuilder::new(self)
    }

    fn get_user(&self) -> requests::GetUserBuilder<'_, '_, C, D> {
        requests::GetUserBuilder::new(self)
    }

    fn replace_user(&self) -> requests::ReplaceUserBuilder<'_, '_, C, D, No> {
        requests::ReplaceUserBuilder::new(self)
    }

    fn delete_user(&self) -> requests::DeleteUserBuilder<'_, '_, C, D> {
        requests::DeleteUserBuilder::new(self)
    }

    fn list_permissions(&self) -> requests::ListPermissionsBuilder<'_, '_, C, D> {
        requests::ListPermissionsBuilder::new(self)
    }
}

impl<'a, C, D> IntoPermissionClient<'a, C, D, Self, PermissionStruct<'a, C, D, Self>>
    for UserStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn into_permission_client<IntoCowStr>(
        self,
        permission_name: IntoCowStr,
    ) -> PermissionStruct<'a, C, D, Self>
    where
        IntoCowStr: Into<Cow<'a, str>>,
    {
        PermissionStruct::new(Cow::Owned(self), permission_name.into())
    }
}

impl<'a, C, D> WithPermissionClient<'a, C, D, Self, PermissionStruct<'a, C, D, Self>>
    for UserStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn with_permission_client<IntoCowStr>(
        &'a self,
        permission_name: IntoCowStr,
    ) -> PermissionStruct<'a, C, D, Self>
    where
        IntoCowStr: Into<Cow<'a, str>>,
    {
        PermissionStruct::new(Cow::Borrowed(self), permission_name.into())
    }
}
