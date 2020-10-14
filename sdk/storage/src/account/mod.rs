pub mod account;
pub mod prelude;

use crate::core::client::Client;

pub trait Account<C>
where
    C: Client,
{
    #[allow(clippy::needless_lifetimes)]
    fn get_account_information<'a>(
        &'a self,
    ) -> account::requests::GetAccountInformationBuilder<'a, C>;
}

impl<C> Account<C> for C
where
    C: Client,
{
    #[allow(clippy::needless_lifetimes)]
    fn get_account_information<'a>(
        &'a self,
    ) -> account::requests::GetAccountInformationBuilder<'a, C> {
        account::requests::GetAccountInformationBuilder::new(self)
    }
}
