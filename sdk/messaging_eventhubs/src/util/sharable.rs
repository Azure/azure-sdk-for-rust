use std::sync::Arc;

use fe2o3_amqp::session::SessionHandle;
use tokio::sync::RwLock;

#[derive(Debug)]
pub(crate) enum Sharable<C> {
    Owned(C),
    Shared(Arc<RwLock<C>>),
    None,
}

impl<C> Sharable<C> {
    pub(crate) fn clone_as_shared(&mut self) -> Option<Arc<RwLock<C>>> {
        match self {
            Self::Owned(_) => {
                let owned = std::mem::replace(self, Self::None);
                if let Self::Owned(owned) = owned {
                    let shared = Arc::new(RwLock::new(owned));
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

impl<T> Sharable<SessionHandle<T>> {
    pub(crate) async fn close(&mut self) -> Result<(), fe2o3_amqp::session::Error> {
        match self {
            Self::Owned(session) => session.close().await,
            Self::Shared(session) => {
                let mut session = session.write().await;
                session.close().await
            }
            Self::None => unreachable!(),
        }
    }

    pub(crate) async fn close_if_owned(&mut self) -> Result<(), fe2o3_amqp::session::Error> {
        match self {
            Self::Owned(session) => session.close().await,
            Self::Shared(session) => match Arc::strong_count(session) {
                1 => session.write().await.close().await,
                _ => Ok(()),
            },
            Self::None => unreachable!(),
        }
    }

    pub(crate) async fn is_ended(&self) -> bool {
        match self {
            Self::Owned(session) => session.is_ended(),
            Self::Shared(session) => session.read().await.is_ended(),
            Self::None => unreachable!(),
        }
    }
}
