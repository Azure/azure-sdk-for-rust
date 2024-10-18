// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use typespec_client_core::create_enum;

create_enum!(
    #[doc = "Lease status of an Azure resource."]
    LeaseStatus,
    (Locked, "locked"),
    (Unlocked, "unlocked")
);

create_enum!(
    #[doc = "State of a lease of an Azure resource."]
    LeaseState,
    (Available, "available"),
    (Leased, "leased"),
    (Expired, "expired"),
    (Breaking, "breaking"),
    (Broken, "broken")
);

create_enum!(
    #[doc = "Lease duration of an Azure resource."]
    LeaseDuration,
    (Infinite, "infinite"),
    (Fixed, "fixed")
);

create_enum!(
    #[doc = "The lease action to perform on an Azure resource."]
    LeaseAction,
    (Acquire, "acquire"),
    (Renew, "renew "),
    (Change, "change"),
    (Release, "release "),
    (Break, "break")
);
