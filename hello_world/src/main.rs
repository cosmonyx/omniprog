////////////////////////////////////////////////////////////////////////////////
//
// This is a basic Rust program.
// -- It uses the chrono crate as an external dependency.

use chrono::Local;

fn main() {
    println!("Hello World");
    println!("Current time: {}", Local::now());
}