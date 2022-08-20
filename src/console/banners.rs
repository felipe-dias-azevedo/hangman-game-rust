pub fn show_header() {
    println!("|--------------|");
    println!("| Hangman Game |");
    println!("|--------------|");
}

pub fn display_hangman(ammount_errors: u8, tip: &Option<String>) {
    println!();
    if let Some(tipo) = tip {
        println!("Tip: {}", tipo);
    }
    println!("  _______       ");
    println!(" |/      |      ");
    println!(" |      {}{}{}  ",
             if ammount_errors >= 1 { '(' } else { ' ' },
             if ammount_errors >= 1 { '_' } else { ' ' },
             if ammount_errors >= 1 { ')' } else { ' ' });
    println!(" |      {}{}{}  ",
             if ammount_errors >= 3 { '\\' } else { ' ' },
             if ammount_errors >= 2 { '|' } else { ' ' },
             if ammount_errors >= 3 { '/' } else { ' ' });
    println!(" |       {}     ",
             if ammount_errors >= 2 { '|' } else { ' ' });
    println!(" |      {} {}   ",
             if ammount_errors >= 4 { '/' } else { ' ' },
             if ammount_errors >= 4 { '\\' } else { ' ' });
    println!(" |              ");
    println!("_|___           ");
    println!();
}

pub fn show_win() {
    println!("\nCongratulations! You've won!\n");

    println!("       ___________      ");
    println!("      '._==_==_=_.'     ");
    println!("      .-\\:      /-.    ");
    println!("     | (|:.     |) |    ");
    println!("      '-|:.     |-'     ");
    println!("        \\::.    /      ");
    println!("         '::. .'        ");
    println!("           ) (          ");
    println!("         _.' '._        ");
    println!("        '-------'       \n");
}

pub fn show_loss(word: &String) {
    println!("\nI'm sorry! You've lost!\nYour word was {}.\n", word);

    println!("    _______________         ");
    println!("   /               \\       ");
    println!("  /                 \\      ");
    println!("//                   \\/\\  ");
    println!("\\|   XXXX     XXXX   | /   ");
    println!(" |   XXXX     XXXX   |/     ");
    println!(" |   XXX       XXX   |      ");
    println!(" |                   |      ");
    println!(" \\__      XXX      __/     ");
    println!("   |\\     XXX     /|       ");
    println!("   | |           | |        ");
    println!("   | I I I I I I I |        ");
    println!("   |  I I I I I I  |        ");
    println!("   \\_             _/       ");
    println!("     \\_         _/         ");
    println!("       \\_______/           \n");
}