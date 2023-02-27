use async_lock::RwLock;
use std::sync::Arc;

pub struct ExpiresValue<V>
where
    V: Clone + Send + Sync,
{
    pub expires_on: time::OffsetDateTime,
    pub value: V,
}

#[async_trait::async_trait]
pub(crate) trait AutoRefreshing<V>
where
    V: Clone + Send + Sync,
{
    fn get_current(&self) -> Arc<RwLock<Option<ExpiresValue<V>>>>;
    async fn get_latest(&self) -> V;
    async fn get_value(&self) -> V {
        let curent = self.get_current();
        // if the current cached features is good, return that.
        if let Some(current) = curent.read().await.as_ref() {
            if !is_expired(current) {
                return current.value.clone();
            }
        }
        let mut guard = curent.write().await;
        // check again in case another thread refreshed the features while we were
        // waiting on the write lock
        if let Some(current) = guard.as_ref() {
            if !is_expired(current) {
                return current.value.clone();
            }
        }

        let result = self.get_latest().await;

        *guard = Some(ExpiresValue {
            value: result.clone(),
            expires_on: time::OffsetDateTime::now_utc()
                + std::time::Duration::from_secs(match std::env::var("FEATURE_EXPIRE_ON") {
                    Ok(s) => match s.parse::<u64>() {
                        Ok(i) => i,
                        Err(_) => 20,
                    },
                    Err(_) => 20,
                }),
        });
        result
    }
}
fn is_expired<V>(val: &ExpiresValue<V>) -> bool
where
    V: Clone + Send + Sync,
{
    val.expires_on < time::OffsetDateTime::now_utc()
}
