/// This module contains the Factorial struct and its implementation
/// 
/// Factorial struct has a field num of type usize.
/// Factorial struct has a method new which takes a number and returns a Factorial instance.
/// Factorial struct has a method calculate which returns a usize value.
/// Factorial struct has a Display implementation which returns a formatted string.
///
/// Example:
/// ```
/// use rusty_libby::factorial::Factorial;
/// 
/// let factorial = Factorial::new(5);
/// assert_eq!(factorial.calculate(), 120);
/// assert_eq!(factorial.to_string(), "Factorial of 5 is 120");
/// ```


use std::fmt;

pub struct Factorial {
    pub num: usize,
}

impl Factorial {
    pub fn new(num: usize) -> Self {
        Self { num }
    }

    pub fn calculate(&self) -> usize {
        let mut result = 1;
        for i in 1..=self.num {
            result *= i;
        }
        result
    }
}

impl fmt::Display for Factorial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Factorial of {} is {}", self.num, self.calculate())
    }
}
