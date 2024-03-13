use std::cmp::max;
use std::ops::Mul;
use std::ops::Rem;
use std::vec::Vec;

fn main() -> Result<(), String> {
    for n in (3..=33).step_by(6) {
        let input: Vec<u64> = (0..=n).step_by(3).collect();
        println!("input vec  = {:?}", input);

        let output = ntt(&input)?;
        println!("after ntt  = {:?}", output);
        assert_ne!(input, output);

        let original = i_ntt(&output)?;
        println!("after inverse ntt = {:?}\n\n", original);
        assert_eq!(input, original);
    }
    Ok(())
}

/// prime test
fn is_prime(number: u64) -> bool {
    match number {
        0..=1 => false,
        2 => true,
        _ => {
            if number & 1 == 0 { return false; }
            let mut res = true;
            (3..(number as f64).sqrt() as u64 + 1)
                .step_by(2)
                .for_each(|x| { if number % x == 0 { res = false } });
            res
        }
    }
}

/// return prime factors
fn prime_factors_of(mut number: u64) -> Vec<u64> {
    if is_prime(number) { return vec![number]; }
    let mut factors = Vec::new();
    let mut divisor = 2;
    while number > 1 {
        while number % divisor == 0 {
            factors.push(divisor);
            number /= divisor;
        }
        divisor += 1;
    }
    factors
}

/// This trait represents x^y % m
trait PowMod<E, M>
    where
        Self: Copy + Mul<E> + Rem<M>,
{
    type Return;
    fn pow_mod(self, exponent: E, modulus: M) -> Self::Return;
}

/// This trait represents the modular inverse of x^y % m
trait InversePowMod<E, M>
    where
        Self: Copy + Mul<E> + Rem<M>,
{
    type Return;
    fn inv_pow_mod(self, exponent: E, modulus: M) -> Self::Return;
}

/// Defining x^y % m for the u64 type.
impl PowMod<u64, u64> for u64 {
    type Return = Self;
    fn pow_mod(mut self, mut exponent: u64, modulus: u64) -> Self {
        // TODO: 用费马小定理先化简一次？
        let mut result = 1;
        while 0 < exponent {
            if exponent & 1 == 1 {
                result = result * self % modulus;
            }
            self = self.pow(2) % modulus;
            exponent >>= 1;
        }
        result
    }
}

/// Defining modular inverse of x^y % m for the u64 type.
/// This is a Naive implementation, but sufficient for this example.
/// https://www.khanacademy.org/computing/computer-science/cryptography/modarithmetic/a/modular-inverses
impl InversePowMod<u64, u64> for u64 {
    type Return = Self;
    fn inv_pow_mod(self, exponent: u64, modulus: u64) -> Self {
        let pow_mod = self.pow_mod(exponent, modulus);
        (0..modulus).filter(|x| (pow_mod * x) % modulus == 1).collect::<Vec<_>>()[0]
    }
}

/// Returns the Multiplication Matrix of the Integers mod n:
/// (Example, n = 5): http://www.wolframalpha.com/input/?i=integers+mod+5
fn dft_matrix(n: u64) -> Vec<Vec<u64>> {
    let mut matrix: Vec<Vec<u64>> = vec![vec![0; n as usize]; n as usize];
    (0..n).for_each(|x| (0..n)
        .for_each(|y| matrix[x as usize][y as usize] = x * y % n));
    matrix
}

/// Finds a modulus M such that:
///   M is a prime number.
///   M is larger than the number of elements
///   M is larger than the value of any element
fn find_modulus(elements: &[u64]) -> Result<u64, String> {
    let n = elements.len() as u64;
    if n == 0 { return Err("[NttError]: Attempt to transform nothing".to_string()); }

    let max_elem = *elements
        .iter().max()
        .expect("[NttError]: Could not define a maximum element.");
    let largest = max(n, max_elem);
    let start = (largest - 1) / n;

    (start..).find(|&x| {
        let modulus = x * n + 1;
        modulus > largest && is_prime(modulus)
    }).map_or_else(
        || Err("[NttError]: Could not find working modulus for the provided vector."
            .to_string()),
        |x| Ok(x * n + 1))
}

/// Finds a generator under the given modulus:
/// Some number g is a generator for a modulus M if for each
/// prime factor of (M - 1), g^((M - 1) / factor) mod M != 1
/// I still do not fully understand why this works.
fn find_generator(modulus: u64) -> Result<u64, String> {
    let max_value = modulus - 1;
    let prime_factors = prime_factors_of(max_value);
    for generator in 1..modulus {
        if prime_factors
            .iter()
            .map(
                |factor|
                    1 != generator.pow_mod(max_value / factor, modulus)
            )
            .all(|not_one| not_one)
        {
            return Ok(generator);
        }
    }
    Err(format!("[NttError]: No generator exists under the modulus `{}`", modulus))
}

/// Finds the value of omega for the Number-Theoretic Transform under a given modulus M.
/// Omega is defined as the following:
///   Let g = a generator under the modulus M
///   Let k = (M - 1) / (the number of elements that will be transformed)
///   Let omega = g^k mod M
fn find_primitive_root(n: u64, modulus: u64) -> Result<u64, String> {
    let k = (modulus - 1) / n;
    let generator = find_generator(modulus)?;
    Ok(generator.pow_mod(k, modulus))
}

/// Number-Theoretic Transform
fn ntt(elements: &[u64]) -> Result<Vec<u64>, String> {
    let n = elements.len() as u64;
    let modulus = find_modulus(elements)?;
    let omega = find_primitive_root(n, modulus)?;
    Ok(
        dft_matrix(n)
            .iter()
            .map(|row| {
                elements
                    .iter()
                    .zip(row)
                    .map(
                        |(elem, ij)|
                            elem * omega.pow_mod(*ij, modulus)
                    )
                    .sum::<u64>()
                    % modulus
            }).collect::<Vec<u64>>()
    )
}

/// Inverse Number-Theoretic Transform
fn i_ntt(elements: &[u64]) -> Result<Vec<u64>, String> {
    let n = elements.len() as u64;
    let modulus = find_modulus(elements)?;
    let omega = find_primitive_root(n, modulus)?;
    Ok(
        dft_matrix(n)
            .iter()
            .map(|row| {
                elements
                    .iter()
                    .zip(row)
                    .map(
                        |(elem, ij)|
                            elem * omega.inv_pow_mod(*ij, modulus)
                    )
                    .sum::<u64>()
                    * n.inv_pow_mod(1, modulus)
                    % modulus
            }).collect::<Vec<u64>>()
    )
}