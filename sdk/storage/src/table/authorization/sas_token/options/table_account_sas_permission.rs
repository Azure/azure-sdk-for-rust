use azure_core::SasError;

use super::constants::*;
use std::str::FromStr;

/// Specifies the signed permissions for the account SAS.
/// Permissions are only valid if they match the specified signed resource type; otherwise they are ignored.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TableAccountSasPermission {
    /// Indicates that Read is permitted.
    Read,
    /// Indicates that Write is permitted.
    Write,
    /// Indicates that Delete is permitted.
    Delete,
    /// Indicates that List is permitted.
    List,
    /// Indicates that Add is permitted.
    Add,
    /// Indicates that Update is permitted.
    Update,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct TableAccountSasPermissions([char; 6]);

impl TableAccountSasPermissions {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn all() -> Self {
        Self(['r', 'w', 'd', 'l', 'a', 'u'])
    }

    pub fn add_permission(mut self, permission: TableAccountSasPermission) -> Self {
        match permission {
            TableAccountSasPermission::Read => self.0[0] = READ,
            TableAccountSasPermission::Write => self.0[1] = WRITE,
            TableAccountSasPermission::Delete => self.0[2] = DELETE,
            TableAccountSasPermission::List => self.0[3] = LIST,
            TableAccountSasPermission::Add => self.0[4] = ADD,
            TableAccountSasPermission::Update => self.0[5] = UPDATE,
        };
        self
    }
}

impl From<TableAccountSasPermissions> for String {
    fn from(builder: TableAccountSasPermissions) -> Self {
        let mut permissions = String::with_capacity(builder.0.len());
        for permission in builder.0 {
            if permission != char::default() {
                permissions.push(permission);
            }
        }
        permissions
    }
}

impl FromStr for TableAccountSasPermissions {
    type Err = SasError;

    fn from_str(slice: &str) -> Result<Self, Self::Err> {
        let permissions = TableAccountSasPermissions::new();
        for permissions_char in slice.chars() {
            let permission = match permissions_char {
                READ => Ok(TableAccountSasPermission::Read),
                WRITE => Ok(TableAccountSasPermission::Write),
                DELETE => Ok(TableAccountSasPermission::Delete),
                LIST => Ok(TableAccountSasPermission::List),
                ADD => Ok(TableAccountSasPermission::Add),
                UPDATE => Ok(TableAccountSasPermission::Update),
                _ => {
                    return Err(SasError::SasParsingError {
                        field: "".into(),
                        input: "".into(),
                    })
                }
            }?;
            permissions.add_permission(permission);
        }
        Ok(permissions)
    }
}
