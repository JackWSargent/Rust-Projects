use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("Guess a number between 1 and 100.");
    println!("{number}");

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if num < 1 || num > 100 {
                    println!("The number {} is out of range.", num);
                    continue;
                } 
                num
            }
            Err(_) => {
                println!("Failed to parse the input as a number.");
                continue;
            }
        };

        println!("You guessed {}!", guess);

        match guess.cmp(&number) {
            Ordering::Less => println!("Number is greater"),
            Ordering::Greater => println!("Number is less"),
            Ordering::Equal => {
                println!("Number is equal");
                break;
            }
        }

    }
    

}
