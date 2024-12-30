use std::io::{self, Read};

fn main() {
    for byte in io::stdin().bytes() {
        let ch = byte.unwrap() as char;
        println!("{}", ch);
        if ch == 'q' {
            break;
        }
    }
}