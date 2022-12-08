#![allow(unused)] // Allow unused code

use std::io;  // Import the io library from the standard library
use rand::Rng;  // what this library does is generate random numbers for us
use std::io::{Write, BufReader, BufRead, ErrorKind}; // bufreader is a buffered reader that reads from a file
use std::fs::File;
use std::cmp::Ordering;

pub fn iff() {
    let age = 8;
    if (age >=1) && (age <= 12) {
        println!("You are a child");
    } else if (age >= 13) && (age <= 19) {
        println!("You are a teenager");
    } else if age >= 24 && age <=30 {
        println!("You are an adult ");  
    } else {
        println!("You are old");
    }
}