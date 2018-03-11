

pub fn vectors() {
  println!("--- vectors");

  let mut v1:Vec<i32> = Vec::new();
  v1.push(1);
  v1.push(2);
  println!("v1 = {:?}", v1);

  let arr:[isize;3] = [1,2,3];
  let idx:usize = 0;
  println!("arr[idx] = {}", arr[idx]);
  println!("v1[idx] = {}", v1[idx]);
  
  if let Some(o) = v1.get(0) { println!("got {}", o)};

  for x in &v1 { // borrow RO, instead of move
    println!("obj = {}", x);
  }

  while let Some(o) = v1.pop() {
    println!("value = {}", o);
  }


}