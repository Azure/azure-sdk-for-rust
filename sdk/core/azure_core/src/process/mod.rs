#[cfg(not(feature = "tokio_process"))]
mod thread;

#[cfg(not(feature = "tokio_process"))]
pub use thread::run_command;

#[cfg(feature = "tokio_process")]
pub use self::tokio::run_command;

#[cfg(feature = "tokio_process")]
mod tokio {
    use std::io::Error;
    use std::{ffi::OsStr, process::Output};

    /// Run a command with the given arguments until it terminates, returning the output
    pub async fn run_command<S, I, A>(program: S, args: I) -> Result<Output, Error>
    where
        S: AsRef<OsStr>,
        I: IntoIterator<Item = A>,
        A: AsRef<OsStr>,
    {
        tokio::process::Command::new(program)
            .args(args)
            .output()
            .await
    }
}
