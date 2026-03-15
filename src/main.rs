use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("Guess the number");

    const MAX_GUSSES: u32 = 5;
    let secret_number = rand::thread_rng().gen_range(1..=100);

    for i in 1..=MAX_GUSSES {
        let mut guess = String::new();

        print!("Please inpute your guess (1 <= x <= 100): ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut guess)
            .expect("Faild to read line!");

        let guess: u32 = guess
            .trim()
            .parse()
            .expect("Can't convert from String to u32");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        if i == MAX_GUSSES {
            println!("Sorry you loss maybe next time ;)");
            println!("P.S\n\tMy number was {secret_number}");
        }
    }
}
