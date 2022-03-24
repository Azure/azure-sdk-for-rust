// Set of properties that can be use in a connection string provided to KustoConnectionStringBuilder.
// For a complete list of properties go to https://docs.microsoft.com/en-us/azure/kusto/api/connection-strings/kusto

use hashbrown::HashMap;
use lazy_static::lazy_static;

enum ConnectionStringKey {
    DataSource,
    FederatedSecurity,
    UserId,
    Password,
    ApplicationClientId,
    ApplicationKey,
    ApplicationCertificate,
    ApplicationCertificateThumbprint,
    AuthorityId,
    ApplicationToken,
    UserToken,
    MsiAuth,
    MsiParams,
    AzCli,
}

impl ConnectionStringKey {
    fn to_str(&self) -> &'static str {
        match self {
            ConnectionStringKey::DataSource => "Data Source",
            ConnectionStringKey::FederatedSecurity => "AAD Federated Security",
            ConnectionStringKey::UserId => "AAD User ID",
            ConnectionStringKey::Password => "Password",
            ConnectionStringKey::ApplicationClientId => "Application Client Id",
            ConnectionStringKey::ApplicationKey => "Application Key",
            ConnectionStringKey::ApplicationCertificate => "ApplicationCertificate",
            ConnectionStringKey::ApplicationCertificateThumbprint => "Application Certificate Thumbprint",
            ConnectionStringKey::AuthorityId => "Authority Id",
            ConnectionStringKey::ApplicationToken => "ApplicationToken",
            ConnectionStringKey::UserToken => "UserToken",
            ConnectionStringKey::MsiAuth => "MSI Authentication",
            ConnectionStringKey::MsiParams => "MSI Params",
            ConnectionStringKey::AzCli => "AZ CLI",
        }
    }
}

lazy_static! {
    static ref ALIAS_MAP: HashMap<&'static str, ConnectionStringKey> = {
        let mut m = HashMap::new();
        m.insert("data source", ConnectionStringKey::DataSource);
        m.insert("addr", ConnectionStringKey::DataSource);
        m.insert("address", ConnectionStringKey::DataSource);
        m.insert("network address", ConnectionStringKey::DataSource);
        m.insert("server", ConnectionStringKey::DataSource);

        m.insert("aad federated security", ConnectionStringKey::FederatedSecurity);
        m.insert("federated security", ConnectionStringKey::FederatedSecurity);
        m.insert("federated", ConnectionStringKey::FederatedSecurity);
        m.insert("fed", ConnectionStringKey::FederatedSecurity);
        m.insert("aadfed", ConnectionStringKey::FederatedSecurity);

        m.insert("aad user id", ConnectionStringKey::UserId);
        m.insert("user id", ConnectionStringKey::UserId);
        m.insert("uid", ConnectionStringKey::UserId);
        m.insert("user", ConnectionStringKey::UserId);

        m.insert("password", ConnectionStringKey::Password);
        m.insert("pwd", ConnectionStringKey::Password);

        m.insert("application client id", ConnectionStringKey::ApplicationClientId);
        m.insert("appclientid", ConnectionStringKey::ApplicationClientId);

        m.insert("application key", ConnectionStringKey::ApplicationKey);
        m.insert("appkey", ConnectionStringKey::ApplicationKey);

        m.insert("application certificate", ConnectionStringKey::ApplicationCertificate);


        m.insert("application certificate thumbprint", ConnectionStringKey::ApplicationCertificateThumbprint);
        m.insert("appcert", ConnectionStringKey::ApplicationCertificateThumbprint);

        m.insert("authority id", ConnectionStringKey::AuthorityId);
        m.insert("authorityid", ConnectionStringKey::AuthorityId);
        m.insert("authority", ConnectionStringKey::AuthorityId);
        m.insert("tenantid", ConnectionStringKey::AuthorityId);
        m.insert("tenant", ConnectionStringKey::AuthorityId);
        m.insert("tid", ConnectionStringKey::AuthorityId);

        m.insert("application token", ConnectionStringKey::ApplicationToken);
        m.insert("apptoken", ConnectionStringKey::ApplicationToken);

        m.insert("user token", ConnectionStringKey::UserToken);
        m.insert("usertoken", ConnectionStringKey::UserToken);

        m.insert("msi auth", ConnectionStringKey::MsiAuth);
        m.insert("msi_auth", ConnectionStringKey::MsiAuth);
        m.insert("msi", ConnectionStringKey::MsiAuth);

        m.insert("msi params", ConnectionStringKey::MsiParams);
        m.insert("msi_params", ConnectionStringKey::MsiParams);
        m.insert("msi_type", ConnectionStringKey::MsiParams);

        m.insert("az cli", ConnectionStringKey::AzCli);

        m
    };
}


// TODO: when available
// pub const PUBLIC_APPLICATION_CERTIFICATE_NAME: &str = "Public Application Certificate";
// pub const INTERACTIVE_LOGIN_NAME: &str = "Interactive Login";
// pub const LOGIN_HINT_NAME: &str = "Login Hint";
// pub const DOMAIN_HINT_NAME: &str = "Domain Hint";
/*

        m.insert("application certificate private key", ConnectionStringKey::ApplicationCertificatePrivateKey);
        m.insert("application certificate x5c", ConnectionStringKey::ApplicationCertificateX5C);
        m.insert("application certificate send public certificate", ConnectionStringKey::ApplicationCertificateX5C);
        m.insert("application certificate sendx5c", ConnectionStringKey::ApplicationCertificateX5C);
        m.insert("sendx5c", ConnectionStringKey::ApplicationCertificateX5C);
                    ConnectionStringKey::ApplicationCertificatePrivateKey => "Application Certificate PrivateKey",
            ConnectionStringKey::ApplicationCertificateX5C => "Application Certificate x5c",
 */

#[derive(Debug, thiserror::Error)]
pub enum ConnectionStringError {
    #[error("Missing value for key '{}'", key)]
    MissingValue { key: String },
    #[error("Unexpected key '{}'", key)]
    UnexpectedKey { key: String },
    #[error("Parsing error: {}", msg)]
    ParsingError { msg: String },
}

#[derive(Default)]
pub struct ConnectionStringBuilder<'a>(ConnectionString<'a>);

impl<'a> ConnectionStringBuilder<'a> {
    pub fn new() -> Self {
        Self(ConnectionString::default())
    }

    pub fn new_with_aad_application_key_authentication(
        service_url: &'a str,
        authority_id: &'a str,
        application_client_id: &'a str,
        application_key: &'a str,
    ) -> Self {
        Self(ConnectionString {
            data_source: Some(service_url),
            federated_security: Some(true),
            application_client_id: Some(application_client_id),
            application_key: Some(application_key),
            authority_id: Some(authority_id),
            ..ConnectionString::default()
        })
    }

    pub fn build(&self) -> String {
        let mut kv_pairs = Vec::new();

        if let Some(data_source) = self.0.data_source {
            kv_pairs.push(format!("{}={}", ConnectionStringKey::DataSource.to_str(), data_source));
        }
        if let Some(user_id) = self.0.user_id {
            kv_pairs.push(format!("{}={}", ConnectionStringKey::UserId.to_str(), user_id));
        }
        if let Some(application_client_id) = self.0.application_client_id {
            kv_pairs.push(format!(
                "{}={}",
                ConnectionStringKey::ApplicationClientId.to_str(), application_client_id
            ));
        }
        if let Some(application_key) = self.0.application_key {
            kv_pairs.push(format!("{}={}", ConnectionStringKey::ApplicationKey.to_str(), application_key));
        }
        if let Some(application_token) = self.0.application_token {
            kv_pairs.push(format!("{}={}", ConnectionStringKey::ApplicationToken.to_str(), application_token));
        }
        if let Some(authority_id) = self.0.authority_id {
            kv_pairs.push(format!("{}={}", ConnectionStringKey::AuthorityId.to_str(), authority_id));
        }

        kv_pairs.join(";")
    }

    pub fn data_source(&'a mut self, data_source: &'a str) -> &'a mut Self {
        self.0.data_source = Some(data_source);
        self
    }

    pub fn user_id(&'a mut self, user_id: &'a str) -> &'a mut Self {
        self.0.user_id = Some(user_id);
        self
    }

    pub fn application_client_id(&'a mut self, application_client_id: &'a str) -> &'a mut Self {
        self.0.application_client_id = Some(application_client_id);
        self
    }

    pub fn application_token(&'a mut self, application_token: &'a str) -> &'a mut Self {
        self.0.application_token = Some(application_token);
        self
    }

    pub fn application_key(&'a mut self, application_key: &'a str) -> &'a mut Self {
        self.0.application_key = Some(application_key);
        self
    }

    pub fn authority_id(&'a mut self, authority_id: &'a str) -> &'a mut Self {
        self.0.authority_id = Some(authority_id);
        self
    }
}

/// A kusto service connection string.
///
/// The key are a subset of what is defined in the
/// https://docs.microsoft.com/en-us/azure/kusto/api/connection-strings/kusto
#[derive(Debug, Default)]
pub struct ConnectionString<'a> {
    /// The URI specifying the Kusto service endpoint.
    /// For example, https://mycluster.kusto.windows.net or net.tcp://localhost
    pub data_source: Option<&'a str>,
    /// A Boolean value that instructs the client to perform Azure Active Directory login.
    pub federated_security: Option<bool>,
    /// A String value that instructs the client to perform user authentication with the indicated user name.
    pub user_id: Option<&'a str>,
    pub user_token: Option<&'a str>,
    /// ...
    pub password: Option<&'a str>,
    /// A String value that provides the application client ID to use when authenticating.
    pub application_client_id: Option<&'a str>,
    /// A String value that provides the application key to use when authenticating using an application secret flow.
    pub application_key: Option<&'a str>,
    /// A String value that instructs the client to perform application authenticating with the specified bearer token.
    pub application_token: Option<&'a str>,
    /// ...
    pub application_certificate: Option<&'a str>,
    /// A String value that provides the thumbprint of the client
    /// certificate to use when using an application client certificate authenticating flow.
    pub application_certificate_thumbprint: Option<&'a str>,
    /// A String value that provides the name or ID of the tenant in which the application is registered.
    pub authority_id: Option<&'a str>,
    /// Denotes if MSI authorization should be used
    pub msi_auth: Option<bool>,
    pub msi_params: Option<&'a str>,
    pub az_cli: Option<bool>,
}

impl<'a> PartialEq for ConnectionString<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.data_source == other.data_source
            && self.federated_security == other.federated_security
            && self.user_id == other.user_id
            && self.user_token == other.user_token
            && self.password == other.password
            && self.application_client_id == other.application_client_id
            && self.application_key == other.application_key
            && self.application_token == other.application_token
            && self.application_certificate == other.application_certificate
            && self.application_certificate_thumbprint == other.application_certificate_thumbprint
            && self.authority_id == other.authority_id
            && self.msi_auth == other.msi_auth
            && self.msi_params == other.msi_params
            && self.az_cli == other.az_cli
    }
}

impl<'a> ConnectionString<'a> {
    pub fn new(connection_string: &'a str) -> Result<Self, ConnectionStringError> {
        let mut data_source = None;
        let mut federated_security = None;
        let mut user_id = None;
        let mut user_token = None;
        let mut password = None;
        let mut application_client_id = None;
        let mut application_token = None;
        let mut application_key = None;
        let mut application_certificate = None;
        let mut application_certificate_thumbprint = None;
        let mut authority_id = None;
        let mut msi_auth = None;
        let mut msi_params = None;
        let mut az_cli = None;

        let kv_str_pairs = connection_string
            .split(';')
            .filter(|s| !s.chars().all(char::is_whitespace));

        for kv_pair_str in kv_str_pairs {
            let mut kv = kv_pair_str.trim().split('=');
            let k = match kv.next() {
                Some(k) if k.chars().all(char::is_whitespace) => {
                    return Err(ConnectionStringError::ParsingError {
                        msg: "No key found".to_owned(),
                    });
                }
                None => {
                    return Err(ConnectionStringError::ParsingError {
                        msg: "No key found".to_owned(),
                    });
                }
                Some(k) => k,
            };
            let v = match kv.next() {
                Some(v) if v.chars().all(char::is_whitespace) => {
                    return Err(ConnectionStringError::MissingValue { key: k.to_owned() });
                }
                None => return Err(ConnectionStringError::MissingValue { key: k.to_owned() }),
                Some(v) => v,
            };

            if let Some(key) = ALIAS_MAP.get(&*k.to_ascii_lowercase()) {
                match key {
                    ConnectionStringKey::DataSource => data_source = Some(v),
                    e @ ConnectionStringKey::FederatedSecurity => federated_security = Some(parse_boolean(v, e.to_str())?),
                    ConnectionStringKey::UserId => user_id = Some(v),
                    ConnectionStringKey::UserToken => user_token = Some(v),
                    ConnectionStringKey::Password => password = Some(v),
                    ConnectionStringKey::ApplicationClientId => application_client_id = Some(v),
                    ConnectionStringKey::ApplicationToken => application_token = Some(v),
                    ConnectionStringKey::ApplicationKey => application_key = Some(v),
                    ConnectionStringKey::ApplicationCertificate => application_certificate = Some(v),
                    ConnectionStringKey::ApplicationCertificateThumbprint => application_certificate_thumbprint = Some(v),
                    ConnectionStringKey::AuthorityId => authority_id = Some(v),
                    e @ ConnectionStringKey::MsiAuth => msi_auth = Some(parse_boolean(v, e.to_str())?),
                    ConnectionStringKey::MsiParams => msi_params = Some(v),
                    e @ ConnectionStringKey::AzCli => az_cli = Some(parse_boolean(v, e.to_str())?),
                }
            }  else {
                return Err(ConnectionStringError::UnexpectedKey { key: k.to_owned() });
            }
        }

        Ok(Self {
            data_source,
            federated_security,
            user_id,
            user_token,
            password,
            application_client_id,
            application_key,
            application_token,
            application_certificate,
            application_certificate_thumbprint,
            authority_id,
            msi_auth,
            msi_params,
            az_cli,
        })
    }
}

fn parse_boolean(term: &str, name: &str) -> Result<bool, ConnectionStringError> {
    match term.to_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(ConnectionStringError::ParsingError {
            msg: format!(
                "Unexpected value for {}: {}. Please specify either 'true' or 'false'.",
                name, term
            ),
        }),
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn it_parses_empty_connection_string() {
        assert_eq!(
            ConnectionString::new("").unwrap(),
            ConnectionString::default()
        );
    }

    #[test]
    fn it_returns_expected_errors() {
        assert!(matches!(
            ConnectionString::new("Data Source="),
            Err(ConnectionStringError::MissingValue { key }) if key == "Data Source"
        ));
        assert!(matches!(
            ConnectionString::new("="),
            Err(ConnectionStringError::ParsingError { msg: _ })
        ));
        assert!(matches!(
            ConnectionString::new("x=123;"),
            Err(ConnectionStringError::UnexpectedKey { key }) if key == "x"
        ));
    }

    #[test]
    fn it_parses_basic_cases() {
        assert!(matches!(
            ConnectionString::new("Data Source=ds"),
            Ok(ConnectionString {
                data_source: Some("ds"),
                ..
            })
        ));
        assert!(matches!(
            ConnectionString::new("addr=ds"),
            Ok(ConnectionString {
                data_source: Some("ds"),
                ..
            })
        ));
        assert!(matches!(
            ConnectionString::new("Application Client Id=cid;Application Key=key"),
            Ok(ConnectionString {
                application_client_id: Some("cid"),
                application_key: Some("key"),
                ..
            })
        ));
        assert!(matches!(
            ConnectionString::new("Federated=True;AppToken=token"),
            Ok(ConnectionString {
                federated_security: Some(true),
                application_token: Some("token"),
                ..
            })
        ));
    }
}
