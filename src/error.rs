use std::{
    error,
    fmt::{self, Display, Formatter},
    io, result,
};

macro_rules! impl_from_error {
    ($error_type:ty, $error_kind:expr, $error_msg:expr) => {
        impl From<$error_type> for Error {
            fn from(err: $error_type) -> Self {
                Error::new($error_kind(err), $error_msg)
            }
        }
    };
}

macro_rules! err {
    ($kind:expr, $($arg:tt)*) => {{
		use crate::error::Error;

        let kind = $kind;
        let message = format!($($arg)*);
        return Err(Error::new( kind, message ));
    }};
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub fn new<M: Into<String>>(kind: ErrorKind, message: M) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?}", self.message, self.kind)
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Io(io::Error),
}

impl_from_error!(io::Error, |err| ErrorKind::Io(err), "IO exception");

impl error::Error for Error {}

pub(crate) use err;

pub type Result<T> = result::Result<T, Error>;
