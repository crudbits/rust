

fn main() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(0);
    
    let v = 1;
    let v = v+1;
    take(v);

    println!("Gave {}", v);

    take_vec(&vec);
    println!("Given {}", vec[0]);
}


fn take_vec(vec: &Vec<i32>) {
    for i in vec {
        println!("elem {}", i);
    }
}

fn take(a: i32) -> i32 {
    println!("Took {}", a);
    let a = a * 2;
    println!("Doubled: {}", a);

    a
}
