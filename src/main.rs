/// A CLI app that supports Math and Word operations.
///
/// This app supports the following commands:
///
/// 1. Math:
///   - Factorial: Calculate the factorial of a number.
///   - Fibonacci: Calculate the Fibonacci of a number.
///   - PrimeOrNot: Check if a number is prime.
///
/// 2. Word:
///   - Palindrome: Check if a word is a palindrome.
///   - ReverseString: Reverse a string.
///
/// To see the usage of a specific command, use the `--help` flag with the command.
///
/// For example, to see the usage of the `Math` command, run:
///
/// ```
/// $ rusties math --help
/// ```
///
/// To see the usage of the `Word` command, run:
///
/// ```
/// $ rusties word --help
/// ```
///
/// Example usage:
///
/// To calculate the factorial of 5, run:
///
/// ```
/// $ rusties math factorial 5
/// ```
///
/// To check if 7 is a prime number, run:
///
/// ```
/// $ rusties math prime-or-not 7
/// ```
///
/// To check if the word "madam" is a palindrome, run:
///
/// ```
/// $ rusties word palindrome madam
/// ```
/// 
/// To reverse the word "hello", run:
/// 
/// ```
/// $ rusties word reverse-string hello
/// ```

use structopt::StructOpt;

mod cli;
use cli::{Command, CommandLineArgs, MathAction, WordAction};

use rusty_libby::{
    factorial::Factorial, fibonacci::Fibonacci, palindrome::Palindrome, prime_or_not::PrimeOrNot,
    reverse_string::ReverseString,
};

fn main() {
    let args = CommandLineArgs::from_args();

    match args.command {
        Command::Math(math) => match math {
            MathAction::Factorial { num } => {
                println!("Factorial of {} is {}", num, Factorial::new(num));
            }
            MathAction::Fibonacci { num } => {
                println!("Fibonacci of {} is {}", num, Fibonacci::new(num));
            }
            MathAction::PrimeOrNot { num } => {
                println!("{}", PrimeOrNot::new(num));
            }
        },
        Command::Word(word) => match word {
            WordAction::Palindrome { word } => {
                let palindrome = Palindrome::new(word);
                println!("{}", palindrome);
            }
            WordAction::ReverseString { word } => {
                let reverse_string = ReverseString::new(word);
                println!("{}", reverse_string);
            }
        },
    }
}
