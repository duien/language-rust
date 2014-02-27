use std;


mod test {
  #[inline]
  fn world() {}

  fn world<T>(a: ~T, b: &T) -> &T {
    // Something...
  }

}


struct Gc<T, 'a> {
  priv ptr: 'a T
}

impl<T, 'a> Gc<T, 'a> {

}
