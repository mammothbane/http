use core::error;
use core::fmt;

use crate::header;
#[cfg(feature = "alloc")]
use crate::header::MaxSizeReached;
use crate::method;
use crate::status;
#[cfg(feature = "alloc")]
use crate::uri;

/// A generic "error" for HTTP connections
///
/// This error type is less specific than the error returned from other
/// functions in this crate, but all other errors can be converted to this
/// error. Consumers of this crate can typically consume and work with this form
/// of error for conversions with the `?` operator.
pub struct Error {
    inner: ErrorKind,
}

/// A `Result` typedef to use with the `http::Error` type
pub type Result<T> = core::result::Result<T, Error>;

enum ErrorKind {
    StatusCode(status::InvalidStatusCode),
    Method(method::InvalidMethod),
    #[cfg(feature = "alloc")]
    Uri(uri::InvalidUri),
    #[cfg(feature = "alloc")]
    UriParts(uri::InvalidUriParts),
    HeaderName(header::InvalidHeaderName),
    #[cfg(feature = "alloc")]
    HeaderValue(header::InvalidHeaderValue),
    #[cfg(feature = "alloc")]
    MaxSizeReached(MaxSizeReached),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("http::Error")
            // Skip the noise of the ErrorKind enum
            .field(&self.get_ref())
            .finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.get_ref(), f)
    }
}

impl Error {
    /// Return true if the underlying error has the same type as T.
    pub fn is<T: error::Error + 'static>(&self) -> bool {
        self.get_ref().is::<T>()
    }

    /// Return a reference to the lower level, inner error.
    pub fn get_ref(&self) -> &(dyn error::Error + 'static) {
        use self::ErrorKind::*;

        match self.inner {
            StatusCode(ref e) => e,
            Method(ref e) => e,
            #[cfg(feature = "alloc")]
            Uri(ref e) => e,
            #[cfg(feature = "alloc")]
            UriParts(ref e) => e,
            HeaderName(ref e) => e,
            #[cfg(feature = "alloc")]
            HeaderValue(ref e) => e,
            #[cfg(feature = "alloc")]
            MaxSizeReached(ref e) => e,
        }
    }
}

impl error::Error for Error {
    // Return any available cause from the inner error. Note the inner error is
    // not itself the cause.
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.get_ref().source()
    }
}

#[cfg(feature = "alloc")]
impl From<MaxSizeReached> for Error {
    fn from(err: MaxSizeReached) -> Error {
        Error {
            inner: ErrorKind::MaxSizeReached(err),
        }
    }
}

impl From<status::InvalidStatusCode> for Error {
    fn from(err: status::InvalidStatusCode) -> Error {
        Error {
            inner: ErrorKind::StatusCode(err),
        }
    }
}

impl From<method::InvalidMethod> for Error {
    fn from(err: method::InvalidMethod) -> Error {
        Error {
            inner: ErrorKind::Method(err),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<uri::InvalidUri> for Error {
    fn from(err: uri::InvalidUri) -> Error {
        Error {
            inner: ErrorKind::Uri(err),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<uri::InvalidUriParts> for Error {
    fn from(err: uri::InvalidUriParts) -> Error {
        Error {
            inner: ErrorKind::UriParts(err),
        }
    }
}

impl From<header::InvalidHeaderName> for Error {
    fn from(err: header::InvalidHeaderName) -> Error {
        Error {
            inner: ErrorKind::HeaderName(err),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<header::InvalidHeaderValue> for Error {
    fn from(err: header::InvalidHeaderValue) -> Error {
        Error {
            inner: ErrorKind::HeaderValue(err),
        }
    }
}

impl From<core::convert::Infallible> for Error {
    fn from(err: core::convert::Infallible) -> Error {
        match err {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inner_error_is_invalid_status_code() {
        if let Err(e) = status::StatusCode::from_u16(6666) {
            let err: Error = e.into();
            let ie = err.get_ref();
            #[cfg(feature = "alloc")]
            assert!(!ie.is::<header::InvalidHeaderValue>());
            assert!(ie.is::<status::InvalidStatusCode>());
            ie.downcast_ref::<status::InvalidStatusCode>().unwrap();

            #[cfg(feature = "alloc")]
            assert!(!err.is::<header::InvalidHeaderValue>());
            assert!(err.is::<status::InvalidStatusCode>());
        } else {
            panic!("Bad status allowed!");
        }
    }
}
