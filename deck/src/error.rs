use std::fmt;

/// Error type for the deck module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
  /// Invalid handle (handle value is 0).
  InvalidHandle,
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::InvalidHandle => write!(f, "Invalid handle: handle value is 0"),
    }
  }
}

impl std::error::Error for Error {}
