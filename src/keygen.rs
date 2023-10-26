use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::One;
use rand::rngs::OsRng;

pub fn generate_keypair(bits: usize) -> (BigUint, BigUint, BigUint) {
    let e = BigUint::from(3u32);
    let mut rng = OsRng;

    loop {
        let p = super::utils::generate_prime(bits / 2, 5);
        let q = super::utils::generate_prime(bits / 2, 5);
        if p == q {
            continue;
        }

        let n = &p * &q;
        let totient = (p.clone() - 1u32) * (q.clone() - 1u32);

        if e.gcd(&totient) == BigUint::one() {
            let d = e.modpow(&(&totient - BigUint::from(2u32)), &totient);
            return (n, e, d);
        }
    }
}
