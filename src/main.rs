use std::char;
use std::io;
mod work;
use work::interpreter;
fn main() {
    let mut word = String::new();
    io::stdin().read_line(&mut word).ok();
    let words = word.trim().to_string();
    let chars_vec: Vec<char> = words.chars().collect();
    let result = interpreter(&chars_vec);
    println!("{}", result);
}
