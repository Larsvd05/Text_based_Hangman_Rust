use std::io;
use crate::common::Difficulty as Difficulty;

pub fn ask_difficulty() -> Difficulty {
    loop {
        println!("Enter E for Easy, M for Medium, H for Hard:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim().to_lowercase().as_str() {
            "e" => break Difficulty::Easy,
            "m" => break Difficulty::Medium,
            "h" => break Difficulty::Hard,
            _ => println!("Invalid choice, try again."),
        }
    }
}