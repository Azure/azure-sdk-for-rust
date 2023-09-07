use azure_core::{
    error::{ErrorKind, ResultExt},
    tokio::fs::FileStreamBuilder,
};
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use clap::Parser;
use std::path::PathBuf;
use tokio::fs::File;

#[derive(Debug, Parser)]
struct Args {
    /// Name of the container to upload
    container_name: String,
    /// Blob name
    blob_name: String,
    /// File path to upload
    file_path: PathBuf,

    /// Offset to start uploading from
    #[clap(long)]
    offset: Option<u64>,

    /// how much to buffer in memory during streaming reads
    #[clap(long)]
    buffer_size: Option<usize>,

    #[clap(long)]
    block_size: Option<u64>,

    /// storage account name
    #[clap(env = "STORAGE_ACCOUNT")]
    account: String,

    /// storage account access key
    #[clap(env = "STORAGE_ACCESS_KEY")]
    access_key: String,
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::init();
    let args = Args::parse();

    let storage_credentials =
        StorageCredentials::Key(args.account.clone(), args.access_key.clone());
    let blob_client = BlobServiceClient::new(&args.account, storage_credentials)
        .container_client(&args.container_name)
        .blob_client(&args.blob_name);

    let file = File::open(&args.file_path).await?;

    let mut builder = FileStreamBuilder::new(file);

    if let Some(buffer_size) = args.buffer_size {
        builder = builder.buffer_size(buffer_size);
    }

    if let Some(offset) = args.offset {
        builder = builder.offset(offset);
    }

    if let Some(block_size) = args.block_size {
        builder = builder.block_size(block_size);
    }

    let mut handle = builder.build().await?;

    if let Some(block_size) = args.block_size {
        let mut block_list = BlockList::default();
        for offset in (handle.offset..handle.stream_size).step_by(block_size as usize) {
            log::info!("trying to upload at offset {offset} - {block_size}");
            let block_id = format!("{:08X}", offset);
            blob_client.put_block(block_id.clone(), &handle).await?;
            log::info!("uploaded block {block_id}");
            block_list
                .blocks
                .push(BlobBlockType::new_uncommitted(block_id));
            handle.next_block().await?;
        }
        blob_client.put_block_list(block_list).await?;
    } else {
        // upload as one large block
        blob_client.put_block_blob(handle).await?;
    }

    let blob = blob_client.get_content().await?;
    let s = String::from_utf8(blob).map_kind(ErrorKind::DataConversion)?;
    println!("retrieved contents == {s:?}");

    Ok(())
}
