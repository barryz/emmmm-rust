extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


#[derive(Debug)]
struct Person {
    name: String,
    email: String,
    age: u32,
    address: String,
}

impl Person {
    fn add_age(&mut self) {
        self.age += 30;
    }

    fn older(&self, other: &Person) -> bool {
        self.age > other.age
    }
}

fn main() {
    let s_number = rand::thread_rng().gen_range(1, 101);

    let (x, y, z) = guess_game(s_number);

    println!("The result value {}", x+y+z);
    
    println!("helloworld");

    let mut p = Person{
        name: String::from("barryz"), 
        email: String::from("barryzxb@gmail.com"),
        age: 28,
        address: String::from("Shanghai China"),
    };

    println!("==========================");

    println!("person is {:#?}", p);

    // modify p, so p must be mutable.
    p.add_age();

    println!("after person's adding, the  age is {}", p.age);

    let ap = Person{
        name: String::from("barryz1"), 
        email: String::from("barryzxb1@gmail.com"),
        age: 20,
        address: String::from("Shanghai China"),
    };

    if p.older(&ap) {
        println!("{} is older than {}", p.name, ap.name);
    } else {
        println!("{} is younger than {}", p.name, ap.name);
    }

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
