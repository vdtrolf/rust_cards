use colored::Colorize;
use std::io;

mod mod_01_codes;
mod mod_02_rust;

use mod_01_codes::learn_cards_codes;
use mod_02_rust::learn_cards_rust;

fn main() {
    println!("Hello, world!");
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    print_welcome();

    let mut _last: String = String::new();

    loop {
        let mut input: String = String::new();
        io::stdin() // Get the standard input stream
            .read_line(&mut input) // The rea read_line function reads data until it reaches a '\n' character
            .expect("Unable to read Stdin"); // In case the read operation fails, it panics with the given message

        match input.trim() {
            "a" | "A" => print_welcome(),
            "0" | "q" | "Q" => break,
            "c" | "C" => learn_cards_codes(true),
            "r" | "R" => learn_cards_rust(true),
            _ => println!("No idea"),
        }
        println!("");
        println!("Which module ?");
        _last = input.clone();
    }
}

fn print_welcome() {
    println!("{}", "FLASH CARDS".red().bold());
    println!("");
}
