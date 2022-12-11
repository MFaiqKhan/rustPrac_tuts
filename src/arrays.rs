#![allow(unused)] // Allow unused code

use std::io;  // Import the io library from the standard library
use rand::Rng;  // what this library does is generate random numbers for us
use std::io::{Write, BufReader, BufRead, ErrorKind}; // bufreader is a buffered reader that reads from a file
use std::fs::File;
use std::cmp::Ordering;


pub fn arrays() {
    let arr_first = [2,22,222,2222];
    println!("Output Element is {}, Array total length is {} ",arr_first[0], arr_first.len());
}

// can contains multiple data types
// elements must be of same data type in an array
// arrays are fixed length, once declared cannot change the length