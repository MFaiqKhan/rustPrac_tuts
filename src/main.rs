// The connection between a program and a data source or destination is called a stream. An input stream handles data flowing into a program. An output stream handles data flowing out of a program.
#![allow(unused)] // Allow unused code

use std::io;  // Import the io library from the standard library
use rand::Rng;  // what this library does is generate random numbers for us
use std::io::{Write, BufReader, BufRead, ErrorKind}; // bufreader is a buffered reader that reads from a file
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("What's your name?");
    let mut name = String::new(); // String::new() creates a new empty string, mutable variable
    let greeting = "Nice to meet you, "; // &str is a string literal, immutable variable
    io::stdin().read_line(&mut name) // this code is used to read a line of input from the user and store it in the name variable. The expect() method is used to handle any errors that might occur while reading the line of input.
        .expect("Failed to read line"); // read_line() takes a mutable reference to a string as an argument
    println!("Hello, world!"); // println! is a macro that prints to the console , everything ending with a ! is a macro

    println!("Hello {}! {}", name.trim_end(), greeting); // trim_end() removes the trailing newline character from the string, and {} acting as a space holder for the arguments passed after the string literal in the println! macro call 
}



// difference b/w BufReader and BufRead is that BufReader is a buffered reader that reads from a file and BufRead is a trait that is implemented by BufReader and other buffered readers

// The file actually compiled on Run through gcc compiler when I actually put the config file in bin folder under the main cargo project folder

// Every variable inside rust is immutable by default, to make it mutable we have to use mut keyword

// The :: operator is used for both associated functions and namespaces

// rust have immutable variables because it safes a lot of time and memory for the compiler to compile the code, plus safety and security

// difference b/w String and &str is that String is a heap allocated string and &str is a string literal

// In Rust, error handling is built into the language itself, 
//rather than being provided by a library like in some other languages. 
//This means that the Rust compiler can statically check for error handling and 
//will not allow the program to compile if it detects any issues with the error handling in the code.