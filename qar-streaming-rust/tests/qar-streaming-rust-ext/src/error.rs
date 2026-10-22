use std::fmt;
use std::path::PathBuf;

use qaros_sys::QarStatusCode;

pub type Result<Value> = std::result::Result<Value, Error>;

#[derive(Debug)]
pub enum Error {
    /// The extension (or the base library it binds) could not be opened, or
    /// lacks symbols.
    Load { path: PathBuf, message: String },
    /// An extension call returned an error result; the message is read with
    /// the base library's `qar_result_message`.
    Api {
        function: &'static str,
        code: QarStatusCode,
        message: String,
    },
    /// A call returned success without the handle it promised.
    MissingHandle(&'static str),
    /// The argument cannot be passed (index out of range, interior NUL).
    InvalidArgument(String),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Load { path, message } => {
                write!(formatter, "loading {}: {message}", path.display())
            }
            Self::Api {
                function,
                code,
                message,
            } => write!(
                formatter,
                "{function} failed with status {}: {message}",
                code.0
            ),
            Self::MissingHandle(function) => {
                write!(formatter, "{function} returned success without a handle")
            }
            Self::InvalidArgument(message) => write!(formatter, "invalid argument: {message}"),
        }
    }
}

impl std::error::Error for Error {}
