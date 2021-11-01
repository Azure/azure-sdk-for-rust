# API Versions

Each service may have multiple API versions. Each API version has a set of OpenAPI 2.0 documents, listed under a `Tag` in a [readme.md](https://github.com/Azure/azure-rest-api-specs/blob/master/specification/storage/resource-manager/readme.md) file in a `Configuration` section. Sometimes there are multiple `Tag`s for the same API version. For example, [Tag: package-2018-07](https://github.com/Azure/azure-rest-api-specs/blob/master/specification/storage/resource-manager/readme.md#tag-package-2018-07) and [Tag: package-2018-07-only](https://github.com/Azure/azure-rest-api-specs/blob/master/specification/storage/resource-manager/readme.md#tag-package-2018-07-only). Rust code is generated for each tag and organized as a crate feature. The first tag without `-preview` is selected as the default feature:

``` toml
[features]
default = ["package-2019-06"]
"package-2020-08-preview" = []
"package-2019-06" = []
"package-2019-04" = []
"package-2018-11" = []
"package-2018-07" = []
"package-2018-07-only" = []
"package-2018-03" = []
"package-2018-02" = []
"package-2017-10" = []
"package-2017-06" = []
"package-2016-12" = []
"package-2016-05" = []
"package-2016-01" = []
"package-2015-06" = []
"package-2015-05-preview" = []
```

The default feature will be used when this dependency is specified:
``` toml
[dependencies]
azure_mgmt_storage = { git = "https://github.com/Azure/azure-sdk-for-rust" }
```

To use another tag, for example to try out the newer preview API, you must disable the default:
``` toml
[dependencies]
azure_mgmt_storage = { git = "https://github.com/Azure/azure-sdk-for-rust", default-features = false, features = ["package-2020-08-preview"] }
```

A few use cases require using more than tag. This can be accomplished a couple of ways. Specify the feature `no-default-version` and the other tags you wish to use.

``` toml
[dependencies]
azure_mgmt_storage = { git = "https://github.com/Azure/azure-sdk-for-rust", features = ["no-default-version", "package-2019-06", "package-2018-02"] }
```

Or it can be done by [renaming the dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#renaming-dependencies-in-cargotoml):
``` toml
[dependencies]
azure_mgmt_storage_2019_06 = { package = "azure_mgmt_storage", git = "https://github.com/Azure/azure-sdk-for-rust", default-features = false, features = ["package-2019-06"] }
azure_mgmt_storage_2018_02 = { package = "azure_mgmt_storage", git = "https://github.com/Azure/azure-sdk-for-rust", default-features = false, features = ["package-2018-02"] }
```
