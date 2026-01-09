# Design principles for next-generation SDKs

This document is an attempt to write down some design principles for the next generation of SDKs. The main focus is to call out intended changes to default behavior, which would be considered breaking changes within a major version of existing SDKs. With the next generation of new major SDK versions we have the opportunity to make these changes - but it is still a trade-off - any "breaking" change increases the adoption barrier - so, we have to evaluate benefits of applying new default behavior vs. the adoption risk.



## RUST driver as foundation

At this point the plan is to use the native C-level APIs provided in the RUST driver (azure_data_cosmos_native) to implement the communication with the service - and wrap these native APIs in the SDKs for Java, .Net, Go, Python (and RUST)



## RUST driver vs. SDK layering

RUST driver (azure_data_cosmos_native) and RUST SDK (azure_data_cosmos) will have different versioning and supportability stories. 

### Supportability

I think we have to be very clear in documents (and if necessary get legal approval) that we will not support the RUST driver (azure_data_cosmos_native) directly - we will support the SDKs using the driver internally (RUST SDK azure_data_cosmos as well as other languages) - but if a customer or application is using the driver directly this should technically be possible, we should document the API surface area etc. - but we should be very clear that we won't support it. The author of the component using the azure_data_cosmos_native driver should own the supportability story there.

**GA-BLOCKER**

For the RUST SDK (azure_data_cosmos) and the new major versions of SDKs in other languages we should seriously evaluate whether we can change the support lifecycle. Today the Azure SDK supports each major version for at least 3+ years after announcing deprecation. This is a very long time - but on the other hand there is no guarantee for providing hotfixes for any specific minor version - customers are expected to always upgrade to the very latest version if they want to consume any fix. This has caused major support concerns by enterprise customers and at least in Java we have moved to announcing certain minor versions for which we would provide hotfixes if needed (security hotfixes or very critical bug fixes) allowing customers to minimize changes they have to take (no need to update external dependencies etc.). It would probably be good to start a discussion with CELA or whoever needed to see whether we can formalize such a model - basically declaring certain minor versions as LTS (Long Term support) vs. Short-term-support and then be explicitly about the support lifecycle for each . The idea of expecting customers to always jump to the latest version just to consume a hotfix is naive and not justifiable.

### Versioning

Both - RUST driver (azure_data_cosmos_native) and SDK (azure_data_cosmos) should stick to semantic versioning - but the SDK (azure_data_cosmos) will have to be much more careful if/when to accept breaking changes (new major versions), because each major version will have to be supported for a long time. Since the driver as mentioned above won't ever be supported directly we can in theory publish new major versions with breaking changes as often as we want - we just have to realize that we still have to be able to maintain supported SDKs using the driver internally without exposing breaking changes.



## Connection modes

An explicit design goal for the next generation of SDKs is to not have to expose direct mode transport. The SDKs should not have to participate in routing requests to the right backend nodes/replica.

To still be able to satisfy the latency SLAs (which are today only applicable for applications using SDKs with direct mode) we will have to rely on using Gateway 2.0 (aka Thin Client Proxy)

**PREVIEW-BLOCKER**

Since Gateway 2.0 is a service-side feature still in preview and rollout and some calls still going to ComputeGateway this means the new generation of SDKs have to at least support the following two transports

- HTTP/2 with RNTBD-payloads against Gateway 2.0 endpoint
- HTTP/2 with REST contracts against ComputeGateway endpoint



**GA-BLOCKER**

It is likely that we will also have to provide at least one option for customers who can't use HTTP/2 for whatever reason - and the same option would also help us in cases where accounts are not yet migrated form Routing Gateway to Compute Gateway

- HTTP/1.1 with REST contracts against ComputeGateway



## Timeouts





Encoding

HA capabilities

PPAF

PPCB

Hedging

Retriable writes

Diagnostics

Motivation

Correlation

Lazy serialization

Thresholds

Ability to efficiently emit metrics, traces and diagnostics for exceptions/logs

Configuration