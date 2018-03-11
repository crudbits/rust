
fn main() {
  let arr1 : [isize; 4] = [0,1,2,3];
  let part1 = &arr1[1..4];
  println!("array = {:?}, full_slize = {:?}", arr1, part1);
}
