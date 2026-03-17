use crate::common as common;
use common::Difficulty as Difficulty;
use crate::user_interaction;

use std::collections::HashSet;

// Define a struct to represent the Hangman game (encapsulation)
pub struct HangmanGame {
    secret_word: String,
    guessed_letters: HashSet<char>,
    wrong_guesses: u8,
    max_wrong_guesses: u8,
    difficulty: Difficulty,
}

impl HangmanGame {
    pub fn setup() -> Self {
        println!("Welcome to Hangman!");
        println!("Choose difficulty: Easy (10 guesses), Medium (8 guesses), Hard (6 guesses)");

        let difficulty = user_interaction::ask_difficulty();
        println!("You chose difficulty: {:?}", difficulty);

        let secret_word = user_interaction::ask_secret_word();
        println!("You chose secret word: {:?}", secret_word);

        HangmanGame::new(&secret_word, difficulty)
    }

    // Constructor (associated function)
    fn new(secret_word: &str, difficulty: Difficulty) -> Self {
        let max_wrong_guesses = match difficulty {
            Difficulty::Easy => 10,
            Difficulty::Medium => 8,
            Difficulty::Hard => 6,
        };
        HangmanGame {
            secret_word: secret_word.to_string(),
            guessed_letters: HashSet::new(),
            wrong_guesses: 0,
            max_wrong_guesses,
            difficulty,
        }
    }

    pub fn play(&mut self) {
        loop {
          let letter = user_interaction::ask_letter();
          println!("You guessed: {}", letter);
        }
    }
}
