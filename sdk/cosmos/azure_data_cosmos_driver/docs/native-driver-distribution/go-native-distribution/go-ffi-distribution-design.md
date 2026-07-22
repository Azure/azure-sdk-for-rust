<!-- cspell:ignore amd64 arm64 azcosmos azcosmoscore azcosmosdriverlinuxamd64gnu checksums glibc GOARCH GOOS libc LDFLAGS libazurecosmos librdkafka metapackage musl onnxruntime riscv SRCDIR -->
# Go FFI native-driver distribution design

> **Status:** Design discussion for Go Central SDK and architecture-board review.
> This is not an ADR. It captures the distribution problem, candidate packaging
> models, and the customer/developer experience trade-offs for a Go SDK backed by
> the Rust Cosmos driver through FFI.

## 1. Problem statement

The Go v2 FFI path lets Go reuse the Rust Cosmos driver instead of re-owning the
same driver logic in Go. That reduces implementation drift, but it moves a real
decision into distribution:

> Where should the platform-specific native driver binaries live, and how should
> a Go customer get the correct one?

The native driver is built once per supported OS/architecture/libc target. If the
initial supported set includes Windows, macOS, and Linux on x64/amd64 and ARM64,
the SDK needs roughly this matrix:

| Target family         | Initial target examples                                      |
| --------------------- | ------------------------------------------------------------ |
| Windows               | amd64, arm64                                                 |
| macOS                 | amd64, arm64                                                 |
| Linux glibc           | amd64, arm64                                                 |
| Optional or follow-up | Windows x86, Linux musl/Alpine, additional long-tail targets |

With an optimized native driver around **~5 MB per target**, a six-target matrix
is roughly **~30 MB before compression**. Expanding to ten targets approaches the
earlier **~50 MB** mental model. The design question is not only binary size; it
is **who downloads which targets, when, and how visible the native dependency is
to the customer**.

## 2. Go module mechanics that shape the design

Go has three mechanics that matter here.

First, a GitHub repository and a Go module are not the same thing. A repository
can contain many modules, each rooted at its own `go.mod`. Go's module system
downloads modules, versions, and packages; customers normally do not clone the
entire repository to consume an SDK.

```text
GitHub repository
└── sdk/data/azcosmos/                 <- one Go module
    └── go.mod
└── sdk/data/azcosmos-driver-linux-amd64/
    └── go.mod                         <- another Go module
```

Second, Go builds packages, not an entire repository by default. A cgo
requirement in the Cosmos module does not automatically mean unrelated modules in
the Azure SDK for Go repository require cgo. Customers and CI invoke `go build`,
`go test`, or `go list` against specific modules/packages.

Third, cgo link flags are collected from packages in the build graph. The cgo
documentation says `#cgo LDFLAGS` directives from any package in the program are
concatenated at link time. That means a platform-specific driver package can
contribute the native library path, but only if that package is imported by the
program. A `require` entry alone is not enough; the package must be reachable
through an import, often a blank import.

```go
//go:build linux && amd64

package azcosmos

import _ "github.com/Azure/azure-cosmos-go-native-drivers/azcosmos-driver-linux-amd64"
```

## 3. Design goals

| Goal                                                        | Why it matters                                                                                                 |
| ----------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| Keep the common path close to `go get` / `go build`         | Cosmos Go v1 feels like a normal Go SDK; Go v2 should not surprise mainstream customers.                       |
| Avoid making every customer download every long-tail binary | The native matrix can grow over time, and module-cache footprint matters.                                      |
| Keep versioning safe                                        | The Go wrapper, C header, and native driver ABI must match.                                                    |
| Work in enterprise and offline environments                 | Build-time network downloads and ad-hoc install scripts are often blocked.                                     |
| Keep the Azure SDK for Go repository manageable             | Committing many binary artifacts affects contributors who clone the repo, even if customers use module ZIPs.   |
| Make unsupported platforms fail clearly                     | Customers should see a direct "no driver configured for this platform" message, not an obscure linker failure. |

## 4. Industry reference points

### 4.1 Confluent Kafka: bundle-first hybrid

`confluent-kafka-go` is the closest market precedent. It is a Go library backed
by the native `librdkafka` implementation. Its developer documentation describes
bundled platform-specific static builds as the default, with dynamic/manual
linking as an escape hatch for special cases:

- bundled static builds for common macOS, Linux glibc, Linux musl, and Windows
  targets
- cgo remains required
- a dynamic/manual mode exists for unsupported platforms or special features

Reference:
[`kafka/README.md#librdkafka`](https://github.com/confluentinc/confluent-kafka-go/blob/master/kafka/README.md#librdkafka)
and
[`librdkafka_vendor`](https://pkg.go.dev/github.com/confluentinc/confluent-kafka-go/v2/kafka/librdkafka_vendor).

The useful lesson is not "copy Confluent exactly." The useful lesson is that a
Go library can be native-backed and still preserve a mostly normal customer
experience by bundling common platform binaries and documenting escape hatches.

### 4.2 ONNX Runtime Go: wrapper-only, user supplies native runtime

`onnxruntime_go` keeps the Go wrapper separate from the platform runtime. The
customer supplies the matching `onnxruntime.dll`, `.so`, or `.dylib` and points
the wrapper at it before initialization.

Reference:
[`onnxruntime_go` requirements](https://github.com/yalue/onnxruntime_go#requirements).

The useful lesson is that wrapper-only distribution keeps the Go module small,
but it makes native acquisition part of the customer setup. That is often
acceptable in ML/runtime scenarios; it is a harder fit for a first-party Azure
data SDK's default path.

### 4.3 go-sqlite3: cgo toolchain requirement is explicit

`go-sqlite3` is a widely used Go package that directly documents the cgo and
compiler requirement.

Reference:
[`go-sqlite3` installation](https://github.com/mattn/go-sqlite3#installation).

The useful lesson is that the Go ecosystem accepts cgo in some packages, but it
does not hide the compiler requirement. Cosmos should be equally explicit:
prebuilt Rust artifacts remove the need for a Rust toolchain and manual native
library copying, but **cgo still requires a C build toolchain**.

### 4.4 Go community discussion: no wheel-like standard

The Go community has discussed prebuilt cgo dependencies and the absence of a
Python-wheel-like mechanism for `go.mod`.

Reference:
[`golang-nuts` discussion](https://groups.google.com/g/golang-nuts/c/ahZXdoClBGg).

The useful lesson is that Go does not provide a standard native-binary packaging
solution. Cosmos has to choose and own a distribution model.

## 5. Option A: one public module bundles the full native matrix

This is the simplest Confluent-like shape.

```text
github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos
├── go.mod
├── client.go
├── internal/core/
├── internal/native/
│   ├── windows-amd64/azurecosmos.lib
│   ├── windows-arm64/azurecosmos.lib
│   ├── darwin-amd64/libazurecosmos.a
│   ├── darwin-arm64/libazurecosmos.a
│   ├── linux-amd64-gnu/libazurecosmos.a
│   └── linux-arm64-gnu/libazurecosmos.a
├── driver_windows_amd64.go
├── driver_darwin_arm64.go
└── driver_linux_amd64.go
```

Selection happens through Go build tags:

```go
//go:build darwin && arm64

package azcosmos

/*
#cgo LDFLAGS: -L${SRCDIR}/internal/native/darwin-arm64 -lazurecosmos
*/
import "C"
```

Customer flow:

```text
Customer app
  imports github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos
       │
       ▼
Go downloads one azcosmos module ZIP
       │
       ▼
Go build tags select the one matching native library
       │
       ▼
Application links with the Rust driver
```

Salient features:

| Area                | Effect                                                                                  |
| ------------------- | --------------------------------------------------------------------------------------- |
| Customer experience | Best. One `go get`; one module; no platform package choices.                            |
| Download footprint  | Worst. Every `azcosmos` customer downloads the default native matrix.                   |
| Version safety      | Strong. Go wrapper and native bits are versioned together.                              |
| Repository impact   | High if the module lives in `azure-sdk-for-go`; binaries are committed with SDK source. |
| Long-tail targets   | Adding one target increases the default module for everyone.                            |

This model is easiest to explain to customers and hardest to defend on module
size.

**Size gist:** customers download/cache the whole default native matrix as part
of one `azcosmos` module. A six-target default set is roughly ~30 MB before
compression; a ten-target set approaches ~50 MB. The final app links only the
current platform's native driver.

## 6. Option B: split modules in one repository

This is a multi-module model where the public SDK stays zero-touch for default
platforms, but each native binary lives in its own Go module.

The concrete shape would look like this if all modules live in the Azure SDK for
Go repository:

```text
github.com/Azure/azure-sdk-for-go
└── sdk/data/azcosmos-core/
    ├── go.mod
    ├── core.go
    └── include/azurecosmos.h

└── sdk/data/azcosmos-driver-linux-amd64-gnu/
    ├── go.mod
    ├── link_linux_amd64.go
    └── native/libazurecosmos.a

└── sdk/data/azcosmos-driver-darwin-arm64/
    ├── go.mod
    ├── link_darwin_arm64.go
    └── native/libazurecosmos.a

└── sdk/data/azcosmos/
    ├── go.mod
    ├── client.go
    ├── default_driver_linux_amd64.go
    ├── default_driver_darwin_arm64.go
    └── unsupported_driver.go
```

`azcosmos-core` contains the shared Go wrapper and C ABI declarations:

```go
// sdk/data/azcosmos-core/go.mod
module github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos-core

go 1.25.0
```

```c
/* sdk/data/azcosmos-core/include/azurecosmos.h */
#pragma once

const char* azurecosmos_abi_version(void);
void* azurecosmos_client_new(const char* endpoint, const char* key);
void azurecosmos_client_free(void* client);
```

```go
// sdk/data/azcosmos-core/core.go
package azcosmoscore

/*
#cgo CFLAGS: -I${SRCDIR}/include
#include "azurecosmos.h"
*/
import "C"

func ABIVersion() string {
    return C.GoString(C.azurecosmos_abi_version())
}
```

Each driver module contains exactly one native library and the link flags for
that target:

```go
// sdk/data/azcosmos-driver-linux-amd64-gnu/go.mod
module github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos-driver-linux-amd64-gnu

go 1.25.0
```

```go
// sdk/data/azcosmos-driver-linux-amd64-gnu/link_linux_amd64.go
//go:build linux && amd64 && !musl

package azcosmosdriverlinuxamd64gnu

/*
#cgo LDFLAGS: -L${SRCDIR}/native -lazurecosmos
*/
import "C"
```

The public `azcosmos` module depends on the core module and the default driver
modules. That keeps common platforms on the normal `go get` / `go build` path:

```go
// sdk/data/azcosmos/go.mod
module github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos

go 1.25.0

require (
    github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos-core v1.2.3
    github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos-driver-darwin-arm64 v1.2.3
    github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos-driver-linux-amd64-gnu v1.2.3
    github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos-driver-windows-amd64 v1.2.3
)
```

`azcosmos` imports the active default driver through build-tagged blank imports.
This import is important: a `require` entry alone does not make the driver's
cgo link flags participate in the final link.

```go
// sdk/data/azcosmos/default_driver_linux_amd64.go
//go:build linux && amd64 && !musl

package azcosmos

import _ "github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos-driver-linux-amd64-gnu"
```

```go
// sdk/data/azcosmos/default_driver_darwin_arm64.go
//go:build darwin && arm64

package azcosmos

import _ "github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos-driver-darwin-arm64"
```

`azcosmos` can also provide a clear unsupported-platform error instead of letting
customers hit a low-level linker failure:

```go
// sdk/data/azcosmos/unsupported_driver.go
//go:build !(linux && amd64 && !musl) && !(darwin && arm64) && !(windows && amd64)

package azcosmos

import "errors"

func nativeDriverAvailabilityError() error {
    return errors.New("azcosmos: no native Cosmos driver package is configured for this target")
}
```

The public client surface remains the package customers import:

```go
// sdk/data/azcosmos/client.go
package azcosmos

import "github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos-core"

func NativeDriverVersion() string {
    return azcosmoscore.ABIVersion()
}
```

A customer on a default platform still writes ordinary Go application code. They
do not import the driver package directly:

```go
// customer-app/go.mod
module example.com/customer-app

go 1.25.0

require github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos v1.2.3
```

```go
// customer-app/main.go
package main

import (
    "fmt"

    "github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos"
)

func main() {
    fmt.Println(azcosmos.NativeDriverVersion())
}
```

The customer runs normal Go commands:

```bash
go get github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos@v1.2.3
go build ./...
```

For `GOOS=linux GOARCH=amd64`, Go includes `default_driver_linux_amd64.go`.
That file blank-imports `azcosmos-driver-linux-amd64-gnu`, so the driver module
contributes `#cgo LDFLAGS` and the app links against
`native/libazurecosmos.a`:

```text
customer app
  imports azcosmos
      │
      ▼
azcosmos imports azcosmos-core
      │
      ▼
linux/amd64 build tag activates default_driver_linux_amd64.go
      │
      ▼
azcosmos-driver-linux-amd64-gnu is imported
      │
      ▼
driver module contributes -L${SRCDIR}/native -lazurecosmos
      │
      ▼
customer binary links with the Rust native driver
```

For libc variants, Go does not provide a built-in `glibc` versus `musl` target
dimension. If both are supported, the design needs an explicit convention such
as a custom `musl` build tag, a separate opt-in package, or a default Linux
choice with the other variant documented as optional. For example, if Linux musl
is published but not included in the default `azcosmos` module, the customer
would add the optional driver and activate it with a blank import or small Azure
SDK metapackage:

```bash
go get github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos@v1.2.3
go get github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos-driver-linux-amd64-musl@v1.2.3
```

```go
//go:build linux && amd64 && musl

package main

import _ "github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos-driver-linux-amd64-musl"
```

Salient features:

| Area                | Effect                                                                                                                                          |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| Customer experience | Good. Common platforms still use `go get azcosmos` / `go build`.                                                                                |
| Download footprint  | Better than one giant module in cache shape, but the default `azcosmos` module may still cause all default driver module ZIPs to be downloaded. |
| Version safety      | Manageable if all modules are released together and exact versions are pinned.                                                                  |
| Repository impact   | Still high for `azure-sdk-for-go` contributors because the repo contains all native modules.                                                    |
| Long-tail targets   | Better. Optional targets can be extra modules not included in default `azcosmos`.                                                               |

The subtle but critical rule is that `azcosmos` must import the selected driver
packages. Merely listing them in `go.mod` does not make their cgo link flags
participate.

**Size gist:** if the default `azcosmos` module requires six default driver
modules, customers may still download/cache roughly the same ~30 MB matrix
before compression. The difference is that the payload is split into separate
module ZIPs, optional targets can stay out of the default path, and the final app
still links only the current platform's native driver.

## 7. Option C: split modules, native drivers in a separate repository

This keeps the customer-facing SDK in `azure-sdk-for-go`, but moves the binary
payload modules to a separate Azure-owned repository.

```text
github.com/Azure/azure-sdk-for-go
└── sdk/data/azcosmos/
└── sdk/data/azcosmos-core/

github.com/Azure/azure-cosmos-go-native-drivers
└── azcosmos-driver-windows-amd64/
└── azcosmos-driver-windows-arm64/
└── azcosmos-driver-darwin-amd64/
└── azcosmos-driver-darwin-arm64/
└── azcosmos-driver-linux-amd64-gnu/
└── azcosmos-driver-linux-arm64-gnu/
```

Customer flow is still Go-native:

```text
Customer runs:
  go get github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos

Go module graph includes:
  azcosmos
  azcosmos-core
  github.com/Azure/azure-cosmos-go-native-drivers/azcosmos-driver-darwin-arm64

Customer does not clone the native-driver repo manually.
```

The public SDK can hide the separate repository behind build-tagged blank
imports:

```go
//go:build linux && amd64

package azcosmos

import _ "github.com/Azure/azure-cosmos-go-native-drivers/azcosmos-driver-linux-amd64-gnu"
```

Salient features:

| Area                | Effect                                                                       |
| ------------------- | ---------------------------------------------------------------------------- |
| Customer experience | Good for default platforms if `azcosmos` wires driver imports automatically. |
| Download footprint  | Similar to Option B from the customer's module-cache perspective.            |
| Version safety      | Requires stronger release discipline across repositories.                    |
| Repository impact   | Better for Azure SDK for Go contributors; binary churn lives elsewhere.      |
| Governance          | Needs clear ownership, release, security, signing, and support boundaries.   |

This model separates two concerns:

```text
Go customers care about modules.
Azure SDK contributors care about repositories.
```

Separate modules help customer download granularity. A separate repository helps
the Azure SDK for Go repository avoid carrying binary payloads in its Git history.

**Size gist:** for customers, this is similar to Option B if `azcosmos` still
depends on the default driver modules. The default download/cache can still be
the default matrix, but the binary modules come from a separate repository and
the final app links only one target.

## 8. Option D: small SDK module plus GitHub release assets

In this model, the Go SDK module does not carry native binaries. Native drivers
are published as release assets:

```text
GitHub release v1.2.3
├── azurecosmos-native-windows-amd64.zip
├── azurecosmos-native-darwin-arm64.tar.gz
├── azurecosmos-native-linux-amd64-gnu.tar.gz
└── checksums.txt
```

Customer flow requires an acquisition step:

```text
go get azcosmos
download native asset for current platform
configure linker path or runtime loader path
go build
```

That can be wrapped in a helper command:

```bash
go run github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos/cmd/azcosmos-native-install
go build ./...
```

Salient features:

| Area                | Effect                                                                             |
| ------------------- | ---------------------------------------------------------------------------------- |
| Customer experience | Weaker. There is an extra step unless the customer builds custom automation.       |
| Download footprint  | Best for customers who only fetch one platform.                                    |
| Version safety      | Must validate asset version and checksum against SDK version.                      |
| Enterprise/offline  | Harder. Build-time downloads are often blocked; vendoring is less straightforward. |
| Supportability      | More failure modes: proxy, cache, checksum, install path, linker path.             |

This model is attractive for tools and CLIs. It is more difficult to make feel
like a normal first-party Go SDK.

**Size gist:** customers download only their platform's native asset, so the
native payload is closer to ~5 MB before compression instead of the whole
matrix. The trade-off is an explicit install/download step and more failure
modes.

## 9. Option E: wrapper-only SDK; customer supplies native driver

This is the ONNX Runtime-style model. The SDK contains the Go wrapper and header,
but customers bring the matching native library.

```text
Customer app
  imports azcosmos
  downloads/copies libazurecosmos for their platform
  configures path before client creation
```

Example shape:

```go
azcosmos.SetNativeDriverPath("/opt/azurecosmos/libazurecosmos.so")
client, err := azcosmos.NewClient(endpoint, credential, nil)
```

Salient features:

| Area                | Effect                                                          |
| ------------------- | --------------------------------------------------------------- |
| Customer experience | Weak for the default Azure SDK path.                            |
| Download footprint  | Minimal Go module footprint.                                    |
| Version safety      | Customer can accidentally mix incompatible SDK/native versions. |
| Enterprise/offline  | Predictable for customers with strict packaging systems.        |
| Supportability      | Native setup becomes a customer-owned prerequisite.             |

This is useful as an advanced fallback but should be treated carefully if the
goal is a normal SDK experience.

**Size gist:** the Go module stays small because it carries no native matrix.
Customers still need one native driver for their platform, but they acquire and
place it themselves.

## 10. Option F: standalone native-backed Go SDK

This model creates a separate public SDK identity for the FFI-backed package,
instead of making it the default `azcosmos` module.

```text
github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos
github.com/Azure/azure-sdk-for-go/sdk/data/azcosmosnative
```

or:

```text
github.com/Azure/azure-cosmos-go-native/azcosmos
```

Salient features:

| Area                | Effect                                                                  |
| ------------------- | ----------------------------------------------------------------------- |
| Customer experience | Honest about native requirements, but splits product identity.          |
| Download footprint  | Existing `azcosmos` users are unaffected.                               |
| Migration           | Requires guidance on which SDK to choose.                               |
| Supportability      | Clearer native boundary, but more docs/samples/support surface.         |
| Governance          | Requires Azure SDK board alignment because it changes package identity. |

This model is less a packaging trick and more a product-positioning decision.

**Size gist:** existing `azcosmos` customers avoid the native payload entirely.
Customers who opt into the native-backed SDK pay whatever distribution model
that SDK chooses: bundled matrix, split driver modules, or per-platform asset.

## 11. Default driver set selection

If the split-module model is used, there are two distinct decisions:

1. **Which driver modules are published?**
2. **Which of those are included in the default `azcosmos` experience?**

For example:

```text
Published modules:
  windows-amd64
  windows-arm64
  darwin-amd64
  darwin-arm64
  linux-amd64-gnu
  linux-arm64-gnu
  windows-386
  linux-amd64-musl

Default azcosmos imports:
  windows-amd64
  windows-arm64
  darwin-amd64
  darwin-arm64
  linux-amd64-gnu
  linux-arm64-gnu

Optional:
  windows-386
  linux-amd64-musl
```

Build-time selection then happens through build tags:

```text
GOOS=darwin GOARCH=arm64
       │
       ▼
default_driver_darwin_arm64.go is included
       │
       ▼
azcosmos-driver-darwin-arm64 is imported
       │
       ▼
darwin/arm64 native library contributes link flags
```

For a platform outside the default set, the SDK should fail clearly:

```text
azcosmos: no native Cosmos driver package is configured for linux/amd64/musl.
Add the linux/amd64/musl driver package or use a supported default platform.
```

Optional targets need an activation story. The most Go-native pattern is a small
blank-import package or explicit driver import:

```go
// For a non-default target, if explicitly supported.
import _ "github.com/Azure/azure-cosmos-go-native-drivers/azcosmos-driver-linux-amd64-musl"
```

## 12. Versioning and ABI safety

All models need a version contract between:

- Go wrapper package
- `azurecosmos.h`
- native Rust driver library
- C ABI version

The safest customer-facing rule is exact version alignment:

```text
azcosmos v1.2.3
  requires azcosmos-core v1.2.3
  requires azcosmos-driver-* v1.2.3
  expects native ABI version 1.2.3
```

At minimum, the native driver should expose an ABI/version function:

```c
const char* azurecosmos_abi_version(void);
```

The Go wrapper can validate it during initialization and produce a direct error
if the linked library is wrong:

```text
azcosmos: native driver ABI mismatch:
  Go wrapper expects 1.2.3
  linked native driver reports 1.2.1
```

This validation matters most for manual, dynamic, or optional-driver paths. It
is still useful for bundled paths because it catches packaging mistakes.

## 13. Customer experience comparison

| Model                           | Common-platform customer flow              | Customer-visible native setup               | Download behavior                                                              |
| ------------------------------- | ------------------------------------------ | ------------------------------------------- | ------------------------------------------------------------------------------ |
| A. Single bundled module        | `go get azcosmos`; `go build`              | cgo toolchain only                          | Downloads whole default matrix in one module                                   |
| B. Split modules, same repo     | `go get azcosmos`; `go build`              | cgo toolchain only                          | Downloads default driver modules as dependencies; links only the active target |
| C. Split modules, separate repo | `go get azcosmos`; `go build`              | cgo toolchain only                          | Downloads driver modules from a second repo through Go module system           |
| D. Release assets/download      | `go get`; install native asset; `go build` | explicit download/install step              | Downloads only target asset                                                    |
| E. Wrapper-only/manual native   | `go get`; install/configure native lib     | explicit customer-managed native dependency | Minimal Go module, external native payload                                     |
| F. Standalone native SDK        | depends on package identity                | depends on chosen submodel                  | isolates impact from existing SDK                                              |

## 14. Repository and release comparison

| Model                           | Azure SDK for Go repo impact                                         | Release coordination                  | Reviewability                                     |
| ------------------------------- | -------------------------------------------------------------------- | ------------------------------------- | ------------------------------------------------- |
| A. Single bundled module        | Highest; all binaries live under one module                          | Simple, one module version            | Large binary diffs in SDK PRs                     |
| B. Split modules, same repo     | High; binaries still live in repo, but module payloads are separated | Moderate; many modules, same repo     | Binary PRs still affect SDK repo                  |
| C. Split modules, separate repo | Lower; native payload outside main SDK repo                          | Higher; cross-repo version alignment  | Cleaner SDK PRs, separate native PRs              |
| D. Release assets/download      | Low source impact                                                    | Higher; release assets must match SDK | Asset publishing/reproducibility must be reviewed |
| E. Wrapper-only/manual native   | Lowest source impact                                                 | Lower for SDK, higher for customers   | Native setup mostly outside SDK PRs               |
| F. Standalone native SDK        | Depends on repository choice                                         | Product-level coordination            | Clearer separation, more product surface          |

## 15. Discussion checkpoints for Go Central SDK review

These are the questions that likely need board-level alignment:

1. Is the default Azure SDK experience allowed to require cgo and a C toolchain
   when prebuilt native driver artifacts are included?
2. Should the default Cosmos Go module include all mainstream platform drivers,
   or should some platforms be opt-in?
3. Is binary payload in `azure-sdk-for-go` acceptable, or should native driver
   packages live in a separate Azure-owned repository?
4. Does the Azure SDK release system support synchronized versioning across a
   public SDK module, core wrapper module, and several driver modules?
5. How should optional/long-tail platforms be activated: direct blank import,
   Azure SDK metapackage, dynamic/manual native path, or unsupported?
6. What is the signing, checksum, and provenance story for native artifacts?
7. What exact error should customers see when no driver is configured for their
   target?

## 16. Current read

The split-module design is the strongest candidate for discussion because it
preserves the common-platform customer experience while making the native payload
more granular than a single bundled module.

The key remaining decision is repository placement:

```text
Same repo:
  simpler release alignment
  larger azure-sdk-for-go checkout and binary churn

Separate native-driver repo:
  cleaner azure-sdk-for-go developer experience
  more explicit cross-repo release/version governance
```

For customer experience, the separate-repo option can still be nearly invisible
if the driver packages are normal Go modules and the public `azcosmos` package
imports the default drivers automatically.
