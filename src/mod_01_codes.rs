use rust_cards::mod_files::get_cardsfile_specs;
use rust_cards::mod_utils::print_md_txt;
use rust_cards::mod_utils::print_title;
use rust_cards::mod_files::read_cardsfile;

static TITLE: &str = " 1-Variables";

pub fn learn_cards_codes(show_all: bool) {
    let cards_file_content: String = read_cardsfile("codes.md");
    let cards_specs: rust_cards::mod_files::CardsFileSpecs =
        get_cardsfile_specs(&cards_file_content.as_str());

    if show_all {
        print_title(TITLE);
        println!("===> {} {}", cards_specs.cards_name, cards_specs.cards_type);
        print_md_txt(&cards_file_content.as_str());
    } else {
        println!("{}", TITLE);
    }
}
