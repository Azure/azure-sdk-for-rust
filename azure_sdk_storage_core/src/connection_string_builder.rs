use crate::connection_string::*;

pub struct ConnectionStringBuilder<'a>(ConnectionString<'a>);

impl<'a> ConnectionStringBuilder<'a> {
    pub fn new() -> Self {
        Self(ConnectionString::default())
    }

    pub fn build(&self) -> String {
        let mut kv_pairs = Vec::new();

        if let Some(account_name) = self.0.account_name {
            kv_pairs.push(format!("{}={}", ACCOUNT_NAME_KEY_NAME, account_name));
        }
        if let Some(account_key) = self.0.account_key {
            kv_pairs.push(format!("{}={}", ACCOUNT_KEY_KEY_NAME, account_key));
        }
        if let Some(sas) = self.0.sas {
            kv_pairs.push(format!("{}={}", SAS_KEY_NAME, sas));
        }
        if let Some(use_development_storage) = self.0.use_development_storage {
            kv_pairs.push(format!(
                "{}={}",
                USE_DEVELOPMENT_STORAGE_KEY_NAME, use_development_storage
            ));
        }
        if let Some(development_storage_proxy_uri) = self.0.development_storage_proxy_uri {
            kv_pairs.push(format!(
                "{}={}",
                DEVELOPMENT_STORAGE_PROXY_URI_KEY_NAME, development_storage_proxy_uri
            ));
        }
        if let Some(endpoint_suffix) = self.0.endpoint_suffix {
            kv_pairs.push(format!(
                "{}={}",
                ENDPOINT_SUFFIX_KEY_NAME,
                endpoint_suffix
            ));
        }
        if let Some(default_endpoints_protocol) = self.0.default_endpoints_protocol.as_ref() {
            kv_pairs.push(format!(
                "{}={}",
                DEFAULT_ENDPOINTS_PROTOCOL_KEY_NAME,
                default_endpoints_protocol
            ));
        }
        if let Some(blob_endpoint) = self.0.blob_endpoint {
            kv_pairs.push(format!(
                "{}={}",
                BLOB_ENDPOINT_KEY_NAME,
                blob_endpoint
            ));
        }
        if let Some(blob_secondary_endpoint) = self.0.blob_secondary_endpoint {
            kv_pairs.push(format!(
                "{}={}",
                BLOB_SECONDARY_ENDPOINT_KEY_NAME,
                blob_secondary_endpoint
            ));
        }
        if let Some(table_endpoint) = self.0.table_endpoint {
            kv_pairs.push(format!(
                "{}={}",
                TABLE_ENDPOINT_KEY_NAME,
                table_endpoint
            ));
        }
        if let Some(table_secondary_endpoint) = self.0.table_secondary_endpoint {
            kv_pairs.push(format!(
                "{}={}",
                TABLE_SECONDARY_ENDPOINT_KEY_NAME,
                table_secondary_endpoint
            ));
        }
        if let Some(queue_endpoint) = self.0.queue_endpoint {
            kv_pairs.push(format!(
                "{}={}",
                QUEUE_ENDPOINT_KEY_NAME,
                queue_endpoint
            ));
        }
        if let Some(queue_secondary_endpoint) = self.0.queue_secondary_endpoint {
            kv_pairs.push(format!(
                "{}={}",
                QUEUE_SECONDARY_ENDPOINT_KEY_NAME,
                queue_secondary_endpoint
            ));
        }
        if let Some(file_endpoint) = self.0.file_endpoint {
            kv_pairs.push(format!(
                "{}={}",
                FILE_ENDPOINT_KEY_NAME,
                file_endpoint
            ));
        }
        if let Some(file_secondary_endpoint) = self.0.file_secondary_endpoint {
            kv_pairs.push(format!(
                "{}={}",
                FILE_SECONDARY_ENDPOINT_KEY_NAME,
                file_secondary_endpoint
            ));
        }

        kv_pairs.join(";")
    }

    pub fn with_account_name(&'a mut self, account_name: &'a str) -> &'a mut Self {
        self.0.account_name = Some(account_name);
        self
    }

    pub fn with_account_key(&'a mut self, account_key: &'a str) -> &'a mut Self {
        self.0.account_key = Some(account_key);
        self
    }

    pub fn with_sas(&'a mut self, sas: &'a str) -> &'a mut Self {
        self.0.sas = Some(sas);
        self
    }

    pub fn with_endpoint_suffix(&'a mut self, endpoint_suffix: &'a str) -> &'a mut Self {
        self.0.endpoint_suffix = Some(endpoint_suffix);
        self
    }

    pub fn with_default_endpoints_protocol(
        &'a mut self,
        default_endpoints_protocol: EndpointProtocol,
    ) -> &'a mut Self {
        self.0.default_endpoints_protocol = Some(default_endpoints_protocol);
        self
    }

    pub fn with_use_development_storage(
        &'a mut self,
        use_development_storage: bool,
    ) -> &'a mut Self {
        self.0.use_development_storage = Some(use_development_storage);
        self
    }

    pub fn with_development_storage_proxy_uri(
        &'a mut self,
        development_storage_proxy_uri: &'a str,
    ) -> &'a mut Self {
        self.0.development_storage_proxy_uri = Some(development_storage_proxy_uri);
        self
    }

    pub fn with_blob_endpoint(&'a mut self, blob_endpoint: &'a str) -> &'a mut Self {
        self.0.blob_endpoint = Some(blob_endpoint);
        self
    }

    pub fn with_blob_secondary_endpoint(
        &'a mut self,
        blob_secondary_endpoint: &'a str,
    ) -> &'a mut Self {
        self.0.blob_secondary_endpoint = Some(blob_secondary_endpoint);
        self
    }

    pub fn with_table_endpoint(&'a mut self, table_endpoint: &'a str) -> &'a mut Self {
        self.0.table_endpoint = Some(table_endpoint);
        self
    }

    pub fn with_table_secondary_endpoint(
        &'a mut self,
        table_secondary_endpoint: &'a str,
    ) -> &'a mut Self {
        self.0.table_secondary_endpoint = Some(table_secondary_endpoint);
        self
    }

    pub fn with_queue_endpoint(&'a mut self, queue_endpoint: &'a str) -> &'a mut Self {
        self.0.queue_endpoint = Some(queue_endpoint);
        self
    }

    pub fn with_queue_secondary_endpoint(
        &'a mut self,
        queue_secondary_endpoint: &'a str,
    ) -> &'a mut Self {
        self.0.queue_secondary_endpoint = Some(queue_secondary_endpoint);
        self
    }

    pub fn with_file_endpoint(&'a mut self, file_endpoint: &'a str) -> &'a mut Self {
        self.0.file_endpoint = Some(file_endpoint);
        self
    }

    pub fn with_file_secondary_endpoint(
        &'a mut self,
        file_secondary_endpoint: &'a str,
    ) -> &'a mut Self {
        self.0.file_secondary_endpoint = Some(file_secondary_endpoint);
        self
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn it_builds_generic_connection_strings() {
        assert_eq!(ConnectionStringBuilder::new().build(), "");
        assert_eq!(
            ConnectionStringBuilder::new()
                .with_account_name("a")
                .with_account_key("b")
                .build(),
            "AccountName=a;AccountKey=b"
        );
        assert_eq!(
            ConnectionStringBuilder::new()
                .with_account_name("a")
                .with_sas("b")
                .with_default_endpoints_protocol(EndpointProtocol::Https)
                .with_blob_endpoint("c")
                .build(),
            "AccountName=a;SharedAccessSignature=b;DefaultEndpointsProtocol=https;BlobEndpoint=c"
        );
    }

    #[test]
    fn it_builds_endpoints_with_development_storage() {
        assert_eq!(
            ConnectionStringBuilder::new()
                .with_use_development_storage(true)
                .with_development_storage_proxy_uri("a")
                .build(),
            "UseDevelopmentStorage=true;DevelopmentStorageProxyUri=a"
        );
        assert_eq!(
            ConnectionStringBuilder::new()
                .with_use_development_storage(false)
                .build(),
            "UseDevelopmentStorage=false"
        );
    }

    #[test]
    fn it_builds_all_endpoints() {
        assert_eq!(
            ConnectionStringBuilder::new()
                .with_blob_endpoint("b1")
                .with_blob_secondary_endpoint("b2")
                .with_table_endpoint("t1")
                .with_table_secondary_endpoint("t2")
                .with_queue_endpoint("q1")
                .with_queue_secondary_endpoint("q2")
                .with_file_endpoint("f1")
                .with_file_secondary_endpoint("f2")
                .build(),
            "BlobEndpoint=b1;BlobSecondaryEndpoint=b2;TableEndpoint=t1;TableSecondaryEndpoint=t2;QueueEndpoint=q1;QueueSecondaryEndpoint=q2;FileEndpoint=f1;FileSecondaryEndpoint=f2"
        );
    }
}
