

fn skip_prefix<'a>(line: &'a str, prefix: &str) -> &'a str {
  &line[0..4]
}

fn immuts_to_mutable() {
  let mut a:Box<i32> = Box::new(5);
  let &
  let y1 = &a;
  let y2 = &a;

  println!("a = {}", *a);

  // println!("y1 = {} y2 = {}", y1, y2);
  // a = Box::new(2);
  // println!("y1 = {} y2 = {}", y1, y2);
}

fn main() {
  immuts_to_mutable();
  if 1 == 1 {return}
  let line = "lang:en=Hello World!";
  let lang = "en";
  let y = &5;
  // let y2 = &y;
  
  let v;
  {
      let p = format!("lang:{}=", lang);  // -+ p goes into scope
      v = skip_prefix(line, p.as_str());  //  |
  }                                       // -+ p goes out of scope
  println!("v = {}", v);


  let i:i32 = 1;
  let i2:&i32 = &1;

  println!("{:?} {:?}", &i, &i2);
}

fn lf1<'a>(line: &'a i32, prefix: &str) -> &'a str { "hi" }

fn lf2() -> &'static str { "hi" }
