use failure::Fail;
use hidapi::HidError;
use std::error;
use std::fmt::{self, Display, Formatter};
/// A possible error value when opening the sensor or taking a reading.
#[derive(Debug)]
pub enum Error {
    /// A hardware access error.
    Hid(Box<dyn error::Error + Send + Sync>),
    /// The sensor returned an invalid message or a single read timeout
    /// expired.
    InvalidMessage,
    /// A checksum error.
    Checksum,
    /// The sensor did not report all values before the timeout expired.
    ///
    /// Note that this can only occur when calling
    /// [`Sensor::read`][crate::Sensor::read].
    /// [`Sensor::read_one`][crate::Sensor::read_one] returns
    /// [`Error::InvalidMessage`] on timeout.
    Timeout,
    /// The configured timeout was too large.
    InvalidTimeout,
    /// Hint against exhaustive matching.
    ///
    /// This enum may be extended with additional variants, so users should not
    /// count on exhaustive matching.
    #[doc(hidden)]
    __Nonexhaustive,
}

impl From<HidError> for Error {
    fn from(err: HidError) -> Self {
        Error::Hid(Box::new(err.compat()))
    }
}

impl From<zg_co2::Error> for Error {
    fn from(err: zg_co2::Error) -> Self {
        match err {
            zg_co2::Error::InvalidMessage => Error::InvalidMessage,
            zg_co2::Error::Checksum => Error::Checksum,
            _ => unreachable!(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::InvalidMessage => write!(f, "invalid message"),
            Error::Checksum => write!(f, "checksum error"),
            Error::Hid(err) => err.fmt(f),
            Error::InvalidTimeout => write!(f, "invalid timeout"),
            _ => unreachable!(),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Hid(cause) => Some(cause.as_ref()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<super::Error>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<super::Error>();
    }
}
