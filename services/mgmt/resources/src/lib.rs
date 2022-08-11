#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-subscriptions-2021-01")]
pub mod package_subscriptions_2021_01;
#[cfg(all(feature = "package-subscriptions-2021-01", not(feature = "no-default-tag")))]
pub use package_subscriptions_2021_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-resources-2021-04")]
pub mod package_resources_2021_04;
#[cfg(all(feature = "package-resources-2021-04", not(feature = "no-default-tag")))]
pub use package_resources_2021_04::{models, Client, ClientBuilder};
#[cfg(feature = "package-policy-2021-06")]
pub mod package_policy_2021_06;
#[cfg(all(feature = "package-policy-2021-06", not(feature = "no-default-tag")))]
pub use package_policy_2021_06::{models, Client, ClientBuilder};
#[cfg(feature = "package-locks-2020-05")]
pub mod package_locks_2020_05;
#[cfg(all(feature = "package-locks-2020-05", not(feature = "no-default-tag")))]
pub use package_locks_2020_05::{models, Client, ClientBuilder};
#[cfg(feature = "package-features-2021-07")]
pub mod package_features_2021_07;
#[cfg(all(feature = "package-features-2021-07", not(feature = "no-default-tag")))]
pub use package_features_2021_07::{models, Client, ClientBuilder};
