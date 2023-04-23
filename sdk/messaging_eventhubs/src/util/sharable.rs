use std::sync::Arc;

use tokio::sync::Mutex;

#[derive(Debug)]
pub(crate) enum Sharable<C> {
    Owned(C),
    Shared(Arc<Mutex<C>>),
    None,
}

impl<C> Sharable<C> {
    pub(crate) fn clone_as_shared(&mut self) -> Option<Arc<Mutex<C>>> {
        match self {
            Self::Owned(_) => {
                let owned = std::mem::replace(self, Self::None);
                if let Self::Owned(owned) = owned {
                    let shared = Arc::new(Mutex::new(owned));
                    *self = Self::Shared(shared.clone());
                    Some(shared)
                } else {
                    // This should never happen
                    unreachable!()
                }
            }
            Self::Shared(shared) => Some(shared.clone()),
            Self::None => None,
        }
    }
}
