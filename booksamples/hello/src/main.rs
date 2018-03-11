
#![allow(dead_code)]
#![allow(unused_variables)]

mod sh;
use std::mem;

fn main() {
  sh::stack_and_heap();
  println!("Hello, world!");

  let mut v:Vec<i32> = Vec::new();
  v.push(1);
  println!("vec size = {}", v.len());

  let v2 = vec![1,2,3];

  {
    let v:Vec<i32> = Vec::new();
    println!("inside, vec size = {}", v.len());
    moved(v);
  }
  println!("after move len = {}", v.len());
  let a = 2;
  cube(a);

  let b:f32 = 2.0;
  let b_to_pi = pow_to_pi(b);
  println!("b_to_pi = {}", b_to_pi);
  scope_and_shadowing();

}

fn pow_to_pi(x: f32)  -> f32 {
  let result = f32::powf(x, std::f32::consts::PI);
  result
}

fn cube(x: i32) {
  let a_cubed = i32::pow(x, 3);
  println!("a_cubed = {}, size of a_cubed = {}", a_cubed, mem::size_of_val(&a_cubed));
}

fn scope_and_shadowing() {
  let a = 1;
  {
    let b = 2;
    print!("a = {}, b = {}", a, b);
  }
}

fn moved(v: Vec<i32>) {
  println!("moved length = {}", v.len());
}



