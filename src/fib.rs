use std::mem::{swap, replace};

use num_bigint::BigUint;
use num_traits::identities::{Zero, One};
use ilog2::msb;

pub fn fib(n: u64) -> BigUint {

    let mut fibs_cache = [
        BigUint::one(),
        BigUint::zero(),
        BigUint::one(),
        BigUint::zero(),
        BigUint::zero(),
        BigUint::zero(),
    ];
    let (mut fibs, mut fibs_next) = fibs_cache.split_at_mut(3);

    for bit_lvl in (0..=msb(n)).rev() {

        let bit_val: usize = if (n >> bit_lvl) & 1 != 0 {1} else {0};

        for i in 0usize..2usize {
            let j = i + bit_val;
            
            fibs_next[i] = 
                &fibs[(j+1) / 2 + 1] * &fibs[j / 2 + 1]
                + &fibs[(j+1) / 2] * &fibs[j / 2];
        }

        fibs_next[2] = &fibs_next[0] + &fibs_next[1];

        swap(&mut fibs, &mut fibs_next);
    };

    return replace(&mut fibs[1], BigUint::zero());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_test1() {
        let expected_vals: [u32; 9] = [0,1,1,2,3,5,8,13,21];

        for (i, val) in expected_vals.iter().enumerate() {
            assert_eq!(fib(i as u64), BigUint::from(*val));
        }
        assert_eq!(fib(0), BigUint::zero());
        
    }
    #[test]
    fn fib_test2() {
        let n: u64 = 100;
        assert_eq!(fib(n) + fib(n+1), fib(n+2));
        
    }
}
