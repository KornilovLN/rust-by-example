use log::{info, trace, warn};

use std::time::{Duration, Instant};

use std::fmt;

extern crate ansi_term;
use ansi_term::Colour;

extern crate sort;
use sort::bubble_sort;
use sort::quicksort;
use sort::selection_sort;
use sort::heapsort;
use sort::insertion_sort;
use sort::merge_sort;
use sort::introsort;

use crate::md_utils;


const MAX_EL_OUT: i32 = 24;
const DATA_SRC: [u32; 64]=[ 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
    15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
    15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
    15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

pub fn sort_time(method: &str) {
    let mut data = DATA_SRC.clone();

    let start = Instant::now();

    match method {
        "Bubble"    => {print!("{}: ",method); bubble_sort(&mut data);      trace!("trc: Bubble");}
        "Quicksort" => {print!("{}: ",method); quicksort(&mut data);        trace!("trc: Quicksort");},
        "Selection" => {print!("{}: ",method); selection_sort(&mut data);   trace!("trc: Selection");},
        "Heapsort"  => {print!("{}: ",method); heapsort(&mut data);         trace!("trc: Heapsort");},
        "Insertion" => {print!("{}: ",method); insertion_sort(&mut data);   trace!("trc: Insertion");},
        "Merge"     => {print!("{}: ",method); merge_sort(&mut data);       trace!("trc: Merge");},
        "Introsort" => {print!("{}: ",method); introsort(&mut data);        trace!("trc: Introsort");},
        _ => todo!()
    }

    let duration = start.elapsed();
    println!("Time elapsed is: {}", Colour::Yellow.paint(format!("{:?}", duration )));

    println!("MAXIMUM ELEMENTS OUT == {}", MAX_EL_OUT);
    md_utils::out_arr(DATA_SRC, "source: ");
    md_utils::out_arr(data, "result: ");

    println!();

}

//--- Сортировка массива разными методами
//--- https://doc.rust-lang.ru/rust-cookbook/algorithms/sorting.html

pub fn test_all_sort_methods(){
    println!("\n<<< md_sort_benchmark: Сортировка массива разными методами >>>");
    println!("https://doc.rust-lang.ru/rust-cookbook/algorithms/sorting.html");

    let frm_title=format!("{}",
                         Colour::Yellow
                        .paint("<<< Сортировка массива разными методами >>> \n".to_string()));
    println!("{}", frm_title);
    sort_time("Bubble");
    sort_time("Quicksort");
    sort_time("Selection");
    sort_time("Heapsort");
    sort_time("Insertion");
    sort_time("Merge");
    sort_time("Introsort");
}