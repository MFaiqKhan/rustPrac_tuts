#![allow(unused)] // Allow unused code

use std::io;  // Import the io library from the standard library
use rand::Rng;  // what this library does is generate random numbers for us
use std::io::{Write, BufReader, BufRead, ErrorKind}; // bufreader is a buffered reader that reads from a file
use std::fs::File;
use std::cmp::Ordering;

pub fn vectors () {
    let vec:Vec<i32> = Vec::new(); // Vec::new() creates a new empty vector of type i32
    println!("vec: {:?}", vec); // {:?} is a placeholder for the vec variable

    let mut vec2 = vec![1,2,3,4,5]; // vec! is a macro that creates a new vector with the values passed to it
    println!("vec2: {:?}", vec2);
    println!("First Index {}", vec2[0]); // indexing a vector is done using square brackets (

    vec2.push(6); // push() adds a new element to the end of the vector

    println!("vec2: {:?}", vec2); // vec2: [1, 2, 3, 4, 5, 6]
    println!("5th Index {}", vec2[5]); // 5th Index 6

    let second : &i32 = &vec2[1]; // & is used to create a reference to the value at the index 1
    println!("Second Index {}", second); // Second Index , 2
    match vec2.get(1) { // get() returns an Option<&T> which is either Some(&T) or None
        Some(second) => println!("Second Index {}", second),  // Some(&T) is a reference to the value at the index 1
        None => println!("There is no second element") // None is returned if the index is out of bounds
    }

    for i in &vec2 { //  & is used to create a reference to the value at the index i   
        println!("i: {}", i); //  i: 1, 2, 3, 4, 5, 6
    }

    for i in &mut vec2 { //  &mut is used to create a mutable reference to the value at the index i   
        *i *= 2; // * is used to dereference the value at the index i and multiply it by 2
        println!("i: {}", i); // i: 2, 4, 6, 8, 10, 12
    }

    println!("vec2: {:?}", vec2); // vec2: [2, 4, 6, 8, 10, 12]

    let row = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]]; // 2D vector
    println!("row: {:?}", row); // row: [[1, 2, 3], [4, 5, 6], [7, 8, 9]]

    println!("Vec Length: {}", vec2.len()); // len() returns the number of elements in the vector
    println!("Vec Capacity: {}", vec2.capacity()); // capacity() returns the number of elements the vector can hold without reallocating
    println!("Vec is Empty: {}", vec2.is_empty()); // is_empty() returns true if the vector contains no elements
    println!("Vec Contains 2: {}", vec2.contains(&2)); // contains() returns true if the vector contains the specified element

    println!("Pop : {:?}", vec2.pop()); // pop() removes the last element from the vector and returns it in an Option<T>




}

/* 
Overall, a Vector in Rust is a type of data structure that stores a sequence of values of the same type.
 It is dynamically sized, which means that the number of elements in a Vector can grow or shrink as needed. 
You can create a Vector, add elements to it, and access its elements using indexing syntax. 

A Vector is declared using the Vec<T> syntax, where T is the type of the values that the Vector will store.
 For example, to create a Vector that stores i32 values, you would write Vec<i32>.

*/