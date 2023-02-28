use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    //camelcase style bites a** when looks line everything uses snake styling in rust librarys.
    //Compiler forces you to use snakecases so must learn to use/changeslearn habbits.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
