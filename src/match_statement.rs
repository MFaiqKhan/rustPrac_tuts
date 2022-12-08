#![allow(unused)] // Allow unused code

use std::io;  // Import the io library from the standard library
use rand::Rng;  // what this library does is generate random numbers for us
use std::io::{Write, BufReader, BufRead, ErrorKind}; // bufreader is a buffered reader that reads from a file
use std::fs::File;
use std::cmp::Ordering;

pub fn matchh() {
    let age = 42;
    match age { // match is a control flow operator that allows us to compare a value against a series of patterns and then execute code based on which pattern matches
        1..=18 => println!("You are a child"),
        19..=24 => println!("You are a teenager"),
        25 | 30 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not an Important Birthday"), //  _ is a catchall value, it matches any left value
    };

    let voting_age = 18;
    match age.cmp(&voting_age) { // cmp() compares two values and can be used to compare two values of any type that implements the PartialOrd trait
        Ordering::Less => println!("You are not old enough to vote"), // Ordering is an enum that is part of the std::cmp module. It has three possible values: Less, Greater, and Equal
        Ordering::Greater => println!("You are old enough to vote"), // here we are using the Greater variant of the Ordering enum, which will be executed if the value of age is greater than the value of voting_age
        Ordering::Equal => println!("You are old enough to vote"),
    };

    // what is &voting_age? & is a reference, it allows us to refer to some value without taking ownership of it.
}


// In this code, the match statement compares the value of age against each of the patterns in the arms of the
// match expression. When it finds a pattern that matches the value of age, 
//it executes the code associated with that pattern. In this case, the code would print "you are a child" 
//because the value of age is 8, which matches the first arm of the match expression.