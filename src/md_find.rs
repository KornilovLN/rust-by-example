use std::time::{Duration, Instant};

extern crate ansi_term;
use ansi_term::Colour;

use crate::md_utils;

const FIND_SAMPLE: u32 = 10;
const DATA_SRC: [u32; 64]=[ 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
    15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
    15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
    15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

//--- Возвращает в векторе vec позицию элемента FIND_SAMPLE через Option
fn simple_search(vec: &[u32], value: &u32) -> Option<usize> {
    let mut i: usize = 0;

    for elm in vec {
        if elm == value {
            return Some(i);
        }
        i += 1;
    }
    None
}

//--- находит индексы всех вхождений FIND_SAMPLE и запоминает индексы в вектор
fn all_search(vec: &[u32], value: &u32) -> Vec<usize> {
    let mut i: usize = 0;
    let mut vec_index: Vec<usize> = vec![];

    for elm in vec {
        if elm == value {
            vec_index.push(i);
        }
        i += 1;
    }
    vec_index
}

//--- Поиск в массиве первого вхождения числа
//--- Самопальный алгоритм
pub fn find_time() {
    println!("\n<<< md_find: Поиск в массиве первого вхождения числа >>>");
    println!("Самопальный алгоритм");

    let frm_title = format!("{}",
        Colour::Yellow.paint("\n<<< Testing simple find algoritm >>>\n".to_string()) );
    println!("{}", frm_title);

    let data = DATA_SRC.clone();

    let start = Instant::now();

    md_utils::out_arr(data, "source: ");

    let val: u32 = FIND_SAMPLE;
    match simple_search(&data, &val) {
        None => println!("{} Не найдено!", val),
        Some(ind) => println!("Индекс найденного элемента '{}': i={}", val, ind)
    }

    let duration = start.elapsed();
    println!("Time elapsed is: {}", Colour::Yellow.paint(format!("{:?}", duration )));
    println!();
}

//--- Поиск в массиве всех вхождений числа
//--- Самопальный алгоритм
pub fn find_all_time() {
    println!("\n<<< md_find: Поиск в массиве всех вхождений числа >>>");
    println!("Самопальный алгоритм");

    let frm_title = format!("{}",
                            Colour::Yellow.paint("\n<<< Testing find all indexes >>>\n".to_string()) );
    println!("{}", frm_title);

    //--- Клонируем, ибо DATA_SRC еще понадобится
    let data = DATA_SRC.clone();

    //--- Включаем секундомер
    let start = Instant::now();

    //--- В этом массиве ищем
    md_utils::out_arr(data, "source: ");

    //--- вот этот образец
    let val: u32 = FIND_SAMPLE;
    //const len_data: usize = DATA_SRC.len();

    //--- Ищем элементы и запоминаем их индексы в вектор
    let res_indexes = all_search(&data, &val);

    //--- Закончили работу - засекли время
    let duration = start.elapsed();

    //--- Вывели на печать
    let len_indexes = res_indexes.len();
    let mut i = 0;
    for el in 0..len_indexes {
        println!("Индекс элемента'{}': i={}", val, res_indexes[i]);
        i += 1;
    }

    println!("Time elapsed is: {}", Colour::Yellow.paint(format!("{:?}", duration )));
    println!();
}