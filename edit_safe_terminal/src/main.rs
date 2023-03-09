use std::env;
use std::fs::File; //File::create(), File::open()
use std::io;
use std::io::Read; //Need for reading, sad it is not inside fs::File, but Im not smart to understand that why

enum Choice {
    Add,
    Copy,
}

//x == filename
//Need more error handeling like option/result or unwrap_or()
fn main() {
    //First value is target/debug/file. Need to read more how to fix this or how to go around this right
    let file: Vec<_> = env::args().collect();

    if file.len() > 0 {
        let file: &str = &file[1];
        let e_file = File::open(file).expect("No file found");

        println!("What you want do for the file? Use numbers");
        println!("1)Add text\n2)Backup\n");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Could not read line");
        //No point to first change number and then Choice/enum. Bad thing from me. Hashmap possible
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
            Choice::Add => add_to_file(&e_file),
            // Choice::Add => {
            //     let mut txt = String::new();
            //     e_file
            //         .read_to_string(&mut txt)
            //         .expect("error while writing");
            //     println!("{}", txt);
            // }
            Choice::Copy => backup_file(),
        };
    } else {
        println!("There was no .txt file given.");
        for arg in file.into_iter() {
            println!("{}", arg);
        }
    }
}

fn add_to_file(mut x: &File) {
    let mut txt = String::new();
    x.read_to_string(&mut txt).expect("Error while writing");
    println!("{}", txt);
}

fn backup_file() {
    println!("Backup file");
}
