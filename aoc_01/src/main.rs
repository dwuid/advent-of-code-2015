
use std::io::Read;
use std::fs::File;

fn get_final_level(challenge: &str) -> i32 {
    let mut level = 0;
    for character in challenge.chars() {
        match character {
            '(' => level += 1,
            ')' => level -= 1,
            _   => panic!("Invalid challenge character")
        }
    }

    level
}

fn arriving_at_basement(challenge: &str) -> Option<usize> {
    let mut level = 0;
    for (index, character) in challenge.chars().enumerate() {
        match character {
            '(' => level += 1,
            ')' => level -= 1,
            _   => panic!("Invalid challenge character")
        }

        if level < 0 {
            return Some(index + 1)
        }
    }

    None
}

fn main() {
    let mut file = File::open("../input.txt")
        .expect("Cannot open challenge file");

    let mut challenge = String::new();
    file.read_to_string(&mut challenge).expect("Cannot read challenge");

    println!("Finally arriving at level {}.", get_final_level(&challenge));

    match arriving_at_basement(&challenge) {
        Some(index) => println!("Entering basement at step {}.", index),
        None        => println!("Avoiding basement.")
    }
}

