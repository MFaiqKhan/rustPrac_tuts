use std::rc::Rc;
#[derive(Debug)] // this is used to print the struct



struct Truckk {
    capacity: i32,
}

pub fn rcc() {
    // Rc is a reference counted pointer, it is used to share data between multiple owners
    // Rc is used when we want to share data between multiple owners, but we don't want to transfer ownership

    let (t1, t2, t3) = (Rc::new(Truckk { capacity: 1 }), Rc::new(Truckk { capacity:2 }), Rc::new(Truckk { capacity: 3 })); // Rc::new is used to create a new Rc pointer
    // what is doing is creating a new Rc pointer and storing the data on the heap

    let warehouse_1 = vec![t1, Rc::clone(&t2)]; // Rc::clone is used to clone the Rc pointer, it does not clone the data, it just increments the reference count
    let warehouse_2 = vec![Rc::clone(&t2), t3];

    // Rc::clone is used to clone the Rc pointer, it does not clone the data

    println!("{:?}", warehouse_1);
    println!("{:?}", warehouse_2);

    println!("{}",Rc::strong_count(&t2)); // strong_count returns the number of references to the data

    std::mem::drop(warehouse_1); // std::mem::drop is used to drop the Rc pointer, it decrements the reference count

    println!("{}",Rc::strong_count(&t2)); // strong_count returns the number of references to the data

    // strong_count returns the number of references to the data
}