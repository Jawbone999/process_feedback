#[macro_use]
mod macros;

mod status;
pub use status::Status;

#[cfg(test)]
mod tests {
    use super::*;

    fn message(task_name: &str, status: Status, message: Option<String>) {
        println!("{task_name} {status:?} {message:?}");
    }

    async fn message_async(task_name: &str, status: Status, message: Option<String>) {
        println!("{task_name} {status:?} {message:?}");
    }

    #[test]
    fn nested() {
        process!("Outer", message, {
            println!("Outer Start");
            for i in 1..=3 {
                process!(&format!("Inner {i}"), message, {
                    Ok(Some(format!("{i}^2 == {}", i * i)))
                });
            }
            println!("Outer End");

            Ok(None)
        });
    }

    #[tokio::test]
    async fn nested_async() {
        process_async!("Outer Async", message_async, async {
            println!("Outer Async Start");
            for i in 1..=3 {
                process_async!(&format!("Inner Async {i}"), message_async, async {
                    Ok(Some(format!("{i}^2 == {}", i * i)))
                });
            }
            println!("Outer Async End");

            Ok(None)
        });
    }
}
