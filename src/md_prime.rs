
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn find_twin_primes(start: u32, end: u32) -> Vec<(u32, u32)> {
    let mut twin_primes = Vec::new();
    for n in start..(end - 1) {
        if is_prime(n) && is_prime(n + 2) {
            twin_primes.push((n, n + 2));
        }
    }
    twin_primes
}

pub fn out_vec(dt: Vec<(u32, u32)>, prompt: &str){
    print!("{}",prompt);
    let mut cnt = 0;
    let len = dt.len();
    let mut crlf = 0;
    for el in &dt {
        print!("({:5}, {:5})  ", el.0, el.1);
        cnt += 1;
        if cnt == len {
            break;
        }
        crlf += 1;
        if crlf == 8 {
            println!();
            crlf = 0;
        }
    }
}

//--- Близкие пары простых чисел на интервале от 0 до 1000:
//--- test ChatGPT
pub fn find_twin_primes_go() {
    let start = 1;
    let end = 10000;
    let twin_primes = find_twin_primes(start, end);
    //println!("Близкие пары простых чисел на интервале от {} до {}: {:?}", start, end, twin_primes);
    println!("\n<<< md_prime: Близкие пары простых чисел на интервале от {} до {} >>>", start, end);
    out_vec(twin_primes, "");
}