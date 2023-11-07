///--- Section import module ----------------------------------------------------------------------
use std::fmt;               //--- модуль `fmt` форматирования для вывода типов данных.


///--- Section const ------------------------------------------------------------------------------
const PI: f64 = 3.1415926535897932384626433832795028841971;

///--- Section Structures and Enums ---------------------------------------------------------------
#[derive(Debug)]
pub struct ArgFun{      //--- Cтруктура содержит аргумент и результат функции
    arg: f64,
    fun: f64
}
#[derive(Debug)]
struct ListXY {     //--- Cтруктура содержит вектор структур ArgFun
    vec: Vec<ArgFun>,
}

///--- Section traits -----------------------------------------------------------------------------
impl fmt::Display for ArgFun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Записываем первый элемент в выходной поток: `f`.
        // `fmt::Result` показывает выполнилась операция успешно или нет.
            write!(f, "|  {:-0.4}  |", self.arg);
            write!(f, "  {:-0.7}  |\n", self.fun);
        Ok(())
    }
}

///--- Trait для отрисовки структуры ListXY таблицей
impl fmt::Display for ListXY {

    /*
    //--- Как вариант: Display сразу рисует ListXY, извлекая из него вектор и поля
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Записываем первый элемент в выходной поток: `f`.
        // `fmt::Result` показывает выполнилась операция успешно или нет.
        writeln!(f, "{}{}{}{}{}", "+", "----------", "+", "-------------", "+");
        writeln!(f, "|  {}    |  {}      |", "Arg:", "Func:");
        writeln!(f, "{}{}{}{}{}", "+", "----------", "+", "-------------", "+");
        let v_iter = self.vec.iter();
        for v in v_iter {
            write!(f, "|  {:-0.4}  |", v.arg);
            write!(f, "  {:-0.7}  |\n", v.fun);
        }
        writeln!(f, "{}{}{}{}{}", "+", "----------", "+", "-------------", "+");
        Ok(())
    }
    */

    //--- А этот Display использует средства дисплея от ArgFun
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Записываем первый элемент в выходной поток: `f`.
        // `fmt::Result` показывает выполнилась операция успешно или нет.
        writeln!(f, "{}{}{}{}{}", "+", "----------", "+", "-------------", "+");
        writeln!(f, "|  {}    |  {}      |", "Arg:", "Func:");
        writeln!(f, "{}{}{}{}{}", "+", "----------", "+", "-------------", "+");
        let v_iter = self.vec.iter();
        for v in v_iter { //--- Тут каждое v имеет свой Display метод
            write!(f,"{}", v);
        }
        writeln!(f, "{}{}{}{}{}", "+", "----------", "+", "-------------", "+");
        Ok(())
    }
}

///--- Section Implementation structures ----------------------------------------------------------


///--- Функция демонстрации работы
pub fn sin_table() {
    println!("\n<<< Генератор, вектор структуры, трейт-форматтер, замыкания: Основы Rust >>>");
    println!("https://doc.rust-lang.ru/stable/rust-by-example/std/vec.html\n");

    let mut list: ListXY;

    //--- генерируем структуру векторов аргументов и функций f64 в количестве 12
    let mut argfun = create_vec_argfun_f64 (12.0, 12);
    list = ListXY {vec: argfun};

    println!("{}", list);
}


///--- Section function ---------------------------------------------------------------------------
///--- Метод создает вектор пар значений arg и fun типа f64
pub fn create_vec_argfun_f64(step: f64, sz: i32) -> Vec<ArgFun> {
    let mut argfun: Vec<ArgFun> = vec![];
    let mut i = 0;
    while i <= sz {
        let ag = PI * (i as f64) / step;
        let fu = (ag.sin() * 10000000.0).round() / 10000000.0;
        let st_argfun = ArgFun { arg: ag, fun: fu };
        argfun.push(st_argfun);
        i += 1;
    }
    argfun
}