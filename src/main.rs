extern crate num;
extern crate collections;
extern crate rand;

use num::bigint::BigUint;
use num::bigint::ToBigUint;

use std::io::BufferedReader;
use std::io::println;
use std::io;

mod sieve;
mod miller_rabin;

fn str2big(number:&str) -> BigUint {
    let mut res = (0i).to_biguint().unwrap();
    let ten = (10i).to_biguint().unwrap();
    for ch in number.chars() {
        if '0' <= ch && ch <= '9' {
            res = res.mul(&ten);
            let value = ((ch as i32) - ('0' as i32)).to_biguint().unwrap();
            res = res.add(&value);
        }
    }
    return res;
}

fn main() {
    println("INPUT:");
    let mut reader = BufferedReader::new(io::stdin());
    let input = reader.read_line().ok().unwrap();
    let v = str2big(input);
    println!("v is \"{}\" number",if miller_rabin::miller_rabin(v.clone(), 20u) {"prime"} else {"composite"});
}