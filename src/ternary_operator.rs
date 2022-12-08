#![allow(unused)] // Allow unused code

use std::io;  // Import the io library from the standard library
use rand::Rng;  // what this library does is generate random numbers for us
use std::io::{Write, BufReader, BufRead, ErrorKind}; // bufreader is a buffered reader that reads from a file
use std::fs::File;
use std::cmp::Ordering;

pub fn ternary_operator() {
    let mut my_age: u8 = 20;
    let can_vote: bool = if my_age >= 18 {true} else {false}; // this is a ternary operator in rust as "?" is foor handling the errors, its not for ternary operator
    println!("Can Vote: {}", can_vote);
}

// basically rust dont have ternary operator, but we can use if else statement to achieve the same result

// Rust does not have the ternary operator because it's not needed. Everything evaluates to some value, and if / else statements are no exception:

//let r = 42.42;
//let sgn_r = if r >= 0. { 1. } else { -1. };