use crate::Status;

#[cfg(feature = "sync")]
pub trait Messenger {
    fn send(&self, task_name: &str, status: Status, message: Option<String>);
}

/// The MessengerAsync trait defines the interface for sending status updates.
#[cfg(feature = "async")]
#[async_trait::async_trait]
pub trait AsyncMessenger {
    async fn send(&self, task_name: &str, status: Status, message: Option<String>);
}

impl Messenger for () {
    fn send(&self, task_name: &str, status: Status, message: Option<String>) {}
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl AsyncMessenger for () {
    async fn send(&self, task_name: &str, status: Status, message: Option<String>) {}
}
