#![allow(unused)] // Allow unused code

use std::io;  // Import the io library from the standard library
use rand::Rng;  // what this library does is generate random numbers for us
use std::io::{Write, BufReader, BufRead, ErrorKind}; // bufreader is a buffered reader that reads from a file
use std::fs::File;
use std::cmp::Ordering;

pub fn random() {
    let random_num = rand::thread_rng( ).gen_range(1..100); // gen_range is a method on the thread_rng() method, it generates a random number between 1 and 100
    // thread_rng() is a function that returns a random number generator, it is a thread local random number generator
    println!("Random number is : {}", random_num);
}

//In the context of the rand library in Rust, the word "thread" refers to a concurrent execution path in a computer program. In other words, a thread is a separate flow of control within a program that can be executed concurrently with other threads.

//In a multithreaded program, each thread has its own local data and execution state, but can also share data and resources with other threads. This can be useful for improving the performance and scalability of a program, as it allows different parts of the program to be executed in parallel on different CPU cores.

//The thread_rng function in the rand library is specifically designed to be used in a multithreaded environment. 
//It returns a reference to a thread-local RNG, which means that each thread in the program will have its own 
//instance of the RNG that is local to that thread. This can help to avoid synchronization issues and 
//data races when multiple threads need to generate random numbers.