#![no_std]
use core::convert::TryInto;

fn mulmod_u64(a: u64, b: u64, m: u64) -> u64 {
    let aa = a as u128;
    let bb = b as u128;
    let mm = m as u128;
    let rr = aa * bb % mm;
    rr as _
}

fn powmod_u64(mut a: u64, mut p: u64, m: u64) -> u64 {
    let mut y = 1;
    while p > 0 {
        if p % 2 == 1 {
            y = mulmod_u64(y, a, m);
        }
        a = mulmod_u64(a, a, m);
        p /= 2;
    }
    y
}

fn improved_felmat_test(n: u64, a: u64) -> bool {
    if a == 0 {
        return true;
    }
    let s = (n - 1).trailing_zeros();
    let d = n >> s;
    let mut ap = powmod_u64(a, d, n);
    if ap == 1 {
        return true;
    }
    for _ in 0..s {
        if ap == n - 1 {
            return true;
        }
        ap = mulmod_u64(ap, ap, n);
    }
    false
}

/// ref. https://miller-rabin.appspot.com/
fn miller_rabin_primality_test(n: u64) -> bool {
    let table = if n < 341531 {
        [9345883071009581737].iter()
    } else if n < 1050535501 {
        [336781006125, 9639812373923155].iter()
    } else if n < 350269456337 {
        [
            4230279247111683200,
            14694767155120705706,
            16641139526367750375,
        ]
        .iter()
    } else if n < 55245642489451 {
        [
            2,
            141889084524735,
            1199124725622454117,
            11096072698276303650,
        ]
        .iter()
    } else if n < 7999252175582851 {
        [
            2,
            4130806001517,
            149795463772692060,
            186635894390467037,
            3967304179347715805,
        ]
        .iter()
    } else if n < 585226005592931977 {
        [
            2,
            123635709730000,
            9233062284813009,
            43835965440333360,
            761179012939631437,
            1263739024124850375,
        ]
        .iter()
    } else {
        [2, 325, 9375, 28178, 450775, 9780504, 1795265022].iter()
    };
    for &a in table {
        if !improved_felmat_test(n, a % n) {
            return false;
        }
    }
    true
}

/// primality test
///
/// ```
/// use is_prime_for_primitive_int::is_prime_u64;
/// assert_eq!(is_prime_u64(2), true);
/// assert_eq!(is_prime_u64(57), false);
/// assert_eq!(is_prime_u64(91), false);
/// assert_eq!(is_prime_u64(97), true);
/// ```
pub fn is_prime_u64(n: u64) -> bool {
    if n == 2 || n == 3 || n == 5 || n == 7 {
        return true;
    }
    if n < 11 || n % 2 == 0 || n % 3 == 0 || n % 5 == 0 || n % 7 == 0 {
        return false;
    }
    if n < 11 * 11 {
        return true;
    }
    miller_rabin_primality_test(n)
}

/// primality test
/// ```
/// use is_prime_for_primitive_int::IsPrime;
/// assert_eq!(2.is_prime(), true);
/// assert_eq!(57.is_prime(), false);
/// assert_eq!(91.is_prime(), false);
/// assert_eq!(97.is_prime(), true);
/// ```
pub trait IsPrime {
    fn is_prime(&self) -> bool;
}

impl IsPrime for u64 {
    fn is_prime(&self) -> bool {
        is_prime_u64(*self)
    }
}
impl IsPrime for u32 {
    fn is_prime(&self) -> bool {
        is_prime_u64((*self).into())
    }
}
impl IsPrime for u16 {
    fn is_prime(&self) -> bool {
        is_prime_u64((*self).into())
    }
}
impl IsPrime for u8 {
    fn is_prime(&self) -> bool {
        is_prime_u64((*self).into())
    }
}
impl IsPrime for i64 {
    fn is_prime(&self) -> bool {
        if *self < 0 {
            false
        } else {
            is_prime_u64((*self).try_into().unwrap())
        }
    }
}
impl IsPrime for i32 {
    fn is_prime(&self) -> bool {
        if *self < 0 {
            false
        } else {
            is_prime_u64((*self).try_into().unwrap())
        }
    }
}
impl IsPrime for i16 {
    fn is_prime(&self) -> bool {
        if *self < 0 {
            false
        } else {
            is_prime_u64((*self).try_into().unwrap())
        }
    }
}
impl IsPrime for i8 {
    fn is_prime(&self) -> bool {
        if *self < 0 {
            false
        } else {
            is_prime_u64((*self).try_into().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    fn is_prime_ref(n: u64) -> bool {
        if n == 2 {
            return true;
        }
        if n < 2 || n % 2 == 0 {
            return false;
        }
        let mut d = 3;
        while d * d <= n {
            if n % d == 0 {
                return false;
            }
            d += 2;
        }
        true
    }
    #[test]
    fn is_prime_small() {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        for _ in 0..1_000_000 {
            let n: u32 = rng.gen();
            assert_eq!(is_prime_ref(n.into()), is_prime_u64(n.into()));
        }
    }
    #[test]
    fn is_prime_big() {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        for _ in 0..30 {
            let n = rng.gen();
            assert_eq!(is_prime_ref(n), is_prime_u64(n));
        }
    }
}
