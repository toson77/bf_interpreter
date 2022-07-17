use std::char;
use std::io;
fn main() {
    let mut word = String::new();
    io::stdin().read_line(&mut word).ok();
    let words = word.trim().to_string();
    let chars_vec: Vec<char> = words.chars().collect();
    let mut mem: [usize; 500] = [0; 500];
    let mut pointer: usize = 0;
    let mut l_mem: [usize; 100] = [0; 100];
    let mut l_pointer: usize = 0;
    let mut skipcount: usize = 0;
    let mut i: usize = 0;
    loop {
        if i >= chars_vec.len() {
            break;
        }
        if skipcount >= 1 {
            match chars_vec[i] {
                '[' => skipcount += 1,
                ']' => skipcount -= 1,
                _ => {}
            }
            i += 1;
            continue;
        }
        match chars_vec[i] {
            '>' => pointer += 1,
            '<' => pointer -= 1,
            '+' => mem[pointer] += 1,
            '-' => mem[pointer] -= 1,
            '.' => match char::from_u32(mem[pointer] as u32) {
                Some(x) => print!("{}", x),
                None => println!("None"),
            },
            ',' => todo!(),
            '[' => {
                if mem[pointer] == 0 {
                    skipcount += 1;
                } else {
                    l_mem[l_pointer] = i;
                    l_pointer += 1;
                }
            }
            ']' => {
                if mem[pointer] == 0 {
                    l_pointer -= 1;
                } else {
                    i = l_mem[l_pointer - 1];
                }
            }
            _ => {
                panic!("excep char")
            }
        };
        i += 1;
    }
    println!("{:?}", mem);
}
