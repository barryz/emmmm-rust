extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let s_number = rand::thread_rng().gen_range(1, 101);

    let (x, y, z) = guess_game(s_number);

    println!("The result value {}", x+y+z);
    
    println!("helloworld");
}

// guess_name function of gusess_name.
fn guess_game(s_number: u32) -> (u32, u32, u32) {
    println!("The secret number is {}", s_number);

    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        // mut means mutable
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("{} occured!", err);
                continue;
            },
        };
          

        println!("Your guessed: {}", guess);

        match guess.cmp(&s_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    (s_number, s_number, s_number)
}
