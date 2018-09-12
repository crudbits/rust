

fn main() {
  let s = "my String";

  // let my_byte = s + 1;

  let _s_bytes = s.as_bytes();
  println!("space >{}<", b' ');
  for (_i, &b) in _s_bytes.iter().enumerate() {
    println!("byte >{}<", &b);
    if b' ' == b {
      println!("match found {}={}", &b, b' ');
    }
  }
}


fn main2() {
    println!("Hello, world!");
    // recurse(0);
    let arr = [1,2,3,4,5,1,6,1,2,7,9,4,6,8];
    largest(&arr);
}


fn recurse(i: i32) {
    if i < 1000 {
        println!("Incrementing i: {}", i);
        recurse(i + 1)
    }
    else {
        println!("Done: i={}", i);
    }
}

fn largest(arr: &[i32]) {
    let mut largest: Option<i32> = None;
    for val in arr.iter() {
        // println!("index={} value={}", idx, val);
        if let Some(value) = largest {
            if *val > value {
                println!("found larger = {}", val);
                largest = Some(*val);
            }
        } else {
            largest = Some(*val);
        }
    }

    println!("Largest was = {}", largest.unwrap());
}
