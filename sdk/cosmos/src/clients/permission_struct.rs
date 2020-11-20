use crate::requests;
use crate::traits::*;
use azure_core::HttpClient;
use std::borrow::Cow;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct PermissionStruct<'a, C, D, USER>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    USER: UserClient<C, D> + Clone,
{
    user_client: Cow<'a, USER>,
    permission_name: Cow<'a, str>,
    p_c: PhantomData<C>,
    p_d: PhantomData<D>,
}

impl<'a, C, D, USER> PermissionStruct<'a, C, D, USER>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    USER: UserClient<C, D> + Clone,
{
    pub(crate) fn new(user_client: Cow<'a, USER>, permission_name: Cow<'a, str>) -> Self {
        Self {
            user_client,
            permission_name,
            p_c: PhantomData {},
            p_d: PhantomData {},
        }
    }
}

impl<'a, C, D, USER> HasHttpClient for PermissionStruct<'a, C, D, USER>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    USER: UserClient<C, D> + Clone,
{
    #[inline]
    fn http_client(&self) -> &dyn HttpClient {
        self.user_client.http_client()
    }
}

impl<'a, C, D, USER> HasCosmosClient<C> for PermissionStruct<'a, C, D, USER>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    USER: UserClient<C, D> + Clone,
{
    #[inline]
    fn cosmos_client(&self) -> &C {
        self.user_client.cosmos_client()
    }
}

impl<'a, C, D, USER> HasDatabaseClient<C, D> for PermissionStruct<'a, C, D, USER>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    USER: UserClient<C, D> + Clone,
{
    #[inline]
    fn database_client(&self) -> &D {
        self.user_client.database_client()
    }
}

impl<'a, C, D, USER> HasUserClient<C, D, USER> for PermissionStruct<'a, C, D, USER>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    USER: UserClient<C, D> + Clone,
{
    #[inline]
    fn user_client(&self) -> &USER {
        &self.user_client
    }
}

impl<'a, C, D, USER> PermissionClient<C, D, USER> for PermissionStruct<'a, C, D, USER>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    USER: UserClient<C, D> + Clone,
{
    fn permission_name(&self) -> &str {
        &self.permission_name
    }

    fn create_permission(&self) -> requests::CreatePermissionBuilder<'_, '_, C, D, USER> {
        requests::CreatePermissionBuilder::new(self)
    }

    fn replace_permission(&self) -> requests::ReplacePermissionBuilder<'_, '_, C, D, USER> {
        requests::ReplacePermissionBuilder::new(self)
    }

    fn get_permission(&self) -> requests::GetPermissionBuilder<'_, '_, C, D, USER> {
        requests::GetPermissionBuilder::new(self)
    }

    fn delete_permission(&self) -> requests::DeletePermissionsBuilder<'_, '_, C, D, USER> {
        requests::DeletePermissionsBuilder::new(self)
    }
}
