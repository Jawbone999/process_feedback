pub type ResultFeedback<T, E> = Result<(T, Option<String>), (E, Option<String>)>;

pub trait WithMessage<T, E> {
    fn with_msg<S: Into<String>>(self, message: S) -> ResultFeedback<T, E>;

    fn no_msg(self) -> ResultFeedback<T, E>;
}

impl<T, E> WithMessage<T, E> for Result<T, E> {
    fn with_msg<S: Into<String>>(self, message: S) -> ResultFeedback<T, E> {
        match self {
            Ok(value) => Ok((value, Some(message.into()))),
            Err(error) => Err((error, Some(message.into()))),
        }
    }

    fn no_msg(self) -> ResultFeedback<T, E> {
        match self {
            Ok(value) => Ok((value, None)),
            Err(error) => Err((error, None)),
        }
    }
}
