pub fn get_result<T, E>(x) -> Result<T, E>
  where
    T: fnOnce(),
    E: fnOnce()
  {
  }
