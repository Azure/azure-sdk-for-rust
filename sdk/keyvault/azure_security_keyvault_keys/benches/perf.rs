// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    error::{Error, ErrorKind, Result},
    http::RequestContent,
};
use azure_core_test::TestContext;
use azure_security_keyvault_keys::{
    models::{CreateKeyParameters, CurveName, KeyType},
    KeyClient,
};
use clap::Parser;
use futures::TryStreamExt as _;
use std::sync::Arc;

// Could also consider a PerfContext, but we'd still want to probably forward methods from TestContext so may not be worth it
// given this is purely an internal API: no reason to complicate design and make maintenance more expensive.
async fn list_keys(ctx: TestContext, args: &Args) -> Result<()> {
    let recording = ctx.recording();
    let endpoint = recording
        .var_opt("AZURE_KEYVAULT_URL", None)
        .ok_or_else(|| Error::new(ErrorKind::Other, "expected AZURE_KEYVAULT_URL"))?;

    let client = Arc::new(KeyClient::new(&endpoint, recording.credential(), None)?);

    // Setup
    let params = CreateKeyParameters {
        kty: Some(KeyType::EC),
        curve: Some(CurveName::P256),
        ..Default::default()
    };
    let params: RequestContent<CreateKeyParameters> = params.try_into()?;
    for i in 0..args.count {
        client
            .create_key(&format!("key{i}"), params.clone(), None)
            .await?;
    }

    // Run
    // This is a place where we could also define some helpers that takes a capture.
    for _ in 0..args.common.parallel {
        let client = client.clone();
        tokio::spawn(async move {
            let mut count = 0;
            let mut pager = client.list_key_properties(None).expect("expected pager");
            while let Ok(Some(_)) = pager.try_next().await {
                count += 1;
            }
            println!("listed {count} keys");
        });
    }

    Ok(())
}

// We'd have a functional macro or something to define this, just like criterion.
// The advantage here is that we set `harness = false` but it's not criterion's `main` so it ignores it.
// Would run with:
//
//     cargo bench -p azure_security_keyvault_keys --bench perf -- --parallel 64 --count 50
#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Like criterion, we probably should run each async test serially.
    // That said, it might also be that the perf framework calls each method individually, in which case we can use a separate file for each test
    // and you don't need a separate function.
    {
        // We'd actually want to create a new TestContext (or PerfContext) in azure_core_test and have it do the work
        // must like it does for tests. In this case, we don't want it to bootstrap the TestProxy, though, but still find the .env file
        // and all that. So it probably needs to be a separate proc macro or something. If we assume a single perf test per module (file),
        // it could generate the `main` and everything. Imagine a declaration like:
        //
        // #[perf::test]
        // async fn list_secrets(ctx: PerfContext<Args>) -> azure_core::Result<()> {
        //     todo!()
        // }
        // #[derive(clap::Parser)]
        // ...
        //
        // And it does all of this, basically. `PerfContext<T>` could constrain `T` to a `PerfArgs` trait or something
        // so if no specific args are needed, we can default it to a `CommonArgs` as shown below, where `PerfArgs: Parser`, which `CommonArgs` implements.
        let ctx =
            TestContext::new(env!("CARGO_MANIFEST_DIR"), module_path!(), "list_keys").unwrap();
        list_keys(ctx, &args).await.unwrap();
    }
}

// Would be defined in azure_core_test or something common.
#[derive(Debug, Default, clap::Args)]
struct CommonArgs {
    #[arg(long)]
    bench: bool,

    #[arg(long, default_value_t = 1)]
    parallel: usize,
}

// Defined by each perf test module.
#[derive(Debug, Default, Parser)]
#[command(about, long_about = None, version)]
struct Args {
    #[command(flatten)]
    common: CommonArgs,

    #[arg(long, default_value_t = 10)]
    count: usize,
}
