#[macro_export]
macro_rules! process {
    ($task_name:expr, $message:expr, $task:expr) => {{
        $message(&$task_name, crate::Status::Running, None);

        let result = $task;

        match result {
            Ok(msg) => $message(&$task_name, crate::Status::Finished, msg),
            Err(msg) => $message(&$task_name, crate::Status::Failed, msg),
        }
    }};
}

#[macro_export]
macro_rules! process_async {
    ($task_name:expr, $message:expr, $task:expr) => {{
        $message(&$task_name, crate::Status::Running, None).await;

        let result = $task.await;

        match result {
            Ok(msg) => $message(&$task_name, crate::Status::Finished, msg).await,
            Err(msg) => $message(&$task_name, crate::Status::Failed, msg).await,
        }
    }};
}
