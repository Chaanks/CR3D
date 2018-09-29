
// An enum containing Error types
#[derive(Debug)]
pub enum Error {
    UnknownError(String),
}

pub type Issue<T> = Result<T, Error>;

impl From<String> for Error {
    fn from(s: String) -> Error {
        Error::UnknownError(s)
    }
}