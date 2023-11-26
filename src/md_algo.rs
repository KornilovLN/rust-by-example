use std::io;
use log::{info, trace, warn};

use std::time::{Duration, Instant};

use std::fmt;
use std::string;
extern crate ansi_term;
use ansi_term::Colour;

use crate::md_utils;

const PI_64:  f64 = 3.1415926535897932384626433832795028841971;
const APPROX_LEN: [u64; 10]=[1, 10, 100, 1000, 10000, 100000, 1000000,
                             10000000, 100000000,  1000000000];

fn approximate_pi(n: u64) -> f64 {
    let step = 1.0 / n as f64;
    let mut res = 0.0;
    for i in 0..n {
        let x = (i as f64 + 0.5) * step;
        res += 4.0 / (1.0 + x * x);
    }    
    step * res
}    

//-----------------------------------------------------------------------------------------
//--- Вычисление числа PI простой аппроксимацией с разной точностью
//--- Самопал - переписал из теста питона для сравнения
//-----------------------------------------------------------------------------------------
pub fn test_approx_pi_value() {
    println!("\n<<< Вычисление числа PI простой аппроксимацией с разной точностью >>>");

    let mut start = Instant::now();

    let mut cnt: usize = 0;
    for i in APPROX_LEN {
        start = Instant::now();       
        let pi = approximate_pi(i);
        let duration = start.elapsed();

        println!("\ntest: {cnt}");
        println!("N approximation = {i}");
        println!("PI = {}", pi);
        println!("Time elapsed is: {}", Colour::Yellow.paint(format!("{:?}", duration )));
        println!();

        cnt += 1;
    }    
        
    println!("End of test");     
}

//-----------------------------------------------------------------------------------------
//--- Найти наилучшее приближение числа PI рациональной дробью
//--- понадобится число PI в float формате
//---
//-----------------------------------------------------------------------------------------
pub fn test_approx_pi_ratio() {
    println!("\n<<< Найти наилучшее приближение числа PI рациональной дробью >>>");

    let mut start = Instant::now();

    let mut chislitel: i64 = 3;
    let mut znamenatel: i64 = 1;

    let mut err_old = 1000.0_f64;
    let mut err_pi = 1.0_f64;

    let mut stop = false;
    while !stop {
    
        //--- вычисляем следующий числитель с пом знаменателя и константы PI_64
        chislitel = (znamenatel as f64 * PI_64).round() as i64;

        //--- целочисленное деление - дает частное и остаток
        let drb = chislitel / znamenatel;
        let ost = chislitel % znamenatel; 

        //--- вычисляем отн ошибку в процентах
        let pi: f64 =    chislitel as f64 / znamenatel as f64;
        err_pi = 100.0_f64 * (pi - PI_64) / PI_64;

        //--- следим за ошибкой: печатаем только когда ошибка уменьшается 
        if err_pi.abs() < err_old.abs() {
            println!("{:10}:{:9} -> (drb,ost): {:1}:{:9}  =>  {:.20}    Err_PI: {:+.35}", 
                    chislitel, znamenatel, drb, ost, pi, err_pi);
            err_old = err_pi;
        }

        if err_pi.abs()  <= 0.000000000000001_f64 {
            stop = true;
        }else{
            znamenatel += 1;
        } 
    }
    
    let duration = start.elapsed();
    println!("\nКонец цикла: Аппроксимации числа ПИ рациональной дробью");
    println!("Time elapsed is: {}", Colour::Yellow.paint(format!("{:?}", duration )));

}