// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock},
};

static SOURCE_CACHE: OnceLock<Mutex<HashMap<PathBuf, Option<Arc<str>>>>> = OnceLock::new();

pub(crate) fn get(path: &Path) -> Option<Arc<str>> {
    let cache = SOURCE_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut cache = cache.lock().expect("source cache mutex poisoned");

    if let Some(source) = cache.get(path) {
        return source.clone();
    }

    let source = fs::read_to_string(path).ok().map(Arc::<str>::from);
    cache.insert(path.to_path_buf(), source.clone());
    source
}
