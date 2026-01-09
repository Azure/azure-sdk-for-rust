# Design principles for next-generation SDKs

This document is an attempt to write down some design principles for the next generation of SDKs. The main focus is to call out intended changes to default behavior, which would be considered breaking changes within a major version of existing SDKs. With the next generation of new major SDK versions we have the opportunity to make these changes - but it is still a trade-off - any "breaking" change increases the adoption barrier - so, we have to evaluate benefits of applying new default behavior vs. the adoption risk.



## Rust driver as foundation

At this point the plan is to use the native C-level APIs provided in the Rust driver (azure_data_cosmos_native) to implement the communication with the service - and wrap these native APIs in the SDKs for Java, .Net, Go, Python (and Rust)



## Rust driver vs. SDK layering

Rust driver (azure_data_cosmos_native) and Rust SDK (azure_data_cosmos) will have different versioning and supportability stories. 

### Supportability

I think we have to be very clear in documents (and if necessary get legal approval) that we will not support the Rust driver (azure_data_cosmos_native) directly - we will support the SDKs using the driver internally (Rust SDK azure_data_cosmos as well as other languages) - but if a customer or application is using the driver directly this should technically be possible, we should document the API surface area etc. - but we should be very clear that we won't support it. The author of the component using the azure_data_cosmos_native driver should own the supportability story there.

**GA-BLOCKER**

For the Rust SDK (azure_data_cosmos) and the new major versions of SDKs in other languages we should seriously evaluate whether we can change the support lifecycle. Today the Azure SDK supports each major version for at least 3+ years after announcing deprecation. This is a very long time - but on the other hand there is no guarantee for providing hotfixes for any specific minor version - customers are expected to always upgrade to the very latest version if they want to consume any fix. This has caused major support concerns by enterprise customers and at least in Java we have moved to announcing certain minor versions for which we would provide hotfixes if needed (security hotfixes or very critical bug fixes) allowing customers to minimize changes they have to take (no need to update external dependencies etc.). It would probably be good to start a discussion with CELA or whoever needed to see whether we can formalize such a model - basically declaring certain minor versions as LTS (Long Term support) vs. Short-term-support and then be explicitly about the support lifecycle for each . The idea of expecting customers to always jump to the latest version just to consume a hotfix is naive and not justifiable.

### Versioning

Both - Rust driver (azure_data_cosmos_native) and SDK (azure_data_cosmos) should stick to semantic versioning - but the SDK (azure_data_cosmos) will have to be much more careful if/when to accept breaking changes (new major versions), because each major version will have to be supported for a long time. Since the driver as mentioned above won't ever be supported directly we can in theory publish new major versions with breaking changes as often as we want - we just have to realize that we still have to be able to maintain supported SDKs using the driver internally without exposing breaking changes.



## Connection modes

An explicit design goal for the next generation of SDKs is to not have to expose direct mode transport. The SDKs should not have to participate in routing requests to the right backend nodes/replica.

To still be able to satisfy the latency SLAs (which are today only applicable for applications using SDKs with direct mode) we will have to rely on using Gateway 2.0 (aka Thin Client Proxy)



Since Gateway 2.0 is a service-side feature still in preview and rollout and some calls still going to ComputeGateway this means the new generation of SDKs have to at least support the following two transports

- HTTP/2 with RNTBD-payloads against Gateway 2.0 endpoint **GA-BLOCKER**
- HTTP/2 with REST contracts against ComputeGateway endpoint **PREVIEW-BLOCKER**



**GA-BLOCKER**

It is likely that we will also have to provide at least one option for customers who can't use HTTP/2 for whatever reason - and the same option would also help us in cases where accounts are not yet migrated form Routing Gateway to Compute Gateway

- HTTP/1.1 with REST contracts against ComputeGateway **GA-BLOCKER**



## Timeouts

In current SDKs today we have hard-coded default timeouts (5-6s connect timeout, 5-6s request timeout for direct, 30 seconds for gateway). It is possible to override some of these timeouts and we also allow customers to specify E2E timeouts in Java, Python and implicitly via Cancellation Token in .Net.

Tuning the default timeouts - especially connect timeout - is critical for any latency sensitive application (because connect failure is the indication of service node being down in many cases). 

**PREVIEW-BLOCKER**

For the next generation of SDKs IMO we should consider dynamically tuning default timeouts (maybe with some configurable boundaries) - but especially the connect timeout - and even request timeout can probably be dynamically tuned without making every single customer do this. We had discussed this before for the .Net SDK - but decided against it because it would be a breaking behavior change. Which means the next major version SDKs is the time to reconsider. A proposal could be to start with much more aggressive connect timeouts (let's say 500ms instead of 5 seconds today) and only increase the default timeout when for a certain endpoint connect-retries after timeout are successfully connecting with the higher value (let's sys timeouts after 500ms have resulted in successful connect after 1 second) - this would help to address the edge cases where clients connect form outside of Azure - but allow better (P9x latency impact) defaults without customers having to worry about it.



## Encoding

Today binary encoding is only supported in the .Net SDK - all other SDKs depend on transcoding happening in the backend. For the next generation of SDKs we have the following options

1) Implement an option to apply transcoding in the RUST driver - as such providing this capability across all SDKs using the RUST driver - but it would mean there is still transcoding happening (not ideal from a latency/efficiency perspective)
2) Implement transcoding where appropriate in the actual SDKs - really allowing to avoid transcoding (ideal for latency) - but relatively expensive because a binary encoding implementation in a SDK is non-trivial and pretty risky.
3) Not implementing binary encoding in SDKs or driver and purely rely on transcoding to happen in the service



**PREVIEW-BLOCKER**

My recommendation would be to start with #3 for the initial preview

**GA-BLOCKER**

but then implement #1 for sure (automatically providing #2 for RUST as well) and adding #2 on a need-base in other SDKs. Shifting transcoding from service to RUST driver still has benefits especially for query (network density can be increased), any possible RU-benefits can be realized and we shift the COGs out of the service.



## HA capabilities

### Preferred regions

We have various options (often inconsistent) across SDKs how customers can determine which regions should be used.

- ApplicationPreferredRegions (cx specifies a list of regions in order to be used for reads and writes for multi-write-region accounts)
- ApplicationRegion (a way how the App can tell the SDK the region in which the app runs - the SDK will come-up with the list of proximity prioritized preferred regions based on it). Proximity evaluation was hard-coded initially in .Net - now the service has a mechanism to populate the proximity in the account metadata
- Excluded regions - a mechanism for customers to disallow using specific regions (for entire client or per request)

**PREVIEW-BLOCKER**

My recommendation to normalize on a consistent way to configure this is

- ApplicationRegion support with the service-based proximity determination
- ExcludedRegion support
- If we think we need to keep ApplicationPreferredRegion support to allow customers to overwrite the priority it should only be a way to prioritize regions - not by filtering regions that can be used for reads/writes. Basically, if an account has 5 regions (R1 - R5) - and ApplicationPreferredRegions is set to R1, R3 - it should result in effectively still using all 5 regions - but prioritized list would be R1, R3, R2, R4, R5. My preference would be to not expose ApplicationPreferredRegion in the client at all (and rather have an emergency overwrite capability in the service - account specific)

### PPAF

**PREVIEW-BLOCKER for strong consistency**

Like in .Net and Java PPAF enablement should purely depend on service-side config. If the account topology indicates PPAF is enabled, the RUST driver should enable the feature - there is no need for any client-side option to disable PPAF (it actually would be dangerous to enable PPAF only service-side)



### PPCB

**PREVIEW-BLOCKER**

PPCB should always be enabled by default. I would even recommend to not allow to disable it.



### Hedging

Same as PPCB - IMO we should force-enable hedging - but we probably need to allow the behavior to be configured - or invest more in dynamically tuning the thresholds similar to the default timeouts. I think this would be good to discuss and brainstorm a bit - because customers have sometimes been hesitant when learning about the hedging capability due to possibly higher RU-usage - while in reality this is one of the most powerful HA improvements. I am wondering whether combining hedging with for example the throughput bucket feature (assigning hedged operations a specific throughput bucket) could be an efficient way to address customer's RU concerns.

**PREVIEW-BLOCKER**

Bottom-line - I strongly believe we have to enable this by default - maybe even prevent disallowing it - but we need to take the RU-impact concern from customers seriously - so, I don't think porting whatever we have in .Net, Java or Python right now is sufficient for RUST driver V1



### Retriable writes

In our current SDKs by default we do not retry write operations when we can't guarantee idempotency. So, if the request payload was sent to the network already and we hit a timeout for example we would not automatically retry - because it is very possible that the operation was successfully processed - we simply do not know and as such a retry would not be idempotent.

In .Net for PPAF/PPCB we added an option to opt-into enabling automatic retries for writes - this comes with the consequence that applications need to be able to reasonably handle certain error conditions like 409/Conflict or 412/Pre-condition failed or 404/NotFound for delete operations. 

In Java we allow customers as well to enable automatic retries for writes - but with two different flavors - one identical to .Net the other in addition allowing to use some SDK-injected system properties in the document to be able to transparently handle 409/412/404 when retries are happening see https://github.com/Azure/azure-sdk-for-java/pull/34227. 

**PREVIEW-BLOCKER**

For the next generation of SDKs we have the following options

1) Do not enable automatic retries for writes - even when we still (after documenting broadly for long time that retries for writes should be added by applications) see regular CRI (Customer reported incidents) indicating customers expect us to do these retries in the SDK
2) Enable automatic retries by default. Maybe allow disabling them? But just the simple way .Net is using
3) Enable automatic retries by default - optionally allow using SDK-system properties like Java does
4) Change the APIs - and only expose APIs that are idempotent by nature - similar like ConcurrentDictionary - via CreateOrUpdate etc.



My 2 cents - while I would love the idea of refactoring APIs to always be idempotent by nature I am afraid that this change would cause too much churn and be an adoption blocker. So, my recommendation would be to implement #3



## Diagnostics

### Motivation

For all cases where we will wrap the RUST driver in another SDK (Python, Go, Java, .Net) we have to assume that the application developer has no context/knowledge of RUST. We can't expect them to debug into the RUST code etc. This means the native C-APIs in the RUST driver we use have to act as a black-box. If any non-happy-path situation happens (errors but also increased latency etc.) we have to return enough context to allow debugging the root cause. Luckily we do not plan to support direct mode - so, the service interactions are simpler. 



### Correlation

Unlike direct mode with Gateway mode the caller has not much context to allow identifying an RCA. The benefit is that we should have that info in Gateway service telemetry - but for many debugging scenarios you have to correlate client-specific dimensions (which POD/VM is the caller) with server-side dimensions (region, partition, cluster/tenant etc.). So, whenever possible the diagnostcis exposed form the RUST driver to allow correlation witha t least these dimensions

- PartitionId
- Service-Node (at least from Gateway/Proxy)
- ActivityId + CorrelatedActivityId
- The service has started to use OTel based tracing and Otel correlation vector - this could become a pretty pwoerful tool when wired up with OTEL telemetry emitted from applications - we should sync-up with owners of this in the service to udnerstand how sampling works today and what we would need to ensure to allow correlation between client-side Otel traces and service
- UserAgent (suffix) needs to still be reflected inservice-telemetry since that is the only available option today for adding end-to-end correlation between application and service entrypoint - there are open issues in the service because we loose this correlation when Compute/Proxy call Backend - but hopefully that correlation link can be maintained in the future (basically original UserAgent fwded to Backend somehow)



### Lazy serialization

In .Net and Java in direct mode it is absolutely critical today that we only serialize the diagnostics context into Json when requested (lazily) - serializing the diagnostic context into json would otherwise have a too high burden on CPU usage. On the other hand we also use the diagnostics context for example in Java to generate OTel traces and metrics - which means we pretty much always collect the diagnostics - and only make the json serialization conditional. 

In the RUST driver we need to design the native API in a way that allows a similar model. We have to expose expose enough info in a typed contract to allow making the decision whether to serialize the full diagnostics or not (usually this would happen on error, when latency or RU-usage exceeds certain thresholds and/or based on sampling) - but in general we would probably want to keep the actual diagnostics content as an opaque string.

**PREVIEW-BLOCKER**

My recommendation would be to use a similar model as in the Java SDK - CosmosDiagnosticsContext.java - it should have enough info to make the decision in the place immediately around the native API to decide whether to serialize the opaque diagnostics string or not. If we are time-wise not able to fit this into preview maybe we can start by always serializing the diagnostics into an opaque string - that way later when doing it lazily we would improve perf - and not have to worry avoid breaking perf regressions when starting to expose the diagnostics info we absolutely have to expose eventually to allow debugging.

**GA-BLOCKER**

While I think technically we should keep the diagnostics content opaque (as in no hard model-contract because then we would not be able to iterate on it within a major version flexible enough) the single RUST driver also would provide us the opportunity to have consistent diagnostics - which would make using generative AI (or traditional TSGs) to self-diagnose certain issues much easier. So, we should probably still invest in adding documentation (targeting humans / gen AI - not applications) about the diagnostics content. Setting expectations there is critical to allow iterating on diagnostics content while making better use of the know how.



### Thresholds

**PREVIEW-BLOCKER** (probably debatable - but I consider it preview-blocking, because we know that one of the early adopters will have very large workloads)

One lesson learned form our Otel endeavors in .Net and Java is that we have to be super careful about the overhead at runtime. Even low single digit percent overhead for latency or CPU has resulted in pushback from some customers. So, it becomes critical for any metrics, logs or traces being emitted to find ways to apply thresholds. For metrics for example when you add let's say PartitionId as a dimension the number of time series you are looking at can be overwhelming. So, only emitting metrics with PartitionId dimension for requests violating certain thresholds like latency or statusCode can help to significantly reduce the number of concurrent timeseries in any time-interval - also applying hard-limits can be useful. The same is true for logs or traces - if a handful of requests is slow or failing you want as much debugging info as possible - but if there is an outage - and 30 percent of your requests are failing logging details for all of them can result in your app seeing much higher impact than needed because of resource exhaustion due to noisy logging/tracing. So, applying thresholds and hard guard-rails which are configurable and designing how-to do so at runtime without restarts is essential - we need to design the diagnostics capturing around these requirements, because otherwise collecting diagnostics when it is needed most - for very large workloads - becomes too challenging. The Java SDK in combination with Spark connector are a pretty good starting point for how thresholds and guard-rails could be applied - but this will need more investigation/work than just porting it.



## Configuration

**PREVIEW-BLOCKER** (probably also debatable whether sufficient for GA - same argument as for Thresholds above)

In both .Net as well as Java SDK we have a series of config knobs that are not exposed in public API but can be tuned as a fail-safe via environment variable (or system properties in Java). For the RUST driver we need to expose a clean API to modify these internal plan-B config overrides - how that gets exposed in each SDK really depends on the typical config approach there - but the APIs need to exist to do this cleanly - and we need to have documentation for all the internal knobs in place. Part of the work should be that diagnostics (for example shown in errors) should include a list of all config overrides - and we should also be specific in documentation whether using any such knob voids SLAs/supportability



## Authentication

I think the general consensus is that the next generation of SDKs should assume that most customers use EntraID instead of key-based authentication. In  today's data plane SDKs we have certain APIs (to create/update/delete databases and containers or to modify throughput of a database or container) which can only be used when using key-based auth. When using EntraID customers have to instead use the management SDKs

This obviously is not a good solution - for MSFT internal customers some hacks exist to make the SDK work with Entra as well - but those are also not broadly supportable. 

So, for the next generation of SDKs we have two options

1) Keep the split brain (cx can/must use normal SDK for key-based auth and management SDK/APIs for Entra based Auth)
2) Remove the APIs altogether (which would effectively make it impossible to execute those operations with key-based auth)
3) Have these APIs be implemented in a way that allows their usage for both Entra and key-based auth - this might require calling different endpoints under the hood or changes to the service-side. The service team is also working on a unified Azure RBAC model - so, we should sync-up

My 2 cents

- I think for GA implementing #3 cleanly is a MUST - **GA-BLOCKER**
- For preview I would rather remove the APIs than start with anything that puts us into a corner and would be breaking later - so #2 - **PREVIEW-BLOCKER** - if we decide to go with #1 for preview we have to be extra careful to double-check that we can transition to #3 reasonably well