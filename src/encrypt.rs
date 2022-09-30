use mod_exp::mod_exp;
use std::io;
use std::io::*;

pub fn encrypt(message: &mut String, nums: &mut String) {
    let mut iter = nums.split_whitespace();

    let e = iter.next().unwrap().parse::<u128>().unwrap();
    let n = iter.next().unwrap().parse::<u128>().unwrap();

    print!("{} {}\n", e, n);

    for ch in message.chars() {
        let ascii = ch as u128;

        let c = mod_exp(ascii, e, n);
        
        io::stdout().write_all(&c.to_be_bytes()).unwrap();
    }
}
