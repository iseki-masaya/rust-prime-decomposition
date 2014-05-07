use std::iter::range_step;

pub struct Sieve {
    size    : uint,
    primes  : Vec<uint>,
    isPrime : Vec<bool>
}

fn eratosthenes(size: uint) -> (Vec<uint>, Vec<bool>) {
    //FIXME: Investigate how to initialize the elents of vector.
    let mut isPrime: Vec<bool> = Vec::new();
    for i in range(0u,size) {
        if i < 2 {
            isPrime.push(false);
        }
        else {
            isPrime.push(true);
        }
    }

    let mut primes: Vec<uint> = Vec::new();
    for i in range(2u,size) {
        if *isPrime.get(i) {
            primes.push(i);
            for j in range_step(2*i,size,i) {
                *isPrime.get_mut(j) = false;
            }
        }
    }
    return (primes, isPrime);
}

impl Sieve {
    pub fn new(size: uint) -> Sieve {
        let (primes, isPrime) = eratosthenes(size);
        return Sieve {
            size    : size,
            primes  : primes,
            isPrime : isPrime
        }
    }

    pub fn is_prime(&mut self, value: uint) -> bool {
        if value < self.isPrime.len() {
            return *self.isPrime.get(value);
        }
        return false;
    }

    pub fn get_prime(&mut self, index: uint) -> uint {
        if index < self.primes.len() {
            return *self.primes.get(index);
        }
        return 0u;
    }
}