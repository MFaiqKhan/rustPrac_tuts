#![allow(unused)] // Allow unused code

use std::io;  // Import the io library from the standard library
use rand::Rng;  // what this library does is generate random numbers for us
use std::io::{Write, BufReader, BufRead, ErrorKind}; // bufreader is a buffered reader that reads from a file
use std::fs::File;
use std::cmp::Ordering;

pub fn sstrings() {
    //static method is called to create an empty string value, and this value is then assigned to st1
    let mut st1 = String::new(); 
    //println!("{}", st1); // prints an empty string

    // You can then use methods on the String type to modify the value of st1,
    // such as by adding characters to the string or changing its case.

    st1.push('a'); // push() method is used to add a character to the end of the string
    st1.push_str("  word"); // push_str() method is used to add a string slice to the end of the string
    println!("{}", st1); // prints abc

    // In this case, since the string contains only individual characters with no whitespace, 
    //the split_whitespace() method will split the string into individual characters.
    // The resulting iterator is then used in the for loop to iterate over the characters in the string.

    for w in st1.split_whitespace() {
        println!("{}", w); 
    }

    let st2 = st1.replace("  word", "b"); // replace() method is used to replace all occurrences of a substring with another substring
    println!("{}", st2); // prints ab  

    let st3 = String::from("x c c s f f w f q f a f a f a g t q k l l a"); // String::from() is used to create a string from a string literal
    println!("{}", st3); // prints x c c s f f w f q f a f a f a g t q k l l a

    let mut v1: Vec<char> = st3.chars().collect();// chars() method is used to convert a string into a vector of characters and collect() method is used to collect the characters into a vector
    v1.sort(); // sort() method is used to sort the characters in the string, outut: a a a a a c c f f f f f f g k l l q q s t w x
    println!("{:?}", v1); // prints ["a", "a", "a", "a", "a", "c", "c", "f", "f", "f", "f", "f", "f", "g", "k", "l", "l", "q", "q", "s", "t", "w", "x"]

    v1.dedup(); // dedup() method is used to remove all consecutive duplicate characters from the string, output: a c f g k l q s t w x
    println!("{:?}", v1); // prints ["a", "c", "f", "g", "k", "l", "q", "s", "t", "w", "x"]

    for (i, c) in v1.iter().enumerate() { // iter() method is used to iterate over the characters in the string and enumerate() method is used to get the index of each character.
        println!("{}: {}", i, c); // prints 0: a, 1: c, 2: f, 3: g, 4: k, 5: l, 6: q, 7: s, 8: t, 9: w, 10: x
    }

    let st4: &str = "HELLO WORLD"; // string literal
    let mut st5: String = st4.to_string(); // string from string literal
    println!("{}", st5); // prints HELLO WORLD

    let byte_arr1 = st5.as_bytes(); // as_bytes() method is used to convert the string into an array of bytes
    println!("{:?}", byte_arr1); // prints [72, 69, 76, 76, 79, 32, 87, 79, 82, 76, 68]

    let st6 = &st5[0..6]; // string slice
    println!("{}, string length : {}", st6, st6.len()); // prints HELLO

    st5.clear(); // clear() method is used to clear the string, making it equal to ""
    println!("{}", st5); // prints nothing

    // Combine String: 
let st7 = String::from("This is ");
let st8 = String::from("a word");

let st9 = st7 + &st8; // + operator is used to combine two strings, but this method is not very efficient because it requires allocating a new string and copying the contents of the two strings into it.
println!("{}", st9); // prints This is a a word

for char in st9.bytes() {
    println!("{}", char); // prints 84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 97, 32, 119, 111, 114, 100
}


}


// In Rust, the String::from() method is a static method that is used to create a new String from a given string slice.
// This method is often used when you want to convert a string literal or a string slice into a String so that
// you can use the methods of the String type on it.

// what is {:?} in println!()?
// {:?} is a placeholder for the Debug trait, which is used to print the value of a variable in a human-readable format.