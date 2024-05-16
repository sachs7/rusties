/// This module contains a struct that reverses a string
/// 
/// ReverseString struct has a field word of type String.
/// ReverseString struct has a method new which takes a word and returns a ReverseString instance.
/// ReverseString struct has a method reverse which returns a String value.
/// ReverseString struct has a Display implementation which returns a formatted string.
/// 
/// Example:
/// ```
/// use rusty_libby::reverse_string::ReverseString;
/// 
/// let reverse_string = ReverseString::new("hello".to_string());
/// assert_eq!(reverse_string.to_string(), "Reverse of hello is olleh");
/// ```

pub struct ReverseString {
    pub word: String,
}

impl ReverseString {
    pub fn new(word: String) -> Self {
        ReverseString { word }
    }

    fn reverse(&self) -> String {
        self.word.chars().rev().collect()
    }
}

impl std::fmt::Display for ReverseString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Reverse of {} is {}", self.word, self.reverse())
    }
}
