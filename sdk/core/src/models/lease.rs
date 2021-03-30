create_enum!(LeaseStatus, (Locked, "locked"), (Unlocked, "unlocked"));

create_enum!(
    LeaseState,
    (Available, "available"),
    (Leased, "leased"),
    (Expired, "expired"),
    (Breaking, "breaking"),
    (Broken, "broken")
);

create_enum!(LeaseDuration, (Infinite, "infinite"), (Fixed, "fixed"));

create_enum!(
    LeaseAction,
    (Acquire, "acquire"),
    (Renew, "renew "),
    (Change, "change"),
    (Release, "release "),
    (Break, "break")
);
