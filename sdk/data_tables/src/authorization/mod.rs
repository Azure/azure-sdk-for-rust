pub mod authorization_policy;


#[derive(PartialEq, Clone, Eq)]
pub enum AuthorizationToken {
    SASToken {},
    BearerToken {},
    SharedKeyToken { account: String, key: String },
}

impl std::fmt::Debug for AuthorizationToken {
    // We provide a custom implementation to hide the key value.
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "AuthorizationToken::{}(***hidden***)",
            match self {
                AuthorizationToken::SASToken {} => "SASToken",
                AuthorizationToken::BearerToken {} => "BearerToken",
                AuthorizationToken::SharedKeyToken { .. } => "SharedKeyToken",
            }
        )
    }
}
