use ring::hmac;

pub mod authorization_policy;
pub mod sas_token;

const EMULATOR_ACCOUNT: &str = "devstoreaccount1";
const EMULATOR_KEY: &str =
    "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==";

#[derive(PartialEq, Clone, Eq)]
pub struct AccountCredential {
    account: String,
    key: Vec<u8>,
}

impl AccountCredential {
    pub fn new(account: impl Into<String>, key: impl AsRef<[u8]>) -> Self {
        Self {
            account: account.into(),
            key: base64::decode(key).unwrap(),
        }
    }

    pub fn new_emulator() -> Self {
        Self {
            account: EMULATOR_ACCOUNT.into(),
            key: base64::decode(EMULATOR_KEY).unwrap(),
        }
    }

    /// Get a reference to the table credential's account.
    pub fn account(&self) -> &str {
        self.account.as_ref()
    }

    pub fn sign(&self, message: impl AsRef<[u8]>) -> String {
        base64::encode(hmac::sign(
            &hmac::Key::new(hmac::HMAC_SHA256, &self.key),
            message.as_ref(),
        ))
    }
}

impl std::fmt::Debug for AccountCredential {
    // We provide a custom implementation to hide the key value.
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        const SECRET_DEBUG_VALUE: &'static str = "...";
        f.debug_struct("AccountCredential")
            .field("account", &self.account)
            .field("key", &SECRET_DEBUG_VALUE)
            .finish()
    }
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum AuthorizationToken {
    SASToken(String),
    BearerToken {},
    SharedKeyToken(AccountCredential),
}
