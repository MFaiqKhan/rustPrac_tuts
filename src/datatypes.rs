#![allow(unused)] // Allow unused code

use std::io;  // Import the io library from the standard library
use rand::Rng;  // what this library does is generate random numbers for us
use std::io::{Write, BufReader, BufRead, ErrorKind}; // bufreader is a buffered reader that reads from a file
use std::fs::File;
use std::cmp::Ordering;

pub fn datatypes() {
 // statically typed language, so have to define types of a variable

    // unsigned integers : u8, u16, u32, u64, u128, usize
    // signed integers : i8, i16, i32, i64, i128, isize

    println!("Max u32 is : {}", u32::MAX); // u32::MAX is a constant that is defined in the u32 data type, it is a static method, MAX means maximum value of u32
    println!("Max u64 is : {}", u64::MAX);
    println!("Max u128 is : {}", u128::MAX);
    println!("Max usize is : {}", usize::MAX);
    println!("Max f32 is : {}", f32::MAX);
    println!("Max f64 is : {}", f64::MAX);

    // boolean type
    let is_active: bool = !false; // boolean type is either true or false
    println!("Is active : {}", is_active);
}