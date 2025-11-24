use colored::Colorize;


pub fn print_md_txt(txt_md: &str) {
    let mut is_text: bool = false;
    let parts = txt_md.split("\n");
    for l in parts {
        if is_text {
            print_line(l);
        }
        is_text = true;
    }
    println!("");
}

fn print_line(l: &str) {
    let inc: bool = l.starts_with(">");

    if l.starts_with("###") {
        println!("{}", l[3..].trim().on_red().bold());
    } else if l.starts_with("##") {
        println!("{}", l[2..].trim().cyan().bold());
    } else if l.starts_with("#") {
        println!("{}", l[1..].trim().green().bold());
    } else {
        let parts = l.split(" ");
        let mut inb: bool = false;
        let mut ini: bool = false;
        let mut buf: String = ("").to_string();
        for t in parts {
            if t.starts_with("^") && t.ends_with("^") {
                let mut s = t[1..].to_string();
                s.pop();
                print!("{} ", s.cyan().italic());
            } else if t.starts_with("^") {
                ini = true;
                buf = (&t[1..]).to_string();
            } else if t.ends_with("^") && ini {
                let mut newbuf = buf.to_string() + " " + t;
                newbuf.pop();
                print!("{} ", newbuf.cyan().italic());
                buf = ("").to_string();
                ini = false;
            } else if ini {
                let newbuf = buf.to_string() + " " + t;
                buf = newbuf;
            } else if t.starts_with("**") && t.ends_with("**") {
                let mut s = t[2..].to_string();
                s.pop();
                s.pop();
                print!("{} ", s.cyan().bold());
            } else if t.starts_with("**") {
                inb = true;
                buf = (&t[2..]).to_string();
            } else if t.ends_with("**") && inb {
                let mut newbuf = buf.to_string() + " " + t;
                newbuf.pop();
                newbuf.pop();
                print!("{} ", newbuf.yellow().bold());
                buf = ("").to_string();
                inb = false;
            } else if inb {
                let newbuf = buf.to_string() + " " + t;
                buf = newbuf;
            } else {
                if inc {
                    print!("{} ", t.green());
                } else {
                    print!("{} ", t);
                }
            }
        }
        print!("\n");
    }
}

pub fn print_title(title: &str) {
    println!("{}\n", title.trim().red().bold().underline());
}


