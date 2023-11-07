//================================================================================================

//--- строки: Declaring and performing string operations
//--- Rust Cookbook
pub fn string_tutor() {
    println!("\n<<< md_string_ex: Declaring and performing string operations >>>");
    println!("Rust Cookbook\n");

    let rand_string = "I love Rust cookbook <3";

    println!("ZB: {}", rand_string);
    println!("length of the string is {}",rand_string.len() );
    // или так: Count using iterator count
    let count = rand_string.chars().count();
    print!("count {}\n",count );

    // Splits in string
    let (first,second) = rand_string.split_at(7);
    println!("First part : {0}\tSecond part : {1}",first,second);

    println!("__________________________________________________________________________________");
    // printing all chars
    let mut chars = rand_string.chars();      //--- получили итератор на chars
    let mut indiv_chars = chars.next(); //--- взяли первый char
    loop {
        match indiv_chars {                         //--- каждый char, если он есть
            Some(x) => print!("{}  ",x ),     //--- распечатали
            None => break                           //--- иначе - вышли из цикла
        }
        indiv_chars = chars.next();                 //--- взяли следующий char
    }
    println!();

    println!("__________________________________________________________________________________");
    // То же, но через пробел iterate over whitespace
    let mut iter = rand_string.split_whitespace();
    let mut indiv_word = iter.next();
    loop {
        match indiv_word {
            Some(x) => print!("{}\t",x ),
            None => break
        }
        indiv_word = iter.next();
    }
    println!();

    println!("__________________________________________________________________________________");
    // iterate over next line
    let rand_string2 = "I love \n everything about \n Rust <3";
    let mut iter_line = rand_string2.lines();
    let mut indiv_sent = iter_line.next();
    loop {
        match indiv_sent {
            Some(x) => println!("{}",x ),
            None => break
        }
        indiv_sent = iter_line.next();
    }
}

//================================================================================================

//--- массивы: Declaring arrays and using slices in Rust
//--- Rust Cookbook
pub fn array_tutor() {
    println!("\n<<< md_string_ex: Declaring arrays and using slices in Rust >>>");
    println!("Rust Cookbook\n");

    // Defining an array
    let rand_array = [1,2,3];
    println!("random array {:?}",rand_array );
    // indexing starts with 0
    println!("random array 1st element {}",rand_array[0] );
    println!("random array length {}",rand_array.len() );
    // last two elements
    println!("random array {:?}",&rand_array[1..3] );


}