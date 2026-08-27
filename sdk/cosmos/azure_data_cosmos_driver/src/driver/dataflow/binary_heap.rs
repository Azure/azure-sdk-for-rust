// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Comparator-driven binary-heap operations used by ordered merge stages.

/// Pushes `item` while preserving the heap defined by `precedes`.
pub(crate) fn push_by<T>(heap: &mut Vec<T>, item: T, precedes: impl Fn(&T, &T) -> bool) {
    heap.push(item);
    let mut index = heap.len() - 1;
    while index > 0 {
        let parent = (index - 1) / 2;
        if !precedes(&heap[index], &heap[parent]) {
            break;
        }
        heap.swap(index, parent);
        index = parent;
    }
}

/// Removes the highest-priority item from the heap defined by `precedes`.
pub(crate) fn pop_by<T>(heap: &mut Vec<T>, precedes: impl Fn(&T, &T) -> bool) -> Option<T> {
    if heap.is_empty() {
        return None;
    }

    let item = heap.swap_remove(0);
    if !heap.is_empty() {
        sift_down_by(heap, 0, &precedes);
    }
    Some(item)
}

/// Replaces the highest-priority item while preserving the heap.
pub(crate) fn replace_root_by<T>(heap: &mut [T], item: T, precedes: impl Fn(&T, &T) -> bool) {
    debug_assert!(!heap.is_empty());
    heap[0] = item;
    sift_down_by(heap, 0, &precedes);
}

fn sift_down_by<T>(heap: &mut [T], mut index: usize, precedes: &impl Fn(&T, &T) -> bool) {
    loop {
        let left = index * 2 + 1;
        if left >= heap.len() {
            return;
        }
        let right = left + 1;
        let next = if right < heap.len() && precedes(&heap[right], &heap[left]) {
            right
        } else {
            left
        };
        if !precedes(&heap[next], &heap[index]) {
            return;
        }
        heap.swap(index, next);
        index = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supports_min_and_max_heaps() {
        for (precedes, expected) in [
            (
                (|left: &i32, right: &i32| left < right) as fn(&i32, &i32) -> bool,
                vec![1, 2, 3, 4],
            ),
            (
                (|left: &i32, right: &i32| left > right) as fn(&i32, &i32) -> bool,
                vec![4, 3, 2, 1],
            ),
        ] {
            let mut heap = Vec::new();
            for item in [3, 1, 4, 2] {
                push_by(&mut heap, item, precedes);
            }
            let actual: Vec<_> = std::iter::from_fn(|| pop_by(&mut heap, precedes)).collect();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn replaces_root() {
        let precedes = |left: &i32, right: &i32| left > right;
        let mut heap = Vec::new();
        for item in [3, 1, 4, 2] {
            push_by(&mut heap, item, precedes);
        }

        replace_root_by(&mut heap, 0, precedes);

        let actual: Vec<_> = std::iter::from_fn(|| pop_by(&mut heap, precedes)).collect();
        assert_eq!(actual, [3, 2, 1, 0]);
    }
}
