use std::{fs::File, io::{self, Read}};

struct PuzzleResult {
    result: String,
}

impl PuzzleResult {
    fn new(initial: &str) -> PuzzleResult {
        let mut s_result = String::from(initial);

        fn1(&mut s_result);
        s_result = fn2(s_result.clone());
        

        let mut m_result = s_result.clone();
        let i_variable = 42;
        
        m_result.push_str(&i_variable.to_string());
        
        let parsed_i_variable: i32 = m_result.chars().filter(|c| c.is_digit(10)).collect::<String>().parse().expect("Failed to parse integer");
        m_result = (parsed_i_variable * 3).to_string();
        let a_variable = 10;

        m_result.push_str(&a_variable.to_string());

        let parsed_a_variable: i32 = m_result.parse().expect("Failed to parse integer");
        m_result = (parsed_a_variable - a_variable).to_string();

        PuzzleResult { result: m_result }
    }


    fn validate_answer(&self, user_input: &str) -> bool {
        user_input.trim() == self.result
    }


}

fn main() {
    let puzzle_result = PuzzleResult::new("Rust");



    
    println!("Enter your answer: ");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    
    if puzzle_result.validate_answer(&user_input) {
        match File::open("flag.txt") {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Failed to read flag.txt");
                println!("\n{}", contents);
            }
            Err(_) => println!("Failed to open flag.txt"),
        }
    } else {
        println!("Sorry, the answer is incorrect. Try again!");
    }
}

fn fn1(s: &mut String) {
    let reversed: String = s.chars().rev().enumerate().map(|(i, c)| if i % 2 == 0 { c.to_uppercase().next().unwrap() } else { c }).collect();
    s.clear();
    s.push_str(&reversed);
}

fn fn2(s: String) -> String {
    let mut encoded: Vec<char> = s.chars().map(|c| {
        match c {
            'a'..='z' => ((c as u8 - b'a' + 13) % 26 + b'a') as char,
            'A'..='Z' => ((c as u8 - b'A' + 13) % 26 + b'A') as char,
            '0'..='9' => ((c as u8 - b'0' + 5) % 10 + b'0') as char,
            _ => c,
        }
    }).collect();
    encoded.reverse();
    encoded.into_iter().collect()
}
