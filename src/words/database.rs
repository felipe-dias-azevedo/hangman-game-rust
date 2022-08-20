use rusqlite::Connection;
use crate::words::Word;

const DATABASE_PATH: &str = "./words.db";

fn get_connection() -> Option<Connection> {
    let con = Connection::open(DATABASE_PATH);

    if con.is_err() {
        eprintln!("Couldn't set connection to words database.\nError: {}", con.unwrap_err());
        return None;
    }

    Some(con.unwrap())
}

fn create_table(connection: &Connection) -> Option<usize> {
    let create_table = connection.execute(
        "CREATE TABLE IF NOT EXISTS words (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        value VARCHAR(100) NOT NULL,
        tip TEXT
        )",
        ()
    );
    match create_table {
        Ok(s) => Some(s),
        Err(err) => {
            eprintln!("Couldn't create table to words database.\nError: {}", err);
            None
        }
    }
}

pub(crate) fn get_new_word() -> Option<Word> {
    let connection = get_connection();

    if connection.is_none() {
        return None;
    }

    let connection = connection.unwrap();

    let table_created = create_table(&connection);

    if table_created.is_none() {
        return None;
    }

    let query = connection.prepare("SELECT id, value, tip FROM words ORDER BY RANDOM() LIMIT 1");

    if query.is_err() {
        eprintln!("Couldn't get word from words database.\nError: {}", query.unwrap_err());
        return None;
    }

    let mut query = query.unwrap();

    let words = query.query_map([], |row| {
        Ok(Word {
            id: row.get_unwrap(0),
            value: row.get_unwrap(1),
            tip: row.get(2).unwrap_or(None),
        })
    });

    let word = words
        .expect("Couldn't get word from words database.")
        .into_iter().nth(0);

    if word.is_none() {
        eprintln!("Words database is empty.");
        return None;
    }

    let word = word.unwrap();

    let word = word.expect("Couldn't get word from words database.");

    return Some(word);
}