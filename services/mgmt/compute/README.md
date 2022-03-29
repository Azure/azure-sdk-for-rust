# azure_mgmt_compute crate

The is an [Azure SDK for Rust](https://github.com/Azure/azure-sdk-for-rust) crate that is generated from the Azure REST API specifications listed in:

https://github.com/Azure/azure-rest-api-specs/blob/main/specification/compute/resource-manager/readme.md

The default `Tag` is `package-2021-12-01`.

The following `Tag`s are available:

- `package-2021-12-01` has 276 operations from 5 API versions: `2021-03-01`, `2021-07-01`, `2021-10-01`, `2021-11-01`, `2021-12-01`. Use crate feature `package-2021-12-01` to enable. The operations will be in the `package_2021_12_01` module.
- `package-2021-12-01-only` has 38 operations from 1 API versions: `2021-12-01`. Use crate feature `package-2021-12-01-only` to enable. The operations will be in the `package_2021_12_01_only` module.
- `package-2021-11-01` has 276 operations from 5 API versions: `2021-03-01`, `2021-07-01`, `2021-08-01`, `2021-10-01`, `2021-11-01`. Use crate feature `package-2021-11-01` to enable. The operations will be in the `package_2021_11_01` module.
- `package-2021-11-01-only` has 171 operations from 1 API versions: `2021-11-01`. Use crate feature `package-2021-11-01-only` to enable. The operations will be in the `package_2021_11_01_only` module.
- `package-2021-08-01-only` has 38 operations from 1 API versions: `2021-08-01`. Use crate feature `package-2021-08-01-only` to enable. The operations will be in the `package_2021_08_01_only` module.
- `package-2021-08-01` has 275 operations from 3 API versions: `2021-03-01`, `2021-07-01`, `2021-08-01`. Use crate feature `package-2021-08-01` to enable. The operations will be in the `package_2021_08_01` module.
- `package-2021-10-01` has 275 operations from 4 API versions: `2021-03-01`, `2021-04-01`, `2021-07-01`, `2021-10-01`. Use crate feature `package-2021-10-01` to enable. The operations will be in the `package_2021_10_01` module.
- `package-2021-10-01-only` has 27 operations from 1 API versions: `2021-10-01`. Use crate feature `package-2021-10-01-only` to enable. The operations will be in the `package_2021_10_01_only` module.
- `package-2021-07-01` has 275 operations from 3 API versions: `2021-03-01`, `2021-04-01`, `2021-07-01`. Use crate feature `package-2021-07-01` to enable. The operations will be in the `package_2021_07_01` module.
- `package-2021-07-01-only` has 207 operations from 1 API versions: `2021-07-01`. Use crate feature `package-2021-07-01-only` to enable. The operations will be in the `package_2021_07_01_only` module.
- `package-2021-06-01-preview` has 265 operations from 5 API versions: `2019-04-01`, `2020-09-30`, `2020-12-01`, `2021-03-01`, `2021-06-01-preview`. Use crate feature `package-2021-06-01-preview` to enable. The operations will be in the `package_2021_06_01_preview` module.
- `package-2021-06-01-preview-only` has 4 operations from 1 API versions: `2021-06-01-preview`. Use crate feature `package-2021-06-01-preview-only` to enable. The operations will be in the `package_2021_06_01_preview_only` module.
- `package-2021-04-01` has 272 operations from 4 API versions: `2019-04-01`, `2020-09-30`, `2021-03-01`, `2021-04-01`. Use crate feature `package-2021-04-01` to enable. The operations will be in the `package_2021_04_01` module.
- `package-2021-04-01-only` has 208 operations from 1 API versions: `2021-04-01`. Use crate feature `package-2021-04-01-only` to enable. The operations will be in the `package_2021_04_01_only` module.
- `package-2021-03-01` has 261 operations from 4 API versions: `2019-04-01`, `2020-09-30`, `2020-12-01`, `2021-03-01`. Use crate feature `package-2021-03-01` to enable. The operations will be in the `package_2021_03_01` module.
- `package-2021-03-01-only` has 189 operations from 1 API versions: `2021-03-01`. Use crate feature `package-2021-03-01-only` to enable. The operations will be in the `package_2021_03_01_only` module.
- `package-2020-12-01` has 215 operations from 3 API versions: `2019-04-01`, `2019-12-01`, `2020-12-01`. Use crate feature `package-2020-12-01` to enable. The operations will be in the `package_2020_12_01` module.
- `package-2020-12-01-only` has 188 operations from 1 API versions: `2020-12-01`. Use crate feature `package-2020-12-01-only` to enable. The operations will be in the `package_2020_12_01_only` module.
- `package-2020-10-01-preview` has 237 operations from 6 API versions: `2017-01-31`, `2019-04-01`, `2019-12-01`, `2020-06-01`, `2020-09-30`, `2020-10-01-preview`. Use crate feature `package-2020-10-01-preview` to enable. The operations will be in the `package_2020_10_01_preview` module.
- `package-2020-10-01-preview-only` has 26 operations from 1 API versions: `2020-10-01-preview`. Use crate feature `package-2020-10-01-preview-only` to enable. The operations will be in the `package_2020_10_01_preview_only` module.
- `package-2020-09-30` has 218 operations from 4 API versions: `2017-01-31`, `2019-04-01`, `2020-06-01`, `2020-09-30`. Use crate feature `package-2020-09-30` to enable. The operations will be in the `package_2020_09_30` module.
- `package-2020-09-30-only` has 69 operations from 1 API versions: `2020-09-30`. Use crate feature `package-2020-09-30-only` to enable. The operations will be in the `package_2020_09_30_only` module.
- `package-2020-06-30` has 205 operations from 5 API versions: `2017-01-31`, `2019-04-01`, `2019-12-01`, `2020-06-01`, `2020-06-30`. Use crate feature `package-2020-06-30` to enable. The operations will be in the `package_2020_06_30` module.
- `package-2020-06-30-only` has 30 operations from 1 API versions: `2020-06-30`. Use crate feature `package-2020-06-30-only` to enable. The operations will be in the `package_2020_06_30_only` module.
- `package-2020-06-01` has 204 operations from 5 API versions: `2017-01-31`, `2019-04-01`, `2019-12-01`, `2020-05-01`, `2020-06-01`. Use crate feature `package-2020-06-01` to enable. The operations will be in the `package_2020_06_01` module.
- `package-2020-06-01-only` has 143 operations from 1 API versions: `2020-06-01`. Use crate feature `package-2020-06-01-only` to enable. The operations will be in the `package_2020_06_01_only` module.
- `package-2020-05-01` has 191 operations from 4 API versions: `2017-01-31`, `2019-04-01`, `2019-12-01`, `2020-05-01`. Use crate feature `package-2020-05-01` to enable. The operations will be in the `package_2020_05_01` module.
- `package-2020-05-01-only` has 29 operations from 1 API versions: `2020-05-01`. Use crate feature `package-2020-05-01-only` to enable. The operations will be in the `package_2020_05_01_only` module.
- `package-2019-12-01` has 184 operations from 4 API versions: `2017-01-31`, `2019-04-01`, `2019-11-01`, `2019-12-01`. Use crate feature `package-2019-12-01` to enable. The operations will be in the `package_2019_12_01` module.
- `package-2019-12-01-only` has 156 operations from 1 API versions: `2019-12-01`. Use crate feature `package-2019-12-01-only` to enable. The operations will be in the `package_2019_12_01_only` module.
- `package-2019-11-01` has 174 operations from 4 API versions: `2017-01-31`, `2019-04-01`, `2019-07-01`, `2019-11-01`. Use crate feature `package-2019-11-01` to enable. The operations will be in the `package_2019_11_01` module.
- `package-2019-11-01-only` has 22 operations from 1 API versions: `2019-11-01`. Use crate feature `package-2019-11-01-only` to enable. The operations will be in the `package_2019_11_01_only` module.
- `package-2019-07` has 174 operations from 3 API versions: `2017-01-31`, `2019-04-01`, `2019-07-01`. Use crate feature `package-2019-07` to enable. The operations will be in the `package_2019_07` module.
- `package-2019-07-01` has 167 operations from 4 API versions: `2017-01-31`, `2019-03-01`, `2019-04-01`, `2019-07-01`. Use crate feature `package-2019-07-01` to enable. The operations will be in the `package_2019_07_01` module.
- `package-2019-07-01-only` has 168 operations from 1 API versions: `2019-07-01`. Use crate feature `package-2019-07-01-only` to enable. The operations will be in the `package_2019_07_01_only` module.
- `package-2019-03-01` has 156 operations from 3 API versions: `2017-01-31`, `2019-03-01`, `2019-04-01`. Use crate feature `package-2019-03-01` to enable. The operations will be in the `package_2019_03_01` module.
- `package-2019-04-01-only` has 1 operations from 1 API versions: `2019-04-01`. Use crate feature `package-2019-04-01-only` to enable. The operations will be in the `package_2019_04_01_only` module.
- `package-2019-03-01-only` has 150 operations from 1 API versions: `2019-03-01`. Use crate feature `package-2019-03-01-only` to enable. The operations will be in the `package_2019_03_01_only` module.
- `package-2018-10-01-Disks` has 136 operations from 5 API versions: `2017-01-31`, `2017-09-01`, `2018-06-01`, `2018-09-30`, `2018-10-01`. Use crate feature `package-2018-10-01-Disks` to enable. The operations will be in the `package_2018_10_01_disks` module.
- `package-2018-10-01` has 136 operations from 4 API versions: `2017-01-31`, `2017-09-01`, `2018-06-01`, `2018-10-01`. Use crate feature `package-2018-10-01` to enable. The operations will be in the `package_2018_10_01` module.
- `package-2018-10-01-only` has 101 operations from 1 API versions: `2018-10-01`. Use crate feature `package-2018-10-01-only` to enable. The operations will be in the `package_2018_10_01_only` module.
- `package-2018-09-30-only` has 16 operations from 1 API versions: `2018-09-30`. Use crate feature `package-2018-09-30-only` to enable. The operations will be in the `package_2018_09_30_only` module.
- `package-2018-06-exclude-gallery` has 123 operations from 3 API versions: `2017-01-31`, `2017-09-01`, `2018-06-01`. Use crate feature `package-2018-06-exclude-gallery` to enable. The operations will be in the `package_2018_06_exclude_gallery` module.
- `package-2018-06` has 136 operations from 3 API versions: `2017-01-31`, `2017-09-01`, `2018-06-01`. Use crate feature `package-2018-06` to enable. The operations will be in the `package_2018_06` module.
- `package-2018-06-01` has 134 operations from 4 API versions: `2017-01-31`, `2017-09-01`, `2018-04-01`, `2018-06-01`. Use crate feature `package-2018-06-01` to enable. The operations will be in the `package_2018_06_01` module.
- `package-compute-only-2018-06` has 130 operations from 1 API versions: `2018-06-01`. Use crate feature `package-compute-only-2018-06` to enable. The operations will be in the `package_compute_only_2018_06` module.
- `package-2018-04-01` has 121 operations from 3 API versions: `2017-01-31`, `2017-09-01`, `2018-04-01`. Use crate feature `package-2018-04-01` to enable. The operations will be in the `package_2018_04_01` module.
- `package-2018-04` has 114 operations from 4 API versions: `2017-01-31`, `2017-09-01`, `2017-12-01`, `2018-04-01`. Use crate feature `package-2018-04` to enable. The operations will be in the `package_2018_04` module.
- `package-compute-2018-04` has 115 operations from 1 API versions: `2018-04-01`. Use crate feature `package-compute-2018-04` to enable. The operations will be in the `package_compute_2018_04` module.
- `package-disks-2018-04` has 16 operations from 1 API versions: `2018-04-01`. Use crate feature `package-disks-2018-04` to enable. The operations will be in the `package_disks_2018_04` module.
- `package-2017-12` has 114 operations from 4 API versions: `2017-01-31`, `2017-03-30`, `2017-09-01`, `2017-12-01`. Use crate feature `package-2017-12` to enable. The operations will be in the `package_2017_12` module.
- `package-compute-2017-12` has 109 operations from 3 API versions: `2017-03-30`, `2017-09-01`, `2017-12-01`. Use crate feature `package-compute-2017-12` to enable. The operations will be in the `package_compute_2017_12` module.
- `package-compute-only-2017-12` has 92 operations from 1 API versions: `2017-12-01`. Use crate feature `package-compute-only-2017-12` to enable. The operations will be in the `package_compute_only_2017_12` module.
- `package-skus-2017-09` has 1 operations from 1 API versions: `2017-09-01`. Use crate feature `package-skus-2017-09` to enable. The operations will be in the `package_skus_2017_09` module.
- `package-2017-03` has 101 operations from 2 API versions: `2017-01-31`, `2017-03-30`. Use crate feature `package-2017-03` to enable. The operations will be in the `package_2017_03` module.
- `package-compute-2017-03` has 96 operations from 1 API versions: `2017-03-30`. Use crate feature `package-compute-2017-03` to enable. The operations will be in the `package_compute_2017_03` module.
- `package-container-service-2017-01` has 5 operations from 1 API versions: `2017-01-31`. Use crate feature `package-container-service-2017-01` to enable. The operations will be in the `package_container_service_2017_01` module.
- `package-container-service-2016-09` has 5 operations from 1 API versions: `2016-09-30`. Use crate feature `package-container-service-2016-09` to enable. The operations will be in the `package_container_service_2016_09` module.
- `package-2016-04-preview` has 86 operations from 2 API versions: `2016-04-30-preview`, `2017-01-31`. Use crate feature `package-2016-04-preview` to enable. The operations will be in the `package_2016_04_preview` module.
- `package-compute-2016-04-preview` has 81 operations from 1 API versions: `2016-04-30-preview`. Use crate feature `package-compute-2016-04-preview` to enable. The operations will be in the `package_compute_2016_04_preview` module.
- `package-2016-03` has 62 operations from 1 API versions: `2016-03-30`. Use crate feature `package-2016-03` to enable. The operations will be in the `package_2016_03` module.
- `package-compute-2016-03` has 57 operations from 1 API versions: `2016-03-30`. Use crate feature `package-compute-2016-03` to enable. The operations will be in the `package_compute_2016_03` module.
- `package-container-service-2016-03` has 5 operations from 1 API versions: `2016-03-30`. Use crate feature `package-container-service-2016-03` to enable. The operations will be in the `package_container_service_2016_03` module.
- `package-container-service-2015-11-preview` has 4 operations from 1 API versions: `2015-11-01-preview`. Use crate feature `package-container-service-2015-11-preview` to enable. The operations will be in the `package_container_service_2015_11_preview` module.
- `package-compute-2015-06` has 55 operations from 1 API versions: `2015-06-15`. Use crate feature `package-compute-2015-06` to enable. The operations will be in the `package_compute_2015_06` module.
- `package-2015-06-preview` has 59 operations from 2 API versions: `2015-06-15`, `2015-11-01-preview`. Use crate feature `package-2015-06-preview` to enable. The operations will be in the `package_2015_06_preview` module.