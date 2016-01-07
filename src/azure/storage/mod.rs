mod client;
#[macro_use]
mod blob;
mod container;

pub use self::client::Client;
pub use self::blob::{Blob, BlobType, CopyStatus};
pub use self::container::{Container, PublicAccess};


use std::fmt;
use std::str::FromStr;
use azure::core::enumerations;

use azure::core::errors::TraversingError;
use azure::core::parsing::FromStringOptional;

create_enum!(LeaseStatus,
                            (Locked,        "locked"),
                            (Unlocked,      "unlocked")
);

create_enum!(LeaseState,
                            (Available,     "available"),
                            (Leased,        "leased"),
                            (Expired,       "expired"),
                            (Breaking,      "breaking"),
                            (Broken,        "broken")
);

create_enum!(LeaseDuration,
                            (Infinite,      "infinite"),
                            (Fixed,         "fixed")
);
