
enum OptionalTuple {
    Value(i32, i32, i32),
    Missing,
}

fn main() {
  {
    let c = 'c';
    match c {
     x => println!("x: {} c: {}", x, c), // x inside match shadows outside x
    }
  }
  let tpl = OptionalTuple::Value(5, -2, 3);
  match tpl {
    OptionalTuple::Value(x1, _, x3) => println!("Got a tuple with x1 = {} and x3 = {}!", x1, x3), // ignore 1 value
    OptionalTuple::Missing => println!("No such luck."),
  }
  match tpl {
    OptionalTuple::Value(..) => println!("Got a tuple!"), // ignore all values
    OptionalTuple::Missing => println!("No such luck."),
  }

  let c = match tpl {
    OptionalTuple::Value(a,b,c) => Option::Some(b),
    _ => Option::None
  };
  
  {
    // destructuring
    struct Point<'a> {
      x: &'a str,
      y: i32,
    }
    let mut point = Point { x: "hi", y: 3 };
    // point.x = "hello";
    // match point {
    //   Point { y, .. } => println!("y is {}", y), // ignore field x
    // }
    
    let mut s = point.x;
    println!("s = {}", s);
    // *s = "hello";
    // *s = "that";

    let (bla, Point{y, x}) = (12, point);
    println!("x = {}, p = {}", x, point.x);

    println!("bla = {}, x = {}", bla, x);
  }

  let x = 5;
  match x {
      1 => println!("one"),
      2 => println!("two"),
      3 => println!("three"),
      4 => println!("four"),
      5 => println!("five"),
      _ => println!("something else"),
  }



}
