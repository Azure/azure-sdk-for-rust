// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Request Unit (RU) charging model for the in-memory emulator.

/// Configurable RU charging rates.
#[derive(Clone, Debug)]
pub struct RuChargingModel {
    /// Base RU for a 1KB read.
    pub read_base_ru: f64,
    /// Base RU for a 1KB create.
    pub create_base_ru: f64,
    /// Multiplier for replace/delete relative to create (default 1.5).
    pub write_multiplier: f64,
    /// Additional RU per top-level JSON property on writes (indexing).
    pub indexing_ru_per_property: f64,
}

impl Default for RuChargingModel {
    fn default() -> Self {
        Self {
            read_base_ru: 1.0,
            create_base_ru: 5.8,
            write_multiplier: 1.5,
            indexing_ru_per_property: 0.3,
        }
    }
}

impl RuChargingModel {
    /// Returns the size bucket multiplier (doubling from 1KB).
    ///
    /// Caps the multiplier at `2^62` so that `next_power_of_two` cannot panic
    /// in debug or silently wrap to 0 in release for pathological inputs
    ///. Real Cosmos limits documents to 2 MB, so this cap is
    /// purely defensive — it never triggers for any realistic body.
    fn size_multiplier(doc_size: usize) -> f64 {
        if doc_size == 0 {
            return 1.0;
        }
        let kb = ((doc_size as f64) / 1024.0).ceil().max(1.0);
        // Anything that would round to >= 2^62 KB is past any reasonable cap;
        // saturate to 2^62 so `next_power_of_two` (which returns the next pow2
        // *>= self*) stays defined and produces 2^62.
        const MAX_KB: u64 = 1u64 << 62;
        let kb_int: u64 = if kb >= MAX_KB as f64 {
            MAX_KB
        } else {
            kb as u64
        };
        kb_int.next_power_of_two() as f64
    }

    /// Computes RU charge for a point read.
    pub fn compute_read_ru(&self, doc_size: usize) -> f64 {
        self.read_base_ru * Self::size_multiplier(doc_size)
    }

    /// Computes RU charge for a create operation.
    pub fn compute_create_ru(&self, doc_size: usize, num_properties: usize) -> f64 {
        let base = self.create_base_ru * Self::size_multiplier(doc_size);
        base + self.indexing_ru_per_property * num_properties as f64
    }

    /// Computes RU charge for a replace or delete operation.
    /// (Renamed from compute_replace_ru to make the dual usage explicit.)
    pub fn compute_replace_or_delete_ru(&self, doc_size: usize, num_properties: usize) -> f64 {
        let base = self.create_base_ru * self.write_multiplier * Self::size_multiplier(doc_size);
        base + self.indexing_ru_per_property * num_properties as f64
    }

    /// Computes the number of top-level properties in a JSON value.
    pub fn count_properties(body: &serde_json::Value) -> usize {
        match body.as_object() {
            Some(obj) => obj.len(),
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_read_1kb() {
        let model = RuChargingModel::default();
        assert!((model.compute_read_ru(512) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn read_2kb() {
        let model = RuChargingModel::default();
        assert!((model.compute_read_ru(1025) - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn read_4kb() {
        let model = RuChargingModel::default();
        assert!((model.compute_read_ru(3000) - 4.0).abs() < f64::EPSILON);
    }

    #[test]
    fn create_1kb_5_props() {
        let model = RuChargingModel::default();
        let ru = model.compute_create_ru(512, 5);
        let expected = 5.8 + 0.3 * 5.0;
        assert!((ru - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn replace_1kb_5_props() {
        let model = RuChargingModel::default();
        let ru = model.compute_replace_or_delete_ru(512, 5);
        let expected = 5.8 * 1.5 + 0.3 * 5.0;
        assert!((ru - expected).abs() < f64::EPSILON);
    }
}
