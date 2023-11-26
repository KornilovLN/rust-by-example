
//--- https://spec-zone.ru/rust/std/io/fn.stdin
use std::io;

extern crate ansi_term;
use ansi_term::Colour;

use std::error::Error;
mod md_utils;
mod md_about;
mod md_concept;

mod md_format;
mod md_structures;
mod md_random;
mod md_sort;
mod md_sort_benchmark;
mod md_find;
mod md_arch;
mod md_string_ex;
mod md_closure;
mod md_template;
mod md_prime;
mod md_tables;
mod md_algo;
mod md_azbuka;
mod md_turtle;

mod md_life;


//================================================================================================

fn prepare_run() {

    //--- Информация о процессоре
    md_utils::iron();

    //--- Заголовок программы
    md_about::target("rust_by_example", "CLI: Учебно-исследовательская программа");

    md_utils::title_into_line("about.json", "-", 84);
    md_about::get_json_from_file();
    md_utils::line_char("-", 84);

    //--- Подробно о программе из файла concept
    let reading_file_result = md_concept::read_concept();
    let file_result = match reading_file_result {
        Ok(file) => file,
        Err(error) => {
            let frm_err = format!("{}: {}",
                Colour::Red.paint("Проблема открытия 'concept.txt'".to_string()),    
                Colour::Yellow.paint(error.to_string()) );
            panic!("{}",frm_err);
        },
    };
/*    fn concept_info() {
        let reading_file_result: Result<(), io::Error> = md_concept::read_concept();
        let _file_result = match reading_file_result {
            Ok(file) => file,
            Err(error) => {
                let frm_err = format!("{}: {}",
                                    Colour::Red.paint("Проблема открытия 'concept.txt'".to_string()),
                                    Colour::Yellow.paint(error.to_string())
                );
                panic!("{}",frm_err);
            },
        };
    }
*/
    md_utils::waiter(5);    //--- пауза
}

//----------------------------------------------------------------------------------------

fn edit_about(about: &mut md_about::StAbout) {    
    println!("\n\t--- Редактируем  edit_about(about: &mut md_about::StAbout) ---------------------");
    about.datetime = String::from("28.09.2023 23:46:00");
    about.firstname = String::from("Leon");
    about.secondname = String::from("Nicolaevich");
    println!("\t----------------------------------------------------------------------------------");
}

fn save_about(about: &mut md_about::StAbout) {
    println!("\n\t--- Сохраняем, для проверки читаем, парсерим и выводим about.json -----");
    md_about::save_json_about(&about);
    md_about::get_json_from_file();
    println!("\t-------------------------------------------------------------------------");
}

//=== Секция Run ==============================================================

pub fn run() -> Result<(), Box<dyn Error>> {
    //--- Создать структуру для считывания about.json в нее
    let mut about = md_about::StAbout::new("", "", "", "",
                                                    "", "", "");
    prepare_run();

    //--- Остальной код ------------------------------------------------------------------

    md_format::are_you_on_linux();

    loop {
        select_work();

        let mut work_num_str= String::new();
        io::stdin()
            .read_line(&mut work_num_str)
            .expect("Failed to read line");

        let work_num = work_num_str
            .trim()
            .parse::<i32>()
            .unwrap();

        work_num_str.clear();

        let res_menu = match work_num {
            1 => { md_utils::title_with_border("md_format: в работе");
                md_format::test_fmt_out();
                md_format::test_fmt_debug();
                md_format::test_fmt_display();
                md_format::test_fmt_list();
                md_format::test_fmt_detail();
            },
            2 => { md_utils::title_with_border("md_structures: в работе");
                md_structures::test_literals();
                md_structures::out_test_enum_c();
                md_structures::test_enum_webevent();
                md_structures::test_elist();
            },
            3 => { md_utils::title_with_border("md_random:   в   работе");
                md_random::simple_rand();
                md_random::range_rand();
                md_random::distrib_rand();
                md_random::normal_rand();
                md_random::show_pwd_rand(16);
                md_random::show_pwd_by_set_rand(32);
            },
            4 => { md_utils::title_with_border("md_sort:    в    работе");
                md_sort::sort_float_vector();
                md_sort::sort_vector_structures();
                md_sort_benchmark::test_all_sort_methods();
            },
            5 => { md_utils::title_with_border("md_find:    в    работе");
                md_find::find_time();
                md_find::find_all_time();
                md_arch::tar_pack();
            },
            6 => { md_utils::title_with_border("md_string_ex:  в работе");
                md_string_ex::string_tutor();
                md_string_ex::array_tutor();
            },
            7 => { md_utils::title_with_border("md_closure:   в  работе");
                md_closure::string_tutor();
                md_closure::vector_generator_and_closure();
                md_closure::sin_table();
            },
            8 => { md_utils::title_with_border("md_template: Other modules в работе");
                md_template::sin_table();
            },
            9 => {
                md_utils::title_with_border("md_life: Игра Life по правилам Конвея (cicle times 250)");
                md_life::go();
            },
            10 => {
                md_utils::title_with_border("md_prime: Поиск пар простых чисел на интервале 0..10000");
                md_prime::find_twin_primes_go(1000);
            },
            11 => {
                md_utils::title_with_border("md_tables: Тестирование табличного вывода структур");
                md_tables::table_test();
            },
            12 => {
                md_utils::title_with_border("md_algo: Тестирование аппроксимации числа ПИ");
                md_algo::test_approx_pi_value();
            },
            13 => {    
                md_utils::title_with_border("md_algo: Аппроксимация числа ПИ рациональной дробью");
                md_algo::test_approx_pi_ratio();
            },
            14 => {
                md_utils::title_with_border("md_azbuka: Тестирование вывода в консоль нескольких алфавитов");
                md_azbuka::azbuka_go();
            },
            15 => {
                md_utils::title_with_border("md_turtle: Тестирование черепашьей графики");
                md_turtle::simple_go();
            },

            0 => { print!("\n{}", "Конец работы");
                break;
            },
            _ => { let frmerr = format!("{} ",
                                     Colour::Red
                                    .bold()
                                    .paint("Ошибка: Нет такого варианта, повторите ввод"));
                   println!("{}", frmerr);
            },
        };

        println!();

    }

    //------------------------------------------------------------------------------------

    //--- Только для тестирования редактирования и сохранения файла
    //edit_about(&mut about);
    //save_about(&mut about);

    Ok(())
}


fn select_work() {
    println!();
    let frmfn = format!("{} ",
                            Colour::Yellow
                                .bold()
                                .paint("Предлагаемые наборы функций для тестирования"));
    println!("{}\n", frmfn);

    println!("1 =>\tmd_format::test_fmt_out();
        md_format::test_fmt_debug();
        md_format::test_fmt_display();
        md_format::test_fmt_list();
        md_format::test_fmt_detail();");
    println!("2 =>\tmd_structures::test_literals();
        md_structures::out_test_enum_c();
        md_structures::test_enum_webevent();
        md_structures::test_elist();");
    println!("3 =>\tmd_random::simple_rand();
        md_random::range_rand();
        md_random::distrib_rand();
        md_random::normal_rand();
        md_random::show_pwd_rand(16);
        md_random::show_pwd_by_set_rand(32);");
    println!("4 =>\tmd_sort::sort_float_vector();
        md_sort::sort_vector_structures();
        md_sort_benchmark::test_all_sort_methods();");
    println!("5 =>\tmd_find::find_time();
        md_find::find_all_time();
        md_arch::tar_pack();");
    println!("6 =>\tmd_string_ex::string_tutor();
        md_string_ex::array_tutor();");
    println!("7 =>\tmd_closure::string_tutor();
        md_closure::vector_generator_and_closure();
        md_closure::sin_table();");
    println!("8 =>\tmd_template::sin_table();");
    println!("9 =>\tmd_life::go();");
    println!("10=>\tmd_prime::find_twin_primes_go(quantity);");
    println!("11=>\tmd_tables::table_test();");
    println!("12=>\tmd_algo::test_approx_pi_value(); Тестирование аппроксимации числа ПИ");
    println!("13=>\tmd_algo::test_approx_pi_ratio(); Аппроксимация числа ПИ рациональной дробью");
    println!("14=>\tmd_azbuka::azbuka_go();");
    println!("15=>\tmd_turtle::simple_go();");

    println!("\n0 =>\tКонец работы");

    println!();

    let frmprompt = format!("{} ", Colour::Yellow
                                            .bold()
                                            .paint("Выберите номер функции для работы:"));
    println!("{}", frmprompt);
}
