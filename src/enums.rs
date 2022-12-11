#![allow(unused)] // Allow unused code

use std::io;
use std::process::Output;  // Import the io library from the standard library
use rand::Rng;  // what this library does is generate random numbers for us
use std::io::{Write, BufReader, BufRead, ErrorKind}; // bufreader is a buffered reader that reads from a file
use std::fs::File;
use std::cmp::Ordering;

pub fn enums ( ) {
   pub enum Directions { // enums are used to define a set of possible values that a variable can hold
        Up,
        Down,
        Left,
        Right,
    }

    // what this does is define a method called is_left() that can be called on a Directions value
    
    impl Directions {  // impl is used to define an implementation of a struct or enum
        pub fn is_left(&self) -> bool { // self is used to refer to the current instance of the struct or enum
            match self { // match is used to compare a value against a series of patterns and then execute code based on which pattern matches the value
                Directions::Left => true, // if the value of self is Directions::Left, then return true
                _=> false, // if the value of self is not Directions::Left, then return false
            } 
        }
   }
    
    let the_direction:Directions = Directions::Left; // the_direction is of type Directions and it holds the value of the direction that the person or object is currently moving in
    // output the direction that the person or object is moving in
    // If the value of the_direction is Directions::Up, then print "Going Up"

    match the_direction {  
        Directions::Up => println!("Going Up"),
        Directions::Down => println!("Going Down"),
        Directions::Left => println!("Going Left"),
        Directions::Right => println!("Going Right"),
    }

    println!("Is Left: {}", the_direction.is_left());
}


/* 
This code defines a function called enums that uses an enum to represent a set of directions. 
The Directions enum is defined with four variants: Up, Down, Left, and Right. 
These variants are used to represent the four possible directions that a person or object can move in.

The Directions enum is marked as pub, which means that it can be used outside of the enums function where it is defined. 
This allows the Directions enum to be used in other parts of the program.

The enums function also defines an implementation of the Directions enum that provides a method called is_left().
 This method uses a match expression to determine whether the Directions value it is called on is Directions::Left. 
 If it is, the method returns true, otherwise it returns false.

Inside the enums function, a variable called the_direction is defined and initialized with the value Directions::Left.
This variable is of type Directions and it holds the value of the direction that the person or object is currently
moving in.

Next, the match expression is used to print a message indicating the direction that the person or object is moving in */