// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use url::Url;

/// Appends new path segments to the target [`Url`].
pub trait AppendPathSegments {
    fn append_path_segments<'a>(&mut self, segments: impl IntoIterator<Item = &'a str>);
}

impl AppendPathSegments for Url {
    fn append_path_segments<'a>(&mut self, segments: impl IntoIterator<Item = &'a str>) {
        let mut path_segments = self
            .path_segments_mut()
            .expect("the URL must not be a 'cannot-be-a-base' URL");
        for segment in segments {
            path_segments.push(segment.as_ref());
        }
    }
}
