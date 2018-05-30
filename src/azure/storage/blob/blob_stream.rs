use chrono::{DateTime, Utc};
use core::errors::AzureError;
use core::lease::LeaseId;
use core::range::Range;
use futures::prelude::*;
use storage::blob::Blob;
use storage::client::Client;

pub struct BlobStream<'a> {
    client: &'a Client,
    container_name: &'a str,
    blob_name: &'a str,
    snapshot: Option<&'a DateTime<Utc>>,
    lease_id: Option<&'a LeaseId>,
    cur_pos: u64,
    ending_pos: u64,
    increment: u64,
    future: Option<Box<Future<Item = (Blob, Vec<u8>), Error = AzureError>>>,
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
            client: client,
            container_name: container_name,
            blob_name: blob_name,
            snapshot: snapshot,
            lease_id: lease_id,
            cur_pos: range.start,
            ending_pos: range.end,
            increment: increment,
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
                let f: Box<Future<Item = (Blob, Vec<u8>), Error = AzureError>> =
                    Box::new(Blob::get(
                        self.client,
                        &self.container_name,
                        &self.blob_name,
                        self.snapshot,
                        Some(&range),
                        self.lease_id,
                    ));
                Some(f)
            }
            Some(_) => None,
        };

        if let Some(s) = item_to_add {
            self.future = Some(s);
        };

        let retval = if let Some(ref mut future) = self.future {
            match future.poll() {
                Ok(Async::Ready(t)) => Ok(Async::Ready(Some(t.1))),
                Ok(Async::NotReady) => Ok(Async::NotReady),
                Err(e) => Err(e),
            }
        } else {
            unreachable!();
        };

        // now if the previous future has completed
        // reset self.future so at the next iteration
        // we will perform the following request

        match retval {
            Ok(Async::Ready(Some(_))) => {
                self.future = None;
                self.cur_pos += self.increment;
            }
            _ => {}
        }
        retval
    }
}
