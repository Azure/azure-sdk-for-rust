# Azure Cosmos DB spec test macros

Procedural macros for the Azure Cosmos DB spec-test DSL in
`azure_data_cosmos_spec`. `#[spec_test]` checks the shape of a spec at compile
time and registers it with the spec runner.

This crate is test infrastructure (`publish = false`); it is not part of the
supported SDK surface. Use it through `azure_data_cosmos_spec::spec_test`. See
[spec 0030] for the design.

## Key concepts

`#[spec_test]` requires each spec to be `async`, have a `///` doc comment, and
declare a return type such as `SpecResult`. It reads these attributes:

- `#[spec_test(id = "...")]` overrides the stable spec ID.
- `#[service_profiles(all)]` or `#[service_profiles(MultiRead, MultiWrite)]`
  selects service profiles.
- `#[requires(TopologyControl, ...)]` declares the capabilities the spec needs.
- `#[known_issue("reason")]` or `#[known_issue("reason", MultiRead)]` records
  that the SDK doesn't meet the spec yet.
- `#[profile(basic_items)]` on a parameter selects its container profile.

An unknown profile or capability name is a compile error.

## Contributing

See the [CONTRIBUTING.md] for details on building, testing, and contributing to these libraries.

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit <https://opensource.microsoft.com/cla/>.

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct]. For more information see the [Code of Conduct FAQ] or contact <opencode@microsoft.com> with any additional questions or comments.

<!-- LINKS -->
[CONTRIBUTING.md]: https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md
[Microsoft Open Source Code of Conduct]: https://opensource.microsoft.com/codeofconduct/
[Code of Conduct FAQ]: https://opensource.microsoft.com/codeofconduct/faq/
[spec 0030]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/docs/specs/0030-cosmos-spec-tests.md
