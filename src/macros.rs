#[macro_export]
macro_rules! process {
    ($task_name:expr, $messenger:expr, $task:expr) => {{
        $messenger.send(&$task_name, crate::Status::Running, None);

        let result = $task;

        match result {
            Ok(msg) => $messenger.send(&$task_name, crate::Status::Finished, msg),
            Err(msg) => $messenger.send(&$task_name, crate::Status::Failed, msg),
        }
    }};
}

#[macro_export]
macro_rules! process_async {
    ($task_name:expr, $messenger:expr, $task:expr) => {{
        $messenger
            .send_async(&$task_name, crate::Status::Running, None)
            .await;

        let result = $task.await;

        match result {
            Ok(msg) => {
                $messenger
                    .send_async(&$task_name, crate::Status::Finished, msg)
                    .await
            }
            Err(msg) => {
                $messenger
                    .send_async(&$task_name, crate::Status::Failed, msg)
                    .await
            }
        }
    }};
}
