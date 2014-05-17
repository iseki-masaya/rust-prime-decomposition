use num::bigint::BigUint;
use num::bigint::ToBigUint;

use num::Integer;
use std::num::One;

use rand;
use rand::Rng;

use std::io::println;

fn mod_exp(mut n : BigUint, mut e : BigUint, m : BigUint) -> BigUint {
    let zero = (0u).to_biguint().unwrap();
    let mut res = (1u).to_biguint().unwrap();

    while zero.lt(&e) {
        if e.is_odd() {
            res = (res.mul(&n)).mod_floor(&m);
        }
        n = (n.mul(&n)).mod_floor(&m);
        e = e.shr(&1u);
    }
    return res;
}

pub fn miller_rabin(n :BigUint, t:uint) -> bool {
    let one = (1u).to_biguint().unwrap();
    let two = (2u).to_biguint().unwrap();
    if n.lt(&two) {
        return false;
    }
    else if n.eq(&two) {
        return true;
    }
    else if n.is_even() {
        return false;
    }

    let phi_n = n.sub(&one);
    let rng_lim = phi_n.sub(&one);
    let mut q = n.sub(&one);
    let mut k : i32 = 0;
    while q.is_odd() {
        k += 1;
        q = q.shr(&1u);
    }
    let mut rng = rand::task_rng();
    for i in range(0u,t) {
        // let b = ( rng.gen::<uint>() ).to_biguint().unwrap();
        // let e = ( rng.gen::<uint>() ).to_biguint().unwrap();
        // let a = mod_exp(b.clone(), e.clone(), n.clone());

        let mut a = ( rng.gen::<uint>() ).to_biguint().unwrap();
        a = a.mod_floor(&rng_lim).add(&one);
        let mut x = mod_exp(a.clone(), q.clone(), n.clone());
        if x.eq(&one) {
            continue;
        }
        let mut found : bool = false;
        for j in range(0,k) {
            if x.eq(&phi_n) {
                found = true;
                break;
            }
            x = (x.mul(&x)).mod_floor(&n);
        }
        if found {
            continue;
        }
        return false;
    }
    return true;
}