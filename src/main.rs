mod words;
mod console;


fn main() {

    console::banners::show_header();

    let mut words_played: Vec<String> = Vec::new();

    loop {
        let word = words::get_new_unique_word(&words_played);

        if word.is_none() {
            break;
        }

        let word = word.unwrap();
        let mut errors: u8 = 0;
        let mut won = false;
        let mut found_positions: Vec<i8> = Vec::new();
        let mut typed: Vec<char> = Vec::new();

        while errors < 5 && !won {
            console::banners::display_hangman(errors, &word.tip);
            console::display_letters(&word.value, &found_positions);
            let new_letter = console::get_new_letter(&typed);
            typed.push(new_letter);
            if !words::is_char_in(&word.value, new_letter) {
                errors += 1;
                continue
            }
            let new_positions = words::get_positions_in(&word.value, new_letter);
            found_positions = words::collect_founds(&found_positions, new_positions);

            if found_positions.len() == word.value.len() {
                won = true;
                console::display_letters(&word.value, &found_positions);
            }
        }

        if won {
            console::banners::show_win();
        } else {
            console::banners::show_loss(&word.value);
        }

        if !console::ask_repeat() {
            break;
        }

        words_played.push(word.value)
    }
}
