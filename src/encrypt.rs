use num_bigint::BigUint;

pub fn encrypt(public_key: &(BigUint, BigUint), plaintext: &[u8]) -> Result<BigUint, &'static str> {
    let padded = super::utils::pkcs1v15_pad(plaintext)?;
    let m = BigUint::from_bytes_be(&padded);
    Ok(super::utils::mod_pow(&m, &public_key.1, &public_key.0))
}
