/// Return [`Err`] if the handle's value is 0.
pub fn check_handle(handle: u64) -> Result<u64, ()> {
  if handle == 0 {
    Err(())
  } else {
    Ok(handle)
  }
}
