pub struct ResultFeedback<T, E>(pub Result<T, E>, pub Option<String>);

pub trait WithMessage<T, E> {
    fn with_msg<S: Into<String>>(self, message: S) -> ResultFeedback<T, E>;

    fn no_msg(self) -> ResultFeedback<T, E>;
}

impl<T, E> WithMessage<T, E> for Result<T, E> {
    fn with_msg<S: Into<String>>(self, message: S) -> ResultFeedback<T, E> {
        ResultFeedback(self, Some(message.into()))
    }

    fn no_msg(self) -> ResultFeedback<T, E> {
        ResultFeedback(self, None)
    }
}
