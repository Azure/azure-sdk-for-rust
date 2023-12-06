#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-streaming-2022-11")]
pub mod package_streaming_2022_11;
#[cfg(all(feature = "package-streaming-2022-11", not(feature = "without_tag_import")))]
pub use package_streaming_2022_11::*;
#[cfg(feature = "package-metadata-2023-01")]
pub mod package_metadata_2023_01;
#[cfg(all(feature = "package-metadata-2023-01", not(feature = "without_tag_import")))]
pub use package_metadata_2023_01::*;
#[cfg(feature = "package-metadata-2022-08")]
pub mod package_metadata_2022_08;
#[cfg(all(feature = "package-metadata-2022-08", not(feature = "without_tag_import")))]
pub use package_metadata_2022_08::*;
#[cfg(feature = "package-encoding-2022-07")]
pub mod package_encoding_2022_07;
#[cfg(all(feature = "package-encoding-2022-07", not(feature = "without_tag_import")))]
pub use package_encoding_2022_07::*;
#[cfg(feature = "package-encoding-2022-05-preview")]
pub mod package_encoding_2022_05_preview;
#[cfg(all(feature = "package-encoding-2022-05-preview", not(feature = "without_tag_import")))]
pub use package_encoding_2022_05_preview::*;
