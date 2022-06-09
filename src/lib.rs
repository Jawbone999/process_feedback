use std::fmt::{Debug, Display};

/// Code block returns Result<T, E> and the sender thingy handles it along with the name.
///

#[derive(Debug, Clone, Copy)]
enum Status {
    Running,
    Finished,
    Failed,
}

struct Printer {
    id: String,
}

impl Printer {
    pub fn communicate(
        &self,
        name: &str,
        status: Status,
        msg: Option<&dyn Debug>,
    ) -> Result<(), ()> {
        let id = &self.id;
        println!("PRINTER #{id}: {name} ({status:?}) - {msg:?}");

        Ok(())
    }
}

macro_rules! process {
    ($name:expr, $send:expr, $code:expr) => {{
        $send(&$name, crate::Status::Running, None);
        let result = $code;

        match result {
            Ok(msg) => $send(&$name, crate::Status::Finished, Some(&msg)),
            Err(msg) => $send(&$name, crate::Status::Failed, Some(&msg)),
        }
    }};
}

#[cfg(test)]
mod tests {
    use crate::Printer;

    #[test]
    fn sync() {
        let printer = Printer {
            id: "Sync Test".into(),
        };
        let send = |name, status, msg| printer.communicate(name, status, msg);
        process!("SyncTask", send, {
            println!("Here is some task stuff...");
            let x = 1 + 4;
            println!("Woo! {x}");
            match x {
                5 => Ok(String::from("Nice!")),
                _ => Err(String::from("Nooo!")),
            }
        });
    }

    #[tokio::test]
    async fn not_sync() {
        println!("HERE ASYNC!")
    }
}
