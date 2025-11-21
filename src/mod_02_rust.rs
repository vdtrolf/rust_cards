use rust_cards::mod_utils::print_md_txt;
use rust_cards::mod_utils::print_title;

static TITLE: &str = " 2-RUST";
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

pub fn learn_cards_rust(show_all: bool) {
    if show_all {
        print_title(TITLE);
        print_md_txt(EXP_TEXT);
    } else {
        println!("{}", TITLE);
    }
}
