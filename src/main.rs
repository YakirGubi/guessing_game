//! Guesstin Game
//!
//! In this game the computer will chose a rundom number between 1 to 100 (including 1 and 100) and
//! you will need to guess what is the number. You have five try and every time you are wrong the
//! computer tell you if the real number higher or lower.
//!
//! Have fun ;)

use rand::Rng;
use std::{
    cmp::Ordering,
    error::Error,
    io::{self, Write},
    str::FromStr,
};

fn main() {
    let max_gusses = 5;
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Wellcom to guess the number");
    println!("you have {max_gusses} gusses, good luck.");

    // Game loop
    for _ in 0..max_gusses {
        print!("Please input your guess (1 <= x <= 100): ");
        io::stdout().flush().expect("Faild to print!");

        let guess: u32 = get_input().expect("Faild to get the input and convert it to i32");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                return;
            }
        }
    }

    println!("Sorry, you loss. maybe next time ;)");
    println!("P.S\n\tMy number was {secret_number}");
}

/// Get input from the user via the CLI and return it generecly.
///
/// #Returns
/// generic valu by the type that call the function. (unles it an Err)
///
/// #Errors
/// Can't read the input
/// Can't convert from String to <T>
///
/// #Examples
///```
///let input: i32 = get_input().expect("Error message");
///let input: char = get_input().expect("Error message");
///```
fn get_input<T>() -> Result<T, Box<dyn Error>>
where
    T: FromStr,
    T::Err: Error + 'static,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse()?)
}
