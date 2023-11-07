use ansi_term::Colour;
use std::process;

//================================================================================================
fn title() {
    println!("\n==========================================================");
    println!("<<< Набор учебных программ из туториалов: >>>");
    println!("https://doc.rust-lang.ru/stable/rust-by-example/index.html");
    println!("https://doc.rust-lang.ru/rust-cookbook/intro.html");
    println!("author: StarmarkLN (Kornilov LN)");
    println!("Github: https://github.com/KornilovLN");
    println!("e-mail: ln.KornilovStar@gmail.com");
    println!("e-mail: ln.starmark@ekatra.io");
    println!("e-mail: ln.starmark@gmail.com");
    println!("date: 24.07.2023");
    println!("==========================================================\n");
}

fn main() {
    title();

    if let Err(e) = tutor::run() {
        let frm_err = format!(
            "{}",
            Colour::Red.paint("Main: Ошибка в приложении".to_string())
        );
        eprintln!("{}: {}", frm_err, e);
        process::exit(1);
    }
}
