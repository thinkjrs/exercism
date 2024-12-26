use rand::Rng;
/*
 * The right to left binary method from https://en.wikipedia.org/wiki/Modular_exponentiation and
 * pseudocode from Bruce Schneier's book "Applied Cryptography"
 *
 * Without this we get overflow quickly.
 *
 * _Note: This is not using the overflow assertion, which would be necessary
 * to use this in any non-toy capacity._
 */
pub fn modular_exponentiation(base: u64, exponent: u64, modulus: u64) -> u64 {
    match modulus {
        _ if modulus == 0 => panic!("Modulus cannot be zero"),
        _ if modulus == 1 => 0,
        _ => {
            let mut result: u64 = 1;
            let mut base = base % modulus;
            let mut exponent = exponent;

            while exponent > 0 {
                if exponent % 2 == 1 {
                    result = result * base % modulus;
                }
                exponent = exponent >> 1;
                base = base * base % modulus;
            }
            result
        }
    }
}
pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    // g^a mod p
    modular_exponentiation(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    // s = B^a mod p
    modular_exponentiation(b_pub, a, p)
}
