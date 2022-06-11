use task_feedback::{async_task, task, AsyncMessenger, Messenger};

struct Printer(String);

impl Messenger for Printer {
    fn send(&self, task_name: &str, status: task_feedback::Status, message: Option<String>) {
        let name = &self.0;
        println!("{name}: {task_name} has status {status:?} with message: {message:?}")
    }
}

fn add(a: u8, b: u8) -> u8 {
    a + b
}

#[test]
fn simple() {
    let printer = Printer("SimplePrinter".into());
    let computation = || {
        let sum = add(3, 4);
        (Ok::<u8, ()>(sum), Some("sum = 7".into()))
    };

    let result = task("AddTask", &printer, computation);

    assert_eq!(result, Ok(7));
}
