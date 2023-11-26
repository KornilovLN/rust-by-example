extern crate rand;

use rand::{thread_rng, Rng};
use rand::distributions::{Distribution, Uniform};
use rand_distr::{Normal, NormalError};
use rand::distributions::Alphanumeric;

extern crate sort;
use sort::quicksort;

//use crate::md_utils;

//================================================================================================

//--- простое генерирование случайных чисел
//--- https://doc.rust-lang.ru/rust-cookbook/algorithms/randomness.html
pub fn simple_rand() {
    println!("\n<<< md_random: Простое генерирование случайных чисел >>>");
    println!("https://doc.rust-lang.ru/rust-cookbook/algorithms/randomness.html\n");

    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

//================================================================================================

//--- Генерация случайных чисел из диапазона
//--- https://doc.rust-lang.ru/rust-cookbook/algorithms/randomness.html
pub fn range_rand() {
    println!("\n<<< md_random: Генерация случайных чисел из диапазона >>>");
    println!("https://doc.rust-lang.ru/rust-cookbook/algorithms/randomness.html\n");

    let mut rng = rand::thread_rng();

    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));
}

//================================================================================================
const EN_PRINT: bool = false;

//--- печать строки нулями длиной val
fn print_line(val:i32) {
    for i in 0..val {
        print!("O");
    }
    println!();
}

//--- печать массива всех пришедших сл чисел по порядку
fn print_arr(mut arr:&[i32; 101]){
    let mut cnt = 0;
    println!();
    for i in 0..101{
            if cnt % 8 == 0 {
                println!("arr[{}] = {}\t", i, arr[i]);
            }else{
                print!("arr[{}] = {}\t", i, arr[i]);
            }

            cnt += 1;
    }
}

fn print_arr_f32(mut arr:&[f32; 101]){
    let mut cnt = 0;
    println!();
    for i in 0..101{
        if cnt % 8 == 0 {
            println!("arr[{}] = {}\t", i, arr[i]);
        }else{
            print!("arr[{}] = {}\t", i, arr[i]);
        }

        cnt += 1;
    }
}

//--- Печать гистограммы
fn print_arr_line(mut arr:&[i32; 101]){
    println!();
    for i in 0..101{
        print!("{:3} ", i);
        print_line(arr[i]);
    }
}

fn print_arr_line10(mut arr:&[i32; 10]){
    println!();
    for i in 0..10{
        print!("{:3} ", i);
        print_line(arr[i]);
    }
}

//--- Генерация случайных чисел равномерное распределение
//--- https://doc.rust-lang.ru/rust-cookbook/algorithms/randomness.html
pub fn distrib_rand() {
    println!("\n<<< md_random: Генерация Uniform задаёт равномерное распределение >>>");
    println!("https://doc.rust-lang.ru/rust-cookbook/algorithms/randomness.html\n");

    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..101);

    let mut allcnt = 0;
    let mut cnt = 0;
    let mut arr: [i32; 101] = [0; 101];
    loop {
        //--- Получили новое случайное число throw
        let throw = die.sample(&mut rng);

        if EN_PRINT {
            //--- распечатали его (печать в строке не более cnt чисел throw)
            if cnt < 15 {
                print!("{}\t", throw);
                cnt += 1;
            } else {
                println!("{}\t", throw);
                cnt = 0;
            }
        }

        //--- нарастили соответствующий массив количеством пришедших throw
        arr[throw] += 1;

        //--- а это просто счетчик количества опытов получения throw
        allcnt += 1;
        //--- он и остановит работу генерации сл чисел
        if allcnt == 16000 {
            break;
        }
    }

    //--- распечатаем массив как обычно - по порядку и как гистограмму
    println!();
    print_arr(&arr);
    println!();
    print_arr_line(&arr);
}

//================================================================================================

fn quick_sorting_arr(mut arr:&[f32; 101]) -> [f32; 101] {
    let mut data = arr.clone();
    quicksort(&mut data);
    data
}

fn min_arr(arr:&[f32; 101]) -> f32 {
    let mut minel: f32 = 1000000.0;
    for i in 0..101 {
        if minel > arr[i] {
            minel = arr[i];
        }
    }
    minel
}

fn shift_arr_to_positive(mut arr:[f32; 101], shift: f32) -> [f32; 101] {
    for i in 0..101 {
        arr[i] += shift;
    }
    arr
}

fn mul_arr_round(mut arr:[f32; 101], mulk: f32) -> [i32; 101] {
    let mut arri32 = [0i32; 101];
    for i in 0..101 {
        arr[i] *= mulk;
        arr[i] = arr[i].round();
        arri32[i] = arr[i] as i32;
    }
    arri32
}

fn raspredelenie(arr:[i32; 101], diapason: i32) -> [i32; 10] {
    let mut arr10 = [0i32; 10];
    let mut diap10 = [0i32; 10];
    let delta: i32 = diapason / 10;
    for i in 0..101 {
        match arr[i] {
            0..=15 => arr10[0] += 1,
            15..=30 => arr10[1] += 1,
            30..=45 => arr10[2] += 1,
            45..=60 => arr10[3] += 1,
            60..=75 => arr10[4] += 1,
            75..=90 => arr10[5] += 1,
            90..=105 => arr10[6] += 1,
            105..=120 => arr10[7] += 1,
            120..=135 => arr10[8] += 1,
            135..=150 => arr10[9] += 1,
            _ => ()
        }
    }
    arr10
}

//--- Генерация случайных чисел нормальное распределение
//--- https://doc.rust-lang.ru/rust-cookbook/algorithms/randomness.html

pub fn normal_rand() -> Result<(), NormalError>  {
    println!("\n<<< md_random: Генерация Uniform задаёт нормальное распределение >>>");
    println!("https://doc.rust-lang.ru/rust-cookbook/algorithms/randomness.html");

    let mut rng = rand::thread_rng();
    let normal = Normal::new(2.0, 3.0)?;


    let mut arr: [f32; 101] = [0.0; 101];
    let mut allcnt = 0;
    let mut cnt = 0;
    loop {
        //--- Получили новое случайное число throw
        let throw = normal.sample(&mut rng);

        if EN_PRINT {
            //--- распечатали его (печать в строке не более cnt чисел throw)
            if cnt < 3 {
                print!("{}\t", throw);
                cnt += 1;
            } else {
                println!("{}\t", throw);
                cnt = 0;
            }
        }

        arr[allcnt] = throw;

        //--- а это просто счетчик количества опытов получения throw
        allcnt += 1;
        //--- он и остановит работу генерации сл чисел
        if allcnt == 100 {
            break;
        }
    }

    let arr_sort = quick_sorting_arr(&arr);
    println!("\nНесортированный массив случайных чисел нормального распределения: ");
    print_arr_f32(&arr);
    println!("\nОтсортированный массив: ");
    print_arr_f32(&arr_sort);
    let min_val: f32 = min_arr(&arr_sort);
    println!("\nMin element = {}",min_val);
    let abs_shift = min_val.abs();
    let shift_arr = shift_arr_to_positive(arr_sort, abs_shift);
    println!("\nShifted array");
    print_arr_f32(&shift_arr);
    println!("\nMult array");
    let mult_arr = mul_arr_round(shift_arr, 10.0);
    let diapason: i32 = mult_arr[100];
    print_arr(&mult_arr);
    println!("\nDiapason: {}", diapason);
    let array = raspredelenie(mult_arr, diapason);
    print_arr_line10(&array);
    println!("");
    Ok(())
}


//--- Генерация случайных паролей из ряда символов
//--- https://doc.rust-lang.ru/rust-cookbook/algorithms/randomness.html

fn create_pwd_rand(quantity: usize) -> String {
    let mut rng = thread_rng();
    let chars: String = (0..quantity)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();
    chars
}

pub fn show_pwd_rand(quantity: usize) {
    println!("<<<  md_random: Создать случайный пароль из {} разрешенных символов >>>", quantity);
    println!("https://doc.rust-lang.ru/rust-cookbook/algorithms/randomness.html\n");

    let pwd = create_pwd_rand(quantity);
    println!("pwd:\t{}\n", pwd);
}


//--- Генерация случайных паролей из заданного множества символов
//--- https://doc.rust-lang.ru/rust-cookbook/algorithms/randomness.html

fn create_pwd_by_set_rand(quantity: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

    let charset_len: usize = CHARSET.len();
    let mut rng = rand::thread_rng();

    let password: String = (0..quantity)
        .map(|_| {
            let idx = rng.gen_range(0.. charset_len);
            CHARSET[idx] as char
        })
        .collect();

    password
}

pub fn show_pwd_by_set_rand(quantity: usize) {
    println!("<<<  md_random: Генерация случайных паролей из заданного множества символов >>>");
    println!("https://doc.rust-lang.ru/rust-cookbook/algorithms/randomness.html\n");

    let pwd = create_pwd_by_set_rand(quantity);
    println!("pwd:\t{}\n", pwd);
}

