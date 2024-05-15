/// This library is a collection of Math operations like Factorial, Fibonacci, and Prime number check.
/// It also has a String operation to check if a word is a palindrome, and reverse string.
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum MathAction {
    /// Factorial of a number.
    Factorial {
        /// Enter the number.
        #[structopt()]
        num: usize,
    },
    /// Fibonacci of a number.
    Fibonacci {
        /// Enter the number.
        #[structopt()]
        num: usize,
    },
    /// Prime number check.
    PrimeOrNot {
        /// Enter the number.
        #[structopt()]
        num: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, StructOpt)]
pub enum WordAction {
    /// Check if a word is a palindrome.
    Palindrome {
        /// Enter the word.
        #[structopt()]
        word: String,
    },
    /// Reverse a string.
    ReverseString {
        /// Enter the word.
        #[structopt()]
        word: String,
    },
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Math(MathAction),
    Word(WordAction),
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Libby",
    about = "A command line app written in Rust that supports Math and String operations."
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub command: Command,
}
