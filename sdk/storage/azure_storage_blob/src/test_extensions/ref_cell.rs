use std::{
    cell::{Ref, RefCell, RefMut},
    time::Duration,
};

pub(crate) trait RefCellTestExt<T> {
    async fn wait_borrow<'a>(&'a self) -> Ref<'a, T>
    where
        T: 'a;
    async fn wait_borrow_mut<'a>(&'a self) -> RefMut<'a, T>
    where
        T: 'a;
}

impl<T> RefCellTestExt<T> for RefCell<T> {
    async fn wait_borrow<'a>(&'a self) -> Ref<'a, T>
    where
        T: 'a,
    {
        loop {
            match self.try_borrow() {
                Ok(ref_mut) => return ref_mut,
                Err(_) => tokio::time::sleep(Duration::from_millis(1)).await,
            }
        }
    }
    async fn wait_borrow_mut<'a>(&'a self) -> RefMut<'a, T>
    where
        T: 'a,
    {
        loop {
            match self.try_borrow_mut() {
                Ok(ref_mut) => return ref_mut,
                Err(_) => tokio::time::sleep(Duration::from_millis(1)).await,
            }
        }
    }
}
