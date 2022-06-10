use crate::Status;
use async_trait::async_trait;

pub trait Messenger {
    fn send(&self, task_name: &str, status: Status, message: Option<String>);
}

#[async_trait]
pub trait MessengerAsync {
    async fn send_async(&self, task_name: &str, status: Status, message: Option<String>);
}
