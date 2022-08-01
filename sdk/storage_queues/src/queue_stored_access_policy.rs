use azure_core::error::{Error, ErrorKind};
use azure_storage::StoredAccessPolicy;
use std::convert::TryFrom;
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct QueueStoredAccessPolicy {
    pub id: String,
    pub start: OffsetDateTime,
    pub expiry: OffsetDateTime,
    pub is_read_enabled: bool,
    pub is_add_enabled: bool,
    pub is_update_enabled: bool,
    pub is_process_enabled: bool,
}

impl QueueStoredAccessPolicy {
    pub fn new(
        id: impl Into<String>,
        start: impl Into<OffsetDateTime>,
        expiry: impl Into<OffsetDateTime>,
    ) -> Self {
        Self {
            id: id.into(),
            start: start.into(),
            expiry: expiry.into(),
            is_read_enabled: false,
            is_add_enabled: false,
            is_update_enabled: false,
            is_process_enabled: false,
        }
    }

    pub fn enable_read(self) -> Self {
        Self {
            is_read_enabled: true,
            ..self
        }
    }

    pub fn enable_add(self) -> Self {
        Self {
            is_add_enabled: true,
            ..self
        }
    }

    pub fn enable_update(self) -> Self {
        Self {
            is_update_enabled: true,
            ..self
        }
    }

    pub fn enable_process(self) -> Self {
        Self {
            is_process_enabled: true,
            ..self
        }
    }

    pub fn enable_all(self) -> Self {
        Self {
            is_add_enabled: true,
            is_read_enabled: true,
            is_update_enabled: true,
            is_process_enabled: true,
            ..self
        }
    }

    pub fn to_permission_string(&self) -> String {
        let mut permission = String::with_capacity(4);

        if self.is_read_enabled {
            permission.push('r');
        }
        if self.is_add_enabled {
            permission.push('a');
        }
        if self.is_update_enabled {
            permission.push('u');
        }
        if self.is_process_enabled {
            permission.push('p');
        }

        permission
    }
}

impl TryFrom<StoredAccessPolicy> for QueueStoredAccessPolicy {
    type Error = Error;

    fn try_from(sap: StoredAccessPolicy) -> azure_core::Result<Self> {
        let mut queue_sap = Self::new(sap.id, sap.start, sap.expiry);

        for token in sap.permission.chars() {
            match token {
                'r' => {
                    queue_sap = queue_sap.enable_read();
                }
                'a' => {
                    queue_sap = queue_sap.enable_add();
                }
                'u' => {
                    queue_sap = queue_sap.enable_update();
                }
                'p' => {
                    queue_sap = queue_sap.enable_process();
                }
                c => {
                    return Err(Error::with_message(ErrorKind::Credential, || {
                        format!(
                        "Permission token not supported in this service ({}). Received token {}, supported tokens {:?}",
                        "queue",
                        c,
                        vec!['r', 'a', 'u', 'p'],
                        )
                    }))
                }
            }
        }

        Ok(queue_sap)
    }
}

impl From<QueueStoredAccessPolicy> for StoredAccessPolicy {
    fn from(queue_stored_access_policy: QueueStoredAccessPolicy) -> Self {
        let permission = queue_stored_access_policy.to_permission_string();
        StoredAccessPolicy {
            id: queue_stored_access_policy.id,
            start: queue_stored_access_policy.start,
            expiry: queue_stored_access_policy.expiry,
            permission,
        }
    }
}
