use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

impl Add for Point {
  type Output = Point;

  fn add(self, other: Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

fn mesure (premier: Point, deuxieme: Point, resultat: Point) {
  assert_eq!(
    premier + deuxieme,
    resultat
  );
}

fn main() {

  mesure(Point{x: 2, y: 0}, Point{x: 1, y: 3}, Point{x: 3, y: 3});
  mesure(Point{x: 5, y: 2}, Point{x: 0, y: 11}, Point{x: 5, y: 13});

}

