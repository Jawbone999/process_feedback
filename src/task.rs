use crate::messenger::{AsyncMessenger, Messenger};
use crate::ResultFeedback;
use crate::Status;
use std::error::Error;

#[cfg(feature = "sync")]
pub fn task<S, M, F, T, E>(
    name: S,
    message: Option<String>,
    messenger: &M,
    finishes: bool,
    show_error: bool,
    func: F,
) -> Result<T, E>
where
    S: AsRef<str>,
    M: Messenger,
    F: FnOnce() -> ResultFeedback<T, E>,
    E: Error,
{
    messenger.send(name.as_ref(), Status::Running, message);

    let (result, msg) = match func() {
        Ok((value, msg)) => (Ok(value), msg),
        Err((error, msg)) => match show_error {
            true => {
                let msg = msg.unwrap_or_default();
                let msg = format!("{msg} {error}").trim().to_owned();
                (Err(error), Some(msg))
            }
            false => (Err(error), msg),
        },
    };

    let mut status = Status::from(&result);

    if !finishes && matches!(status, Status::Finished) {
        status = Status::Running;
    }

    messenger.send(name.as_ref(), status, msg);

    result
}

#[cfg(feature = "async")]
pub async fn async_task<S, M, F, T, E>(
    name: S,
    message: Option<String>,
    messenger: &M,
    finishes: bool,
    show_error: bool,
    future: F,
) -> Result<T, E>
where
    S: AsRef<str>,
    M: AsyncMessenger,
    F: std::future::Future<Output = ResultFeedback<T, E>>,
    E: Error,
{
    messenger
        .send(name.as_ref(), Status::Running, message)
        .await;

    let (result, msg) = match future.await {
        Ok((value, msg)) => (Ok(value), msg),
        Err((error, msg)) => match show_error {
            true => {
                let msg = msg.unwrap_or_default();
                let msg = format!("{msg} {error}").trim().to_owned();
                (Err(error), Some(msg))
            }
            false => (Err(error), msg),
        },
    };

    let mut status = Status::from(&result);

    if !finishes && matches!(status, Status::Finished) {
        status = Status::Running;
    }

    messenger.send(name.as_ref(), status, msg).await;

    result
}
