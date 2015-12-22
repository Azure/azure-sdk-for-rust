pub mod client;
#[macro_use]
pub mod blob;
pub mod container;


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
