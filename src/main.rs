use rand;
use std::io::*;

fn main() {
    let mut attempts = 3;
    let words_list = vec!["rust", "cargo", "rustacean", "code"];
    let guess_word = words_list[rand::random_range(0..words_list.len())];
    let mut word_progress = make_blanks(guess_word);
    while attempts > 0 {
        let mut input = String::new();
        display_word(&word_progress);
        stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");
        let guess = match input.trim().chars().next() {
            Some(c) => c,
            None => {
                println!("Enter a Letter!");
                continue;
            }
        };
        if !update_blanks(guess_word, &mut word_progress, guess) {
            attempts -= 1;
            println!("You have {attempts} left");
        }
        if !word_progress.contains(&'_') {
            println!("You win! The word was {guess_word}");
            return;
        }
    }
    println!("Too Bad! The Word Was {guess_word}");
}

fn make_blanks(secret_word: &str) -> Vec<char> {
    secret_word.chars().map(|_| '_').collect()
}

fn display_word(blanked_word: &Vec<char>) {
    for ch in blanked_word {
        print!("{ch}");
    }
    println!();
}

fn update_blanks(secret_word: &str, blanked_word: &mut Vec<char>, guess: char) -> bool {
    let mut found = false;
    for (i, ch) in secret_word.chars().enumerate() {
        if ch == guess {
            blanked_word[i] = ch;
            found = true;
        }
    }
    found
}
