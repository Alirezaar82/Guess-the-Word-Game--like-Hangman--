use std::fs::File;
use std::io::{self, BufRead, BufReader};
use rand::seq::SliceRandom;

fn get_random_word() -> String {
    let file = File::open("words_alpha.txt").expect("Not found");
    let reader = BufReader::new(file);

    let words: Vec<String> = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|w| w.len() > 4 && w.len() <= 8)
        .collect();

    let word = words.choose(&mut rand::thread_rng()).unwrap();
    word.to_string()
}

fn check_answer(secret_word: &str, user_guess: char, guess_word: &str) -> (String, bool) {
    let mut has_change = false;
    let secret_chars: Vec<char> = secret_word.chars().collect();
    let mut masked_word: Vec<char> = guess_word.chars().collect();

    for (i, ch) in secret_chars.iter().enumerate() {
        if *ch == user_guess && masked_word[i] == '*' {
            masked_word[i] = *ch;
            has_change = true;
        }
    }

    let result: String = masked_word.iter().collect();
    (result, has_change)
}

fn main() {
    let mut guess_count = 0;
    let secret = get_random_word();
    let len_secret_word = secret.len();
    let mut remaining_guesses = len_secret_word + 1;

    let mut guess_word = "*".repeat(len_secret_word); // Init masked word

    println!("Welcome to HangmanRust game!");
    println!("Guess the secret word letter by letter.");
    println!("You have {} guesses.\n", remaining_guesses);
    println!("Word: {}", guess_word);

    while remaining_guesses > 0 {
        println!("\nEnter a letter:");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read input");
        let user_input = user_input.trim().to_lowercase();

        if user_input.len() != 1 {
            println!("Please enter a single letter.");
            continue;
        }

        let guess_char = user_input.chars().next().unwrap();

        guess_count += 1;

        let (new_guess_word, has_guess) = check_answer(&secret, guess_char, &guess_word);

        if has_guess {
            println!("✅ You found a letter!");
            guess_word = new_guess_word;
        } else {
            remaining_guesses -= 1;
            println!("❌ Wrong guess. {} guesses left.", remaining_guesses);
        }

        println!("Word: {}", guess_word);

        if guess_word == secret {
            println!("\n🎉 You won! The word was: {}", secret);
            println!("Total guesses: {}", guess_count);
            break;
        }
    }

    if guess_word != secret {
        println!("\n💀 You lost! The word was: {}", secret);
    }
}
