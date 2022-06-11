mod messenger;
pub use messenger::{Messenger, MessengerAsync};

mod status;
pub use status::Status;

use std::future::Future;

// trait MyTrait: Future {
//     type MyTraitOutput;

//     fn duplicate<F>(self) -> F
//     where
//         F: Future<Output = Self::MyTraitOutput>;
// }

// impl<Fut> MyTrait for Fut
// where
//     Fut: Future<Output = u8>,
// {
//     type MyTraitOutput = (u8, u8);

//     fn duplicate<F>(self) -> F
//     where
//         F: Future<Output = Self::MyTraitOutput>,
//     {
//         async {
//             let n = self.await;
//             (n, n)
//         }
//     }
// }

fn duplicate<F>(fut: F) -> impl Future<Output = (F::Output, F::Output)>
where
    F: Future<Output = u8>,
{
    async {
        let n = fut.await;
        (n, n)
    }
}

pub fn make_task<S, M, F, T, E>(
    name: S,
    messenger: M,
    future: F,
) -> impl Future<Output = Result<T, E>>
where
    S: AsRef<str>,
    M: MessengerAsync,
    F: Future<Output = (Result<T, E>, Option<String>)>,
{
    async move {
        messenger
            .send_async(name.as_ref(), Status::Running, None)
            .await;

        let (result, msg) = future.await;

        let status = Status::from(&result);

        messenger.send_async(name.as_ref(), status, msg).await;

        result
    }
}
