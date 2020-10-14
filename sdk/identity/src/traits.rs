use oauth2::AccessToken;

pub trait BearerToken {
    fn token_type(&self) -> &str;
    fn scopes(&self) -> &[String];
    fn expires_in(&self) -> u64;
    fn access_token(&self) -> &AccessToken;
}

pub trait RefreshToken {
    fn refresh_token(&self) -> &AccessToken;
}

pub trait ExtExpiresIn {
    fn ext_expires_in(&self) -> u64;
}
