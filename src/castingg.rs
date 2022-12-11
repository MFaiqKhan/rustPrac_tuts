#![allow(unused)] // Allow unused code

use std::io;  // Import the io library from the standard library
use rand::Rng;  // what this library does is generate random numbers for us
use std::io::{Write, BufReader, BufRead, ErrorKind}; // bufreader is a buffered reader that reads from a file
use std::fs::File;
use std::cmp::Ordering;

pub fn casting() {
    let int_u8: u8 = 255;
    let int_u16: u16 = 65535;

    println!("u8: {}", int_u8);
    println!("u16: {}", int_u16);

    let int_32: u32 = int_u8 as u32 + int_u16 as u32; // casting u8 to u32 and u16 to u32, if not casted, the compiler will throw an error
    println!("u32: {}", int_32);
}