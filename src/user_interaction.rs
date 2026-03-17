use std::io;
use crate::common::Difficulty as Difficulty;

use rpassword::read_password;

pub fn ask_difficulty() -> Difficulty {
    loop {
        println!("Enter E for Easy, M for Medium, H for Hard:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "e" => {
                break Difficulty::Easy;
            }
            "m" => {
                break Difficulty::Medium;
            }
            "h" => {
                break Difficulty::Hard;
            }
            _ => println!("Invalid choice, try again."),
        }
    }
}

pub fn ask_secret_word() -> String {
    loop {
        println!("Enter the secret word (this will be hidden):");
        let input = read_password().expect("Failed to read the secret word");
        let input = input.trim();
        if !input.is_empty() {
            break input.to_lowercase();
        }
        println!("Invalid input, try again.");
    }
}

pub fn ask_letter() -> char {
    loop {
        println!("Enter a letter: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input.is_empty() {
            println!("Invalid input, the input can't be empty.");
            continue;
        }

        if input.len() == 1 {
            if let Some(letter) = input.chars().next() {
                if letter.is_alphabetic() {
                    break letter.to_ascii_lowercase();
                }
                println!("Invalid input, the input must be a letter ranging from A-Z or a-z.");
                continue;
            }
        }
        println!("Invalid input, please enter only a single letter.");
    }
}
