# Azure Rust API Exporter

## Generate API Report

This tool generates a JSON report of the API documentation for a specified Rust package.
It uses the following command `cargo +nightly rustdoc -Z unstable-options --output-format json --package {package_name} --all-features` to generate the documentation in JSON format, processes the JSON to remove unnecessary attributes, and outputs a cleaned-up version of the JSON.

## Version Compatibility

The project depends on specific version relationships between several components:

1. **NIGHTLY_TOOLCHAIN**: This value in main.rs must be set to a compatible nightly Rust version that produces the expected format version in the JSON output. Currently, the tool is pinned to a specific nightly (`nightly-2025-01-12`) to ensure consistent output.

2. **rustdoc FORMAT_VERSION**: The rustdoc JSON output has a specific `FORMAT_VERSION` (currently 37). Different nightly versions of Rust may produce different format versions.

3. **rustdoc-types crate**: The version of this dependency in Cargo.toml (currently 0.33.0) must be compatible with the JSON format version produced by the selected nightly toolchain.

4. **rust-api-parser**: This project in the azure-sdk-for-tools repository consumes the JSON files produced by this project and depends on the **rustdoc-types crate**. When updating the above components, ensure that the rust-api-parser tool is also updated to maintain compatibility.

### Version Update Process

When updating the nightly toolchain or the rustdoc-types crate, follow these steps:

-   First check the `FORMAT_VERSION` in a sample JSON output from the new nightly
-   Update the rustdoc-types crate version to match
-   Update the NIGHTLY_TOOLCHAIN in main.rs to pin the new nightly version
-   Update the rust-api-parser project in the azure-sdk-for-tools repository to ensure compatibility with the new JSON format
-   Test the complete workflow to ensure all tools in the chain remain compatible

## Usage

To run the tool, navigate to the root of the `azure-sdk-for-rust` repository and use the following command:

```sh
    cargo run --manifest-path eng/tools/generate_api_report/Cargo.toml -- --package package_name
```

Generates `package_name.rust.json` in the `sdk/service_folder/package_name/review` directory.

For example, to generate the report for a package named `azure_core`, run:

```bash
    cargo run --manifest-path eng/tools/generate_api_report/Cargo.toml -- --package azure_core
```

## Functionality

1. **Generate JSON Documentation**: The tool runs `cargo +nightly rustdoc ...` to generate the JSON documentation.
2. **Process JSON**: The tool reads the JSON file, removes the `span` attribute from each item, and retains important attributes like `crate_version`, `inner`, and `format_version`.
3. **Output Cleaned JSON**: The tool writes the cleaned-up JSON to a new file `package_name/review/package_name.rust.json`.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request with your changes.

## Troubleshooting

If you encounter any issues, please check the [Version Compatibility](#version-compatibility) section.

## License

This project is licensed under the MIT License.
