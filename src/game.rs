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
    game_over: bool,
}

impl HangmanGame {
    const DISPLAY_STAGES: [&str; 11] = [
        "\n=========",
        "      |\n      |\n      |\n      |\n      |\n=========",
        "      +\n      |\n      |\n      |\n      |\n      |\n=========",
        "  +---+\n      |\n      |\n      |\n      |\n      |\n=========",
        "  +---+\n  |   |\n      |\n      |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n      |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n  |   |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|   |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|\\  |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|\\  |\n /    |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|\\  |\n / \\  |\n      |\n=========",
    ];

    pub fn setup() -> Self {
        println!("Welcome to Hangman!\n");
        println!("Choose difficulty: Easy (10 guesses), Medium (8 guesses), Hard (6 guesses)");

        let difficulty = user_interaction::ask_difficulty();
        println!("You chose difficulty: {:?}", difficulty);

        let secret_word = user_interaction::ask_secret_word();

        HangmanGame::new(&secret_word, difficulty)
    }

    fn reset(&mut self) {
        println!("Choose difficulty: Easy (10 guesses), Medium (8 guesses), Hard (6 guesses)");

        let difficulty = user_interaction::ask_difficulty();
        println!("You chose difficulty: {:?}", difficulty);

        let secret_word = user_interaction::ask_secret_word();

        self.secret_word = secret_word.to_string();
        self.guessed_letters = HashSet::new();
        self.wrong_guesses = 0;
        self.max_wrong_guesses;
        self.difficulty;
        self.game_over = false;
        self.setDifficulty(difficulty);
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
            game_over: false,
        }
    }

    pub fn play(&mut self) {
        self.display();
        while !self.game_over {
            let mut letter;
            loop {
                // First check if this letter has been called before.
                letter = user_interaction::ask_letter();
                println!("You guessed: {}", letter);
                if self.check_if_guessed_before(letter) {
                    println!("You have already guessed this letter before, try again.");
                } else {
                    break;
                }
            }
            self.check_guess_in_word(letter); // After the previous check, we can check if the letter exists in the word or not.
            if !self.game_over {
                self.display();
            }
        }
        self.display();
    }

    fn check_if_guessed_before(&self, letter: char) -> bool {
        self.guessed_letters.contains(&letter)
    }

    fn check_guess_in_word(&mut self, letter: char) {
        self.guessed_letters.insert(letter);
        if self.secret_word.contains(letter) {
            println!("The word contains the letter '{}'!", letter);
            if self.check_win() {
                println!("Congratulations! You have won! The word was '{}'.\n\n", self.secret_word);
                self.game_over = true;
            } else {
                println!("{}", self.get_masked_word());
            }
        } else {
            println!("The word does not contain the letter '{}'.", letter);
            self.wrong_guesses += 1;
            if self.check_loss() {
                println!("Game Over! The word was: '{}'.\n", self.secret_word);
                self.game_over = true;
            } else {
                println!("{}", self.get_masked_word());
            }
        }
    }

    fn get_masked_word(&self) -> String {
        let char_map_processed = self.secret_word.chars().map(|letter| {
            if self.guessed_letters.contains(&letter) {
                return letter; // letter can be revealed
            } else {
                return '_'; // letter needs to be unrevealed
            }
        }); // See the documentation for more info on how maps work to iterate and change values: https://doc.rust-lang.org/std/iter/struct.Map.html.

        // Collect and then join the values with a space to seperate it, otherwise it becomes less readable.
        let result = char_map_processed
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        result
    }

    fn display(&self) {
        if self.wrong_guesses <= self.max_wrong_guesses {
            let mut difference = ((HangmanGame::DISPLAY_STAGES.len()-1) as u8) - self.max_wrong_guesses; // Calculate the difference so this is dynamic for all difficulties.
            if difference == 1 {
                difference = difference -1; // Avoid overflow due. The -1 needs to be here, since index [0] is 0, but otherwise you jump to [2], which is index 3.
            }
            println!(
                "\n{}",
                HangmanGame::DISPLAY_STAGES[(self.wrong_guesses + difference) as usize]
            );
        }
        print!("Guessed letters: ");
        let mut letters: Vec<char> = self.guessed_letters.iter().copied().collect();
        letters.sort();
        for letter in letters {
            print!("{}", letter);
        }
        println!();
    }

    fn check_win(&self) -> bool {
        self.secret_word.chars().all(|c: char| self.guessed_letters.contains(&c))
    }

    fn check_loss(&self) -> bool {
        self.wrong_guesses >= self.max_wrong_guesses
    }

    pub fn play_again(&mut self) {
        let play_again_bool = user_interaction::ask_play_again();
        if play_again_bool {
            self.reset();
            self.play();
        } else {
            println!("Thanks for playing, Goodbye!");
            std::process::exit(0);
        }
    }

    fn setDifficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
        self.max_wrong_guesses = match difficulty {
            Difficulty::Easy => 10,
            Difficulty::Medium => 8,
            Difficulty::Hard => 6,
        };
    }
}
