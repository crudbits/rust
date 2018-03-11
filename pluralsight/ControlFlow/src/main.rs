
#![allow(dead_code)]

fn main() {
  if_else();
  while_loop();
  forin();
  match_in();
}

fn if_else() {
  println!("if_else");
  let a:i32 = 10;
  let b = if a > 10 {"hot"} else {"cold"};

  println!("b is = {}", b);

  let today:i32 = 25;
  println!("today is {}",
    if today > 9 {
        if today > 30 {"very hot"}
        else {"hot"}
    } else {
        "cold"
    }
  );
}

fn while_loop() {
  println!("while_loop");
  let mut a:i32 = 1;
  while a < 1000 {
    println!("a = {}", a);
    a *= 2;
  }

  a = 1;
  loop {
    println!("a = {}", a);
    a *= 2;

    if a > 1000 {
      break;
    }
  }
}

fn forin() {
  println!("for_in on 'iterables'");
  println!("basic for loop bounds are lower=1 and upper=11");
  for a in 1..11 {
    println!("a is {}", a);
  }

  println!("positional for loop bounds are lower=1 and upper=11");
  for (p,a) in (1..11).enumerate() {
    println!("position {} = {}", p, a);
  }
}

fn match_in() {
  for a in 1..8 {
    match a {
      1 => println!("{} is the country code for the US",a ),
      7 => println!("{} is the country code for Russia", a),
      1...999 => println!("country code {} is unknown", a),
      _ => println!("invalid input {}", a),
    };
  }
}
