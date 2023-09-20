use crate::{blob::operations::PutBlobResponse, prelude::*};
use azure_core::{
    headers::{Headers, BLOB_CONTENT_LENGTH, BLOB_TYPE},
    prelude::*,
};

operation! {
    PutPageBlob,
    client: BlobClient,
    length: u128,
    ?content_type: BlobContentType,
    ?content_encoding: BlobContentEncoding,
    ?content_language: BlobContentLanguage,
    ?content_disposition: BlobContentDisposition,
    ?metadata: Metadata,
    ?tags: Tags,
    ?lease_id: LeaseId,
    ?sequence_number: SequenceNumber
}

impl PutPageBlobBuilder {
    pub fn into_future(mut self) -> PutPageBlob {
        Box::pin(async move {
            let url = self.client.url()?;

            let mut headers = Headers::new();
            headers.insert(BLOB_TYPE, "PageBlob");
            headers.insert(BLOB_CONTENT_LENGTH, &format!("{}", self.length));
            headers.add(self.content_type);
            headers.add(self.content_encoding);
            headers.add(self.content_language);
            headers.add(self.content_disposition);
            headers.add(self.tags);
            if let Some(metadata) = &self.metadata {
                for m in metadata.iter() {
                    headers.add(m);
                }
            }
            headers.add(self.lease_id);
            headers.add(self.sequence_number);

            let mut request =
                BlobClient::finalize_request(url, azure_core::Method::Put, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;
            PutBlobResponse::from_headers(response.headers())
        })
    }
}

type PutPageBlobResponse = PutBlobResponse;
