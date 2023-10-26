use num_bigint::BigUint;

pub fn decrypt(private_key: &(BigUint, BigUint, BigUint), ciphertext: &BigUint) -> Result<Vec<u8>, &'static str> {
    let c = super::utils::mod_pow(ciphertext, &private_key.2, &private_key.0);
    let padded_msg = c.to_bytes_be();
    super::utils::pkcs1v15_unpad(&padded_msg)
}
