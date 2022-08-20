mod database;

#[allow(dead_code)]
pub struct Word {
    id: u32,
    pub value: String,
    pub tip: Option<String>
}

pub fn get_new_unique_word(words_played: &Vec<String>) -> Option<Word> {
    let mut word = database::get_new_word();
    if word.is_none() {
        return None;
    }
    while words_played.into_iter()
        .filter(|&p| *p == word.as_ref().unwrap().value)
        .count() > 0
    {
        word = database::get_new_word();
    }
    word
}

pub fn is_char_in(word: &String, value: char) -> bool {
    word.to_uppercase().find(value).is_some()
}

pub fn get_positions_in(word: &String, value: char) -> Vec<i8> {
    word.to_uppercase()
        .chars()
        .enumerate()
        .filter(|(_, b)| *b == value)
        .map(|(i, _)| i as i8)
        .collect()
}

pub fn collect_founds(old_positions: &Vec<i8>, new_positions: Vec<i8>) -> Vec<i8> {
    let old = old_positions.into_iter()
        .copied()
        .filter(|&a| a != -1);

    new_positions.into_iter().chain(old).collect()
}