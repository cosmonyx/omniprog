fn main() {
    prog::main();
}

////////////////////////////////////////////////////////////////////////////////
//
// This is a basic Rust program.
// -- Here execution begins from main().

#[cfg(prog010)]
mod prog {
pub fn main() {
    println!("Hello World"); // ! indicates a macro invocation i.e. println! is a macro
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates Rust printing to standard output using println! macro.
//
// -- It shows positional {} placeholders for value formatting.
// -- It shows positional and indexed placeholders for value formatting.
// -- It shows named placeholders for value formatting.
// -- It shows formatting traits like binary, hex, and float precision.

#[cfg(prog020)]
mod prog {
pub fn main() {

    let name = "Neeraj";
    let age = 30;

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("{} is {} years old.", name, age);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    println!("Binary: {:b}", 10);
    println!("Hex: {:x}", 255);
    println!("Float: {:.2}", 3.14159);
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates let binding in Rust. Let binding means creating a variable 
// by binding a value to a name using the let keyword.
//

#[cfg(prog021)]
mod prog {
pub fn main() {

    // Explicit data type
    let a: i32 = 1000;

    // Data type inference
    let b = 42;              // inferred as i32

    println!("a={}, b={}", a, b);
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It shows basic Rust data types and type inference.
//
// -- It shows how the compiler deduces types.
//
// std::mem is a Rust standard library module that provides low-level 
// memory utilities such as size, alignment, and swapping.

#[cfg(prog030)]
mod prog {
pub fn main() {
    // Explicit data types
    let a: i8 = -10;         // signed 8-bit integer
    let b: i16 = -200;       // signed 16-bit integer
    let c: i32 = -1000;      // signed 32-bit integer
    let d: i64 = -5000;      // signed 64-bit integer

    let e: u8 = 10;          // unsigned 8-bit integer
    let f: u16 = 200;        // unsigned 16-bit integer
    let g: u32 = 1000;       // unsigned 32-bit integer
    let h: u64 = 5000;       // unsigned 64-bit integer

    let i: f32 = 3.14;       // 32-bit floating point
    let j: f64 = 2.71828;    // 64-bit floating point

    let k: bool = true;      // boolean
    let l: char = 'R';       // character
    let m: &str = "Hello";   // string slice

    // Data type inference
    let p = 42;              // inferred as i32
    let q = 2.5;             // inferred as f64
    let r = false;           // inferred as bool
    let s = 'A';             // inferred as char
    let t = "Rust";          // inferred as &str

    println!("a={}, type=i8, size={}", a, std::mem::size_of::<i8>());
    println!("b={}, type=i16, size={}", b, std::mem::size_of::<i16>());
    println!("c={}, type=i32, size={}", c, std::mem::size_of::<i32>());
    println!("d={}, type=i64, size={}", d, std::mem::size_of::<i64>());

    println!("e={}, type=u8, size={}", e, std::mem::size_of::<u8>());
    println!("f={}, type=u16, size={}", f, std::mem::size_of::<u16>());
    println!("g={}, type=u32, size={}", g, std::mem::size_of::<u32>());
    println!("h={}, type=u64, size={}", h, std::mem::size_of::<u64>());

    println!("i={}, type=f32, size={}", i, std::mem::size_of::<f32>());
    println!("j={}, type=f64, size={}", j, std::mem::size_of::<f64>());

    println!("k={}, type=bool, size={}", k, std::mem::size_of::<bool>());
    println!("l={}, type=char, size={}", l, std::mem::size_of::<char>());
    println!("m={}, type=&str, size={}", m, std::mem::size_of::<&str>());

    println!("p={}, type=i32, size={}", p, std::mem::size_of::<i32>());
    println!("q={}, type=f64, size={}", q, std::mem::size_of::<f64>());
    println!("r={}, type=bool, size={}", r, std::mem::size_of::<bool>());
    println!("s={}, type=char, size={}", s, std::mem::size_of::<char>());
    println!("t={}, type=&str, size={}", t, std::mem::size_of::<&str>());

    // Pointer size
    println!("pointer size = {} bytes", std::mem::size_of::<*const u8>());
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates Rust fixed-size arrays.
//
// -- It shows arrays of integers, characters, strings, and booleans.
//
#[cfg(prog040)]
mod prog {

pub fn main() {

    // Array with explicit type and size
    let mut arr: [i32; 3] = [10, 20, 30];

    println!("arr elements (initial):");
    for v in arr.iter() {
        println!("{}", v);
    }

    // Assign values
    for i in 0..arr.len() {
        arr[i] = (i as i32) + 1; // (i as i32) converts i from usize to i32
    }

    // Increment array values in a loop
    for i in 0..arr.len() {
        arr[i] += 1;
    }

    println!("arr elements (after increment):");
    for v in arr.iter() {
        println!("{}", v);
    }

    // Array length
    println!("arr length = {}", arr.len());

    // Size of array in memory
    println!("size of arr = {}", std::mem::size_of_val(&arr));

    // Array with inferred type and size
    let arr2 = [1, 2, 3, 4, 5];

    println!("arr2 elements:");
    for v in arr2.iter() {
        println!("{}", v);
    }

    // Array of size 5 initialized with the same value 7
    let arr3 = [7; 5];

    println!("arr3 elements:");
    for v in arr3.iter() {
        println!("{}", v);
    }

    // Array of characters
    let arr4 = ['R', 'u', 's', 't'];

    println!("arr4 characters:");
    for v in arr4.iter() {
        println!("{}", v);
    }

    // Array of string slices
    let arr5 = ["Hello", "Rust", "World"];

    println!("arr5 strings:");
    for v in arr5.iter() {
        println!("{}", v);
    }

    // Array of booleans
    let arr6 = [true, false, true, false];

    println!("arr6 booleans:");
    for v in arr6.iter() {
        println!("{}", v);
    }
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates heap allocation of arrays
//
// -- It shows allocating a fixed-size integer buffer on the heap using Box.
// -- It shows allocating a growable-size integer buffer on the heap using Vec.
// -- The heap memory is automatically freed when the variables go out of scope.
// -- Box<T> and Vec<T> are generic types that take a type argument.
//
#[cfg(prog050)]
mod prog {

pub fn main() {

    // Allocate buffer of integers on heap using Box
    let mut buf: Box<[i32; 5]> = Box::new([0; 5]);

    // Initialize buffer
    for i in 0..buf.len() {
        buf[i] = (i as i32) * 10;
    }

    // Print buffer contents
    println!("Box buffer contents:");
    for v in buf.iter() {
        println!("{}", v);
    }

    // Allocate growable buffer using Vec
    let mut vec: Vec<i32> = vec![0; 5];

    for i in 0..vec.len() {
        vec[i] = (i as i32) * 10;
    }

    println!("Vec buffer contents:");
    for v in vec.iter() {
        println!("{}", v);
    }
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates mutable and immutable variables in Rust.
//
// -- It shows immutable bindings (default behavior).
// -- It shows mutable bindings using the mut keyword.
// -- It shows that immutable variables cannot be modified.
// -- It shows that mutable variables allow value updates.
//
#[cfg(prog070)]
mod prog {
pub fn main() {

    // Immutable variable (default)
    let x = 10;
    println!("x = {}", x);
    // x = 20; // Not allowed (immutable)

    // Mutable variable
    let mut y = 10;
    println!("y = {}", y);

    y = 20; // Allowed (mutable)
    println!("y updated = {}", y);
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

    let mut value = 10; // Owner with mutable binding

    value += 1;
    println!("value = {}", value);

    // Immutable borrows (multiple allowed - read-only access) /////////////////
    let r1 = &value;
    let r2 = &value;

    // let r3 = &mut value;        // Cannot have mutable borrow 
    // *r1 += 1;                   // Cannot modify through immutable reference
    // value += 1;                 // Cannot modify owner while immutably borrowed
    println!("value = {}", value); // Can read owner

    println!("r1 = {}, r2 = {}", r1, r2); // immutable borrows end here as it is a last access

    // Mutable borrow (exclusive access) ///////////////////////////////////////

    let r4 = &mut value;

    // let r5 = &mut value;        // Cannot have second mutable borrow
    // let r6 = &value;            // Cannot have immutable borrow
    // value += 1;                 // Cannot modify owner while mutably borrowed
    // println!("value = {}", value); // Cannot read owner

    *r3 += 5;
    println!("r3 = {}", r3);       // Mutable borrow ends here as it is a last access
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates how Rust enforces memory safety at compile time.
//
// -- Immutable borrows (&T) allow multiple readers simultaneously but they
//    don not allow data modification. This prevents accidental modification 
//    while data is being shared.
// -- Mutable borrows (&mut T) provide exclusive access. This prevents data
//    races and ensures safe modification of the data.
// -- Ownership ensures only one owner of data at a time, preventing 
//    double free and use-after-free.
// -- Lifetime check ensures references never outlive the data they point to.
// -- Values are automatically dropped when they go out of scope.
// -- Bounds-check prevents out-of-bounds memory access.
//

#[cfg(prog081)]
mod prog {
pub fn main() {

    let mut value = String::from("Hello"); // Owner with mutable binding

    // Immutable borrows (multiple allowed - read-only access)
    let r1 = &value;
    let r2 = &value;

    println!("r1 = {}, r2 = {}", r1, r2);

    // Mutable borrow (exclusive access)
    let r3 = &mut value;

    r3.push_str(" World");
    println!("r3 = {}", r3);

    // Ownership transfer (move) prevents use-after-free and double free
    let s1 = String::from("hello");
    let s2 = s1;                     // ownership moved
    println!("s2 = {}", s2);
    // println!("{}", s1);           // compile error: use-after-move

    // Automatic cleanup when scope ends
    {
        let v = vec![1, 2, 3];
        println!("vector = {:?}", v);
    } // v is dropped here

    // Automatic cleanup when scope ends - another example
    {
        let mut b = Box::new(10);
        println!("box value = {}", b);

        // Reassignment drops the previous heap allocation automatically
        b = Box::new(20);
        println!("box reassigned = {}", b);
    } // Box is dropped here automatically

    // Bounds-checked indexing
    {
        let arr = [10, 20, 30];
        println!("arr[1] = {}", arr[1]);
        // println!("{}", arr[5]); // index out of bounds
    }

    // Lifetime checking prevents dangling references
    // let r;
    // {
    //     let x = 10;
    //     r = &x;        // reference to local variable
    // }                  // x goes out of scope here
    // println!("{}", r); // compile-time error: borrowed value does not live long enough
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates common String operations.
//
// -- It shows length, modification, and empty check.
// -- It shows string comparison and character traversal.
// -- It shows replace, reverse, and tokenization.
// -- It shows creating string slices from String and literals.
// -- It shows an array of strings.
//

#[cfg(prog090)]
mod prog {
pub fn main() {

    let mut s = String::from("Hello");

    // Check if empty
    println!("Is empty = {}", s.is_empty());

    // Length
    println!("Length = {}", s.len());

    // Modify
    s.push_str(" Rust");
    s.push('!');
    s.insert(5, ',');
    println!("After modify: {}", s);

    // Compare strings using if-else
    let s2 = String::from("Hello, Rust!");
    if s == s2 {
        println!("Strings are equal");
    } else {
        println!("Strings are not equal");
    }

    // Iterate characters
    for ch in s.chars() {
        println!("char = {}", ch);
    }

    // Replace
    let replaced = s.replace("Rust", "World");
    println!("Replaced: {}", replaced);

    // Reverse string
    let reversed: String = s.chars().rev().collect();
    println!("Reversed = {}", reversed);

    // Tokenize using delimiter
    let text = "apple,banana,grape,orange";
    for token in text.split(',') {
        println!("token = {}", token);
    }

    // A string slice (&str) provides a read-only view into a String object 
    // or a string literal without owning the data.

    // String slice from String object
    let string_slice: &str = &String::from("Hello, Rust!");
    println!("slice from string object = {}", string_slice);

    // String slice from string literal
    let string_slice2: &str = "Hello Slice";
    println!("slice from string literal = {}", string_slice2);

    // Array of strings
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    for f in fruits.iter() {
        println!("fruit = {}", f);
    }

    // Difference between &String and &str
    let ref_string: &String = &s; // Immutable reference
    let ref_str: &str = &s;       // String slice
    println!("&String = {}", ref_string);
    println!("&str = {}", ref_str);
}
}

//////////////////////////////////////////////////////////////////////////////
//
// It demonstrates usage of tuples in Rust.
//
// -- It shows simple tuples and accessing elements using indexing.
// -- It shows tuples with mixed data types.
// -- It shows nested tuples.
// -- It shows returning multiple values from a function using tuples.
//

#[cfg(prog091)]
mod prog {

fn get_stats() -> (i32, i32, i32) {
    (10, 20, 30)
}

pub fn main() {

    // Simple tuple
    let t = (1, 2, 3);
    println!("t = ({}, {}, {})", t.0, t.1, t.2);

    // Tuple with mixed types
    let person = ("Alice", 30, true);
    println!("Name={}, Age={}, Active={}", person.0, person.1, person.2);

    // Tuple destructuring
    let (x, y, z) = (5, 10, 15);
    println!("x={}, y={}, z={}", x, y, z);

    // Nested tuple
    let nested = ((1, 2), (3, 4));
    println!("nested first={}, second={}", nested.0.0, nested.1.1);

    // Tuple returned from function
    let (a, b, c) = get_stats();
    println!("Stats = {}, {}, {}", a, b, c);
}
}

//////////////////////////////////////////////////////////////////////////////
//
// It demonstrates usage of ranges in Rust.
//
// -- It shows inclusive and exclusive ranges.
// -- It shows collecting a range into a vector.
//

#[cfg(prog092)]
mod prog {

pub fn main() {

    // Exclusive range (end not included)
    for i in 0..5 {
        println!("0..5 -> {}", i);
    }

    // Inclusive range (end included)
    for i in 0..=5 {
        println!("0..=5 -> {}", i);
    }

    // Character range
    for c in 'a'..='e' {
        println!("char range: {}", c);
    }

    // Range collected into a vector
    let v: Vec<i32> = (1..6).collect();
    println!("Vector from range: {:?}", v);
}
}

//////////////////////////////////////////////////////////////////////////////
//
// It demonstrates usage of structs in Rust.
//
// -- It shows creating struct instances.
// -- It shows accessing and updating struct fields.
// -- It shows storing struct instances in a vector.
//

#[cfg(prog093)]
mod prog {

struct Person {
    name: String,
    age: u32,
}

pub fn main() {

    // Create struct instance
    let mut p1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("Name={}, Age={}", p1.name, p1.age);

    // Update struct field
    p1.age = 31;
    println!("Updated Age={}", p1.age);

    // Another struct instance
    let p2 = Person {
        name: String::from("Bob"),
        age: 25,
    };

    println!("Person2 Name={}, Age={}", p2.name, p2.age);

    // Store struct instances in a vector
    let mut people: Vec<Person> = Vec::new();

    // Add existing instances
    people.push(p1);
    people.push(p2);

    // Add new instances
    people.push(Person { name: String::from("David"), age: 40 });
    people.push(Person { name: String::from("Emma"), age: 35 });

    for p in &people { // &people is an immutable reference to the vector
        println!("Vector Person Name={}, Age={}", p.name, p.age);
    }
}

}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates Rust functions.
//
// -- It shows defining and calling functions.
// -- It shows functions with multiple return values using tuples.
// -- It shows passing immutable borrow, mutable borrow, and moving ownership
//
#[cfg(prog100)]
mod prog {

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b { (a, b) } else { (b, a) }
}

// Pass argument as immutable borrow
fn display(s: &String) {
    println!("display = {}", s);
}

// Pass argument as mutable borrow
fn update(s: &mut String) {
    s.push_str(" World");
}

// Take ownership of String
fn consume(s: String) {
    println!("consume = {}", s);
}

pub fn main() {

    let result = add(2, 3);
    println!("add result = {}", result);

    let (min, max) = min_max(10, 20);
    println!("min = {}, max = {}", min, max);

    let mut s = String::from("Hello");
    println!("{}", s);

    display(&s);    // Pass argument as immutable borrow
    update(&mut s); // Pass argument as mutable borrow
    display(&s);    // Pass argument as immutable borrow

    // Move ownership
    consume(s);
    // println!("{}", s); // Not allowed as ownership moved
}
}

//////////////////////////////////////////////////////////////////////////////
//
// It demonstrates Rust closures and different capture mechanisms.
//
// -- It shows capture by immutable reference and mutable reference.
// -- It shows capture by move transferring ownership.
//
#[cfg(prog110)]
mod prog {

// Function that accepts a closure

// Defines a generic function that takes a type parameter F, where the 
// constraint specifies that F must be a closure implementing Fn(i32) -> i32.

fn apply<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(value)
}

pub fn main() {

    let multiply = |x: i32, y: i32| -> i32 {
        x * y
    };
    println!("multiply result = {}", multiply(4, 5));

    // Capture by immutable reference
    let factor = 10;
    let scale = |value: i32| value * factor;
    println!("scaled result = {}", scale(3));

    // Capture by immutable reference
    let x = 10;
    let callable = || println!("x = {}", x);
    callable();

    // Capture by mutable reference
    let mut z = 5;
    let mut callable = || z += 1;
    callable();

    // Capture by move (ownership transfer)
    let s = String::from("Rust");
    let callable = move || println!("moved value = {}", s);
    callable();

    // Passing closure as argument
    let square = |v: i32| v * v;
    let result = apply(square, 6);
    println!("closure result = {}", result);
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It shows basic Rust control flow constructs.
//
// -- It shows conditional branching with if / else.
// -- It shows infinite looping with loop.
// -- It shows conditional looping with while.
// -- It shows iteration using for and range.
// -- It shows pattern matching with match.
//
#[cfg(prog120)]
mod prog {
pub fn main() {

    // if / else
    let number = 5;
    if number % 2 == 0 {
        println!("even");
    } else {
        println!("odd");
    }

    // loop
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            break;
        }
    }

    // while
    let mut n = 0;
    while n < 3 {
        n += 1;
    }

    // for and range
    for i in 0..3 {
        println!("i = {}", i);
    }

    // match
    let value = 2;
    match value {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("other"),
    }
}
}

//////////////////////////////////////////////////////////////////////////////
//
// It demonstrates how enums work in Rust.
//
// -- It shows fieldless enums (similar to named constants in C++ enums)
// -- It shows defining an enum with multiple variants with associated data.
// -- It shows pattern matching on enum variants and extracting data.
//

#[cfg(prog130)]
mod prog {

use std::time::{SystemTime, UNIX_EPOCH};

fn random_0_3() -> u32 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    nanos % 4
}

enum StatusCode {
    Success,
    Pending,
    Warning,
    Failure,
}

enum Status {
    Success(i32),
    Pending(i32),
    Warning(String),
    Failure(String),
}

fn check() -> StatusCode {
    let v = random_0_3();

    match v {
        0 => StatusCode::Success,
        1 => StatusCode::Pending,
        2 => StatusCode::Warning,
        _ => StatusCode::Failure,
    }
}

fn get_status() -> Status {
    let v = random_0_3();

    match v {
        0 => Status::Success(100),
        1 => Status::Pending(2),
        2 => Status::Warning("Low memory".to_string()),
        _ => Status::Failure("Operation failed".to_string()),
    }
}

// Traditionally a function might return -1 to indicate failure, but since -1 
// could be a valid value, Option allows returning None to clearly 
// represent "no value".

fn get_number() -> Option<u32> {
    let v = random_0_3();
    if v == 0 {
        None
    } else {
        Some(v)
    }
}

pub fn main() {

    // Fieldless enum
    let code = check();

    match code {
        StatusCode::Success => println!("StatusCode: Success"),
        StatusCode::Pending => println!("StatusCode: Pending"),
        StatusCode::Warning => println!("StatusCode: Warning"),
        StatusCode::Failure => println!("StatusCode: Failure"),
    }

    // Enum with associated data
    let result = get_status();

    match result {
        // e.g. Status::Success(code) is a pattern that matches the Success 
        // variant and extracts the data.
        Status::Success(code) => println!("Success with code {}", code),
        Status::Pending(v) => println!("Pending {}", v),
        Status::Warning(msg) => println!("Warning: {}", msg),
        Status::Failure(msg) => println!("Failure: {}", msg),
    }

    // Built-in enum: Option<T>
    let value = get_number();

    match value {
        Some(v) => println!("Got value {}", v),
        None => println!("No value found"),
    }    
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates Rust pattern usage. Patterns match a value’s structure
// and optionally extract inner data.
//
// -- It shows patterns in match
// -- It shows patterns in if let
// -- It shows patterns in while let
// -- It shows patterns in tuple destructuring (let binding)
// -- It shows patterns in for loop destructuring
// -- It shows patterns in function parameters.
//

#[cfg(prog140)]
mod prog {

use std::time::{SystemTime, UNIX_EPOCH};

fn random_0_3() -> u32 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    nanos % 4
}

enum Status {
    Success(i32),
    Pending(i32),
    Warning(String),
    Failure(String),
}

fn get_status() -> Status {
    let v = random_0_3();

    match v {
        0 => Status::Success(100),
        1 => Status::Pending(2),
        2 => Status::Warning("Low memory".to_string()),
        _ => Status::Failure("Operation failed".to_string()),
    }
}

fn get_number() -> Option<u32> {
    let v = random_0_3();
    if v == 0 {
        None
    } else {
        Some(v)
    }
}

fn print_pair((a, b): (i32, i32)) {
    println!("Function param pattern: a={}, b={}", a, b);
}

pub fn main() {

    // Pattern in match (enum destructuring)
    let result = get_status();
    match result {
        Status::Success(code) => println!("match pattern: {}", code), // Extracts inner value from enum and prints it
        Status::Pending(v) => println!("match pending {}", v),
        Status::Warning(msg) => println!("match warning {}", msg),
        Status::Failure(msg) => println!("match failure {}", msg),
    }

    let result = get_status();
    let x = match result {
        Status::Success(code) => code, // Extracts inner value from enum and returns it in x
        Status::Pending(v) => v,
        Status::Warning(msg) => {
            println!("match warning {}", msg);
            -1
        }
        Status::Failure(msg) => {
            println!("match failure {}", msg);
            -1
        }
    };
    println!("Returned value: {}", x);

    // Pattern in if let
    let status = get_status();
    if let Status::Success(v) = status { // Status::Success(v) is a pattern that matches the Success variant and extracts its inner value into v.
        println!("if let pattern: {}", v);
    }

    // Pattern in while let
    let mut total = 0;
    let mut val = get_number();

    while let Some(v) = val { // while val is Some(x)
        total += v;
        val = get_number();
    }
    println!("total = {}", total);

    // Pattern in tuple destructuring
    let (x, y) = (10, 20); // (x, y) is the pattern.
    println!("let pattern: x={}, y={}", x, y);

    // Pattern in for loop destructuring
    let pairs = [(1, "A"), (2, "B")];
    for (num, ch) in pairs {
        println!("for pattern: {} {}", num, ch);
    }

    // Pattern in function parameter
    print_pair((7, 8));
}

}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates common Rust error handling mechanisms.
//
// -- It shows Option that represents an enum that may contain a value (Some)
//    or no value (None).
// -- It shows Result that represents an enum that contains either a success
//    value (Ok) or an error value (Err).
// -- It shows unwrap() that extracts the value from Option or Result or panics
//    if it is None or Err.
// -- It shows expect() that is same as unwrap but allows a custom panic
//    error message.
// -- panic!: Stops the program due to an unrecoverable error and by default
//    unwinds the stack, running destructors (Drop).
// -- Rust does not support traditional exception handling.
//

#[cfg(prog141)]
mod prog {

fn find_even(v: i32) -> Option<i32> {
    if v % 2 == 0 {
        Some(v)
    } else {
        None
    }
}

fn do_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}

fn do_panic() {
    println!("About to panic");
    panic!("something went wrong");
}

pub fn main() {

    // Option example
    match find_even(4) {
        Some(v) => println!("Even value = {}", v),
        None => println!("No even value"),
    }

    // Result example
    match do_divide(10, 2) {
        Ok(v) => println!("Division result = {}", v),
        Err(e) => println!("Error = {}", e),
    }

    // unwrap example
    let x = do_divide(20, 2).unwrap();
    println!("unwrap result = {}", x);

    // expect example
    let y = do_divide(30, 3).expect("Division failed");
    println!("expect result = {}", y);

    // panic example
    do_panic();
}
}

//////////////////////////////////////////////////////////////////////////////
//
// It demonstrates Rust traits.
//
// -- A struct defines a custom data type.
// -- A trait defines an interface (a set of methods) that can be implemented
//    for a struct.
// -- impl provides the implementation of the trait for the struct.
//

#[cfg(prog160)]
mod prog {

struct Car {
    model: String,
}

trait CarOps {
    fn get_model(&self) -> &str;
    fn set_model(&mut self, model: String);
}

impl CarOps for Car {

    fn get_model(&self) -> &str {
        &self.model
    }

    fn set_model(&mut self, model: String) {
        self.model = model;
    }
}

pub fn main() {

    let mut c = Car { model: String::from("Tesla") };
    println!("model = {}", c.get_model());

    c.set_model(String::from("BMW"));
    println!("updated model = {}", c.get_model());
}
}

//////////////////////////////////////////////////////////////////////////////
//
// It demonstrates common Rust containers.
//
// -- It shows Vec (dynamic array).
// -- It shows LinkedList.
// -- It shows BTreeMap (ordered map).
// -- It shows HashMap (unordered map).
//
#[cfg(prog170)]
mod prog {

use std::collections::{LinkedList, BTreeMap, HashMap};

pub fn main() {

    // Vector (dynamic array)
    let mut vec: Vec<i32> = Vec::new();
    vec.push(100);
    vec.push(200);
    println!("Vector len={}, second={}", vec.len(), vec[1]);

    // LinkedList
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_back(5);
    list.push_back(10);
    println!("LinkedList front={:?}", list.front());

    // BTreeMap (ordered map)
    let mut bmap: BTreeMap<&str, i32> = BTreeMap::new();
    bmap.insert("Alice", 30);
    bmap.insert("Bob", 25);
    println!("BTreeMap Alice={:?}", bmap.get("Alice"));

    // HashMap (unordered map)
    let mut hmap: HashMap<&str, i32> = HashMap::new();
    hmap.insert("X", 1);
    hmap.insert("Y", 2);
    println!("HashMap X={:?}", hmap.get("X"));
}
}
