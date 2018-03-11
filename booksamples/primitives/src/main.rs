
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::mem;

fn main() {
  let i:u8 = 255;
  assert!(255 ==  i);
  assert_eq!(255, i);
  println!("i = {}", i);

  let arr:[i32; 20] = [0; 20];
  println!("arr = {:?}", arr);

  let a = 0;
  let b = if a > 0 { 1 } else { 0 };
  assert!(b == 0);

  let mut c = 5;
  {
    let y = &mut c;
    // let y2 = &c;
    *y += 1;
    println!("y = {}", y);
  }
  assert_eq!(6, c);

  scope();
}

fn scope() {
  println!("scope()");
  let y: &i32;
  let x = 5;
  y = &x;
  // println!("y = {}", y);
}
