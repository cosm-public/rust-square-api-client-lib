//! For getting library/crate package information defined in Cargo.toml

use package_info_derive::PackageInfo;

/// For getting library/crate package information defined in Cargo.toml
/// see [package_info](https://crates.io/crates/package_info)
#[derive(PackageInfo)]
pub(crate) struct CargoPackageInfo;
