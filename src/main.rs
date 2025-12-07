use colored::Colorize;
use inquire::Text;

mod mod_01_codes;
mod mod_02_rust;

use mod_01_codes::learn_cards_codes;
use mod_02_rust::learn_cards_rust;

fn main() {
    let mut last: String = String::new();

    loop {
        println!("");
        let name = Text::new("0 to 14 ?").prompt();

        match name {
            Ok(name) => {
                if process_val(name.as_str(), last.as_str()) {
                    break;
                };
                last = name.clone();
            }
            Err(_) => break,
        }
    }
}

fn process_val(value: &str, _last: &str) -> bool {
    let mut do_break: bool = false;
    print_welcome();

    match value {
        "a" | "A" => print_welcome(),
        "0" | "q" | "Q" => do_break = true,
        "c" | "C" => learn_cards_codes(true),
        "r" | "R" => learn_cards_rust(true),
        _ => println!("No idea"),
    }
    println!("");
    println!("Which module ?");

    return do_break;
}

fn print_welcome() {
    println!("{}", "FLASH CARDS".red().bold());
    println!("");
}
