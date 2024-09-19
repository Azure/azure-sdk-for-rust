# Azure OpenAI Inference SDK for Rust

## Introduction

This SDK provides Rust types to interact with both OpenAI and Azure OpenAI services.

Note: Currently request and response models have as few fields as possible, leveraging the server side defaults wherever it can.

### Features

All features are showcased in the `example` folder of this crate. The following is a list of what is currently supported:

- Supporting both usage with OpenAI and Azure OpenAI services by using `OpenAIClient` or `AzureOpenAIClient`, respectively.
- Key credential authentication is supported.
- [Azure Only] Azure Active Directory (AAD) authentication is supported.
- `ChatCompletions` operation supported (limited fields).
- Streaming for `ChatCompletions` is supported

## Authentication methods

### Azure Active Directory

This authentication method is only supported for Azure OpenAI services.

```rust
AzureOpenAIClient::new(
    endpoint,
    Arc::new(DefaultAzureCredentialBuilder::new().build()?),
    None,
)?
```

### Key Credentials

This method of authentication is supported both for Azure and non-Azure OpenAI services.

```rust
OpenAIClient::with_key_credential(secret, None)?
```
