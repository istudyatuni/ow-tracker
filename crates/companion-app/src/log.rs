use tracing::error;

pub trait LogError {
    #[expect(unused)]
    fn log(self) -> Self;
    fn log_msg(self, msg: &str) -> Self;
}

impl<T, E> LogError for Result<T, E>
where
    E: std::error::Error,
{
    fn log(self) -> Self {
        if let Err(ref e) = self {
            error!("{e}");
        }
        self
    }
    fn log_msg(self, msg: &str) -> Self {
        if let Err(ref e) = self {
            error!("{msg}: {e}");
        }
        self
    }
}

impl<T> LogError for Option<T> {
    fn log(self) -> Self {
        if self.is_none() {
            error!("option is empty");
        }
        self
    }
    fn log_msg(self, msg: &str) -> Self {
        if self.is_none() {
            error!("{msg}");
        }
        self
    }
}
