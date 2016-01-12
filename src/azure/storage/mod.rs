pub mod client;
pub mod container;
pub mod blob;

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
