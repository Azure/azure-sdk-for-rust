#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-features-2021-07")]
pub mod package_features_2021_07;
#[cfg(feature = "package-locks-2020-05")]
pub mod package_locks_2020_05;
#[cfg(feature = "package-policy-2021-06")]
pub mod package_policy_2021_06;
#[cfg(feature = "package-resources-2021-04")]
pub mod package_resources_2021_04;
#[cfg(feature = "package-subscriptions-2021-01")]
pub mod package_subscriptions_2021_01;
#[cfg(all(feature = "default_tag", feature = "package-resources-2021-04"))]
pub use package_resources_2021_04::*;
