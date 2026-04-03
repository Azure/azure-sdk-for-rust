// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub(crate) trait VecExt<T> {
    // Pushes a value to the `Vec` and returns a mutable reference to that value.
    // This is a stand-in, since the official `std` implementation `put_mut` is nightly-only.
    fn push_and_peek_mut(&mut self, value: T) -> &mut T;
}

impl<T> VecExt<T> for Vec<T> {
    fn push_and_peek_mut(&mut self, value: T) -> &mut T {
        self.push(value);
        // SAFETY: We JUST pushed a value. It's not empty.
        self.last_mut().unwrap()
    }
}
