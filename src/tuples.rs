#![allow(unused)] // Allow unused code

use std::io;  // Import the io library from the standard library
use rand::Rng;  // what this library does is generate random numbers for us
use std::io::{Write, BufReader, BufRead, ErrorKind}; // bufreader is a buffered reader that reads from a file
use std::fs::File;
use std::cmp::Ordering;

pub fn tuple() {
    let my_tup:(u8, String, f64, bool) = (20, "faiq".to_string(), 3.14, true); // tuples can hold different data types
    println!("My Name is: {}", my_tup.1); // tuples are indexed starting from 0, and accessing a tuple element is done using the dot notation followed by the index number of the element you want to access

    let (t1,t2,t3, t4) = my_tup;
    println!("age: {}", t1);
}

