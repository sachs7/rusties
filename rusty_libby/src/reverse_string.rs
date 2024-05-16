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
        write!(f, "Reverse of {} is: {}", self.word, self.reverse())
    }
}
