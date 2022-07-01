use crate::messenger::{AsyncMessenger, Messenger};
use crate::ResultFeedback;
use crate::Status;

#[cfg(feature = "sync")]
pub fn task<S, M, F, T, E>(
    name: S,
    message: Option<String>,
    messenger: &M,
    finishes: bool,
    func: F,
) -> Result<T, E>
where
    S: AsRef<str>,
    M: Messenger,
    F: FnOnce() -> ResultFeedback<T, E>,
{
    messenger.send(name.as_ref(), Status::Running, message);

    let (result, msg) = match func() {
        Ok((value, msg)) => (Ok(value), msg),
        Err((error, msg)) => (Err(error), msg),
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
    future: F,
) -> Result<T, E>
where
    S: AsRef<str>,
    M: AsyncMessenger,
    F: std::future::Future<Output = ResultFeedback<T, E>>,
{
    messenger
        .send(name.as_ref(), Status::Running, message)
        .await;

    let (result, msg) = match future.await {
        Ok((value, msg)) => (Ok(value), msg),
        Err((error, msg)) => (Err(error), msg),
    };

    let mut status = Status::from(&result);

    if !finishes && matches!(status, Status::Finished) {
        status = Status::Running;
    }

    messenger.send(name.as_ref(), status, msg).await;

    result
}
