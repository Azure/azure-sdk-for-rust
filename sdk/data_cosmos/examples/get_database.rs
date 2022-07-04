use azure_core::headers::Headers;
use azure_core::prelude::*;
use azure_core::CustomHeaders;
use clap::Parser;

mod util;

#[derive(Debug, clap::Parser)]
struct Args {
    #[clap(flatten)]
    auth: util::Auth,
    /// The name of the database
    database_name: String,
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let args = Args::parse();
    let client = args.auth.into_client()?;
    let database = client.database_client(args.database_name.clone());

    let mut context = Context::new();

    // Next we create a CustomHeaders type and insert it into the context allowing us to insert custom headers.
    let custom_headers: CustomHeaders = {
        let mut custom_headers = Headers::new();
        custom_headers.insert("MyCoolHeader", "CORS maybe?");
        custom_headers.into()
    };

    context.insert(custom_headers);

    let response = database
        .get_database()
        .context(context)
        .into_future()
        .await?;
    println!("response == {:?}", response);

    Ok(())
}
