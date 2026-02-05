// The MIT License (MIT)
// Copyright (c) 2023 Microsoft Corporation
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Implementation of the MurmurHash3 128-bit hash function.
//
// MurmurHash is a non-cryptographic hash function suitable for general hash-based lookup. The name comes from two basic
// operations, multiply (MU) and rotate (R), used in its inner loop. Unlike cryptographic hash functions, it is not
// specifically designed to be difficult to reverse by an adversary, making it unsuitable for cryptographic purposes.
//
// This contains a rust port of the 128-bit hash function from Austin Appleby's original C++ code in SMHasher.
//
// This is public domain code with no copyrights. From home page of
// <a href="https://github.com/aappleby/smhasher">SMHasher</a>:
//  "All MurmurHash versions are public domain software, and the author disclaims all copyright to their code."

use std::convert::TryInto;

// Helper accessors for 128-bit values encoded in native `u128`.
#[inline]
fn low64(v: u128) -> u64 {
    v as u64
}
#[inline]
fn high64(v: u128) -> u64 {
    (v >> 64) as u64
}

/// Rotate left 64-bit.
#[inline]
pub fn rotate_left_64(val: u64, shift: u32) -> u64 {
    val.rotate_left(shift)
}

#[inline]
pub fn mix(mut value: u64) -> u64 {
    value ^= value >> 33;
    value = value.wrapping_mul(0xff51afd7ed558ccd);
    value ^= value >> 33;
    value = value.wrapping_mul(0xc4ceb9fe1a85ec53);
    value ^= value >> 33;
    value
}

/// Rust SDK implementation of 128 bit murmurhash3 from Dot Net SDK. To match with other SDKs, It is recommended to
/// do the following with number values, especially floats as other SDKs use Doubles
/// -> bytearray(struct.pack("d", #)) where # represents any number. The d will treat it as a double.
/// MurmurHash3 x64 128-bit implementation
/// `span` is the input bytes, `seed` is a 128-bit value whose low/high 64-bit
/// lanes initialize the internal state.
pub fn murmurhash3_128(span: &[u8], seed: u128) -> u128 {
    let c1: u64 = 0x87c37b91114253d5;
    let c2: u64 = 0x4cf5ad432745937f;

    let mut h1: u64 = low64(seed);
    let mut h2: u64 = high64(seed);

    let mut position = 0usize;
    let len = span.len();

    while position + 16 <= len {
        let k1 = u64::from_le_bytes(span[position..position + 8].try_into().unwrap());
        let k2 = u64::from_le_bytes(span[position + 8..position + 16].try_into().unwrap());

        // k1
        let mut k1 = k1.wrapping_mul(c1);
        k1 = rotate_left_64(k1, 31);
        k1 = k1.wrapping_mul(c2);
        h1 ^= k1;
        h1 = rotate_left_64(h1, 27);
        h1 = h1.wrapping_add(h2);
        h1 = h1.wrapping_mul(5).wrapping_add(0x52dce729);

        // k2
        let mut k2 = k2.wrapping_mul(c2);
        k2 = rotate_left_64(k2, 33);
        k2 = k2.wrapping_mul(c1);
        h2 ^= k2;
        h2 = rotate_left_64(h2, 31);
        h2 = h2.wrapping_add(h1);
        h2 = h2.wrapping_mul(5).wrapping_add(0x38495ab5);

        position += 16;
    }

    // tail
    let mut k1: u64 = 0;
    let mut k2: u64 = 0;
    let n = len & 15;

    if n >= 15 {
        k2 ^= (span[position + 14] as u64) << 48;
    }
    if n >= 14 {
        k2 ^= (span[position + 13] as u64) << 40;
    }
    if n >= 13 {
        k2 ^= (span[position + 12] as u64) << 32;
    }
    if n >= 12 {
        k2 ^= (span[position + 11] as u64) << 24;
    }
    if n >= 11 {
        k2 ^= (span[position + 10] as u64) << 16;
    }
    if n >= 10 {
        k2 ^= (span[position + 9] as u64) << 8;
    }
    if n >= 9 {
        k2 ^= span[position + 8] as u64;
    }

    if k2 != 0 {
        k2 = k2.wrapping_mul(c2);
        k2 = rotate_left_64(k2, 33);
        k2 = k2.wrapping_mul(c1);
        h2 ^= k2;
    }

    if n >= 8 {
        k1 ^= (span[position + 7] as u64) << 56;
    }
    if n >= 7 {
        k1 ^= (span[position + 6] as u64) << 48;
    }
    if n >= 6 {
        k1 ^= (span[position + 5] as u64) << 40;
    }
    if n >= 5 {
        k1 ^= (span[position + 4] as u64) << 32;
    }
    if n >= 4 {
        k1 ^= (span[position + 3] as u64) << 24;
    }
    if n >= 3 {
        k1 ^= (span[position + 2] as u64) << 16;
    }
    if n >= 2 {
        k1 ^= (span[position + 1] as u64) << 8;
    }
    if n >= 1 {
        k1 ^= span[position] as u64;
        k1 = k1.wrapping_mul(c1);
        k1 = rotate_left_64(k1, 31);
        k1 = k1.wrapping_mul(c2);
        h1 ^= k1;
    }

    // finalization
    h1 ^= len as u64;
    h2 ^= len as u64;
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);
    h1 = mix(h1);
    h2 = mix(h2);
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);

    ((h2 as u128) << 64) | (h1 as u128)
}

/// MurmurHash3 32-bit implementation
pub fn murmurhash3_32(data: &[u8], seed: u32) -> u32 {
    let c1: u32 = 0xcc9e2d51;
    let c2: u32 = 0x1b873593;
    let length: u32 = data.len() as u32;
    let mut h1: u32 = seed;
    let rounded_end = (length & 0xfffffffc) as usize; // round down to 4 byte block

    let mut i = 0usize;
    while i < rounded_end {
        let k1 = (data[i] as u32)
            | ((data[i + 1] as u32) << 8)
            | ((data[i + 2] as u32) << 16)
            | ((data[i + 3] as u32) << 24);
        i += 4;

        let mut k1 = k1.wrapping_mul(c1);
        k1 = k1.rotate_left(15);
        k1 = k1.wrapping_mul(c2);

        h1 ^= k1;
        h1 = h1.rotate_left(13);
        h1 = h1.wrapping_mul(5).wrapping_add(0xe6546b64);
    }

    // tail
    let mut k1_tail: u32 = 0;
    let tail = (length & 0x03) as usize;
    if tail == 3 {
        k1_tail ^= (data[rounded_end + 2] as u32) << 16;
    }
    if tail >= 2 {
        k1_tail ^= (data[rounded_end + 1] as u32) << 8;
    }
    if tail >= 1 {
        k1_tail ^= data[rounded_end] as u32;
        k1_tail = k1_tail.wrapping_mul(c1);
        k1_tail = k1_tail.rotate_left(15);
        k1_tail = k1_tail.wrapping_mul(c2);
        h1 ^= k1_tail;
    }

    // finalization
    h1 ^= length;
    h1 ^= h1 >> 16;
    h1 = h1.wrapping_mul(0x85ebca6b);
    h1 ^= h1 >> 13;
    h1 = h1.wrapping_mul(0xc2b2ae35);
    h1 ^= h1 >> 16;

    h1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_murmurhash3_128_float() {
        let value: f64 = 374.0;
        let ba_le: [u8; 8] = value.to_le_bytes();
        let ba_vec: Vec<u8> = ba_le.to_vec();
        let seed: u128 = 0;
        let h = murmurhash3_128(&ba_vec, seed);
        let _u128 = h;
        // known results
        assert_eq!(low64(h), 16628891264555680919);
        assert_eq!(high64(h), 12953474369317462);
    }

    #[test]
    fn test_murmurhash3_128_string() {
        let s = "sample-test";
        let bytes = s.as_bytes();
        let seed: u128 = 0;
        let h = murmurhash3_128(bytes, seed);
        let _u128 = h;
        // known results
        assert_eq!(low64(h), 9863137013172825203);
        assert_eq!(high64(h), 15859947107521786564);
    }

    #[test]
    fn test_murmurhash3_32_float() {
        let value: f64 = 374.0;
        let ba_le: [u8; 8] = value.to_le_bytes();
        let ba_vec: Vec<u8> = ba_le.to_vec();
        let seed: u32 = 0;
        let h = murmurhash3_32(&ba_vec, seed);
        let _u32 = h;
        // known results
        assert_eq!(h, 3717946798);
    }

    #[test]
    fn test_murmurhash3_32_string() {
        let s = "sample-test";
        let bytes = s.as_bytes();
        let seed: u32 = 0;
        let h = murmurhash3_32(bytes, seed);
        let _u32 = h;
        // known results
        assert_eq!(h, 2066086989);
    }
}
