// https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html // this is a good resource for macros

use std::collections::HashSet;

// A simple macro;
// macros should be defined before they are used
macro_rules! say_hello {
    () => (
        println!("Hello!");
    );
    ("Rust") => { // here , the input is a string literal, could be any other type, its a specific literal
        println!("Hello, Rust!!")
    };

    // here , the input is a string literal, could be any other type, the $name is an arbitrary literal only, no other type can be used
    ($name: literal) => { println!("Hello, {}!", $name)};
    
    
    ($name: ident) => { println!("Hello, {}!", $name)}; // it is a variable name, not a literal
}

macro_rules! say_hello2 {
    ($name: expr) => { // expr is a general expression, it can be a literal or a variable
        println!("Hello, {}!", $name);
    };
}

// repitition 
macro_rules! set {
    ($($s: expr), *)  => // this is a repitition, it will match any number of expressions
        {
            HashSet::from([$($s), *]) // this is a set literal, it is a macro that takes any number of expressions and creates a set from them
            // does it create random sets? no, it creates a set from the expressions passed to it
            // are they in order? no, they are not in order
        }
}

// macros takes input code and produces output code, that is included in the compiled program

// println! is a macro that prints a string to the console, it takes string slice as first argument 
//and any number of variables as second argument
pub fn printt() {
    let name = "Rust";
    println!("Hello, {}!", name);

    say_hello!();
    say_hello!("Rust");
    say_hello!("Rustaceans");
    let name = "this is ident";
    say_hello!(name);
    say_hello2!(name);
    say_hello2!("this is expr");

    let set = set!(1, 2, 3); // what happens here is that the set! macro is expanded to the code below
    dbg!(set); // here , the dbg! macro is expanded to the code below
    // whats a dbg macro? it is a macro that prints the value of the expression passed to it
}

// you can also create you own macros , there exists a macro that takes html code and output rust code.
