use std::io::Error;
use std::{ffi::OsStr, process::Output};

#[cfg(feature = "tokio_process")]
pub(crate) async fn run_command<S, I, A>(program: S, args: I) -> Result<Output, Error>
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

#[cfg(not(feature = "tokio_process"))]
pub(crate) async fn run_command<S, I, A>(program: S, args: I) -> Result<Output, Error>
where
    S: AsRef<OsStr>,
    I: IntoIterator<Item = A>,
    A: AsRef<OsStr>,
{
    use futures::channel::oneshot;
    use std::io::ErrorKind;

    let (tx, rx) = oneshot::channel();
    let mut cmd = std::process::Command::new(program);
    cmd.args(args);
    std::thread::spawn(move || {
        let output = cmd.output();
        tx.send(output)
    });
    let output = rx
        .await
        .map_err(|err| Error::new(ErrorKind::Other, err))??;
    Ok(output)
}
