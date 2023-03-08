// use std::env::args;
use std::io;

enum Choice {
    Add,
    Copy,
}

//x == filename
//Need more error handeling like option/result or unwrap_or()
fn main() {
    println!("What you want do for the file? Use numbers");
    println!("1)Add text\n2)Backup\n");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Could not read line");

    let choice_int: u8 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => 2,
    };

    let optio: Choice = match choice_int {
        1 => Choice::Add,
        2 => Choice::Copy,
        _ => Choice::Copy,
    };

    match optio {
        Choice::Add => add_to_file(),
        Choice::Copy => backup_file(),
    };
}

fn add_to_file() {
    println!("Add");
}

fn backup_file() {
    println!("Backup file");
}
