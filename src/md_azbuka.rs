extern crate ansi_term;
use ansi_term::Colour;

pub fn azbuka_go() {
    println!("\n<<<azbuka: Распечатка нескольких алфавитов через их unicod >>>");
    println!("В этом источнике находится Таблица символов Юникода для ПК и для html");
    println!("https://symbl.cc/ru/unicode/table/#cyrillic");
    println!("Вот тут автор затеял свой сайт");
    println!("https://symbl.cc/ru/unicode/");

    println!("\nazbuka as u32:");
    for x in 'а' as u32..='я' as u32 {
        print!("{:?} ", std::char::from_u32(x).unwrap());
    }
    println!("\nazbuka as char:");
    for x in 'а'..='я' {
        print!("{} ", x);
    }
    println!();

    println!("\nazbuka as u32:");
    for x in 'Ⰰ' as u32..='Ⱞ' as u32 {
        
        print!("{:?} ", std::char::from_u32(x).unwrap());
    }
    println!("\nazbuka as char:");
    for x in 'Ⰰ'..='Ⱞ' {        
        print!("{} ", x);
    }
    println!();

    println!("\nazbuka иврит as u32:");
    for x in 0x05d0..=0x05f4 as u32 {        
        print!("{:?} ", std::char::from_u32(x).unwrap());
    }
    println!();

    println!("\nazbuka арабская as u32:");
    let mut st: String = String::new();
    for x in 0x0600..=0x06ff as u32 {
        let ch = std::char::from_u32(x).unwrap();
        st.push(ch);  
        st.push(' '); 
        print!("{} ", ch);              
    }
    println!();
    print!("{} ", st);
    println!();
}