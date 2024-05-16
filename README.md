# rusties

Basic Rust library and its usage

### Folder Structure
```
.
Cargo.toml -- Holds project dependencies
src -
 | - cli.rs -- CLI menu
 | - main.rs -- Entry point of the CLI app

rusty_libby -
 | - Cargo.toml -- Holds library dependencies
 | - src
      | - _all files specific to `math` and `word`
```

# How to run and sample output

Run: `cargo run --release` -- this will create a `target/release` folder.

Execute the following commands:

```
➜  rusties git:(main) ✗ target/release/rusties help
            
Rusty Libby 0.1.0
A command line app written in Rust that supports Math and String operations.

USAGE:
    rusties <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    math    Math operations like Factorial, Fibonacci, and Prime number check
    word    String operations like Palindrome and Reverse string
```

## Math functions:

#### Help menu:

```
➜  rusties git:(main) ✗ target/release/rusties math --help     
rusties-math 0.1.0

USAGE:
    rusties math <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    factorial       Factorial of a number
    fibonacci       Fibonacci of a number
    help            Prints this message or the help of the given subcommand(s)
    prime-or-not    Prime number check
```

#### Factorial:

```
➜  rusties git:(main) ✗ target/release/rusties math factorial 4
Factorial of 4 is Factorial of 4 is 24
```

## Word functions:

#### Help menu:

```
➜  rusties git:(main) ✗ target/release/rusties word --help               
rusties-word 0.1.0

USAGE:
    rusties word <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help              Prints this message or the help of the given subcommand(s)
    palindrome        Check if a word is a palindrome
    reverse-string    Reverse a string
```

#### Palindrome:

```
➜  rusties git:(main) ✗ target/release/rusties word palindrome madam
madam is a palindrome

➜  rusties git:(main) ✗ target/release/rusties word palindrome list 
list is not a palindrome
```

# Documentation Tests

```
➜  rusty_libby git:(main) ✗ cargo test
   Compiling rusty_libby v0.1.0 (/rusty_libby)
    Finished test [unoptimized + debuginfo] target(s) in 1.17s
     Running unittests src/lib.rs (target/debug/deps/rusty_libby-382d9f360a4a4dc8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rusty_libby

running 6 tests
test src/fibonacci.rs - fibonacci::fmt (line 9) ... ok
test src/palindrome.rs - palindrome::Palindrome (line 16) ... ok
test src/prime_or_not.rs - prime_or_not::fmt (line 9) ... ok
test src/factorial.rs - factorial::fmt (line 9) ... ok
test src/reverse_string.rs - reverse_string::ReverseString (line 9) ... ok
test src/palindrome.rs - palindrome::Palindrome (line 9) ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.02s

➜  rusty_libby git:(main) ✗ 
```
