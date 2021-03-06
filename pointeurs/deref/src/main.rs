use std::ops::Deref;

impl<T> Deref for MaBoite<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
  let x = 5;
  let y = MaBoite::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y);
}