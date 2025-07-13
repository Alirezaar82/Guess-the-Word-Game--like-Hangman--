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

fn check_answer(secret_word:String,user_answer:String) -> (String, bool) {
    let mut has_change = false;
    let secret_chars: Vec<char> = secret_word.chars().collect();
    let mut masked_word: Vec<char> = vec!['*'; secret_word.len()];

    for (i, ch) in secret_chars.iter().enumerate() {
        if user_answer.contains(*ch) {
            masked_word[i] = *ch;
            has_change = true;
        }
    }
    let result: String = masked_word.iter().collect();
    (result,has_change)
}

fn main() {
    let mut guess_count = 0;
    let secret = get_random_word();
    // println!("Secret word is: {}", secret);
    let len_secret_word = secret.len();
    let mut count = len_secret_word+1;
    let mut guess_word  = String::new();
    for _ in 0..len_secret_word {
        guess_word  += "*";
    }
    println!("{}",secret);
    println!("Welcom to HangmanRust game");
    println!("now guess the word:");
    println!("you have {} more guess",count);
    println!("{}",guess_word );

    while true{
        let mut user_guess = String::new();
        io::stdin().read_line(&mut user_guess).expect("Failed to read input");
        guess_count += 1;
        let (guess_word,has_guess) = check_answer(secret.to_string(),user_guess.trim().to_lowercase());
        let user_guess = user_guess.trim().to_lowercase();
        if secret == user_guess{
            println!("you won!!!");
            println!("Your guess count is {}",guess_count);
            break;
        }
        if has_guess == true{
            println!("you find a letter\n keep going!");
            println!("Guess word: {}", user_guess);
            println!("Result: {}", guess_word);
        }else if has_guess == false{
            count -= 1;
            println!("nothing try again!");
            println!("you have {} more guess",count);
        }
        if count == 0 {
            println!("you loss");
            break;
        }
    }
}
