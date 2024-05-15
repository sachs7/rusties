# rusties

Basic Rust library and its usage

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
    math    
    word
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
