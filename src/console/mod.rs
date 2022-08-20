pub(crate) mod banners;

use std::io::{stdin, stdout, Write};

pub fn display_letters(word: &String, found_positions: &Vec<i8>) {

    let word_length = word.chars();

    for (i, _) in word_length.enumerate() {
        if found_positions.contains(&(i as i8)) {
            print!("{} ", word.chars().nth(i).unwrap_or(' '));
        } else {
            print!("_ ");
        }
    }
    println!()
}

fn get_typed_letter() -> Option<char> {

    let mut s= String::new();

    print!("Character: ");

    let _ = stdout().flush();

    stdin().read_line(&mut s).expect("Didn't enter a correct answer.");

    s.to_uppercase().chars().filter(|c| !['\n', '\r'].contains(c)).nth(0)
}

pub fn get_new_letter(typed: &Vec<char>) -> char {
    let mut new_letter = get_typed_letter();
    while new_letter.is_none() {
        new_letter = get_typed_letter();
    }
    while typed.contains(&new_letter.unwrap()) {
        println!("\nYou've already typed that character! Choose another one!\n");
        new_letter = get_typed_letter();
    }
    new_letter.unwrap()
}

pub fn ask_repeat() -> bool {
    let mut s= String::new();

    print!("Would you like to play it again with another word? [Y/n] ");

    let _ = stdout().flush();

    stdin().read_line(&mut s).expect("Didn't enter a correct answer.");

    s.to_uppercase()
        .chars()
        .filter(|c| !['\n', '\r'].contains(c))
        .nth(0)
        .unwrap_or('N') != 'N'
}
