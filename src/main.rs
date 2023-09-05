//use std::fmt; // Импортируем `fmt`
//================================================================================================


//--- Литералы и операторы
//--- https://doc.rust-lang.ru/stable/rust-by-example/primitives/literals.html

fn test_literals() {

	println!("\n<<< Литералы и операторы >>>");
	println!("https://doc.rust-lang.ru/stable/rust-by-example/primitives/literals.html\n");

	println!("Работа по тексту туториала:\n");
	
	// Целочисленное сложение
    println!("1 + 2 = {}", 1u32 + 2);

    // Целочисленное вычитание
    println!("1 - 2 = {}", 1i32 - 2);
    // ЗАДАНИЕ ^ Попробуйте изменить `1i32` на `1u32`
    // чтобы убедится насколько важен тип данных

    // Булева логика
    println!("true И false будет {}", true && false);
    println!("true ИЛИ false будет {}", true || false);
    println!("НЕ true будет {}", !true);

    // Побитовые операции
    println!("0011 И 0101 будет {:04b}", 0b0011u32 & 0b0101);
    println!("0011 ИЛИ 0101 будет {:04b}", 0b0011u32 | 0b0101);
    println!("0011 исключающее ИЛИ 0101 будет {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 будет {}", 1u32 << 5);
    println!("0x80 >> 2 будет 0x{:x}", 0x80u32 >> 2);

    // Использование подчёркивания для улучшения читаемости!
    println!("Один миллион записан как {}", 1_000_000u32);   
}    

//--- форматирование подробнее
//--- https://doc.rust-lang.ru/stable/rust-by-example/hello/print/fmt.html

use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // Широта
    lat: f32,
    // Долгота
    lon: f32,
}

impl Display for City {
    // `f` — это буфер, данный метод должен записать в него форматированную строку
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` похож на `format!`, только он запишет форматированную строку
        // в буфер (первый аргумент функции)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    // `f` — это буфер, данный метод должен записать в него форматированную строку
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {

		let r: u32 = self.red as u32;
		let g: u32 = self.green as u32;
		let b: u32 = self.blue as u32;
		
		let cl:u32 = (r*256 + g)*256 + b;
        write!(f, "RGB ({:3},{:3},{:3})  0x{:x}", r, g, b, cl)
    }
}

fn test_fmt_detail() {

	println!("\n<<< Форматирование подробнее >>>");
	println!("https://doc.rust-lang.ru/stable/rust-by-example/hello/print/fmt.html\n");

	println!("Работа по тексту туториала:\n");
	
	for city in [
        City { name: "Дублин", lat: 53.347778, lon: -6.259722 },
        City { name: "Осло", lat: 59.95, lon: 10.75 },
        City { name: "Ванкувер", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Поменяйте {:?} на {}, когда добавите реализацию
        // типажа fmt::Display
        println!("{:?}", *color)
    } 
    
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color)
    }     
}
//================================================================================================

//--- форматированный вывод list с помощью display
//--- https://doc.rust-lang.ru/stable/rust-by-example/hello/print/print_display/testcase_list.html

// Определяем структуру с именем `List`, которая хранит в себе `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Получаем значение с помощью индекса кортежа
        // и создаём ссылку на `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Пройдёмся по каждому `v` в `vec`.
        // Номер итерации хранится в `count`.
        for (count, v) in vec.iter().enumerate() {
            // Для каждого элемента, кроме первого, добавим запятую
            // до вызова `write!`. Используем оператор `?` или `try!`,
            // чтобы вернуться при наличии ошибок.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // Закроем открытую скобку и вернём значение `fmt::Result`
        write!(f, "]")
    }
}

fn test_fmt_list() {

	println!("\n<<< форматированный вывод list с помощью display >>>");
	println!("https://doc.rust-lang.ru/stable/rust-by-example/hello/print/print_display/testcase_list.html\n");

	println!("Работа по тексту туториала:\n");
	
	//--- так
    let v = List(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    println!("{}", v);

	//--- или так
    println!("{}", List(vec![4, 6, 7, 2, 5, 8]));    
}
//================================================================================================

//--- форматированный вывод с помощью display
//--- https://doc.rust-lang.ru/stable/rust-by-example/hello/print/print_display.html

// Структура, которая хранит в себе два числа.
// Вывод типажа `Debug` добавлен для сравнения с `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Реализуем `Display` для `MinMax`.
impl fmt::Display for MinMax {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	// Используем `self.номер`, чтобы получить доступ к каждому полю структуры.
    	write!(f, "({}, {})", self.0, self.1)
    }	
}

// Объявим структуру с именованными полями, для сравнения
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// По аналогии, реализуем `Display` для Point2D
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Обращаться к полям структуры Point2D будет по имени
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// Объявим структуру Complex
#[derive(Debug)]
struct Complex {
    re: f64,
    im: f64,
}

// По аналогии, реализуем `Display` для Complex
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Обращаться к полям структуры Complex будет по имени
        write!(f, "{} + {}i", self.re, self.im)
    }
}

fn test_fmt_display() {

	println!("\n<<< форматированный вывод с помощью display >>>");
	println!("https://doc.rust-lang.ru/stable/rust-by-example/hello/print/print_display.html\n");

	println!("Работа по тексту туториала:\n");

    let minmax = MinMax(0, 14);

    println!("Сравниваем структуры:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("Большой диапазон - {big} и маленький диапазон {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Сравниваем точки:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    
    let cmpl = Complex { re: 1.3, im: 2.7 };
    
    println!("Сравниваем вывод комплексных чисел:");
    println!("Display: {}", cmpl);
    println!("Debug: {:#?}", cmpl);    

    // Ошибка. Типажи `Debug` и `Display` были реализованы, но `{:b}`
    // необходима реализация `fmt::Binary`. Следующий код не сработает.
    // println!("Как выглядит Point2D в виде двоичного кода: {:b}?", point);
}
//================================================================================================

//--- форматированный вывод с помощью debug
//--- https://doc.rust-lang.ru/stable/rust-by-example/hello/print/print_debug.html
fn test_fmt_debug() {
	println!("\n<<< форматированный вывод с помощью debug >>>");
	println!("https://doc.rust-lang.ru/stable/rust-by-example/hello/print/print_debug.html\n");


    println!("Атрибут `derive` автоматически реализует");
    println!("необходимые методы, чтобы была возможность");
    println!("напечатать структуру `struct` с помощью `fmt::Debug`.");
    println!("#[derive(Debug)]");
    println!("struct DebugPrintable(i32);.\n");

	println!("Работа по тексту туториала:\n");
	
    // `{}` автоматически будет заменено на аргументы. Они будут преобразованы в строку.
    println!("{} дней", 31);

	// Вывод и реализация `fmt::Debug` для `Structure`.
	// `Structure` - это структура, которая содержит в себе один `i32`.
	#[derive(Debug)]
	struct Structure(i32);

	// Добавим структуру `Structure` в структуру `Deep`.
	// Реализуем для `Deep` возможность вывода с помощью fmt::Debug`.
	#[derive(Debug)]
	struct Deep(Structure);

    // Вывод с помощью `{:?}` аналогичен `{}`.
    println!("{:?} месяцев в году.", 12);
    println!("{1:?} {0:?} - это имя {actor:?}.",
             "Слэйтер",
             "Кристиан",
             actor="актёра");

    // `Structure` теперь можно напечатать!
    println!("Теперь {:?} будет выведена на экран!", Structure(3));

    println!("Проблема с `выводом (derive)`, в том, что у нас не будет контроля");
    println!("над тем, как будет выглядеть результат.");
    println!("Что, если мы хотим напечатать просто `7`?");
    println!("А теперь напечатаем {:?}", Deep(Structure(7)));
    println!("Не получилось");
    
    // Добавим структуру `Person`.
    #[derive(Debug)]
	struct Person<'a> {
    name: &'a str,
    fam: &'a str,
    job: &'a str,
    age: u8
    }
    
    let name = "Peter";
    let fam = "Petrov";
    let job = "programmer";
    let age = 27;
    let peter = Person { name, fam, job, age };

    // Красивый  print структуры
    println!("{:#?}", peter);

}
//================================================================================================

//--- простой форматированный вывод
//--- https://doc.rust-lang.ru/stable/rust-by-example/hello/print.html
fn test_fmt_out() {
	println!("\n<<< простой форматированный вывод >>>");
	println!("https://doc.rust-lang.ru/stable/rust-by-example/hello/print.html\n");


    println!("format!   записывает форматированный текст в String.");
    println!("print!    работает аналогично с format!, но текст выводится в консоль (io::stdout).");
    println!("println!  аналогично print!, но в конце добавляется переход на новую строку.");
    println!("eprint!   аналогично format!, но текст выводится в стандартный поток ошибок (io::stderr).");
    println!("eprintln! аналогично eprint!, но в конце добавляется переход на новую строку.\n");

	println!("Работа по тексту туториала:\n");
	
    // `{}` автоматически будет заменено на аргументы. Они будут преобразованы в строку.
    println!("{} дней", 31);

    // Без суффиксов, 31 является i32. Можно изменить тип 31, используя суффикс.

    // Существует множество способов работы с форматированным выводом. Можно указать
    // позицию для каждого аргумента.
    println!("{0}, это {1}. {1}, это {0}", "Алиса", "Боб");

    // Так же можно именовать аргументы.
    println!("{subject} {verb} {object}",
             object="ленивую собаку",
             subject="Быстрая коричневая лиса",
             verb="прыгает через");

    println!("{} из {} людей знают, что такое 10-ричный код, а остальные нет.", 17, 10);
    println!("{:o} из {:o} людей знают, что такое 8-ричный код, а остальные нет.", 17, 8);
    println!("{:b} из {:b} людей знают, что такое 2-ичный код, а остальные нет.", 17, 2);
    println!("{:x} из {:x} людей знают, что такое 16-ричный код, а остальные нет.", 17, 16);

    // Можно выравнивать текст, сдвигая его на указанную ширину.
    // Данный макрос отобразит в консоли
    // "     1". 5 пробелов и "1".
    println!("{number:>width$}", number=1, width=6);

    // Можно добавить к цифрам пару нулей. Данный макрос выведет "000001".
    println!("{number:0>width$}", number=1, width=6);

    // Компилятор проверит, что в макрос передано правильное количество аргументов.
    println!("Меня зовут {0}, {1} {0}", "Бонд", "Джеймс");
    // ИСПРАВЬТЕ ^ Добавьте недостающий аргумент: "Джеймс"

    // Создаём структуру, которая хранит в себе `i32`. Назовём её `Structure`.
    #[allow(dead_code)]
    struct Structure(i32);

    // Однако, пользовательские типы данных, например, как эта структура
    // требуют более сложной обработки для вывода. Данный код не будет работать.
    //println!("Эта структура `{}` не хочет выводится на экран...", Structure(3));
    // ИСПРАВЬТЕ ^ Закомментируйте эту строку.
}
//================================================================================================

//--- Rust By Example -> атрибут cfg
//--- https://doc.rust-lang.ru/stable/rust-by-example/attribute/cfg.html

// Эта функция будет скомпилирована только в том случае, если целевая ОС будет linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("func: Вы работаете в linux!");
}

// А эта функция будет скомпилирована, если целевая ОС *не* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("func: Вы работаете *не* в linux!");
}

//================================================================================================

fn title() {
	println!("\n==========================================================");
	println!("<<< Набор тестовых программ туториала: >>>");
	println!("https://doc.rust-lang.ru/stable/rust-by-example/index.html");
	println!("author: StarmarkLN (Kornilov LN)");
    println!("Github: https://github.com/KornilovLN");
    println!("e-mail: ln.KornilovStar@gmail.com");
	println!("e-mail: ln.starmark@ekatra.io");
	println!("e-mail: ln.starmark@gmail.com");
	println!("date: 24.07.2023");
	println!("==========================================================\n");
}

//================================================================================================

fn main() {

	title();

    are_you_on_linux();    
    println!("main: Вы уверены?");
    if cfg!(target_os = "linux") {
        println!("main: Да. Это точно linux!");
    } else {
        println!("main: Да. Это точно *не* linux!");
    }


    test_fmt_out();
    test_fmt_debug();
    test_fmt_display();
    test_fmt_list(); 
    test_fmt_detail();
    
	test_literals();    
    
 
}    
