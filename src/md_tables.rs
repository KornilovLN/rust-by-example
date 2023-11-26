use std::fmt;

extern crate ansi_term;
use ansi_term::Colour;

#[derive(Debug)]
struct XY (
    f64,      //--- что-то вроде аргумента или просто одной переменной
    f64,      //--- а это может быть как бы фия армента или вторая перемення
);

#[derive(Debug)]
struct Vec_XY {
    xy: Vec<XY>,    //--- собственно массивструктур XY
    name: String,   //--- "fieldX fieldY"; строку сплитовать на поля
}

impl Vec_XY {
    //--- конструктор
    pub fn new(name: &str) -> Self {
        Self {
            xy : vec!(),
            name: name.to_string(),
        }
    }

    pub fn testload(&mut self, n: i32) {

        for i in 0..n {
            let x = i as f64;
            let y = x * x;
            let mut dt = XY(x, y);

            self.xy.push(dt);
        }
    }

    pub fn out(&mut self) {
        println!("\n{}", self.name);

        let mut iter = self.xy.iter();
        for xy in iter {
            println!("{}\t{}", xy.0, xy.1);
        }

        println!();
    }

}

impl fmt::Display for Vec_XY {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let spl: Vec<&str> = self.name.split(" ").collect();
        let mut ln0 = (spl[0].chars().count()) as i32;
        let mut ln1 = (spl[1].chars().count()) as i32;
        writeln!(f, "{}{}{}{}{}", "+","----------","+","-------------","+");
        writeln!(f, "|  {}   |  {}   |", &spl[0], &spl[1]);
        writeln!(f, "{}{}{}{}{}", "+","----------","+","-------------","+");
        let v_iter = self.xy.iter();
        for v in v_iter {
            let mut fmt_v0 = format!("| {:-8.3} |", v.0);
            let mut fmt_v1 = format!(" {:-11.3} |", v.1);
            write!(f, "{}", fmt_v0);
            write!(f, "{}\n", fmt_v1);
        }
        writeln!(f, "{}{}{}{}{}", "+","----------","+","-------------","+");

        Ok(())
    }
}

///--- Создает тестовую и выводит таблицу в 2 столбца X и Y
pub fn table_test(){
    let mut table = Vec_XY::new("argum function");

    println!("\n<<< md_tables: Вывод вложенной структуры >>>");
    println!("Самопал:\n");

    //--- тестовая загрузка структуры
    table.testload(5);

    println!("посредством debug метода");
    println!("{:#?}\t",table);

    println!("Вывод методом структуры (класса)");
    table.out();

    println!("Display метод");
    println!("{}\t", table);
}

/*
///
pub fn table_arg_fun(name_x: &str, name_y: &str) {
    let stroka = text.to_string();
    let mut len = ((stroka.chars().count()) as i32) + 8; //--- так можно. 8 символов "<<< "*2
    //let len = stroka.len() as i32 + 1; //--- так нельзя - ?????????????????

    let frm_ch = format!("{}",Colour::Green.bold().paint("-".to_string()));
    let frm_stroka = format!("{}",Colour::Yellow.bold().paint(stroka));
    let frm_skoba_l = format!("{}",Colour::Green.bold().paint("<<< ".to_string()));
    let frm_skoba_r = format!("{}",Colour::Green.bold().paint(" >>>".to_string()));
    print!("\n\t");
    print_line_char(&frm_ch, len);
    println!("\t{}{frm_stroka}{}",frm_skoba_l, frm_skoba_r);
    print!("\t");
    print_line_char(&frm_ch, len);
}
*/