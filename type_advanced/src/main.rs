use std::fmt;
use std::io::Error;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}

fn main() {
  type Kilometres = i32;

  let x: i32 = 5;
  let y: Kilometres = 5;

  println!("x + y = {}", x + y);

  type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("salut"));

    fn prend_un_long_type(f: Thunk) {
        f();
    }

    fn retourne_un_long_type() -> Thunk {
        Box::new(|| ())
    }

    prend_un_long_type(retourne_un_long_type());

    println!("{:?}", f());
}
