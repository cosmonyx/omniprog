



////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates file write and read using Rust standard library.
//
// -- It shows opening a file in write mode and writing data.
// -- It shows opening the same file in read mode.
// -- It shows reading file contents into a string.
// -- It shows simple verification of the read data.
//
#[cfg(prog131)]
mod prog {
use std::fs::OpenOptions;
use std::io::{Write, Read};

pub fn main() {

    // Open file in write mode (create if not exists, truncate if exists)
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("data.txt")
        .expect("Failed to open file for writing");

    let message = "Hello Rust";

    file.write_all(message.as_bytes())
        .expect("Write failed");

    // Open file in read mode
    let mut file = OpenOptions::new()
        .read(true)
        .open("data.txt")
        .expect("Failed to open file for reading");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Read failed");

    // Verify contents
    if contents == message {
        println!("Verification successful");
    } else {
        println!("Verification failed");
    }
}
}


////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates Rust struct and associated methods.
//
// -- It shows defining a struct with data fields.
// -- It shows implementing methods using an impl block.
// -- It shows an associated function acting as a constructor.
// -- It shows immutable and mutable methods.
// -- It shows creating and using struct instances.
//
#[cfg(prog110)]
mod prog {

struct Counter {
    value: i32,
}

impl Counter {
    // Associated function returning Self (current instance)
    fn new(arg: i32) -> Self {
        Self { value: arg }
    }

    // Takes immutable borrowed reference as arg (read-only access)
    fn get(&self) -> i32 {
        self.value
    }

    // Takes mutable borrowed reference as arg (allows modification)
    fn increment(&mut self) {
        self.value += 1;
    }
}

pub fn main() {
    let mut counter = Counter::new(0);

    counter.increment();
    counter.increment();

    println!("Counter value = {}", counter.get());
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates immutable and mutable borrows
//
// -- It shows immutable borrows allowing read-only access.
// -- It shows multiple immutable borrows can coexist.
// -- It shows mutable borrow allowing modification.
// -- It shows mutable borrow must be exclusive.
// -- It shows mutable and immutable borrows cannot coexist.
//
#[cfg(prog080)]
mod prog {
pub fn main() {

    // let value = 10;  // Owner with immutable binding - Does not allow mutable borrow
    let mut value = 10; // Owner with mutable binding

    value += 1;
    println!("value = {}", value);

    // Immutable borrows (multiple allowed - read-only access) /////////////////
    let r1 = &value;
    let r2 = &value;

    // *r1 += 1;   // Cannot modify through immutable reference
    // value += 1; // Cannot modify while immutably borrowed

    println!("value = {} r1 = {}, r2 = {}", value, r1, r2); // Can read all

    // immutable borrows end here //////////////////////////////////////////////

    // Mutable borrow (exclusive access) ///////////////////////////////////////

    let r3 = &mut value;

    // let r5 = &mut value; // Cannot have second mutable borrow
    // let r5 = &value;     // Cannot have immutable borrow while mutable borrow exists
    // value += 1;          // Cannot modify owner while mutably borrowed
    // println!("value = {} r1 = {}, r2 = {}", value, r1, r2); // Cannot read r1, r2, or value as mutable borrow is exclusive

    *r3 += 5;
    println!("r3 = {}", r3); // Mutable borrow ends here ///////////////////////

    // mutable borrow ends here ////////////////////////////////////////////////

    // Immutable borrows allowed again /////////////////////////////////////////
    let r1 = &value;
    let r2 = &value;
    println!("r1 = {}, r2 = {}", r1, r2);

    // Immutable / mutable borrows using String (another example) //////////////

    let mut value = String::from("Hello"); // Owner with mutable binding

    // Immutable borrows (multiple allowed - read-only access)
    let r1 = &value;
    let r2 = &value;

    println!("value = {} r1 = {}, r2 = {}", value, r1, r2);

    // Mutable borrow (exclusive access)
    let r3 = &mut value;

    r3.push_str(" World");
    println!("r3 = {}", r3);
}
}

