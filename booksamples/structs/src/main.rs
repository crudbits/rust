


struct A {x: i32}
struct B(i32);

struct Point {
  x: i32,
  y: i32
}
struct PointRef<'a> {
  x: &'a mut i32,
  y: &'a mut i32
}

fn main() {
  let a = A{x: 1};
  let b = B(1);
  let x = b.0;

  let mut point = Point { x: 0, y: 0 };

  {
      let r = PointRef { x: &mut point.x, y: &mut point.y };
      *r.x = 5;
      *r.y = 6;
  }

  assert_eq!(5, point.x);
  assert_eq!(6, point.y);


}
