use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::fs::{self, File};
use base64::{encode,decode};
use std::io ::{self as stdio, Read as read};
use async_std::io;



// Function allows User-Input for 10 seconds only
async fn input_for_10sec() -> Result<String, std::io::Error> {
    let input = io::timeout(Duration::from_secs(10), async {
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).await?;
        Ok(buffer)
    }).await;

    input
}
// --
#[async_std::main]
async fn main() {



    let o_string = "are we really bounded by time?";
    println!("{}", o_string);
    let e_string = encode(o_string);

    let key = generate_key();
    
    let ee_string = fn4(&e_string, key);


    let dd_string = fn4(&ee_string, key);
    

    let d_bytes = decode(&dd_string).expect("Failed");

    let mut result = 0u64;
    for byte in d_bytes {
        result = result.wrapping_mul(256).wrapping_add(byte as u64);
    }

    let num = result.to_string().len();

    save_to_file("sum.txt", result.to_string());
    let _saved_result: u64 = match fs::read_to_string("sum.txt") {
        Ok(content) => content.trim().parse().unwrap_or(0),
        Err(err) => {
            eprintln!("Error reading from file: {}", err);
            return;
        }
    };

    
    println!("Enter");


    let user_input = input_for_10sec().await;
    let user_input = match user_input {
        Ok(input_buff) => input_buff,
        Err(_) => {println!("\nPlease give your input within 10 seconds ⏳⌛\n"); return},
    }; 
    let user_guess: Result<u64, _> = user_input.trim().parse();

    
    match user_guess {
        Ok(guess) => {
            if guess == num.try_into().unwrap() {
                match File::open("flag.txt") {
                    Ok(mut file) => {
                        let mut contents = String::new();
                        file.read_to_string(&mut contents).expect("Failed to read flag.txt");
                        println!("{}", contents);
                    }
                    Err(_) => println!("Failed to open flag.txt"),
                }
            }
            
             else {
                println!("Incorrect guess. Try again!");
            }
        }
        Err(_) => {
            println!("check how the key is being generated");
        }
    }
}

fn generate_key() -> u8 {
    
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs() as u8,
        Err(_) => 0, 
    }
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
