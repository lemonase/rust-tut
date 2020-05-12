use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // seed a random number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        // ask for user input
        println!("Please input your guess: ");
        let mut guess = String::new();

        // read user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // parse and integer or error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // match block
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }

        // print out guess
        println!("You guessd: {}", guess);
    }

}
