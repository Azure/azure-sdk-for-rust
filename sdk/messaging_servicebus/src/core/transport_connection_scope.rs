use std::time::Duration;

pub(crate) trait TransportConnectionScope {
    /// <summary>
    ///   Indicates whether this <see cref="TransportConnectionScope"/> has been disposed.
    /// </summary>
    ///
    /// <value><c>true</c> if disposed; otherwise, <c>false</c>.</value>
    ///
    fn is_disposed(&self) -> bool;

    fn set_is_disposed(&mut self, value: bool);

    /// <summary>
    ///   The recommended timeout to associate with the session.
    /// </summary>
    fn session_timeout(&self) -> Duration {
        Duration::from_secs(30)
    }

    /// <summary>
    /// Disposes of the connection scope.
    /// </summary>
    fn dispose(&mut self);
}
