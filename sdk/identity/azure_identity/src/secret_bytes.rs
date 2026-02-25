// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::Bytes;
use std::fmt;

/// Represents secret bytes, e.g., certificate data.
///
/// Neither the [`Debug`] nor the [`fmt::Display`] implementation will print the bytes.
#[derive(Clone, Eq)]
pub struct SecretBytes(Vec<u8>);

impl SecretBytes {
    /// Create a new `SecretBytes`.
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self(bytes.into())
    }

    /// Get the secret bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}

// NOTE: this is a constant time compare, however LLVM may (and probably will)
// optimize this in unexpected ways.
impl PartialEq for SecretBytes {
    fn eq(&self, other: &Self) -> bool {
        let a = self.bytes();
        let b = other.bytes();

        if a.len() != b.len() {
            return false;
        }

        a.iter().zip(b.iter()).fold(0, |acc, (a, b)| acc | (a ^ b)) == 0
    }
}

impl fmt::Debug for SecretBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretBytes")
    }
}

impl fmt::Display for SecretBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretBytes")
    }
}

impl From<Bytes> for SecretBytes {
    fn from(bytes: Bytes) -> Self {
        Self(bytes.to_vec())
    }
}

impl From<&[u8]> for SecretBytes {
    fn from(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }
}

impl From<Vec<u8>> for SecretBytes {
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_does_not_print_bytes() {
        let secret = SecretBytes::new(b"super-secret".to_vec());
        assert_eq!("SecretBytes", format!("{secret:?}"));
    }

    #[test]
    fn display_does_not_print_bytes() {
        let secret = SecretBytes::new(b"super-secret".to_vec());
        assert_eq!("SecretBytes", format!("{secret}"));
    }

    #[test]
    fn eq_same_bytes() {
        let a = SecretBytes::new(b"hello".to_vec());
        let b = SecretBytes::new(b"hello".to_vec());
        assert_eq!(a, b);
    }

    #[test]
    fn ne_different_bytes() {
        let a = SecretBytes::new(b"hello".to_vec());
        let b = SecretBytes::new(b"world".to_vec());
        assert_ne!(a, b);
    }

    #[test]
    fn ne_different_lengths() {
        let a = SecretBytes::new(b"hello".to_vec());
        let b = SecretBytes::new(b"hello!".to_vec());
        assert_ne!(a, b);
    }

    #[test]
    fn from_bytes_type() {
        let bytes = Bytes::from_static(b"data");
        let secret = SecretBytes::from(bytes);
        assert_eq!(b"data", secret.bytes());
    }

    #[test]
    fn from_slice() {
        let data: &[u8] = b"data";
        let secret = SecretBytes::from(data);
        assert_eq!(b"data", secret.bytes());
    }

    #[test]
    fn from_vec() {
        let secret = SecretBytes::from(b"data".to_vec());
        assert_eq!(b"data", secret.bytes());
    }
}
