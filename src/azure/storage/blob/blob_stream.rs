use chrono::{DateTime, Utc};
use core::errors::AzureError;
use core::lease::LeaseId;
use core::range::Range;
use core::{BlobNameSupport, ContainerNameSupport, LeaseIdSupport, RangeSupport, SnapshotSupport};
use futures::prelude::*;
use storage::blob::responses::GetBlobResponse;
use storage::client::{Blob as BlobTrait, Client};

pub struct BlobStream<'a> {
    client: &'a Client,
    container_name: &'a str,
    blob_name: &'a str,
    snapshot: Option<&'a DateTime<Utc>>,
    lease_id: Option<&'a LeaseId>,
    cur_pos: u64,
    ending_pos: u64,
    increment: u64,
    future: Option<Box<Future<Item = GetBlobResponse, Error = AzureError>>>,
}

impl<'a> BlobStream<'a> {
    pub fn new(
        client: &'a Client,
        container_name: &'a str,
        blob_name: &'a str,
        snapshot: Option<&'a DateTime<Utc>>,
        range: &Range,
        lease_id: Option<&'a LeaseId>,
        increment: u64,
    ) -> BlobStream<'a> {
        BlobStream {
            client,
            container_name,
            blob_name,
            snapshot,
            lease_id,
            cur_pos: range.start,
            ending_pos: range.end,
            increment,
            future: None,
        }
    }
}

impl<'a> Stream for BlobStream<'a> {
    type Item = Vec<u8>;
    type Error = AzureError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        // if cur_pos >= ending_pos we are done!

        if self.cur_pos >= self.ending_pos {
            return Ok(Async::Ready(None));
        }

        let range = Range {
            start: self.cur_pos,
            end: self.cur_pos + self.increment - 1,
        };

        let item_to_add = match self.future {
            None => {
                println!("range == {:?}", range);
                let mut f = self
                    .client
                    .get_blob()
                    .with_container_name(&self.container_name)
                    .with_blob_name(&self.blob_name)
                    .with_range(&range);

                if let Some(l) = self.lease_id {
                    f = f.with_lease_id(l);
                }

                if let Some(sn) = self.snapshot {
                    f = f.with_snapshot(*sn);
                }

                let f = f.finalize();

                Some(Box::new(f))
            }
            Some(_) => None,
        };

        if let Some(s) = item_to_add {
            self.future = Some(s);
        };

        let retval = if let Some(ref mut future) = self.future {
            match future.poll() {
                Ok(Async::Ready(t)) => Ok(Async::Ready(Some(t.data))),
                Ok(Async::NotReady) => Ok(Async::NotReady),
                Err(e) => Err(e),
            }
        } else {
            unreachable!();
        };

        // now if the previous future has completed
        // reset self.future so at the next iteration
        // we will perform the following request
        if let Ok(Async::Ready(Some(_))) = retval {
            self.future = None;
            self.cur_pos += self.increment;
        }

        retval
    }
}
