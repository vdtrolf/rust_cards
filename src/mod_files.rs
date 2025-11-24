use std::fs;

pub struct CardsFileSpecs {
    pub cards_name: String,
    pub cards_type: u8,
    pub encrypted: bool,
}

pub fn get_cardsfile_specs(cards_file: &str) -> CardsFileSpecs {
    let mut cards_name: &str = "";
    let mut cards_type: u8 = 0;
    let mut encrypted: bool = false;
    let mut token_counter: u8 = 0;
    let lines = cards_file.split("\n");
    for line in lines {
        let tokens = line.split(" ");
        for token in tokens {
            match token_counter {
                0 => cards_name = token,
                1 => cards_type = token.to_string().parse().expect("Not a valid number"),
                2 => encrypted = token.starts_with("T"),
                _ => (),
            };
            token_counter += 1;
        }
        break;
    }

    let specs = CardsFileSpecs {
        cards_name: cards_name.to_string(),
        cards_type: cards_type,
        encrypted: encrypted,
    };
    return specs;
}

pub fn read_cardsfile(file_name: &str) -> String {
    let file_path = format!("./src/{}", file_name);
    let _error_message = String::from(format!("Can't read {}", file_path));
    let content = String::from(fs::read_to_string(file_path).expect("Not found"));
    return content;
}