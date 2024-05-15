/// This module contains the Fibonacci struct and its implementation
/// 
/// Fibonacci struct has a field num of type usize.
/// Fibonacci struct has a method new which takes a number and returns a Fibonacci instance.
/// Fibonacci struct has a method calculate which returns a usize value.
/// Fibonacci struct has a Display implementation which returns a formatted string.
/// 
/// Example:
/// ```
/// use rusty_libby::fibonacci::Fibonacci;
/// 
/// let fibonacci = Fibonacci::new(7);
/// assert_eq!(fibonacci.calculate(), 13);
/// assert_eq!(fibonacci.to_string(), "Fibonacci of 7 is 13");
/// ```

use std::fmt;

pub struct Fibonacci {
    pub num: usize,
}

impl Fibonacci {
    pub fn new(num: usize) -> Self {
        Self { num }
    }

    pub fn calculate(&self) -> usize {
        match self.num {
            0 => 0,
            1 => self.num,
            _ => Fibonacci::new(self.num - 1).calculate() + Fibonacci::new(self.num - 2).calculate(),
        }
    }
}

impl fmt::Display for Fibonacci {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fibonacci of {} is {}", self.num, self.calculate())
    }
}
