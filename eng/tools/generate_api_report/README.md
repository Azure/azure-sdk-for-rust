# Azure Rust API Exporter

## Generate API Report

This tool generates a JSON report of the API documentation for a specified Rust package.
It uses the following command `cargo +nightly rustdoc -Z unstable-options --output-format json --package {package_name} --all-features` to generate the documentation in JSON format, processes the JSON to remove unnecessary attributes, and outputs a cleaned-up version of the JSON.

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

## License

This project is licensed under the MIT License.
