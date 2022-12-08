#![allow(unused)] // Allow unused code

use std::io;  // Import the io library from the standard library
use rand::Rng;  // what this library does is generate random numbers for us
use std::io::{Write, BufReader, BufRead, ErrorKind}; // bufreader is a buffered reader that reads from a file
use std::fs::File;
use std::cmp::Ordering;

pub fn maths() {
    let mut num_3: u32 = 10;
    let num_4: u32 = 5;
    println!("{} + {} = {}", num_3, num_4, num_3 + num_4);	
    println!("{} - {} = {}", num_3, num_4, num_3 - num_4);
    println!("{} * {} = {}", num_3, num_4, num_3 * num_4);
    println!("{} / {} = {}", num_3, num_4, num_3 / num_4);
    println!("{} % {} = {}", num_3, num_4, num_3 % num_4);
    num_3 +=1; // num_3 = num_3 + 1
}