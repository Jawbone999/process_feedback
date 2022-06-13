use crate::messenger::*;
use crate::ResultFeedback;
use crate::Status;

#[cfg(feature = "sync")]
pub fn task<S, M, F, T, E>(name: S, messenger: &M, func: F) -> Result<T, E>
where
    S: AsRef<str>,
    M: Messenger,
    F: FnOnce() -> ResultFeedback<T, E>,
{
    messenger.send(name.as_ref(), Status::Running, None);

    let ResultFeedback(result, msg) = func();

    let status = Status::from(&result);

    messenger.send(name.as_ref(), status, msg);

    result
}

#[cfg(feature = "async")]
pub async fn async_task<S, M, F, T, E>(name: S, messenger: &M, future: F) -> Result<T, E>
where
    S: AsRef<str>,
    M: AsyncMessenger,
    F: std::future::Future<Output = ResultFeedback<T, E>>,
{
    messenger.send(name.as_ref(), Status::Running, None).await;

    let ResultFeedback(result, msg) = future.await;

    let status = Status::from(&result);

    messenger.send(name.as_ref(), status, msg).await;

    result
}
