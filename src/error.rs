pub type Result<T> = ::std::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {
    Network(::reqwest::Error),
    Bincode(::bincode::Error),
    Io(::std::io::Error),
    Fmt(::std::fmt::Error),
    Json(::serde_json::Error),
    ParseInt(::std::num::ParseIntError),

    DayDoesNotExist(String),
    MissingSessionToken,
    InvalidSessionToken(::reqwest::StatusCode),
    Input(&'static str),
}

impl ::std::error::Error for Error {}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        <Self as ::std::fmt::Debug>::fmt(self, f)
    }
}

impl From<::reqwest::Error> for Error {
    fn from(e: ::reqwest::Error) -> Error {
        Error::Network(e)
    }
}
impl From<::bincode::Error> for Error {
    fn from(e: ::bincode::Error) -> Error {
        Error::Bincode(e)
    }
}
impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Error {
        Error::Io(e)
    }
}
impl From<::std::fmt::Error> for Error {
    fn from(e: ::std::fmt::Error) -> Error {
        Error::Fmt(e)
    }
}
impl From<::serde_json::Error> for Error {
    fn from(e: ::serde_json::Error) -> Error {
        Error::Json(e)
    }
}
impl From<::std::num::ParseIntError> for Error {
    fn from(e: ::std::num::ParseIntError) -> Error {
        Error::ParseInt(e)
    }
}
