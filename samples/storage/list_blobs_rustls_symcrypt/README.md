# List Blobs with rustls-symcrypt

This sample lists blobs from Azure Storage using [rustls-symcrypt], which provides
FIPS-compatible TLS cryptography via [SymCrypt].

Azure SDK for Rust crates enable `native-tls` by default through `reqwest`, so this
sample requires disabling default features on those crates and enabling `reqwest`
explicitly to use a different TLS crypto provider.

## Prerequisites

The [rustls-symcrypt] crate requires SymCrypt binaries at both build time and run time.

### Windows

1. Download `symcrypt.dll` and `symcrypt.lib` for your CPU architecture from the
   [SymCrypt releases page](https://github.com/microsoft/SymCrypt/releases).
2. Place them in a folder and set the environment variable:

   ```pwsh
   $env:SYMCRYPT_LIB_PATH = "<path-to-symcrypt-lib-folder>"
   ```

3. Restart your terminal after setting the variable.

### Linux (Ubuntu)

Install SymCrypt via the package manager by connecting to [PMC]:

```sh
sudo apt-get install symcrypt
```

Or manually download `libsymcrypt.so*` for your architecture from the
[SymCrypt releases page](https://github.com/microsoft/SymCrypt/releases)
and place them in a directory on your `$LD_LIBRARY_PATH`.

### Azure Linux 3

SymCrypt is pre-installed. Update to the latest version with:

```sh
sudo tdnf update symcrypt
```

For more details, see the [rustls-symcrypt] and [rust-symcrypt] repositories.

## Usage

### Provision Azure resources

This sample includes [Azure Developer CLI][azd] infrastructure to provision a
storage account and upload sample blobs:

```sh
azd up
```

This provisions a storage account, creates a `samples` container, assigns you
the Storage Blob Data Contributor role, and uploads a few sample blobs.

### Run the sample

```sh
export AZURE_STORAGE_ACCOUNT_NAME=$(azd env get-value AZURE_STORAGE_ACCOUNT_NAME)
cargo run
```

The `AZURE_STORAGE_ACCOUNT_NAME` environment variable is set automatically by
`azd up`. You can also pass it explicitly:

```sh
cargo run -- --account-name <storage_account_name>
```

### Clean up

```sh
azd down
```

[azd]: https://learn.microsoft.com/azure/developer/azure-developer-cli/
[PMC]: https://learn.microsoft.com/linux/packages
[rust-symcrypt]: https://github.com/microsoft/rust-symcrypt
[rustls-symcrypt]: https://github.com/microsoft/rustls-symcrypt
[SymCrypt]: https://github.com/microsoft/SymCrypt
