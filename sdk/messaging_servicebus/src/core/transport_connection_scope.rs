use std::time::Duration;

use async_trait::async_trait;

#[async_trait]
pub(crate) trait TransportConnectionScope {
    type Error;

    /// Indicates whether this <see cref="TransportConnectionScope"/> has been disposed.
    ///
    /// # Returns
    ///
    /// `true` if disposed; otherwise, `false`
    fn is_disposed(&self) -> bool;

    fn set_is_disposed(&mut self, value: bool);

    /// The recommended timeout to associate with the session.
    fn session_timeout(&self) -> Duration {
        Duration::from_secs(30)
    }

    /// Disposes of the connection scope.
    async fn dispose(&mut self) -> Result<(), Self::Error>;
}
