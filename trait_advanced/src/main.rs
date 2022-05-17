pub trait Iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
  let mut iter = (1..10).into_iter();

  while let Some(x) = iter.next() {
    println!("{}", x);
  }
}