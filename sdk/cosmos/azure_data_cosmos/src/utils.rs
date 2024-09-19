// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use url::Url;

pub trait WithAddedPathSegments {
    fn with_added_path_segments<'a>(&self, segments: impl IntoIterator<Item = &'a str>) -> Self;
}

impl WithAddedPathSegments for Url {
    fn with_added_path_segments<'a>(&self, segments: impl IntoIterator<Item = &'a str>) -> Self {
        let mut url = self.clone();
        {
            let mut path_segments = url
                .path_segments_mut()
                .expect("the URL must not be a 'cannot-be-a-base' URL");
            for segment in segments.into_iter() {
                path_segments.push(segment);
            }
        }
        url
    }
}
