use crate::messenger::*;
use crate::Status;

#[cfg(feature = "sync")]
pub fn task<S, M, F, T, E>(name: S, messenger: &M, func: F) -> Result<T, E>
where
    S: AsRef<str>,
    M: Messenger,
    F: FnOnce() -> Result<(T, Option<String>), (E, Option<String>)>,
{
    messenger.send(name.as_ref(), Status::Running, None);

    let (result, msg) = match func() {
        Ok((value, msg)) => (Ok(value), msg),
        Err((error, msg)) => (Err(error), msg),
    };

    let status = Status::from(&result);

    messenger.send(name.as_ref(), status, msg);

    result
}

#[cfg(feature = "async")]
pub async fn async_task<S, M, F, T, E>(name: S, messenger: &M, future: F) -> Result<T, E>
where
    S: AsRef<str>,
    M: AsyncMessenger,
    F: std::future::Future<Output = Result<(T, Option<String>), (E, Option<String>)>>,
{
    messenger.send(name.as_ref(), Status::Running, None).await;

    let (result, msg) = match future.await {
        Ok((value, msg)) => (Ok(value), msg),
        Err((error, msg)) => (Err(error), msg),
    };

    let status = Status::from(&result);

    messenger.send(name.as_ref(), status, msg).await;

    result
}
