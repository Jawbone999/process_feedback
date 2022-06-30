/// Status reflects the status of a task.
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[derive(Debug, Clone, Copy)]
pub enum Status {
    /// The task is currently running.
    Running,

    /// The task has completed successfully.
    Finished,

    /// The task has failed.
    Failed,
}

impl<T, E> From<&Result<T, E>> for Status {
    fn from(result: &Result<T, E>) -> Self {
        match result {
            Ok(_) => Self::Finished,
            Err(_) => Self::Failed,
        }
    }
}
