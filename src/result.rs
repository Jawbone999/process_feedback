pub type ResultFeedback<T, E> = Result<(T, Option<String>), (E, Option<String>)>;

pub trait WithMessage<T, E> {
    fn with_msg(self, message: impl Into<String>) -> ResultFeedback<T, E>;

    fn no_msg(self) -> ResultFeedback<T, E>;

    fn with_err_msg(self, message: impl Into<String>) -> Result<T, (E, Option<String>)>;

    fn no_err_msg(self) -> Result<T, (E, Option<String>)>;
}

impl<T, E> WithMessage<T, E> for Result<T, E> {
    fn with_msg(self, message: impl Into<String>) -> ResultFeedback<T, E> {
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

    fn with_err_msg(self, message: impl Into<String>) -> Result<T, (E, Option<String>)> {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err((error, Some(message.into()))),
        }
    }

    fn no_err_msg(self) -> Result<T, (E, Option<String>)> {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err((error, None)),
        }
    }
}
