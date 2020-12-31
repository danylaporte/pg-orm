use std::fmt::{self, Debug, Display, Formatter};

pub enum Error {
    NativeTls(native_tls::Error),
    Postgres(tokio_postgres::Error),
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NativeTls(e) => Debug::fmt(e, f),
            Self::Postgres(e) => Debug::fmt(e, f),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NativeTls(e) => Display::fmt(e, f),
            Self::Postgres(e) => Display::fmt(e, f),
        }
    }
}

impl std::error::Error for Error {}

impl From<native_tls::Error> for Error {
    fn from(e: native_tls::Error) -> Self {
        Self::NativeTls(e)
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(e: tokio_postgres::Error) -> Self {
        Self::Postgres(e)
    }
}
