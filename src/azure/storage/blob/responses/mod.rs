macro_rules! response_from_headers {
    ($cn:ident, $($fh:ident -> $na:ident: $typ:ty),+) => {
        use azure::core::errors::AzureError;
        use http::HeaderMap;
        use azure::core::{
            $($fh,)+
        };

        #[derive(Debug, Clone, PartialEq)]
        pub struct $cn {
            $($na: $typ),+,
        }

        impl $cn {
            pub(crate) fn from_headers(headers: &HeaderMap) -> Result<$cn, AzureError> {
               $(
                    let $na = $fh(headers)?;
                )+

                Ok($cn {
                    $($na,)+
                })
            }

        }
    };
}

mod put_block_list_response;
pub use self::put_block_list_response::PutBlockListResponse;
mod put_block_response;
pub use self::put_block_response::PutBlockResponse;
mod clear_page_response;
pub use self::clear_page_response::ClearPageResponse;
mod put_block_blob_response;
pub use self::put_block_blob_response::PutBlockBlobResponse;
mod list_blobs_response;
pub use self::list_blobs_response::ListBlobsResponse;
mod get_blob_response;
pub use self::get_blob_response::GetBlobResponse;
mod put_blob_response;
pub use self::put_blob_response::PutBlobResponse;
mod update_page_response;
pub use self::update_page_response::UpdatePageResponse;
