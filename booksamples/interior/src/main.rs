

#![allow(unused_variables)]
#![allow(dead_code)]

use std::mem;

struct Foo<'a> {
    x: &'a i32,
}

struct Mine<'a> {
  x:&'a mut i32
}

// impl <'a> Mine<'a> {
//   fn incr(&self) {
//     let mut x1 = self.x;
//     // x1.hi();
//     // let x = &mut self.x;
//     // *self.x += 1;
//   }
// }

fn mine() {
  let xv = &mut 3;
  let mine = Mine {x: &mut 0};
  {
    let mine_mut = &mut mine;
    mine_mut.x = xv;
  }
  println!("mutated x = {}", mine.x);
}

fn main() {
  let mut x = 5;
  {
     let mut y = &mut x;
     *y += 1;
  }

  println!("x = {}", x);
  let y = &5; // this is the same as `let _y = 5; let y = &_y;`
  let f = Foo { x: y };
  println!("f.x = {}", f.x);

  let mut v = vec![1, 2, 3];

  for i in &v {
      // let () = i;
      println!("{}", i);
  }
  
  let mut x = 5;
  {
      let y = &mut x;
      *y += 1;
  }
  
}
