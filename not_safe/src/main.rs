use std::slice;

static HELLO_WORLD: &str = "Hello, world!";

static mut COMPTEUR: u32 = 0;

fn ajouter_au_compteur(valeur: u32) {
    unsafe {
        COMPTEUR += valeur;
    }
}



fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  let len = slice.len();
  let ptr = slice.as_mut_ptr();

  assert!(mid <= len);

  unsafe {
    (
      slice::from_raw_parts_mut(ptr, mid),
      slice::from_raw_parts_mut(ptr.add(mid), len - mid),
    )
  }
}

extern "C" {
  fn abs(input: i32) -> i32;
}

fn main() {
  let mut nombre = 5;

  ajouter_au_compteur(3);

  unsafe fn dangereux() {
    println!("Fonction à risques");
  }

  let r1 = &nombre as *const i32;
  let r2 = &mut nombre as *mut i32;

  println!("Cela vaut : {}", HELLO_WORLD);

  unsafe {
    println!("{}", *r1);
    println!("{}", *r2);
    dangereux();
    println!("La valeur absolue de -3 selon le langage C : {}", abs(-3));
    println!("COMPTEUR : {}", COMPTEUR);
  }

  let mut v = vec![1, 2, 3, 4, 5, 6];

  let r = &mut v[..];

  let (a, b) = r.split_at_mut(3);

  assert_eq!(a, &mut [1, 2, 3]);
  assert_eq!(b, &mut [4, 5, 6]);

  let mut vector = vec![1, 2, 3, 4, 5, 6];
  let (left, right) = split_at_mut(&mut vector, 3);

  assert_eq!(left, &mut [1, 2, 3]);
  assert_eq!(right, &mut [4, 5, 6]);
}
