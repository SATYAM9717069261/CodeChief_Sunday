use std::fs;
use std::fs::File;
use std::io::{self, Write,Read};

const LOWER_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPER_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGIT_CHARS: &str = "0123456789";
const SPECIAL_CHARS: &str = "!@#$%^&*()-_=+[]{}|;:,.<>?";

fn main() {
    println!("Welcome to the Random Password Generator");

    let mut length = read_int("Enter password length (min 8) [Default 10]: ", 10);
    if length < 8 {
        println!("Length too short, defaulting to 8.");
        length = 8;
    }

    let use_upper = read_bool("Include uppercase letters? (y/n) [Default y]: ", true);
    let use_digits = read_bool("Include digits? (y/n) [Default y]: ", true);
    let use_symbols = read_bool("Include symbols? (y/n) [Default n]: ", false);

    let mut count = read_int("How many passwords to generate? [Default 1]: ", 1);
    if count < 1 {
        count = 1;
    }

    let out_path = read_string(
        "Save to file? (Enter filename) [Default: output_1.txt]: ",
        "output_1.txt",
    );

    println!("\nGenerating...");

    // Execute Core Logic
    match generate_and_save_passwords(
        count,
        length,
        use_upper,
        use_digits,
        use_symbols,
        &out_path,
    ) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Generation Error: {}", e);
            std::process::exit(1);
        }
    }
}
fn random_char_from_set(set: &str) -> char {
    let mut file = File::open("/dev/urandom").unwrap();
    let mut buffer = [0u8; 8];

    file.read_exact(&mut buffer).unwrap();

    let idx = (u64::from_ne_bytes(buffer) as usize) % set.len();

    set.chars().nth(idx).unwrap()
}
// --- Core Cryptographic Logic (YOUR TURN) ---
fn random_u64(file: &mut File) -> u64 {
    let mut buffer = [0u8; 8];
    file.read_exact(&mut buffer).unwrap();
    u64::from_ne_bytes(buffer)
}

fn shuffle(bytes: &mut Vec<char>) {
    // TODO 2:
    // Randomize the order of characters.
    let mut file = File::open("/dev/urandom").unwrap();
    for i in (1..bytes.len()).rev(){
        let j = (random_u64(&mut file) as usize) % (i + 1);
        bytes.swap(i,j);
    }
}

fn generate_password(length: i32, upper: bool, digits: bool, symbols: bool) -> String {
    let mut pool = String::from(LOWER_CHARS);
    let mut pwd: Vec<char> = Vec::new();

    if upper {
        pool.push_str(UPPER_CHARS);
    }

    if digits {
        pool.push_str(DIGIT_CHARS);
    }

    if symbols {
        pool.push_str(SPECIAL_CHARS);
    }
    // TODO 3:
    // Build your character pool based on boolean flags.

    pwd.push(random_char_from_set(LOWER_CHARS));

    if upper {
        pwd.push(random_char_from_set(UPPER_CHARS));
    }

    if digits {
        pwd.push(random_char_from_set(DIGIT_CHARS));
    }

    if symbols {
        pwd.push(random_char_from_set(SPECIAL_CHARS));
    }
    // TODO 4:
    // Guarantee at least one character from each selected pool.

    while pwd.len() < length as usize {
        pwd.push(random_char_from_set(&pool));
    }
    // TODO 5:
    // Fill remaining length with random characters.

    shuffle(&mut pwd);
    // TODO 6:
    // Shuffle final password.

    pwd.into_iter().collect()
}

fn evaluate_strength(pwd: &str) -> i32 {
    // TODO 7:
    // Score password out of 5 based on length and complexity.
    // Use contains() / chars().any() to check for uppercase,
    // digits, and symbols.
    let mut score = 0;
    if pwd.len() >= 8{
        score+=1;
    }
    if pwd.chars().any(|c| LOWER_CHARS.contains(c)){
        score+=1;
    }
    if pwd.chars().any(|c| UPPER_CHARS.contains(c)){
        score+=1;
    }
    if pwd.chars().any(|c| DIGIT_CHARS.contains(c)){
        score+=1;
    }
    if pwd.chars().any(|c| SPECIAL_CHARS.contains(c)){
        score+=1;
    }
    score
}


fn generate_and_save_passwords(
    count: i32,
    length: i32,
    upper: bool,
    digits: bool,
    symbols: bool,
    out_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = String::new();
    // TODO 8:
    // Create loop running 'count' times.
    for i in 0..count {
        let pwd = generate_password(length, upper, digits, symbols);
        let strength = evaluate_strength(&pwd);
        output.push_str(
            &format!(
                "Password {}: {}\nStrength: {}/5\n\n",
                i + 1,
                pwd,
                strength
            )
        );
    }
    fs::write(out_path, output)?;
    // TODO 9:
    // Call generate_password() and evaluate_strength().

    // TODO 10:
    // Format results into one string.

    // TODO 11:
    // Write final string to file.

    println!(
        "Successfully saved {} password(s) to '{}'",
        count, out_path
    );

    Ok(())
}
fn read_string(prompt: &str, default_val: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim();

    if input.is_empty() {
        default_val.to_string()
    } else {
        input.to_string()
    }
}

fn read_int(prompt: &str, default_val: i32) -> i32 {
    let input = read_string(prompt, "");

    if input.is_empty() {
        return default_val;
    }

    match input.parse::<i32>() {
        Ok(v) => v,
        Err(_) => {
            println!("Invalid number, using default ({}).", default_val);
            default_val
        }
    }
}

fn read_bool(prompt: &str, default_val: bool) -> bool {
    let input = read_string(prompt, "").to_lowercase();
    if input.is_empty() {
        return default_val;
    }
    input == "y" || input == "yes"
}
