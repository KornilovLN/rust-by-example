use std::ops::Range;
//extern crate range;

use rand::{thread_rng, Rng};

//=================================================================================================

//--- Замыкания: Основы Rust: подробно про замыкания (closures)
//--- https://highload.today/rust-zamykaniya-closures/
pub fn string_tutor() {
    println!("\n<<< md_closure: Замыкания: Основы Rust: подробно про замыкания (closures) >>>");
    println!("https://highload.today/rust-zamykaniya-closures/\n");

    let m = 2.0;
    let c = 1.0;
    println!("Даны коеффициенты: m = {}  c = {}", m, c);

    println!("Есть функция-замыкание: let lin = |x| m * x + c;");
    let lin = |x| m * x + c;

    let x1 = 1.0;
    let x2 = 2.0;
    println!("Вычисляем 2 значения для x = {} и {}; получим -> {:?}, {:?}",x1,x2,lin(x1),lin(x2));

    let lll = lin;
    let x3 = 7.0;
    println!("Вычисляем для x = {}; получим -> {:?}", x3, lll(x3));

    println!("\n//------------------------------------------------------------------------");

    println!("\nПусть существует функция apply, которая имеет аргументами f64 и замыкание");
    println!("fn apply<F>(x: f64, f: F) -> f64
    where F: Fn(f64)->f64  [
    f(x) ] <- квадратные скобки заменить на фигурные");

    fn apply<F>(x: f64, f: F) -> f64
        where F: Fn(f64)->f64  {
        f(x)
    }

    println!("\nИ есть такие замыкания:");
    println!("let res1 = apply(3.0,lin);\nlet res2 = apply(3.14, |x| x.sin());");
    let res1 = apply(3.0,lin);
    let res2 = apply(3.1415926/2.0, |x| x.sin());
    println!("\nВычисляем apply для разных замыканий; -> {}, {} ", res1, res2);

    println!("\n//------------------------------------------------------------------------");
}

//=================================================================================================

const PI: f64 = 3.1415926535897932384626433832795028841971; //3.14159265358979;
fn create_vec_f64 (step:f64, sz:i32) -> Vec<f64> {
    let mut vec: Vec<f64> = vec!();
    let mut i = 0;
    while i <= sz {
        vec.push(PI*(i as f64)/step);
        i += 1;
    }
    vec
}

fn create_vec_arg_f64 (step:f64, sz:i32) -> Vec<f64> {
    let mut vec: Vec<f64> = vec!();
    let mut i = 0;
    while i <= sz {
        vec.push((i as f64)*step);
        i += 1;
    }
    vec
}

//--- Вестора, генератор, замыкания: Основы Rust
//--- https://highload.today/rust-zamykaniya-closures/\n");

pub fn vector_generator_and_closure () {
    println!("\n<<< md_closure: Векторы, генератор, замыкания: Основы Rust >>>");
    println!("https://highload.today/rust-zamykaniya-closures/\n");

    //--- генерируем вектор из f64 в количестве 16
    let vec: Vec<f64> = create_vec_f64 (12.0, 12);

    //--- округляем до неск знаков после запятой с целью распечатать
    let vec_show: Vec<f64> = vec.iter().map(|x| ((x*10000000.0).round())/10000000.0).collect();
    println!("Для исходного вектора-аргументов: {:#?}", vec_show);

    //--- вычисляем новый вектор sine из вектора аргументов vec
    println!("\nС помощью: let sine: Vec<f64> = vec.iter().map(|x| ((x.sin()*10000.0).round())/10000.0).collect();");
    println!("\nСоберем вектор из f64 значений округлив их и выровняв для показа");
    let sine: Vec<f64> = vec.iter().map(|x| ((x.sin()*10000.0).round())/10000.0).collect();

    println!("И распечатаем его: {:#?}", sine);
}

//=================================================================================================

use std::fmt; //--- Импортируем модуль `fmt`.

//--- Определяем структуру с именем `ArgFun`, которая хранит в себе аргументы и функции
#[derive(Debug)]
struct ArgFun{
    arg: f64,
    fun: f64
}
#[derive(Debug)]
struct ListXY {
    vec: Vec<ArgFun>,
}

///--- Создает вектор пар значений аргументов и функций
fn create_vec_argfun_f64 (step:f64, sz:i32) -> Vec<ArgFun> {
    let mut argfun: Vec<ArgFun> = vec![];
    let mut i = 0;
    while i <= sz {
        let ag = PI*(i as f64)/step;
        let fu = (ag.sin()*10000000.0).round()/10000000.0;
        let st_argfun = ArgFun {arg: ag, fun: fu};
        argfun.push(st_argfun);
        i += 1;
    }
    argfun
}
///--- Trait для отрисовки результата генерации 2-х векторов
///--- аргументов и их функций таблицей по столбцам
impl fmt::Display for ListXY {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Записываем первый элемент в предоставленный выходной поток: `f`.
        // `fmt::Result` показывает выполнилась операция успешно или нет.
        writeln!(f, "{}{}{}{}{}", "+","----------","+","-------------","+");
        writeln!(f, "|  {}    |  {}      |", "Arg:", "Func:");
        writeln!(f, "{}{}{}{}{}", "+","----------","+","-------------","+");
        let v_iter = self.vec.iter();
        for v in v_iter {
            write!(f, "|  {:-0.4}  |", v.arg);
            write!(f, "  {:-0.7}  |\n", v.fun);
        }
        writeln!(f, "{}{}{}{}{}", "+","----------","+","-------------","+");

        Ok(())
    }
}

pub fn sin_table() {
    println!("\n<<< rust-by-example: \
              Генератор, вектор структуры, трейт-форматтер, замыкания: Основы Rust >>>");
    println!("https://doc.rust-lang.ru/stable/rust-by-example/std/vec.html\n");

    let mut argfun: Vec<ArgFun>;
    let mut list: ListXY;

    //--- генерируем структуру векторов аргументов и функций f64 в количестве 12
    argfun = create_vec_argfun_f64 (12.0, 12);
    list = ListXY {vec: argfun};

    println!("{}", list);
}