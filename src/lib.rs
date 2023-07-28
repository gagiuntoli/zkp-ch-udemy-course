use num_bigint::BigUint;

/// output = n^exp mod p
pub fn exponentiate(n: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    n.modpow(exponent, modulus)
}

/// output = s = k - c * x mod q
pub fn solve(k: &BigUint, c: &BigUint, x: &BigUint, q: &BigUint) -> BigUint {
    if *k >= c * x {
        return (k - c * x).modpow(&BigUint::from(1u32), q);
    }
    return q - (c * x - k).modpow(&BigUint::from(1u32), q);
}

/// cond1: r1 = alpha^s * y1^c
/// cond2: r2 = beta^s * y2^c
pub fn verify(r1: &BigUint, r2: &BigUint, y1: &BigUint, y2: &BigUint, alpha: &BigUint, beta: &BigUint, c: &BigUint, s: &BigUint, p: &BigUint) -> bool {
    let cond1 = *r1 == alpha.modpow(s, p) * y1.modpow(c, p);
    let cond2 = *r2 == beta.modpow(s, p) * y2.modpow(c, p);
    cond1 && cond2
}