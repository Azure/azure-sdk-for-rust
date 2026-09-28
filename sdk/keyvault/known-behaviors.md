# Key Vault issue investigation context

This is advisory service context, not an automatic-closure list. Match the exact operation, service error, crate version, and reproduction before attributing a report to the service. Also read the affected Rust crate's README, CHANGELOG, source, tests, and any available troubleshooting guide.

The Rust clients are `azure_security_keyvault_keys`, `azure_security_keyvault_secrets`, and `azure_security_keyvault_certificates`. Authentication also involves `azure_identity` and the shared `azure_core` policies. A generic HTTP 400, 401, 403, 409, or 429 does not distinguish an SDK defect from service behavior.

## Authentication challenges, audience, and identity

The Rust Key Vault authorizer sends the first request without authorization to discover the service's authentication challenge, then authenticates and retries. An initial 401 followed by success is not by itself a failure. Inspect `src/authorizer.rs` in the affected crate and the final operation result rather than classifying every 401 as an incorrect token.

Persistent failures may involve the credential, cloud, audience, tenant, or principal. Verify which identity the application actually uses and whether it has access to the target resource. Do not recommend disabling `verify_challenge_resource` or request token contents to diagnose a report.

- https://learn.microsoft.com/azure/key-vault/general/authentication
- https://docs.rs/azure_identity

## Authorization model and propagation

Key Vault data-plane authorization uses either Azure RBAC or vault access policies. Permissions configured in the inactive model do not grant access. Role assignments can take time to propagate; confirm the principal, scope, operation, and active permission model before attributing a 403 to propagation.

A managed identity must be enabled on its hosting resource and granted the required data-plane access. Do not infer that authentication success proves authorization.

- https://learn.microsoft.com/azure/key-vault/general/rbac-guide
- https://learn.microsoft.com/azure/key-vault/general/rbac-migration
- https://learn.microsoft.com/azure/role-based-access-control/troubleshooting
- https://learn.microsoft.com/azure/app-service/overview-managed-identity

## Firewall and private endpoint DNS

Firewall rules can reject requests from networks that are not allowed. Private endpoint access also depends on correct DNS resolution from the application's network. Verify endpoint, network path, and service error details; do not suggest weakening network restrictions as a generic fix.

- https://learn.microsoft.com/azure/key-vault/general/network-security
- https://learn.microsoft.com/azure/key-vault/general/private-link-service

## Soft-delete, name reuse, and purge protection

Deleted vaults and objects may remain recoverable during their retention period, preventing reuse of the same name. Purge protection prevents permanent deletion before the retention period expires.

Prefer recovery or waiting when appropriate. Purging is irreversible and may be disallowed by purge protection; do not automatically purge, suggest disabling protection, or present purge as a harmless prerequisite.

- https://learn.microsoft.com/azure/key-vault/general/soft-delete-overview
- https://learn.microsoft.com/azure/key-vault/general/key-vault-recovery

## Throttling and retries

Service operation limits can produce HTTP 429. Inspect the affected operation, retry configuration, response headers, and final result. Service throttling does not rule out an SDK retry defect.

Use the service's guidance for backoff and workload distribution. Do not recommend unbounded retries or indefinite caching of rotating secrets and keys.

- https://learn.microsoft.com/azure/key-vault/general/service-limits
- https://learn.microsoft.com/azure/key-vault/general/overview-throttling

## Certificate import validation

Certificate import requires supported certificate/key material. A PFX or PEM with mismatched certificate and private key can be rejected by the service. Match the exact validation error against the import requirements; do not request the customer's private key or assume every import failure has this cause.

- https://learn.microsoft.com/azure/key-vault/certificates/tutorial-import-certificate
- https://learn.microsoft.com/azure/key-vault/certificates/about-certificates

## Access policies in infrastructure deployments

An infrastructure deployment that replaces the vault's access-policy collection can remove policies not included in the deployment. Determine the management operation and deployment tool involved instead of attributing it to a Rust data-plane client.

- https://learn.microsoft.com/azure/key-vault/general/assign-access-policy
- https://learn.microsoft.com/azure/key-vault/general/rbac-migration

## Object state and operation-specific restrictions

Disabled, expired, or not-yet-valid objects have operation-specific restrictions. Consult the documentation for the exact object type and operation; do not assume every operation is rejected for an expired object or recommend changing object attributes without understanding the intended policy.

- https://learn.microsoft.com/azure/key-vault/secrets/about-secrets
- https://learn.microsoft.com/azure/key-vault/keys/about-keys
- https://learn.microsoft.com/azure/key-vault/certificates/about-certificates

## Vault versus Managed HSM capabilities

Standard vaults and Managed HSM have different capabilities and endpoints. Check the resource type, cloud, operation, and affected crate's supported APIs. Do not assume that secrets or certificate operations are available merely because an endpoint accepts key operations.

- https://learn.microsoft.com/azure/key-vault/managed-hsm/overview
- https://learn.microsoft.com/azure/key-vault/general/about-keys-secrets-certificates
