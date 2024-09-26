/// Check if the expression takes too long to execute in debug mode.
macro_rules! perf {
  ($name:expr, $e:expr, $threshold_ms:expr) => {
    if log::log_enabled!(log::Level::Debug) {
      let start = std::time::Instant::now();
      let res = $e;
      let elapsed = start.elapsed();
      if elapsed > std::time::Duration::from_millis($threshold_ms) {
        log::warn!("Performance warning({:?}): {:?}", $name, elapsed);
      }
      res
    } else {
      $e
    }
  };
}
pub(crate) use perf;
