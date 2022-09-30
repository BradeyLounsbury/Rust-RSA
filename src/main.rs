use rust_rsa::encrypt::encrypt;
use rust_rsa::decrypt::decrypt;
use std::io;
use std::io::Read;
use std::io::Write;
use std::env;

pub fn main() {
    let mut is_encrypt = false;
    let mut is_decrypt = false;

    let args: Vec<_> = env::args().collect();
    if args[1].to_lowercase() == "encrypt" {
        is_encrypt = true;
    }
    else if args[1].to_lowercase() == "decrypt" {
        is_decrypt = true;
    }

    if is_encrypt {
        let mut message = String::new();
        let mut nums = String::new();

        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read message");

        io::stdin()
            .read_line(&mut nums)
            .expect("Failed to read numbers");

        encrypt(&mut message, &mut nums);
    }

    if is_decrypt {
        let mut nums = String::new();
        let mut c_buf: [u8; 16] = [0; 16];

        io::stdin()
            .read_line(&mut nums)
            .expect("Failed to read numbers");

        let mut num_iter = nums.split_whitespace();
        let e = num_iter.next().unwrap().parse::<u64>().unwrap();
        let n = num_iter.next().unwrap().parse::<u64>().unwrap();
        
        loop {
            let bytes_read = io::stdin().read(&mut c_buf).unwrap();
            if bytes_read > 0 { 
                decrypt(&mut c_buf, e, n); 
            }
        }
    }
}
