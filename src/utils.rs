use num_bigint::{BigUint};
use num_integer::Integer;
use num_traits::{One, Zero};
use rand::Rng;

pub const RSA_KEY_SIZE: usize = 2048;
pub const BYTE_SIZE: usize = 8;
pub const MESSAGE_MAX_SIZE: usize = RSA_KEY_SIZE / BYTE_SIZE - 11;  // For PKCS#1 v1.5 padding

pub fn mod_pow(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    base.modpow(exponent, modulus)
}

pub fn pkcs1v15_pad(message: &[u8]) -> Result<Vec<u8>, &'static str> {
    if message.len() > MESSAGE_MAX_SIZE {
        return Err("Message too long for RSA");
    }

    let mut rng = rand::thread_rng();
    let padding_size = RSA_KEY_SIZE / BYTE_SIZE - message.len() - 3;

    let mut padded_msg = vec![0u8; RSA_KEY_SIZE / BYTE_SIZE];
    padded_msg[0] = 0x00;
    padded_msg[1] = 0x02;

    for i in 2..2 + padding_size {
        padded_msg[i] = rng.gen_range(1..=255);  // Non-zero padding bytes
    }

    padded_msg[2 + padding_size] = 0x00;
    padded_msg[3 + padding_size..].copy_from_slice(message);

    Ok(padded_msg)
}

pub fn pkcs1v15_unpad(padded_msg: &[u8]) -> Result<Vec<u8>, &'static str> {
    if padded_msg.len() != RSA_KEY_SIZE / BYTE_SIZE || padded_msg[0] != 0x00 || padded_msg[1] != 0x02 {
        return Err("Invalid padding");
    }

    let mut index = 2;
    while index < padded_msg.len() {
        if padded_msg[index] == 0x00 {
            break;
        }
        index += 1;
    }

    if index == padded_msg.len() || index < 10 {  // Padding string too short
        return Err("Invalid padding");
    }

    Ok(padded_msg[index+1..].to_vec())
}

pub fn miller_rabin_test(n: &BigUint, a: &BigUint) -> bool {
    let mut s = 0;
    let mut d = n.clone() - BigUint::one();

    while d.is_even() {
        d /= 2u32;
        s += 1;
    }

    let mut x = mod_pow(a, &d, n);
    if x == BigUint::one() || x == n.clone() - BigUint::one() {
        return true;
    }

    for _ in 0..s {
        x = mod_pow(&x, &BigUint::from(2u32), n);
        if x == n.clone() - BigUint::one() {
            return true;
        }
    }

    false
}

pub fn is_prime(n: &BigUint, rounds: u32) -> bool {
    for _ in 0..rounds {
        let a: BigUint = rand::thread_rng().gen_range(BigUint::from(2u32)..n.clone() - 1u32);
        if !miller_rabin_test(n, &a) {
            return false;
        }
    }
    true
}
pub fn generate_prime(bits: usize, rounds: u32) -> BigUint {
    let mut rng = rand::thread_rng();
    loop {
        let n: BigUint = rng.gen_biguint(bits);
        if is_prime(&n, rounds) {
            return n;
        }
    }
}

