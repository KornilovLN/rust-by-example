//================================================================================================

//--- Литералы и операторы
//--- https://doc.rust-lang.ru/stable/rust-by-example/primitives/literals.html

pub fn test_literals() {
    println!("\n<<< md_structures: Литералы и операторы >>>");
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

//================================================================================================

//--- Rust By Example -> С-подобные enum
//--- https://doc.rust-lang.ru/stable/rust-by-example/custom_types/enum/c_like.html

// enum с неявным дискриминатором (начинается с 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum с явным дискриминатором
enum Clr {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

// Функция, которая получает на вход `WebEvent` и ничего не возвращает.
pub fn out_test_enum_c() {
    println!("\n<<< md_structures: out_test_enum_C() >>>");
    println!("https://doc.rust-lang.ru/stable/rust-by-example/custom_types/enum/c_like.html\n");

    // `enums` может быть преобразован в целочисленное значение.
    println!("нулевой элемент {}", Number::Zero as i32);
    println!("первый элемент {}", Number::One as i32);
    println!("второй элемент {}", Number::Two as i32);

    println!("красный цвет #{:06x}", Clr::Red as i32);
    println!("голубой цвет #{:06x}", Clr::Blue as i32);
    println!("Зеленый цвет #{:06x}", Clr::Green as i32);
}

//================================================================================================

//--- Rust By Example -> enum
//--- https://doc.rust-lang.ru/stable/rust-by-example/custom_types/enum.html

// Создаём `enum` для классификации web-событий. Обратите внимание,
// как имена и информация о типе определяют вариант:
// `PageLoad != PageUnload` и `KeyPress(char) != Paste(String)`.
// Все они разные и независимые.
pub enum WebEvent {
    // `enum` может быть как `unit-подобным`,
    PageLoad,
    PageUnload,
    // так и кортежной структурой,
    KeyPress(char),
    Paste(String),
    // или С-подобной структурой.
    Click { x: i64, y: i64 },
}

// Функция, которая получает на вход `WebEvent` и ничего не возвращает.
pub fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("страница загружена"),
        WebEvent::PageUnload => println!("страница выгружена"),
        // Извлечём `c` из `enum`.
        WebEvent::KeyPress(c) => println!("нажата '{}'.", c),
        WebEvent::Paste(s) => println!("нажата \"{}\".", s),
        // Разберём `Click` на `x` и `y`.
        WebEvent::Click { x, y } => {
            println!("кликнуто на x={}, y={}.", x, y);
        }
    }
}

pub fn test_enum_webevent() {
    println!("\n<<< md_structures: test_enum_webevent() >>>");
    println!("https://doc.rust-lang.ru/stable/rust-by-example/custom_types/enum.html\n");

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` создаст `String` из строкового среза.
    let pasted = WebEvent::Paste("мой текст".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

//================================================================================================

//--- Rust By Example -> Связный список List
//--- https://doc.rust-lang.ru/stable/rust-by-example/custom_types/enum/testcase_linked_list.html

use EList::*;
enum EList {
    // Cons: Кортежная структура, которая хранит
    // (элемент, и указатель на следующий узел)
    Cons(u32, Box<EList>),
    // Nil: Узел, обозначающий конец связанного списка
    Nil,
}

// Методы могут быть присоединены к перечислению
impl EList {
    // Создаём пустой список
    fn new() -> EList {
        // `Nil` имеет тип `List`
        Nil
    }

    // Функция, которая принимает список и возвращает тот же список,
    // но с новым элементом в начале
    fn prepend(self, elem: u32) -> EList {
        // `Cons` также имеет тип `eList`
        Cons(elem, Box::new(self))
    }

    // Возвращаем длину списка
    fn len(&self) -> u32 {
        // `self` должен быть сопоставлен (проверен на соответствие),
        // поскольку поведение этого метода зависит от варианта `self`
        // `self` имеет тип `&eList`, а `*self` имеет тип `eList`, сопоставление на
        // конкретном типе `T` предпочтительнее, чем сопоставление по ссылке `&T`
        match *self {
            // Мы не можем завладеть `tail`, т.к. `self` заимствован;
            // вместо этого возьмём ссылку на `tail`
            Cons(_, ref tail) => 1 + tail.len(),
            // Базовый случай: Пустой список имеет нулевую длину
            Nil => 0,
        }
    }

    // Возвращаем представление списка в виде (размещённой в куче) строки
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` похож на `print!`, но возвращает строку
                // размещённую в куче, вместо вывода на консоль
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

pub fn test_elist() {
    println!("\n<<< md_structures: test_elist() >>>");
    println!("https://doc.rust-lang.ru/stable/rust-by-example/custom_types/enum/testcase_linked_list.html\n");

    // Создаём пустой связанный список
    let mut list = EList::new();

    // Присоединяем несколько элементов
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Отображаем окончательное состояние списка
    println!("размер связанного списка: {}", list.len());
    println!("{}", list.stringify());
}
