//! Error and Result module.

use hyper::{header::InvalidHeaderValue, StatusCode};
use regex::Regex;
use std::{
    error::Error as StdError,
    fmt::{self, Display},
    result::Result,
    str::Utf8Error,
};

/// Wrapper Trait over [`std::error::Error`]
pub trait ErrorTrait: std::error::Error {}

/// A simple type alias so as to DRY.
pub type ConnectResult<T> = Result<T, Error>;

/// Boxed error type
pub type Cause = Box<dyn StdError + Send + Sync>;

/// Error type
pub struct Error {
    inner: Box<ErrorImpl>,
}

struct ErrorImpl {
    kind: Kind,
    cause: Option<Cause>,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_tuple("hyper::Error");
        f.field(&self.inner.kind);
        if let Some(ref cause) = self.inner.cause {
            f.field(cause);
        }
        f.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref cause) = self.inner.cause {
            write!(f, "{}: {}", self.description(), cause)
        } else {
            f.write_str(&self.description())
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner
            .cause
            .as_ref()
            .map(|cause| &**cause as &(dyn StdError + 'static))
    }
}

impl Error {
    pub(super) fn new(kind: Kind) -> Error {
        Error {
            inner: Box::new(ErrorImpl { kind, cause: None }),
        }
    }

    pub(super) fn with<C: Into<Cause>>(mut self, cause: C) -> Error {
        self.inner.cause = Some(cause.into());
        self
    }

    #[allow(dead_code)]

    pub(crate) fn find_source<E: StdError + 'static>(&self) -> Option<&E> {
        let mut cause = self.source();
        while let Some(err) = cause {
            if let Some(typed) = err.downcast_ref() {
                return Some(typed);
            }
            cause = err.source();
        }

        None
    }

    pub(super) fn new_network_error<E: Into<Cause>>(cause: E) -> Self {
        Error::new(Kind::NetworkError).with(cause)
    }

    pub(super) fn new_parsing_error<E: Into<Cause>>(cause: E) -> Self {
        Error::new(Kind::ParsingError).with(cause)
    }

    #[allow(dead_code)]
    pub(super) fn new_retry_error<E: Into<Cause>>(cause: E) -> Self {
        Error::new(Kind::RetryError).with(cause)
    }

    pub(super) fn new_connect_error(err: ConnectAPIError) -> Self {
        Error::new(Kind::ConnectAPIError(err))
    }

    pub(super) fn new_internal_error() -> Self {
        Error::new(Kind::InternalError)
    }

    /// The error's standalone message, without the message from the source.
    pub fn message(&self) -> impl fmt::Display + '_ {
        self.description()
    }

    fn description(&self) -> String {
        match &self.inner.kind {
            Kind::HyperError(_) => "this is a Hyper related error!".to_string(),
            Kind::HyperHttpError(_) => "this is a Hyper HTTP related error!".to_string(),
            Kind::InternalError => "internal error".to_string(),
            Kind::InvalidHeaderValue => "invalid header value".to_string(),
            Kind::NetworkError => "network error".to_string(),
            Kind::NotImplementedError => "not implemented error".to_string(),
            Kind::ParsingError => "parsing error".to_string(),
            Kind::RetryError => "retry error".to_string(),
            Kind::RequestNotSuccessful(err) => {
                format!("client returned an unsuccessful HTTP status code: {}", err)
            }
            Kind::SerdeJsonError(_) => "serde deserialization error".to_string(),
            Kind::Utf8Error => "parsing bytes experienced a UTF8 error".to_string(),
            Kind::CustomError(err) => {
                format!("Error: {}", err)
            }
            Kind::ConnectAPIError(err) => {
                format!("Connect API error: {}", err)
            }
        }
    }
}

/// Wrapper type which contains a failed request's status code and body.
#[derive(Debug)]
pub struct RequestNotSuccessful {
    /// Status code returned by the HTTP call.
    pub status: StatusCode,
    /// Body returned by the HTTP call.
    pub body: String,
}

impl RequestNotSuccessful {
    /// Create a new unsuccessful request error.
    pub fn new(status: StatusCode, body: String) -> Self {
        Self { status, body }
    }
}

impl StdError for RequestNotSuccessful {}

impl Display for RequestNotSuccessful {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StatusCode: {}, Body: {}", self.status, self.body)
    }
}

/// Wrapper type which contains Vault errors.
#[derive(Debug)]
pub struct ConnectAPIError {
    /// Error message from the API.
    pub message: String,
    /// Status code returned by the HTTP call.
    pub status: String,
}

impl ConnectAPIError {
    /// Create a new unsuccessful request error.
    pub fn new(status: String, message: &str) -> Self {
        Self {
            status,
            message: message.to_string(),
        }
    }
}

impl StdError for ConnectAPIError {}

impl Display for ConnectAPIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StatusCode: {}, Message: {}", self.status, self.message)
    }
}

/// Wrapper type for custom errors.
#[derive(Debug)]
pub struct CustomError {
    /// Error message.
    pub message: String,
}

impl CustomError {
    /// Create a new custom error.
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl StdError for CustomError {}

impl Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Message: {}", self.message)
    }
}

impl From<hyper::http::Error> for CustomError {
    fn from(err: hyper::http::Error) -> Self {
        Self::new(err.to_string().as_str())
    }
}

impl From<InvalidHeaderValue> for CustomError {
    fn from(_: InvalidHeaderValue) -> Self {
        Self::new("InvalidHeaderValue")
    }
}

impl From<Error> for CustomError {
    fn from(err: Error) -> Self {
        Self::new(format!("Internal error: {}", err).as_str())
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub(super) enum Kind {
    CustomError(CustomError),

    /// The failure was due to a Hyper error
    HyperError(hyper::Error),

    /// The failure was due to a Hyper error
    HyperHttpError(hyper::http::Error),

    InternalError,

    InvalidHeaderValue,

    /// The failure was due to the network client not working properly.
    NetworkError,

    NotImplementedError,

    ParsingError,

    RetryError,

    RequestNotSuccessful(RequestNotSuccessful),

    SerdeJsonError(serde_json::Error),

    Utf8Error,

    ConnectAPIError(ConnectAPIError),
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Self::HyperError(_) => {
                write!(f, "HyperError")
            }
            &Self::HyperHttpError(_) => {
                write!(f, "HyperHttpError")
            }
            Self::InternalError => {
                write!(f, "InternalError")
            }
            Self::InvalidHeaderValue => {
                write!(f, "InvalidHeaderValue")
            }
            Self::NetworkError => {
                write!(f, "NetworkError")
            }
            Self::NotImplementedError => {
                write!(f, "NotImplementedError")
            }
            Self::ParsingError => {
                write!(f, "ParsingError")
            }
            Self::RetryError => {
                write!(f, "RetryError")
            }
            &Self::RequestNotSuccessful(_) => {
                write!(f, "RequestNotSuccessful")
            }
            &Self::SerdeJsonError(_) => {
                write!(f, "SerdeJsonError")
            }
            Self::Utf8Error => {
                write!(f, "Utf8Error")
            }
            &Self::ConnectAPIError(_) => {
                write!(f, "ConnectAPIError")
            }
            &Self::CustomError(_) => {
                write!(f, "CustomError")
            }
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::new(Kind::HyperError(err))
    }
}

impl From<hyper::http::Error> for Error {
    fn from(err: hyper::http::Error) -> Self {
        Error::new(Kind::HyperHttpError(err))
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(_err: InvalidHeaderValue) -> Self {
        Error::new(Kind::InvalidHeaderValue)
    }
}

impl From<RequestNotSuccessful> for Error {
    fn from(err: RequestNotSuccessful) -> Self {
        Error::new(Kind::RequestNotSuccessful(err))
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::new(Kind::Utf8Error).with(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::new(Kind::SerdeJsonError(err))
    }
}

impl From<CustomError> for Error {
    fn from(err: CustomError) -> Self {
        Error::new(Kind::CustomError(err))
    }
}

/// Defines an error from the 1Password Connect API
#[allow(dead_code)]
#[derive(Debug)]
pub struct OPError {
    pub(super) status_code: Option<u16>,
    pub(super) captures: Option<Vec<String>>,
}

/// Handle Connect API error response
pub fn process_connect_error_response(err_message: String) -> Result<OPError, Error> {
    let input_re = Regex::new(r#"(StatusCode):\s+(\d+)"#).unwrap();

    // Execute the Regex
    let captures = input_re.captures(&err_message).map(|captures| {
        captures
            .iter() // All the captured groups
            .skip(1) // Skipping the complete match
            .flatten() // Ignoring all empty optional matches
            .map(|c| c.as_str()) // Grab the original strings
            .collect::<Vec<_>>() // Create a vector
    });

    dbg!(&captures);

    // Match against the captured values as a slice
    let status_code: Option<u16> = match captures.as_deref() {
        Some(["StatusCode", x]) => {
            let x = x.parse().expect("can't parse number");
            Some(x)
        }
        _ => None,
    };

    let return_captures: Option<Vec<String>> =
        captures.map(|b| b.into_iter().map(|c| c.to_owned()).collect::<Vec<_>>());

    Ok(OPError {
        status_code,
        captures: return_captures,
    })
}
