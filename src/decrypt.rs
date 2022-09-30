use primes::factors;
use modinverse::modinverse;
use mod_exp::mod_exp;

fn bytes_to_num(b: &mut [u8; 16]) -> u128 {
    ((b[0] as u128) << 120) +
    ((b[1] as u128) << 112) +
    ((b[2] as u128) << 104) +
    ((b[3] as u128) << 96) +
    ((b[4] as u128) << 88) +
    ((b[5] as u128) << 80) +
    ((b[6] as u128) << 72) +
    ((b[7] as u128) << 64) +
    ((b[8] as u128) << 56) +
    ((b[9] as u128) << 48) +
    ((b[10] as u128) << 40) +
    ((b[11] as u128) << 32) +
    ((b[12] as u128) << 24) +
    ((b[13] as u128) << 16) +
    ((b[14] as u128) << 8) +
    b[15] as u128 
}

pub fn decrypt(ciphertxt: &mut [u8; 16], e: u64, n: u64) {
    let primes = factors(n as u64);
    let p = primes[0] as u64;
    let q = primes[1] as u64;

    let totient = (p - 1) * (q - 1);

    let d = modinverse(e as i64, totient as i64).expect("modinverse error");

    let m = mod_exp(bytes_to_num(ciphertxt) as u64, d as u64, n) as u8;
    print!("{}", m as char);
}