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
pub fn verify(
    r1: &BigUint,
    r2: &BigUint,
    y1: &BigUint,
    y2: &BigUint,
    alpha: &BigUint,
    beta: &BigUint,
    c: &BigUint,
    s: &BigUint,
    p: &BigUint,
) -> bool {
    let cond1 = *r1 == (alpha.modpow(s, p) * y1.modpow(c, p)).modpow(&BigUint::from(1u32), &p);
    let cond2 = *r2 == (beta.modpow(s, p) * y2.modpow(c, p)).modpow(&BigUint::from(1u32), &p);
    cond1 && cond2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_toy_example() {
        // alpha = 4 beta = 9 p = 23 q = 11
        // prover x = 6 k = 7
        // verifier c = 4
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);

        let x = BigUint::from(6u32);
        let k = BigUint::from(7u32);

        let c = BigUint::from(4u32);

        // y1 = alpha^x mod p
        // y2 = beta^x mod p
        let y1 = exponentiate(&alpha, &x, &p);
        let y2 = exponentiate(&beta, &x, &p);
        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        // r1 = alpha^k mod p
        // r2 = beta^k mod p
        let r1 = exponentiate(&alpha, &k, &p);
        let r2 = exponentiate(&beta, &k, &p);
        assert_eq!(r1, BigUint::from(8u32));
        assert_eq!(r2, BigUint::from(4u32));

        let s = solve(&k, &c, &x, &q);
        assert_eq!(s, BigUint::from(5u32));

        let solution = verify(&r1, &r2, &y1, &y2, &alpha, &beta, &c, &s, &p);
        assert!(solution);

        // Fake secret case
        let x_fake = BigUint::from(7u32);
        let s_fake = solve(&k, &c, &x_fake, &q);

        let solution = verify(&r1, &r2, &y1, &y2, &alpha, &beta, &c, &s_fake, &p);
        assert!(!solution);
    }
}
