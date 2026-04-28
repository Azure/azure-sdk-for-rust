// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::cmp::min;

use azure_core::stream::SeekableStream;
use bytes::BufMut;
use crc_fast::{CrcAlgorithm, Digest};
use rand::random;

use crate::data_streams::GeneratedStream;

pub fn random_data_stream_with_checksum(
    len: u64,
    algorithm: CrcAlgorithm,
) -> (impl SeekableStream, u64) {
    let src_bytes: [u8; 9999] = random();

    let mut digest = Digest::new(algorithm);
    let mut read = 0;
    while read < len {
        let to_digest = min(src_bytes.len(), (len - read) as usize);
        digest.update(&src_bytes[..to_digest]);
        read += to_digest as u64;
    }

    (
        GeneratedStream::from_iter(src_bytes.into_iter(), len, None),
        digest.finalize(),
    )
}

pub fn random_data_memory_with_checksum(len: usize, algorithm: CrcAlgorithm) -> (Vec<u8>, u64) {
    let buf: [u8; 9999] = random();
    let mut data = Vec::with_capacity(len);

    for i in (0..len).step_by(buf.len()) {
        data.put(&buf[..min(buf.len(), i - len)]);
    }

    let mut digest = Digest::new(algorithm);
    digest.update(&data);
    (data, digest.finalize())
}
