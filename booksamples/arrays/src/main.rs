
fn main() {
  let xa: [i32; 10] = [1; 10];

  let xv = vec![2; 10];

  // let b = xv.iter();
  // println!("b = {:?}", b);

  for (i, x) in xv.iter().enumerate() {
    if i == 2 {
      break;
    }
    println!("x = {}", x);

    let a: i32 = 1;
    println!("a is {}", a);
  }

  for i in xv {
    println!("i = {}", i);
  }
}
