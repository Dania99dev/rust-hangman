use std::env;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=====================");
    println!("HANGMAN! CAN YOU GUESS THE WORD???");
    println!("");
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("DISASTER! No word to guess!");
    } else if args.len() == 2 {
        let word = args[1].trim().to_uppercase();
        let mut lives = 6;
        let mut placeholder = (0..word.len()).map(|_| "_ ").collect::<String>();
        println!("Word: {}  ( {} ❤ )", placeholder, lives);
        println!("");
        println!("Guess a character: ");
        loop {
            let mut input = String::new();

            io::stdin().read_line(&mut input)?;

            if input.trim().len() == 1 {
                let guessed_char: Vec<char> = input.to_uppercase().trim().chars().collect();
                let word_chars: Vec<char> = word.chars().collect();
                if word_chars.contains(&guessed_char[0]) {
                    println!("The word has {}!", guessed_char[0]);
                    for (i, ch) in word_chars.iter().enumerate() {
                        if ch == &guessed_char[0] {
                            let mut new_placeholder: Vec<char> = placeholder.chars().collect();
                            new_placeholder[i * 2] = *ch;
                            placeholder = new_placeholder.iter().collect();
                        }
                    }
                    println!("Word: {}  ( {} ❤ )", placeholder, lives);
                } else {
                    println!("No {} in the word :(", guessed_char[0]);
                    lives -= 1;
                    if lives == 0 {
                        println!("😵😵Oh crap! You Lost!😵😵");
                        break;
                    }
                    println!("Word: {}  ( {} ❤ )", placeholder, lives);
                }
            } else {
                println!("You should guess a character");
            }
            if !placeholder.contains("_") {
                println!("🚀🚀You won!🚀🚀");
                break;
            }
        }
    } else {
        println!("Invalid inputs.");
    }
    println!("=====================");
    Ok(())
}
