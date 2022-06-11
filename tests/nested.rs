use async_trait::async_trait;
use task_feedback::*;

struct Printer(&'static str);

impl Messenger for Printer {
    fn send(&self, task_name: &str, status: Status, message: Option<String>) {
        let name = self.0;
        println!("{name}: {task_name} {status:?} {message:?}");
    }
}

#[async_trait]
impl MessengerAsync for Printer {
    async fn send_async(&self, task_name: &str, status: Status, message: Option<String>) {
        let name = self.0;
        println!("{name}: {task_name} {status:?} {message:?}");
    }
}

#[test]
fn nested() {
    let printer = Printer("Sync");
    process!("Outer", printer, {
        println!("Outer Start");
        for i in 1..=3 {
            process!(&format!("Inner {i}"), Printer("Sync Inner"), {
                Ok(Some(format!("{i}^2 == {}", i * i)))
            });
        }
        println!("Outer End");

        Ok(None)
    });
}

#[tokio::test]
async fn nested_async() {
    process_async!("Outer Async", Printer("Async"), async {
        println!("Outer Async Start");
        for i in 1..=3 {
            process_async!(&format!("Inner Async {i}"), Printer("Async Inner"), async {
                Ok(Some(format!("{i}^2 == {}", i * i)))
            });
        }
        println!("Outer Async End");

        Ok(None)
    });
}
