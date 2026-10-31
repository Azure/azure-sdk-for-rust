// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Bounded, reproducible JSON samples shared by offline and live tests.

use serde_json::{json, Value};

pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    pub fn below(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound
    }

    pub fn byte(&mut self) -> u8 {
        self.next_u64() as u8
    }

    pub fn value(&mut self, depth: u32) -> Value {
        match self.below(if depth == 0 { 6 } else { 8 }) {
            0 => Value::Null,
            1 => Value::Bool(self.below(2) == 0),
            2 => Value::from(self.below(2_000_001) as i64 - 1_000_000),
            3 => Value::from((self.below(800_001) as i64 - 400_000) as f64 / 4.0 + 0.125),
            4 => Value::from(
                [
                    "",
                    "id",
                    "same",
                    "quote \" slash \\",
                    "\u{e9}\u{2603}\u{1d11e}",
                ][self.below(5) as usize],
            ),
            5 => Value::from("repeated-".repeat(self.below(65) as usize)),
            6 => {
                let count = self.below(5);
                Value::Array((0..count).map(|_| self.value(depth - 1)).collect())
            }
            _ => {
                let count = self.below(5);
                Value::Object(
                    (0..count)
                        .map(|i| (format!("key-{i}-\u{2603}"), self.value(depth - 1)))
                        .collect(),
                )
            }
        }
    }

    pub fn document(&mut self) -> Value {
        let mut spine = self.value(3);
        for level in 0..self.below(17) {
            spine = json!({"level": level, "next": spine});
        }
        json!({
            "random": self.value(4),
            "spine": spine,
            "empty": [null, [], {}, ""],
            "numbers": [-9007199254740992_i64, -1, 0, 31, 32, 255, 256, 65535, 65536, 9007199254740992_u64],
            "repeated": ["same", "same", "same"],
            "vector": (0..32).map(|i| if i % 4 == 0 { json!(i / 4) } else { json!(i as f64 / 4.0) }).collect::<Vec<_>>(),
        })
    }
}
