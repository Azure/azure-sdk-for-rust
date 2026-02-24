// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[repr(C)]
pub struct ClientOptions {
    /// If true, disables certificate validation. Use only for testing.
    danger_allow_invalid_certificates: bool,
}

impl ClientOptions {
    /// Returns whether to allow invalid TLS certificates.
    pub fn allow_invalid_certificates(&self) -> bool {
        self.danger_allow_invalid_certificates
    }
}

// Empty structs are not permitted in standard C (though GNU/Clang allow it).
// So, several of these structs have placeholder fields for future expansion.

#[repr(C)]
pub struct QueryOptions {
    /// Placeholder field used to avoid an empty struct (which is not valid in standard C). This field may be removed at any time.
    _unused: u8,
}

#[repr(C)]
pub struct CreateDatabaseOptions {
    /// Placeholder field used to avoid an empty struct (which is not valid in standard C). This field may be removed at any time.
    _unused: u8,
}

#[repr(C)]
pub struct ReadDatabaseOptions {
    /// Placeholder field used to avoid an empty struct (which is not valid in standard C). This field may be removed at any time.
    _unused: u8,
}

#[repr(C)]
pub struct DeleteDatabaseOptions {
    /// Placeholder field used to avoid an empty struct (which is not valid in standard C). This field may be removed at any time.
    _unused: u8,
}

#[repr(C)]
pub struct CreateContainerOptions {
    /// Placeholder field used to avoid an empty struct (which is not valid in standard C). This field may be removed at any time.
    _unused: u8,
}

#[repr(C)]
pub struct ReadContainerOptions {
    /// Placeholder field used to avoid an empty struct (which is not valid in standard C). This field may be removed at any time.
    _unused: u8,
}

#[repr(C)]
pub struct DeleteContainerOptions {
    /// Placeholder field used to avoid an empty struct (which is not valid in standard C). This field may be removed at any time.
    _unused: u8,
}

#[repr(C)]
pub struct ItemOptions {
    /// Placeholder field used to avoid an empty struct (which is not valid in standard C). This field may be removed at any time.
    _unused: u8,
}
