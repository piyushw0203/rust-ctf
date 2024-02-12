extern crate base64;
use std::io;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::{encode, decode};

fn main() {
    let o_string = "is this the flag?";
    let e_string = encode(o_string);

    println!("{}",e_string);

    // Introduce a custom encryption function with a dynamic key
    let key = generate_key();
    let ee_string = fn4(&e_string, key);
    println!("{}",ee_string);
   
    let dd_string = fn4(&ee_string, key);
    println!("{}",dd_string);

    let d_bytes = decode(&dd_string).expect("Failed");

    let mut result = 0u64;
    for byte in d_bytes {
        result = result.wrapping_mul(256).wrapping_add(byte as u64);
    }




    // Perform operations
    let mut s_result = result;
    fn1(&mut s_result);
    s_result = fn2(s_result);
    fn3(&mut s_result, 100);
    let mut m_result = s_result;
    let i_variable = 42;
    display_time_hint();
    m_result += i_variable;
    m_result *= 3;
    let a_variable = 10;
    m_result -= a_variable;
    s_result = m_result;

   
    match fs::write("key.txt", key.to_string()) {
        Ok(_) => {}
        Err(err) => eprintln!("Error saving key to file: {}", err),
    }
   
    save_to_file("sum.txt", s_result.to_string());
    let saved_result: u64 = match fs::read_to_string("sum.txt") {
        Ok(content) => content.trim().parse().unwrap_or(0),
        Err(err) => {
            eprintln!("Error reading from file: {}", err);
            return;
        }
    };
    
    println!("Enter your guess:");


    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let user_guess: Result<u64, _> = user_input.trim().parse();

    
    match user_guess {
        Ok(guess) => {
            if guess == saved_result {
                println!("Flag Found! You guessed it!");
            } else {
                println!("Incorrect guess. Try again!");
            }
        }
        Err(_) => {
            println!("Incorrect guess. Try again!");
        }
    }
}

fn generate_key() -> u8 {
    
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs() as u8,
        Err(_) => 0, 
    }
}

fn display_time_hint() {
    
    if let Ok(time) = SystemTime::now().duration_since(UNIX_EPOCH) {
        println!("Hint: The key is based on the current time (seconds since UNIX epoch): {}", time.as_secs());
    } else {
        println!("Hint: Unable to retrieve the current time.");
    }
}
fn fn1(value: &mut u64) {
    *value *= 2;
}

fn fn2(value: u64) -> u64 {
    value / 21579
}

fn fn3(value: &mut u64, subtractor: u64) {
    *value = value.wrapping_sub(subtractor);
}

fn fn4(input: &str, key: u8) -> String {
    
    input
        .chars()
        .enumerate()
        .map(|(i, c)| (c as u8 ^ (key + i as u8)) as char)
        .collect()
}


fn save_to_file(filename: &str, content: String) {
    
    match fs::write(filename, content) {
        Ok(_) => {}
        Err(err) => eprintln!("Error saving to file {}: {}", filename, err),
    }
}
