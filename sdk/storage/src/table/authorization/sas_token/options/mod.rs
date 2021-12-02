pub mod table_account_sas_options;
pub mod table_account_sas_permission;
pub mod table_account_sas_resource_type;
pub mod table_sas_ip_range;
pub mod table_sas_options;
pub mod table_sas_protocol;

pub mod constants {

    pub const TABLE_ACCOUNT_SERVICES_IDENTIFIER: &str = "t";

    /// Gets the default service version to use when building shared access signatures.
    pub const SAS_VERSION: &str = "2019-07-07";

    // permissions
    pub const READ: char = 'r';
    pub const WRITE: char = 'w';
    pub const DELETE: char = 'd';
    pub const LIST: char = 'l';
    pub const ADD: char = 'a';
    pub const UPDATE: char = 'u';
    pub const CREATE: char = 'c';

    // resources
    pub const SERVICE: char = 's';
    pub const CONTAINER: char = 'c';
    pub const OBJECT: char = 'o';
}
//
