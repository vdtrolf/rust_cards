use rust_cards::mod_utils::get_cardsfile_specs;
use rust_cards::mod_utils::print_md_txt;
use rust_cards::mod_utils::print_title;
use rust_cards::mod_utils::read_cardsfile;

static TITLE: &str = " 1-Variables";
static EXP_TEXT: &str = "**static** > AAA:f32 = 3.45;
**const**  > BBB:bool = true;
**var**    > let (mut) (_)x:u8 = 11; ^(mut = mutable, _ => not used)^
**Types**  > u.. i.. f.. char bool \"\"
**Array**  > let an_array:[u8,3]=[1,2,4];
       > println!(\"{an_array[1]}\"); ^(=2)^
**Vector** > let vec = vec![]
       > let vec: Vec<i16> = vec![-1,2,5,6]
       > println(\"{vec[1]}\"); ^(=2)^
**Tuple**  > let tup = ('a',2,3.24)
       > println(\"{tup.1}\"); ^(='a')^
       > let (x1,x2,x3) = tup;";

pub fn learn_cards_codes(show_all: bool) {
    let cards_file_content: String = read_cardsfile("codes.md");
    let cards_specs: rust_cards::mod_utils::CardsFileSpecs =
        get_cardsfile_specs(&cards_file_content.as_str());

    if show_all {
        print_title(TITLE);
        println!("===> {} {}", cards_specs.cards_name, cards_specs.cards_type);
        print_md_txt(&cards_file_content.as_str());
    } else {
        println!("{}", TITLE);
    }
}
