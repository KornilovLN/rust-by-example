use ansi_term::Colour;
use std::process;

mod md_utils;

//--- памятка
//--- Shorthand                     Equivalent to                                           Access

//--- for item in collection        for item in IntoIterator::into_iter(collection)         Ownership
//--- for item in &collection       for item in collection.iter()                           Read-only
//--- for item in &mut collection   for item in collection.iter_mut()                       Read-write

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


fn title() {
    md_utils::line_char("=", 84); 
    println!("\t\t<<< Набор учебных программ из туториалов: >>>");
    md_utils::line_char("=", 84);
    println!("\thttps://doc.rust-lang.ru/stable/rust-by-example/index.html");
    println!("\thttps://doc.rust-lang.ru/rust-cookbook/intro.html");
    println!("\thttps://doc.rust-lang.ru/book/ (Язык программирования Rust)");
    println!("\thttps://doc.rust-lang.ru/async-book/ (Async Programming in Rust)");
    println!("\thttps://doc.rust-lang.ru/stable/nomicon/ (The Rustonomicon)");
    println!("\thttps://rust-lang.ru/ (Список материалов по теме RUST)");
    println!("\thttps://ru.dsplib.org/dspl/group___d_f_t___g_r_o_u_p.html");
    println!();
    println!("\tauthor: StarmarkLN (Kornilov LN)");
    println!("\tGithub: https://github.com/KornilovLN");
    println!("\te-mail: ln.KornilovStar@gmail.com");
    println!("\te-mail: ln.starmark@ekatra.io");
    println!("\te-mail: ln.starmark@gmail.com");
    println!("\tdate: 24.07.2023");
    md_utils::line_char("=", 84);
}

