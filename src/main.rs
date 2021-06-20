use std::env;
use std::error::Error;
use std::io;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=====================");
    println!("HANGMAN! CAN YOU GUESS THE WORD???\n");
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("DISASTER! No word to guess!");
    } else if args.len() == 2 {
        let word = args[1].trim().to_uppercase();
        let mut lives = 6;
        let mut placeholder = (0..word.len()).map(|_| "_ ").collect::<String>();
        let mut guess_history = Vec::new();

        println!("Word: {}  ( {} ❤ )\n", placeholder, lives);
        loop {
            print!("Guess a character: ");
            io::stdout().flush().expect("flush failed!");
            let mut input = String::new();

            io::stdin().read_line(&mut input)?;

            if input.trim().len() == 1 {
                let guessed_char: Vec<char> = input.to_uppercase().trim().chars().collect();
                let guessed_char = guessed_char[0];
                if guess_history.contains(&guessed_char) {
                    println!("You already guessed this, try another one\n");
                    continue;
                }
                guess_history.push(guessed_char);
                let word_chars: Vec<char> = word.chars().collect();

                if word_chars.contains(&guessed_char) {
                    println!("The word has {}!", guessed_char);
                    for (i, ch) in word_chars.iter().enumerate() {
                        if ch == &guessed_char {
                            let mut new_placeholder: Vec<char> = placeholder.chars().collect();
                            new_placeholder[i * 2] = *ch;
                            placeholder = new_placeholder.iter().collect();
                        }
                    }
                    println!("Word: {}  ( {} ❤ )\n", placeholder, lives);
                } else {
                    println!("No {} in the word :(", guessed_char);
                    lives -= 1;
                    if lives == 0 {
                        println!("😵😵Oh crap! You Lost!😵😵");
                        break;
                    }
                    println!("Word: {}  ( {} ❤ )\n", placeholder, lives);
                }
            } else {
                println!("You should guess a character\n");
            }
            if !placeholder.contains("_") {
                println!("🚀🚀You won!🚀🚀");
                break;
            }
        }
    } else {
        panic!("Invalid inputs!");
    }
    println!("=====================");
    Ok(())
}
