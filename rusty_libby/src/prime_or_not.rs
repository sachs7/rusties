/// This module contains the PrimeOrNot struct and its implementation.
/// 
/// PrimeOrNot struct has a field num of type usize.
/// PrimeOrNot struct has a method new which takes a number and returns a PrimeOrNot instance.
/// PrimeOrNot struct has a method is_prime which returns a boolean value.
/// PrimeOrNot struct has a Display implementation which returns a formatted string.
/// 
/// Example:
/// ```
/// use rusty_libby::prime_or_not::PrimeOrNot;
/// 
/// let prime_or_not = PrimeOrNot::new(7);
/// assert_eq!(prime_or_not.is_prime(), true);
/// assert_eq!(prime_or_not.to_string(), "7 is Prime");
/// ```

use std::fmt;

pub struct PrimeOrNot {
    pub num: usize,
}

impl PrimeOrNot {
    pub fn new(num: usize) -> Self {
        Self { num }
    }

    pub fn is_prime(&self) -> bool {
        if self.num <= 1 {
            return false;
        }
        for i in 2..self.num {
            if self.num % i == 0 {
                return false;
            }
        }
        true
    }
}

impl fmt::Display for PrimeOrNot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is {}", self.num, if self.is_prime() { "Prime" } else { "Not Prime" })
    }
}
