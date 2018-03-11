
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_parens)]

mod vectors;

struct Point {
  x: f64,
  y: f64
}

enum Color {
  Black,
  Green,
  RGB(u8, u8, u8)
}


fn main() {
  tuples();
  strings();
  enums();
  option();
  arrays();
  vectors::vectors();
}

fn tuples() {
  let mut a:(i32, char) = (1,'b');
  let i:i32 = a.0;
  a.0 = 2;
  let m = -2;
  let neg_m = !m;
  println!("asserting");
  assert!(neg_m == 1);
  let mfive = -5;
  let negfive = !-5;
  println!("negfive = {:b}", negfive);
  assert!(negfive == 4);

  let m5 = 00000100;
  let m5n = 00000010;
  
  let x = 10000010;
  let x_b = 00000001;
  let x2 = 01111101;
}

fn strings() {
  let x:&str = "hello";
  let y = x.to_string();
  
  // y[0] ERROR!
  let y0:u8 = y.as_bytes()[0];

  // expensive walk-from-beginning operation
  let y0_c:char = y.chars().nth(0)
    .unwrap_or_default();
  println!("char 0 = {}", y0_c);

  let hachiko = "dog".to_string();
  let hachiko_slice = &hachiko[1..2];
  println!("slice = {}", hachiko_slice);
}

fn enums() {
  println!("--- enums");
  
  let b = (1,2);
  println!("b.1={} b.2={}", b.0, b.1);

  let c:Color = Color::RGB(1,1,1);
  let c: Color = Color::Green;
  match c {
    Color::Black | Color::Green => println!("Color was {} or {}", "Black", "Green"),
    Color::RGB(a,b,c) => println!("a={} b={} c={}", a,b,c),
  }
  // println!("{:?}", c);s
  // println!("RGB value = {}", c);
}

fn option() {
  println!("--- option");
  // Option<T>
  let x = 3.0;
  let y = 2.0;

  let result:Option<f64> = if y != 0.0 { Some(x/y) } else { None };
  
  // if let
  let result_opt: Option<f64> = result;
  println!("{:?}", result);
  if let Some(z) = result {
    println!("Some");
  } else {
    println!("None");
  }

  if let Some(z) = (if y != 0.0 { Some(x/y) } else { None }) {
    println!("Immediate z={}", z);
  }

  // while let
  let b = Some(12);
  while let Some(z) = b {
    println!("z = {}", z);
    break;
  }
}

use std::mem;

fn arrays() {
  println!("--- arrays");
  let a:[i32; 5] = [0; 5]; // = [0,0,0,0,0];
  let b = [1i32; 10];

  let c = ["hi"; 2];
  println!("c = {:?}", c);

  let mtx1:[[i32;3]; 2] = [
    [1,2,3],
    [4,5,6]
  ];
  let mtx2 = [
    [1,1,1],
    [2,2,2]
  ];

  println!("{:?}", a);
  for (ix,i) in (0..a.len()).enumerate() {
    println!("idx {} = {}", ix, a[i]);
  }

  println!("size of a = {}", mem::size_of_val(&a));

}
