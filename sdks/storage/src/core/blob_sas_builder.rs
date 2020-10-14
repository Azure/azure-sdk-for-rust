use super::rest_client::{generate_storage_sas, SASType};
use super::ClientEndpoint;
use super::IPRange;
use azure_core::{No, ToAssign, Yes};
use chrono::{DateTime, Utc};
use std::marker::PhantomData;
use url::Url;

#[derive(Debug, Clone)]
pub struct KeySet {}
impl ToAssign for KeySet {}
#[derive(Debug, Clone)]
pub struct ValidityEndSet {}
impl ToAssign for ValidityEndSet {}
#[derive(Debug, Clone)]
pub struct AtLeastOnePermission {}
impl ToAssign for AtLeastOnePermission {}

impl<'a, ValidityEndSet, AtLeastOnePermission> ClientEndpoint
    for BlobSASBuilder<'a, Yes, ValidityEndSet, AtLeastOnePermission>
where
    ValidityEndSet: ToAssign,
    AtLeastOnePermission: ToAssign,
{
    fn account(&self) -> &str {
        match self.path.host().unwrap().clone() {
            url::Host::Domain(dm) => {
                let first_dot = dm.find('.').unwrap();
                &dm[0..first_dot]
            }
            url::Host::Ipv4(_) => {
                panic!("IP addresses are not supported in SAS tokens right now");
            }
            _ => panic!("only Domains are supported in canonicalized_resource"),
        }
    }

    fn key(&self) -> &str {
        &self.key()
    }
}

#[derive(Debug, Clone)]
pub struct BlobSASBuilder<'a, KeySet, ValidityEndSet, AtLeastOnePermission>
where
    KeySet: ToAssign,
    ValidityEndSet: ToAssign,
    AtLeastOnePermission: ToAssign,
{
    path: &'a Url,
    p_key: PhantomData<KeySet>,
    p_validity_end: PhantomData<ValidityEndSet>,
    at_least_one_permission: PhantomData<AtLeastOnePermission>,
    key: Option<&'a str>,
    identifier: Option<&'a str>,
    ip_range: Option<&'a IPRange>,
    validity_start: Option<&'a DateTime<Utc>>,
    validity_end: Option<&'a DateTime<Utc>>,
    snapshot_time: Option<&'a DateTime<Utc>>,
    cache_control: Option<&'a str>,
    content_disposition: Option<&'a str>,
    content_encoding: Option<&'a str>,
    content_language: Option<&'a str>,
    content_type: Option<&'a str>,
    allow_read: bool,
    allow_add: bool,
    allow_create: bool,
    allow_write: bool,
    allow_delete: bool,
}

impl<'a> BlobSASBuilder<'a, No, No, No> {
    #[inline]
    pub fn new(path: &'a Url) -> BlobSASBuilder<'a, No, No, No> {
        BlobSASBuilder {
            path,
            p_key: PhantomData {},
            key: None,
            p_validity_end: PhantomData {},
            at_least_one_permission: PhantomData {},
            validity_end: None,
            identifier: None,
            ip_range: None,
            validity_start: None,
            snapshot_time: None,
            cache_control: None,
            content_disposition: None,
            content_encoding: None,
            content_language: None,
            content_type: None,
            allow_read: false,
            allow_add: false,
            allow_create: false,
            allow_write: false,
            allow_delete: false,
        }
    }
}

impl<'a, KeySet, ValidityEndSet, AtLeastOnePermission>
    BlobSASBuilder<'a, KeySet, ValidityEndSet, AtLeastOnePermission>
where
    KeySet: ToAssign,
    ValidityEndSet: ToAssign,
    AtLeastOnePermission: ToAssign,
{
    #[inline]
    pub fn with_key(
        &self,
        key: &'a str,
    ) -> BlobSASBuilder<'a, Yes, ValidityEndSet, AtLeastOnePermission> {
        BlobSASBuilder {
            path: self.path,
            p_key: PhantomData {},
            key: Some(key),
            p_validity_end: self.p_validity_end,
            at_least_one_permission: self.at_least_one_permission,
            validity_end: self.validity_end,
            identifier: self.identifier,
            ip_range: self.ip_range,
            validity_start: self.validity_start,
            snapshot_time: self.snapshot_time,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            content_type: self.content_type,
            allow_read: self.allow_read,
            allow_add: self.allow_add,
            allow_create: self.allow_create,
            allow_write: self.allow_write,
            allow_delete: self.allow_delete,
        }
    }
}

impl<'a, ValidityEndSet, AtLeastOnePermission>
    BlobSASBuilder<'a, Yes, ValidityEndSet, AtLeastOnePermission>
where
    ValidityEndSet: ToAssign,
    AtLeastOnePermission: ToAssign,
{
    #[inline]
    pub fn key(&self) -> &'a str {
        self.key.unwrap()
    }
}

impl<'a, KeySet, ValidityEndSet, AtLeastOnePermission>
    BlobSASBuilder<'a, KeySet, ValidityEndSet, AtLeastOnePermission>
where
    KeySet: ToAssign,
    ValidityEndSet: ToAssign,
    AtLeastOnePermission: ToAssign,
{
    #[inline]
    pub fn with_validity_end(
        &self,
        validity_end: &'a DateTime<Utc>,
    ) -> BlobSASBuilder<'a, KeySet, Yes, AtLeastOnePermission> {
        BlobSASBuilder {
            path: self.path,
            p_key: self.p_key,
            key: self.key,
            p_validity_end: PhantomData {},
            at_least_one_permission: self.at_least_one_permission,
            validity_end: Some(validity_end),
            identifier: self.identifier,
            ip_range: self.ip_range,
            validity_start: self.validity_start,
            snapshot_time: self.snapshot_time,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            content_type: self.content_type,
            allow_read: self.allow_read,
            allow_add: self.allow_add,
            allow_create: self.allow_create,
            allow_write: self.allow_write,
            allow_delete: self.allow_delete,
        }
    }
}

impl<'a, KeySet, AtLeastOnePermission> BlobSASBuilder<'a, KeySet, Yes, AtLeastOnePermission>
where
    KeySet: ToAssign,
    AtLeastOnePermission: ToAssign,
{
    #[inline]
    pub fn validity_end(&self) -> &'a DateTime<Utc> {
        self.validity_end.unwrap()
    }
}

impl<'a, KeySet, ValidityEndSet, AtLeastOnePermission>
    BlobSASBuilder<'a, KeySet, ValidityEndSet, AtLeastOnePermission>
where
    KeySet: ToAssign,
    ValidityEndSet: ToAssign,
    AtLeastOnePermission: ToAssign,
{
    #[inline]
    pub fn allow_read(&self) -> BlobSASBuilder<'a, KeySet, ValidityEndSet, Yes> {
        BlobSASBuilder {
            path: self.path,
            p_key: self.p_key,
            key: self.key,
            p_validity_end: self.p_validity_end,
            at_least_one_permission: PhantomData {},
            validity_end: self.validity_end,
            identifier: self.identifier,
            ip_range: self.ip_range,
            validity_start: self.validity_start,
            snapshot_time: self.snapshot_time,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            content_type: self.content_type,
            allow_read: true,
            allow_add: self.allow_add,
            allow_create: self.allow_create,
            allow_write: self.allow_write,
            allow_delete: self.allow_delete,
        }
    }

    #[inline]
    pub fn allow_add(&self) -> BlobSASBuilder<'a, KeySet, ValidityEndSet, Yes> {
        BlobSASBuilder {
            path: self.path,
            p_key: self.p_key,
            key: self.key,
            p_validity_end: self.p_validity_end,
            at_least_one_permission: PhantomData {},
            validity_end: self.validity_end,
            identifier: self.identifier,
            ip_range: self.ip_range,
            validity_start: self.validity_start,
            snapshot_time: self.snapshot_time,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            content_type: self.content_type,
            allow_read: self.allow_read,
            allow_add: true,
            allow_create: self.allow_create,
            allow_write: self.allow_write,
            allow_delete: self.allow_delete,
        }
    }

    #[inline]
    pub fn allow_create(&self) -> BlobSASBuilder<'a, KeySet, ValidityEndSet, Yes> {
        BlobSASBuilder {
            path: self.path,
            p_key: self.p_key,
            key: self.key,
            p_validity_end: self.p_validity_end,
            at_least_one_permission: PhantomData {},
            validity_end: self.validity_end,
            identifier: self.identifier,
            ip_range: self.ip_range,
            validity_start: self.validity_start,
            snapshot_time: self.snapshot_time,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            content_type: self.content_type,
            allow_read: self.allow_read,
            allow_add: self.allow_add,
            allow_create: true,
            allow_write: self.allow_write,
            allow_delete: self.allow_delete,
        }
    }

    #[inline]
    pub fn allow_write(&self) -> BlobSASBuilder<'a, KeySet, ValidityEndSet, Yes> {
        BlobSASBuilder {
            path: self.path,
            p_key: self.p_key,
            key: self.key,
            p_validity_end: self.p_validity_end,
            at_least_one_permission: PhantomData {},
            validity_end: self.validity_end,
            identifier: self.identifier,
            ip_range: self.ip_range,
            validity_start: self.validity_start,
            snapshot_time: self.snapshot_time,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            content_type: self.content_type,
            allow_read: self.allow_read,
            allow_add: self.allow_add,
            allow_create: self.allow_create,
            allow_write: true,
            allow_delete: self.allow_delete,
        }
    }

    #[inline]
    pub fn allow_delete(&self) -> BlobSASBuilder<'a, KeySet, ValidityEndSet, Yes> {
        BlobSASBuilder {
            path: self.path,
            p_key: self.p_key,
            key: self.key,
            p_validity_end: self.p_validity_end,
            at_least_one_permission: PhantomData {},
            validity_end: self.validity_end,
            identifier: self.identifier,
            ip_range: self.ip_range,
            validity_start: self.validity_start,
            snapshot_time: self.snapshot_time,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            content_type: self.content_type,
            allow_read: self.allow_read,
            allow_add: self.allow_add,
            allow_create: self.allow_create,
            allow_write: self.allow_write,
            allow_delete: true,
        }
    }
}

// methods callable regardless
impl<'a, KeySet, ValidityEndSet, AtLeastOnePermission>
    BlobSASBuilder<'a, KeySet, ValidityEndSet, AtLeastOnePermission>
where
    KeySet: ToAssign,
    ValidityEndSet: ToAssign,
    AtLeastOnePermission: ToAssign,
{
    fn path(&self) -> &'a Url {
        self.path
    }

    #[inline]
    pub fn identifier(&self) -> Option<&'a str> {
        self.identifier
    }

    #[inline]
    pub fn ip_range(&self) -> Option<&'a IPRange> {
        self.ip_range
    }

    #[inline]
    pub fn validity_start(&self) -> Option<&'a DateTime<Utc>> {
        self.validity_start
    }

    #[inline]
    pub fn snapshot_time(&self) -> Option<&'a DateTime<Utc>> {
        self.snapshot_time
    }

    #[inline]
    pub fn cache_control(&self) -> Option<&'a str> {
        self.cache_control
    }

    #[inline]
    pub fn content_disposition(&self) -> Option<&'a str> {
        self.content_disposition
    }

    #[inline]
    pub fn content_encoding(&self) -> Option<&'a str> {
        self.content_encoding
    }

    #[inline]
    pub fn content_language(&self) -> Option<&'a str> {
        self.content_language
    }

    #[inline]
    pub fn content_type(&self) -> Option<&'a str> {
        self.content_type
    }

    #[inline]
    pub fn can_read(&self) -> bool {
        self.allow_read
    }
    #[inline]
    pub fn can_add(&self) -> bool {
        self.allow_add
    }
    #[inline]
    pub fn can_create(&self) -> bool {
        self.allow_create
    }
    #[inline]
    pub fn can_write(&self) -> bool {
        self.allow_write
    }
    #[inline]
    pub fn can_delete(&self) -> bool {
        self.allow_delete
    }

    #[inline]
    pub fn with_identifier(self, identifier: &'a str) -> Self {
        BlobSASBuilder {
            path: self.path,
            p_key: self.p_key,
            key: self.key,
            p_validity_end: self.p_validity_end,
            at_least_one_permission: self.at_least_one_permission,
            validity_end: self.validity_end,
            identifier: Some(identifier),
            ip_range: self.ip_range,
            validity_start: self.validity_start,
            snapshot_time: self.snapshot_time,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            content_type: self.content_type,
            allow_read: self.allow_read,
            allow_add: self.allow_add,
            allow_create: self.allow_create,
            allow_write: self.allow_write,
            allow_delete: self.allow_delete,
        }
    }

    #[inline]
    pub fn with_ip_range(self, ip_range: &'a IPRange) -> Self {
        BlobSASBuilder {
            path: self.path,
            p_key: self.p_key,
            key: self.key,
            p_validity_end: self.p_validity_end,
            at_least_one_permission: self.at_least_one_permission,
            validity_end: self.validity_end,
            identifier: self.identifier,
            ip_range: Some(ip_range),
            validity_start: self.validity_start,
            snapshot_time: self.snapshot_time,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            content_type: self.content_type,
            allow_read: self.allow_read,
            allow_add: self.allow_add,
            allow_create: self.allow_create,
            allow_write: self.allow_write,
            allow_delete: self.allow_delete,
        }
    }

    #[inline]
    pub fn with_validity_start(self, validity_start: &'a DateTime<Utc>) -> Self {
        BlobSASBuilder {
            path: self.path,
            p_key: self.p_key,
            key: self.key,
            p_validity_end: self.p_validity_end,
            at_least_one_permission: self.at_least_one_permission,
            validity_end: self.validity_end,
            identifier: self.identifier,
            ip_range: self.ip_range,
            validity_start: Some(validity_start),
            snapshot_time: self.snapshot_time,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            content_type: self.content_type,
            allow_read: self.allow_read,
            allow_add: self.allow_add,
            allow_create: self.allow_create,
            allow_write: self.allow_write,
            allow_delete: self.allow_delete,
        }
    }

    #[inline]
    pub fn with_snapshot_time(self, snapshot_time: &'a DateTime<Utc>) -> Self {
        BlobSASBuilder {
            path: self.path,
            p_key: self.p_key,
            key: self.key,
            p_validity_end: self.p_validity_end,
            at_least_one_permission: self.at_least_one_permission,
            validity_end: self.validity_end,
            identifier: self.identifier,
            ip_range: self.ip_range,
            validity_start: self.validity_start,
            snapshot_time: Some(snapshot_time),
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            content_type: self.content_type,
            allow_read: self.allow_read,
            allow_add: self.allow_add,
            allow_create: self.allow_create,
            allow_write: self.allow_write,
            allow_delete: self.allow_delete,
        }
    }

    #[inline]
    pub fn with_cache_control(self, cache_control: &'a str) -> Self {
        BlobSASBuilder {
            path: self.path,
            p_key: self.p_key,
            key: self.key,
            p_validity_end: self.p_validity_end,
            at_least_one_permission: self.at_least_one_permission,
            validity_end: self.validity_end,
            identifier: self.identifier,
            ip_range: self.ip_range,
            validity_start: self.validity_start,
            snapshot_time: self.snapshot_time,
            cache_control: Some(cache_control),
            content_disposition: self.content_disposition,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            content_type: self.content_type,
            allow_read: self.allow_read,
            allow_add: self.allow_add,
            allow_create: self.allow_create,
            allow_write: self.allow_write,
            allow_delete: self.allow_delete,
        }
    }

    #[inline]
    pub fn with_content_disposition(self, content_disposition: &'a str) -> Self {
        BlobSASBuilder {
            path: self.path,
            p_key: self.p_key,
            key: self.key,
            p_validity_end: self.p_validity_end,
            at_least_one_permission: self.at_least_one_permission,
            validity_end: self.validity_end,
            identifier: self.identifier,
            ip_range: self.ip_range,
            validity_start: self.validity_start,
            snapshot_time: self.snapshot_time,
            cache_control: self.cache_control,
            content_disposition: Some(content_disposition),
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            content_type: self.content_type,
            allow_read: self.allow_read,
            allow_add: self.allow_add,
            allow_create: self.allow_create,
            allow_write: self.allow_write,
            allow_delete: self.allow_delete,
        }
    }

    #[inline]
    pub fn with_content_encoding(self, content_encoding: &'a str) -> Self {
        BlobSASBuilder {
            path: self.path,
            p_key: self.p_key,
            key: self.key,
            p_validity_end: self.p_validity_end,
            at_least_one_permission: self.at_least_one_permission,
            validity_end: self.validity_end,
            identifier: self.identifier,
            ip_range: self.ip_range,
            validity_start: self.validity_start,
            snapshot_time: self.snapshot_time,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            content_encoding: Some(content_encoding),
            content_language: self.content_language,
            content_type: self.content_type,
            allow_read: self.allow_read,
            allow_add: self.allow_add,
            allow_create: self.allow_create,
            allow_write: self.allow_write,
            allow_delete: self.allow_delete,
        }
    }

    #[inline]
    pub fn with_content_language(self, content_language: &'a str) -> Self {
        BlobSASBuilder {
            path: self.path,
            p_key: self.p_key,
            key: self.key,
            p_validity_end: self.p_validity_end,
            at_least_one_permission: self.at_least_one_permission,
            validity_end: self.validity_end,
            identifier: self.identifier,
            ip_range: self.ip_range,
            validity_start: self.validity_start,
            snapshot_time: self.snapshot_time,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            content_encoding: self.content_encoding,
            content_language: Some(content_language),
            content_type: self.content_type,
            allow_read: self.allow_read,
            allow_add: self.allow_add,
            allow_create: self.allow_create,
            allow_write: self.allow_write,
            allow_delete: self.allow_delete,
        }
    }

    #[inline]
    pub fn with_content_type(self, content_type: &'a str) -> Self {
        BlobSASBuilder {
            path: self.path,
            p_key: self.p_key,
            key: self.key,
            p_validity_end: self.p_validity_end,
            at_least_one_permission: self.at_least_one_permission,
            validity_end: self.validity_end,
            identifier: self.identifier,
            ip_range: self.ip_range,
            validity_start: self.validity_start,
            snapshot_time: self.snapshot_time,
            cache_control: self.cache_control,
            content_disposition: self.content_disposition,
            content_encoding: self.content_encoding,
            content_language: self.content_language,
            content_type: Some(content_type),
            allow_read: self.allow_read,
            allow_add: self.allow_add,
            allow_create: self.allow_create,
            allow_write: self.allow_write,
            allow_delete: self.allow_delete,
        }
    }
}

// methods callable only when fully constructed
impl<'a> BlobSASBuilder<'a, Yes, Yes, Yes> {
    pub fn finalize(self) -> Url {
        let sas = generate_storage_sas(
            &self,
            self.validity_start(),
            self.validity_end(),
            self.path(),
            &self.permission_string(),
            if let Some(identifier) = self.identifier() {
                identifier
            } else {
                ""
            },
            self.ip_range(),
            SASType::Blob,
            self.snapshot_time(),
            if let Some(cache_control) = self.cache_control() {
                cache_control
            } else {
                ""
            },
            if let Some(content_disposition) = self.content_disposition() {
                content_disposition
            } else {
                ""
            },
            if let Some(content_encoding) = self.content_encoding() {
                content_encoding
            } else {
                ""
            },
            if let Some(content_language) = self.content_language() {
                content_language
            } else {
                ""
            },
            if let Some(content_type) = self.content_type() {
                content_type
            } else {
                ""
            },
            "",
            "",
            "",
            "",
        );

        if self.path().query().is_some() {
            Url::parse(&format!("{}&{}", self.path(), &sas)).unwrap()
        } else {
            Url::parse(&format!("{}?{}", self.path(), &sas)).unwrap()
        }
    }

    pub fn permission_string(&self) -> String {
        let mut s = String::with_capacity(5);
        if self.can_read() {
            s.push('r')
        }
        if self.can_add() {
            s.push('a')
        }
        if self.can_create() {
            s.push('c')
        }
        if self.can_write() {
            s.push('w')
        }
        if self.can_delete() {
            s.push('d')
        }

        s
    }
}
