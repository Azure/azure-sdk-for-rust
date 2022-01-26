# API Versions

A `Configuration` section `Tag` in a `readme.md` represents a set of operations for an Azure service for generating a client SDK. Ideally, there is a single tag representing an API version for a service.

Unfortunately, some services use [multiple API versions](https://github.com/Azure/azure-sdk-for-rust/issues/563) in their tags. In this case, different service operations will use different API versions of the same service.

Using the `azure_mgmt_storage` crate as an example. I is generated from the [specification/storage/resource-manager/readme.md](https://github.com/Azure/azure-rest-api-specs/blob/master/specification/storage/resource-manager/readme.md). In its [Cargo.toml](https://github.com/Azure/azure-sdk-for-rust/blob/main/services/mgmt/storage/Cargo.toml), you can see the list of `features`. A crate feature is generated for each tag. The first tag without `-preview` is selected as the default. In its [lib.rs](https://github.com/Azure/azure-sdk-for-rust/blob/main/services/mgmt/storage/src/lib.rs), you can see that a module is generated for each tag as well.

Here is an example `Cargo.toml`.
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
```

The default tag will be used when this dependency is specified as:
``` toml
[dependencies]
azure_mgmt_storage = "0.1"
```

To use another tag, for example to try out the newer preview API, you must disable the default:
``` toml
[dependencies]
azure_mgmt_storage = { version = "0.1", default-features = false, features = ["package-2020-08-preview"] }
```

A few use cases require using more than tag. This can be accomplished a couple of ways. Specify the feature `no-default-version` and the other tags you wish to use.

``` toml
[dependencies]
azure_mgmt_storage = { version = "0.1", features = ["no-default-version", "package-2019-06", "package-2018-02"] }
```

Or it can be done by [renaming the dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#renaming-dependencies-in-cargotoml):
``` toml
[dependencies]
azure_mgmt_storage_2019_06 = { package = "azure_mgmt_storage", version = "0.1", default-features = false, features = ["package-2019-06"] }
azure_mgmt_storage_2018_02 = { package = "azure_mgmt_storage", version = "0.1", default-features = false, features = ["package-2018-02"] }
```
