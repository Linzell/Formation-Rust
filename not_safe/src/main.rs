fn main() {
  let mut nombre = 5;

  unsafe fn dangereux() {
    println!("Fonction à risques");
  }

  let r1 = &nombre as *const i32;
  let r2 = &mut nombre as *mut i32;

  unsafe {
    println!("{}", *r1);
    println!("{}", *r2);
    dangereux();
  }
}
