// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use std::sync::atomic::{AtomicUsize, Ordering};

/// Assigns events to partitions.
///
/// The resolver holds the partition IDs that the client read when it opened.
/// It assigns a partition in one of two ways:
///
/// * Round robin, when the caller gives no partition ID and no partition key.
/// * A hash of the partition key, when the caller gives a partition key.
///
/// The hash matches the other Azure SDKs and the Event Hubs gateway. Two
/// clients in two languages send the same partition key to the same partition.
pub(crate) struct PartitionResolver {
    partitions: Vec<String>,
    next: AtomicUsize,
}

impl PartitionResolver {
    /// Creates a resolver over the given partition IDs.
    ///
    /// The caller must supply at least one partition ID.
    pub(crate) fn new(partitions: Vec<String>) -> Self {
        debug_assert!(
            !partitions.is_empty(),
            "a partition resolver needs at least one partition"
        );
        Self {
            partitions,
            next: AtomicUsize::new(0),
        }
    }

    /// Returns `true` when the given partition ID is one of the known partitions.
    pub(crate) fn contains(&self, partition_id: &str) -> bool {
        self.partitions.iter().any(|p| p == partition_id)
    }

    /// Assigns the next partition in round-robin order.
    pub(crate) fn assign_round_robin(&self) -> &str {
        // The counter wraps. A wrap changes which partition follows which, but
        // every index stays inside the partition range.
        let index = self.next.fetch_add(1, Ordering::Relaxed);
        &self.partitions[index % self.partitions.len()]
    }

    /// Assigns the partition that the partition key hashes to.
    pub(crate) fn assign_for_key(&self, partition_key: &str) -> &str {
        let hash = Self::generate_hash_code(partition_key);
        let index = ((hash as i32) % (self.partitions.len() as i32)).unsigned_abs() as usize;
        &self.partitions[index]
    }

    /// Generates the hash code for a partition key with the Jenkins lookup3 algorithm.
    ///
    /// This is a port of the .NET implementation, which is itself a port of the
    /// Event Hubs service code. The value must match the gateway, so do not
    /// change it without careful thought.
    ///
    /// Source: <https://github.com/Azure/azure-sdk-for-net/blob/main/sdk/eventhub/Azure.Messaging.EventHubs/src/Core/PartitionResolver.cs>
    fn generate_hash_code(partition_key: &str) -> i16 {
        let (hash1, hash2) = Self::compute_hash(partition_key.as_bytes(), 0, 0);
        (hash1 ^ hash2) as u16 as i16
    }

    /// Computes the two lookup3 hash values for the given bytes.
    ///
    /// Source: <https://github.com/Azure/azure-sdk-for-net/blob/main/sdk/eventhub/Azure.Messaging.EventHubs/src/Core/PartitionResolver.cs>
    fn compute_hash(data: &[u8], seed1: u32, seed2: u32) -> (u32, u32) {
        fn le32(bytes: &[u8]) -> u32 {
            u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
        }

        let len = data.len() as u32;
        let mut a = 0xDEAD_BEEF_u32.wrapping_add(len).wrapping_add(seed1);
        let mut b = a;
        let mut c = a.wrapping_add(seed2);

        let chunks = if data.len() > 12 {
            (data.len() - 1) / 12
        } else {
            0
        };

        let mut offset = 0usize;
        for _ in 0..chunks {
            a = a.wrapping_add(le32(&data[offset..offset + 4]));
            b = b.wrapping_add(le32(&data[offset + 4..offset + 8]));
            c = c.wrapping_add(le32(&data[offset + 8..offset + 12]));
            offset += 12;

            a = a.wrapping_sub(c);
            a ^= c.rotate_left(4);
            c = c.wrapping_add(b);

            b = b.wrapping_sub(a);
            b ^= a.rotate_left(6);
            a = a.wrapping_add(c);

            c = c.wrapping_sub(b);
            c ^= b.rotate_left(8);
            b = b.wrapping_add(a);

            a = a.wrapping_sub(c);
            a ^= c.rotate_left(16);
            c = c.wrapping_add(b);

            b = b.wrapping_sub(a);
            b ^= a.rotate_left(19);
            a = a.wrapping_add(c);

            c = c.wrapping_sub(b);
            c ^= b.rotate_left(4);
            b = b.wrapping_add(a);
        }

        let tail = &data[offset..];
        match tail.len() {
            12 => {
                a = a.wrapping_add(le32(&tail[0..4]));
                b = b.wrapping_add(le32(&tail[4..8]));
                c = c.wrapping_add(le32(&tail[8..12]));
            }
            left @ (9..=11) => {
                if left == 11 {
                    c = c.wrapping_add((tail[10] as u32) << 16);
                }
                if left >= 10 {
                    c = c.wrapping_add((tail[9] as u32) << 8);
                }
                c = c.wrapping_add(tail[8] as u32);
                b = b.wrapping_add(le32(&tail[4..8]));
                a = a.wrapping_add(le32(&tail[0..4]));
            }
            8 => {
                b = b.wrapping_add(le32(&tail[4..8]));
                a = a.wrapping_add(le32(&tail[0..4]));
            }
            left @ (5..=7) => {
                if left == 7 {
                    b = b.wrapping_add((tail[6] as u32) << 16);
                }
                if left >= 6 {
                    b = b.wrapping_add((tail[5] as u32) << 8);
                }
                b = b.wrapping_add(tail[4] as u32);
                a = a.wrapping_add(le32(&tail[0..4]));
            }
            4 => {
                a = a.wrapping_add(le32(&tail[0..4]));
            }
            left @ (1..=3) => {
                if left == 3 {
                    a = a.wrapping_add((tail[2] as u32) << 16);
                }
                if left >= 2 {
                    a = a.wrapping_add((tail[1] as u32) << 8);
                }
                a = a.wrapping_add(tail[0] as u32);
            }
            _ => return (c, b),
        }

        c ^= b;
        c = c.wrapping_sub(b.rotate_left(14));

        a ^= c;
        a = a.wrapping_sub(c.rotate_left(11));

        b ^= a;
        b = b.wrapping_sub(a.rotate_left(25));

        c ^= b;
        c = c.wrapping_sub(b.rotate_left(16));

        a ^= c;
        a = a.wrapping_sub(c.rotate_left(4));

        b ^= a;
        b = b.wrapping_sub(a.rotate_left(14));

        c ^= b;
        c = c.wrapping_sub(b.rotate_left(24));

        (c, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn partitions(count: usize) -> Vec<String> {
        (0..count).map(|i| i.to_string()).collect()
    }

    /// The expected values come from the .NET test suite. The Rust hash must
    /// agree with .NET, with the other SDKs, and with the Event Hubs gateway.
    ///
    /// Source: <https://github.com/Azure/azure-sdk-for-net/blob/main/sdk/eventhub/Azure.Messaging.EventHubs/tests/Core/PartitionResolverTests.cs>
    #[test]
    fn hash_matches_dotnet_vectors() {
        // cspell:disable
        let cases: &[(&str, i16)] = &[
            ("7", -15263),
            ("131", 30562),
            ("7149583486996073602", 12977),
            ("FWfAT", -22341),
            ("sOdeEAsyQoEuEFPGerWO", -6503),
            (
                "FAyAIctPeCgmiwLKbJcyswoHglHVjQdvtBowLACDNORsYvOcLddNJYDmhAVkbyLOrHTKLneMNcbgWVlasVywOByANjs",
                5226,
            ),
            (
                "1XYM6!(7(lF5wq4k4m*e$Nc!1ezLJv*1YK1Y-C^*&B$O)lq^iUkG(TNzXG;Zi#z2Og*Qq0#^*k):vXh$3,C7We7%W0meJ;b3,rQCg^J;^twXgs5E$$hWKxqp",
                23950,
            ),
            (
                "E(x;RRIaQcJs*P;D&jTPau-4K04oqr:lF6Z):ERpo&;9040qyV@G1_c9mgOs-8_8/10Fwa-7b7-yP!T-!IH&968)FWuI;(^g$2fN;)HJ^^yTn:",
                -29304,
            ),
            ("!c*_!I@1^c", 15372),
            ("p4*!jioeO/z-!-;w:dh", -3104),
            ("$0cb", 26269),
            ("-4189260826195535198", 453),
        ];
        // cspell:enable

        for (key, expected) in cases {
            assert_eq!(
                PartitionResolver::generate_hash_code(key),
                *expected,
                "the hash for key {key} was incorrect"
            );
        }
    }

    #[test]
    fn round_robin_walks_every_partition_then_wraps() {
        let resolver = PartitionResolver::new(partitions(4));

        let first: Vec<String> = (0..4)
            .map(|_| resolver.assign_round_robin().to_string())
            .collect();
        assert_eq!(first, vec!["0", "1", "2", "3"]);

        // The next pass repeats the same order.
        let second: Vec<String> = (0..4)
            .map(|_| resolver.assign_round_robin().to_string())
            .collect();
        assert_eq!(second, first);
    }

    #[test]
    fn round_robin_with_one_partition_always_returns_it() {
        let resolver = PartitionResolver::new(partitions(1));
        for _ in 0..5 {
            assert_eq!(resolver.assign_round_robin(), "0");
        }
    }

    #[test]
    fn key_assignment_is_stable() {
        let resolver = PartitionResolver::new(partitions(8));
        let first = resolver.assign_for_key("some-key").to_string();
        for _ in 0..5 {
            assert_eq!(resolver.assign_for_key("some-key"), first);
        }
    }

    #[test]
    fn key_assignment_stays_in_range() {
        // A key whose hash is negative must still map into the partition range.
        for count in 1..=32 {
            let resolver = PartitionResolver::new(partitions(count));
            // cspell:disable-next-line
            for key in ["7", "FWfAT", "p4*!jioeO/z-!-;w:dh", "", "$0cb"] {
                let assigned = resolver.assign_for_key(key);
                assert!(
                    resolver.contains(assigned),
                    "key {key} mapped outside the partition range for {count} partitions"
                );
            }
        }
    }

    #[test]
    fn contains_reports_known_partitions() {
        let resolver = PartitionResolver::new(partitions(3));
        assert!(resolver.contains("0"));
        assert!(resolver.contains("2"));
        assert!(!resolver.contains("3"));
        assert!(!resolver.contains("not-a-partition"));
    }
}
