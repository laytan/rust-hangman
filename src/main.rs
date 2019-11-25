use std::io;

// The game state to pass around
struct State {
    lives: u32,
    tries: u32,
    word: String,
    progress: String
}

fn main() {
    println!("Enter the word to guess");
    let first_word = get_word();
    let mut state = init_state(first_word);

    // print giberish so the player cant see the word given
    for _ in 0..100 {
        println!("------------------------------");
    }

    print_game(&state);

    // loop until the game is done
    loop {
        println!("Enter a guess");
        process_turn(&mut state, get_letter());
        let done = print_game(&state);
        if done != 0 {
            break;
        }
    }
}

/**
 * Given the state and a letter will mutate the state according to game rules
 */
fn process_turn(state: &mut State, letter: char) {
    if state.word.contains(letter) {
        let mut new_word = String::new();
        // Loop over all characters in word
        for (i, c) in state.word.chars().enumerate() {
            // if the guessed letter is the current character, add the letter to our new word
            if c == letter {
                new_word.push(letter);
            } else {
                // Else take the current letter from progress and push it

                // get the enumerable of progress and take the char on index i, unwrap takes it out of the returned Option type
                let char_at_i = state.progress.chars().nth(i).unwrap(); 
                new_word.push(char_at_i);
            }
        }
        // Update state
        state.progress = new_word;
    } else {
        state.tries = state.tries + 1;
    }
}

/**
 * Return base state according to given word
 */
fn init_state(word: String) -> State {
    let mut progress = String::from("");
    // For every character in word add a _ to progress
    for _ in word.chars() {
        progress.push('_');
    }

    return State {
        lives: 6,
        tries: 0,
        word: word,
        progress: progress
    };
}

/**
 * Prints out the current state nicely
 */
fn print_game(state: &State) -> u16 {
    if state.progress == state.word {
        println!("You won!");
        return 1;
    } else if state.lives - state.tries < 1 {
        println!("You lost! The word was: {}", state.word);
        return 2;
    } else {
        println!("progress: {}, lives: {}", state.progress, state.lives - state.tries);
        return 0;
    }
}

/**
 * Loop until a valid word is given
 */
fn get_word() -> String {
    loop {
        let mut temp_word = String::new();
        io::stdin().read_line(&mut temp_word)
            .expect("Error reading input.");

        // Trim to get rid of Enter (newline) and parse into a string, ask again if something else is given
        let res: String = match temp_word.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a word");
                continue;
            },
        };
        break res;
    }
}

/**
 * Loop until a valid letter (character) is given
 */
fn get_letter() -> char {
    loop {
        let mut temp_char = String::new();
        io::stdin().read_line(&mut temp_char).expect("Error reading input.");
        // Trim to get rid of Enter (newline), try to parse input into a character, ask again if something else is given
        let res: char = match temp_char.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a letter");
                continue;
            },
        };
        break res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_returns_right_amt_of_lives() {
    let state = init_state(String::from("hangman"));
    assert_eq!(state.lives, 6);
    }

    #[test]
    fn init_returns_right_progress_depending_on_word() {
    let state = init_state(String::from("hangman"));
    assert_eq!(state.progress, String::from("_______"));
    }

    #[test]
    fn print_game_returns_win_if_won() {
        let state = State {
            lives: 6,
            tries: 0,
            word: String::from("hangman"),
            progress: String::from("hangman"),
        };
        assert_eq!(print_game(&state), 1);
    }

    #[test]
    fn print_game_returns_lose_if_lost() {
        let state = State {
            lives: 6,
            tries: 6,
            word: String::from("hangman"),
            progress: String::from("_______")
        };
        assert_eq!(print_game(&state), 2);
    }

    #[test]
    fn print_game_returns_progress_if_progress() {
        let state = State {
            lives: 6,
            tries: 3,
            word: String::from("hangman"),
            progress: String::from("hang___")
        };
        assert_eq!(print_game(&state), 0);
    }

    #[test]
    fn print_won_when_out_of_lives_but_full_word() {
        let state = State {
            lives: 6,
            tries: 6,
            word: String::from("hangman"),
            progress: String::from("hangman")
        };
        assert_eq!(print_game(&state), 1);
    }

    #[test]
    fn process_turn_adds_a_try_on_wrong_letter() {
        let mut state = State {
            lives: 6,
            tries: 0,
            word: String::from("hangman"),
            progress: String::from("hang___")
        };
        process_turn(&mut state, 'o');
        assert_eq!(state.tries, 1);
    }

    #[test]
    fn char_gets_correctly_changed_from_underscore() {
        let mut state = State {
            lives: 6,
            tries: 0,
            word: String::from("abcdefg"),
            progress: String::from("_______")
        };
        process_turn(&mut state, 'b');
        assert_eq!(state.progress, "_b_____");
    }

    #[test]
    fn multiple_chars_get_correctly_changed() {
        let mut state = State {
            lives: 6,
            tries: 0,
            word: String::from("ababab"),
            progress: String::from("______")
        };
        process_turn(&mut state, 'b');
        assert_eq!(state.progress, "_b_b_b");
    }

    #[test]
    fn already_guessed_chars_stay_the_same() {
        let mut state = State {
            lives: 6,
            tries: 0,
            word: String::from("hangman"),
            progress: String::from("h_ng__n")
        };
        process_turn(&mut state, 'm');
        assert_eq!(state.progress, String::from("h_ngm_n"));
    }
}