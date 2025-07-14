/// Enum of all possible errors.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Indicates that the connection to the duckdb database is closed.
    Closed,
    /// Error updating PRAGMA.
    PragmaUpdate {
        name: &'static str,
        exp: &'static str,
        got: String,
    },
    /// Represents a [`duckdb::Error`].
    Duckdb(duckdb::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Duckdb(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Closed => write!(f, "connection to sqlite database closed"),
            Self::PragmaUpdate { exp, got, name } => {
                write!(f, "updating pragma {name}: expected '{exp}', got '{got}'")
            }
            Self::Duckdb(err) => err.fmt(f),
        }
    }
}

impl From<duckdb::Error> for Error {
    fn from(value: duckdb::Error) -> Self {
        Self::Duckdb(value)
    }
}

impl<T> From<crossbeam_channel::SendError<T>> for Error {
    fn from(_value: crossbeam_channel::SendError<T>) -> Self {
        Self::Closed
    }
}

impl From<crossbeam_channel::RecvError> for Error {
    fn from(_value: crossbeam_channel::RecvError) -> Self {
        Self::Closed
    }
}

impl From<futures_channel::oneshot::Canceled> for Error {
    fn from(_value: futures_channel::oneshot::Canceled) -> Self {
        Self::Closed
    }
}
