mod common;
mod user_interaction;


fn main() {
    println!("Welcome to Hangman!");
    println!("Choose difficulty: Easy (10 guesses), Medium (8 guesses), Hard (6 guesses)");

    let difficulty = user_interaction::ask_difficulty();
    println!("You chose difficulty: {:?}", difficulty);

    loop {
        // Letter asking functionality should be added here.
    }
}
