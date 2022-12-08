#![allow(unused)] // Allow unused code



use std::io;  // Import the io library from the standard library
use rand::Rng;  // what this library does is generate random numbers for us
use std::io::{Write, BufReader, BufRead, ErrorKind}; // bufreader is a buffered reader that reads from a file
use std::fs::File;
use std::cmp::Ordering;

pub fn variable() {
    const ONE_MIL: u32 = 1_000_000; // need to define data type in const variable declaration , let is optional, u32 is unsigned 32 bit integer
    const PI:f32 = 3.142; // f32 is 32 bit floating point number

    let age:&str = "47"; // this is a string, string is defined in double quotes , char is defined in single quotes
    
    // shadowing : we can change the value of a variable by using the same variable name and repeating the use of the let keyword as follows:
    // we can do in rust is that: define variable with same name but with different data type , called shadowing
    let mut age: u32 = age.trim().parse() // here we actually convert the string to u32, trim and parse are acting as a method on the string which will remove the trailing newline character from the string and convert the string to u32.
        .expect("Age wasn't assigned a numbe"); // we can easily build up error handling by using the expect() method directly as we code, beauty of rust
        age = age + 1;  //  not we are adding 1 to the age variable
        println!("I'm {} years old and I want ${}", age, ONE_MIL); 
}








// const ONE_MIL: u32 = 1_000_000; // = 1000000 the underscore is used to make the number more readable and avoid accidental errors, it acts as a delimiter, nothing more nothing less
// https://stackoverflow.com/questions/37877381/what-is-the-difference-between-immutable-and-const-variables-in-rust
