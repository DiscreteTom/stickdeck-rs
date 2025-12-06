use crate::error::Error;

/// Return [`Err`] if the handle's value is 0.
pub fn check_handle(handle: u64) -> Result<u64, Error> {
  if handle == 0 {
    Err(Error::InvalidHandle)
  } else {
    Ok(handle)
  }
}
