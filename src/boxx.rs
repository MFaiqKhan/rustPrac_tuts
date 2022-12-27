trait Vehicle{
    fn drive(&self); // what is &self? it is a reference to the object that is calling the method
}

struct Truck;

impl Vehicle for Truck{ // impl is used to implement a trait for a struct
    fn drive(&self) { // here truck is the object that is calling the method
        println!("Truck is Driving")
    }
}

pub fn boxx() {
    // we will use box because we want to store the value on the heap, as the size of the value is not known at compile time
    // because anything can implement vehicle trait, we can store any type of value in the box

    // box is stroed on the heap, and the value is stored on the heap, slower than stack
    let veh: Box<dyn Vehicle>; // dyn is written before the trait name to indicate that the trait is a dynamic trait
    veh = Box::new (Truck);
    veh.drive();
}


// second use case for box is recursive data types
// e.g we have a struct and one of that field of that struct is the struct that it is enclosed in.
// like: struct Node { value: i32, next: Box<Node> }

// memory in box will be freed when the box goes out of scope