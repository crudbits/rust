
fn main() {

  let v = vec![1,2,3];
  let mut v2 = v;

  v2.push(4);
  
  println!("v2 = {:?}", v2);
  v2.truncate(2);
  println!("v2 = {:?}", v2);
}
