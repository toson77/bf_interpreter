use std::char;
use std::io;
fn main() {
    let mut word = String::new();
    io::stdin().read_line(&mut word).ok();
    let words = word.trim().to_string();
    println!("{}", words);
    let mut mem: [u32; 500] = [0; 500];
    let mut pointer: usize = 0;
    let mut l_mem: [u32; 100] = [0; 100];
    let mut l_pointer: u32 = 0;
    let mut skipcount: u32 = 0;
    for symbol in words.chars() {
        match symbol {
            '>' => pointer += 1,
            '<' => pointer -= 1,
            '+' => mem[pointer] += 1,
            '-' => mem[pointer] -= 1,
            '.' => match char::from_u32(mem[pointer]) {
                Some(x) => print!("{}", x),
                None => println!("None"),
            },
            ',' => todo!(),
            '[' => {
                // l_mem[l_mem]
            }
            ']' => todo!(),
            _ => {}
        };
    }
    println!("{:?}", mem);
}
