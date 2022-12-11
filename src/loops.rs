#![allow(unused)] // Allow unused code

use std::io;  // Import the io library from the standard library
use rand::Rng;  // what this library does is generate random numbers for us
use std::io::{Write, BufReader, BufRead, ErrorKind}; // bufreader is a buffered reader that reads from a file
use std::fs::File;
use std::cmp::Ordering;


/* 

The loop keyword is used to create a loop that will continue indefinitely until it is explicitly broken out of using the
break keyword. This type of loop is useful when you want to create an infinite loop that will only 
exit when certain conditions are met.

The while loop is similar to the loop keyword, but it will only continue to execute as long as a specified condition
is true. This type of loop is useful when you want to repeat a certain action until a certain condition is met.

The for loop is used to iterate over a collection of items, such as an array or a vector. 
This type of loop is useful when you want to perform the same action on each item in a collection.

*/


pub fn loops() {
// multiple ways to loop in rust

let arr_a = [1,2,3,4,5,6,7,8,9,10];

let mut loop_idx = 0;// why mut? because we are going to change the value of the variable in the loop
// and index starts at 0 and changes by 1 each time the loop runs

// infinite loop, if condition is not met it will continue to loop

loop { 

    if arr_a[loop_idx] % 2 == 0 { // if the value at the index is even
        loop_idx +=1;
        continue; // continue to next iteration of loop in this case(if)
    }
    if arr_a[loop_idx] == 7 { // if the value at the index is 7
        break; // break out of the loop
    }
    println!("Value at loop_idx ' {} ': is {}",loop_idx, arr_a[loop_idx]); // print the value at the index
    loop_idx +=1; // increment the index by 1, otherwise it will be an infinite loop

};

println!("-----------------------------------------");

// while loop
let arr_b = [1,2,3,4,5,6,7,8,9,10];
let mut loop_idx2 = 0;
while loop_idx2 < arr_b.len() { // while the index is less than the length of the array
    println!("Value at loop_idx2 ' {} ': is {}",loop_idx2, arr_b[loop_idx2]); // print the value at the index
    loop_idx2 +=1; 
};

println!("-----------------------------------------");

// for loop
let arr_c = [1,2,3,4,5,6,7,8,9,10];
let mut loop_idx3 = 0;
for val in arr_c.iter() { // for each value in the array
    println!("Value at loop_idx3 ' {} ': is {}",loop_idx3, val); // print the value at the index
    loop_idx3 +=1; 
};

}






/*     let x = String::from("hello");
    let y = x;

    println!("{}", x); */

/* let mut c = String::from("hello");

    {
        let z = &mut c;
        z.push_str(", world");
        println!("{}", c) // hello, world
    }

    println!("{}", c); */