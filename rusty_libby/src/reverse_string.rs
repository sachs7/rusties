pub struct ReverseString {
    pub word: String,
}

impl ReverseString {
    pub fn new(word: String) -> Self {
        ReverseString { word }
    }

    pub fn reverse(&self) -> String {
        self.word.chars().rev().collect()
    }
}
