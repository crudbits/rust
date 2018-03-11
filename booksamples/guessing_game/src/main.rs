
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate rand;

use std::io;
use std::io::Stdin;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  println!("Guess the number!");

  let (a,b,c) = (1,2,"3");

  
  let secret_number = rand::thread_rng().gen_range(1, 101);
  println!("The secret number is {}", secret_number);
  
  loop {
    println!("Please input your guess.");
    let mut guess:String = String::new();
    
    let std:Stdin = io::stdin();

    std.read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
      Result::Ok(n) => {
        println!("matched >{}>", n);
        n
      },
      Err(_) => continue
    };
    // let guess:u32 = if let Ok(n) = guess.trim().parse() {n} else {continue;};

    println!("You guessed: {}", guess);

    guess.cmp(&secret_number);
    //   _ => println!("something");
    // }

    match guess.cmp(&secret_number) {
      Ordering::Less    => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal   => {
        println!("You win!"); 
        break;
      }
    }
  }



}