/// A module for checking if a word is a palindrome or not
/// 
/// Palindrome struct has a field word of type String.
/// Palindrome struct has a method new which takes a word and returns a Palindrome instance.
/// Palindrome struct has a method is_palindrome which returns a boolean value.
/// Palindrome struct has a Display implementation which returns a formatted string.
/// 
/// Example:
/// ```
/// use rusty_libby::palindrome::Palindrome;
/// 
/// let palindrome = Palindrome::new("madam".to_string());
/// assert_eq!(palindrome.is_palindrome(), true);
/// assert_eq!(palindrome.to_string(), "madam is a palindrome");
/// ```
/// 
/// ```
/// use rusty_libby::palindrome::Palindrome;
/// 
/// let palindrome = Palindrome::new("hello".to_string());
/// assert_eq!(palindrome.is_palindrome(), false);
/// assert_eq!(palindrome.to_string(), "hello is not a palindrome");
/// ```

pub struct Palindrome {
    pub word: String,
}

impl Palindrome {
    pub fn new(word: String) -> Self {
        Self { word }
    }

    fn is_palindrome(&self) -> bool {
        self.word.chars().eq(self.word.chars().rev())
    }
}

impl std::fmt::Display for Palindrome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_palindrome() {
            write!(f, "{} is a palindrome", self.word)
        } else {
            write!(f, "{} is not a palindrome", self.word)
        }
    }
}
