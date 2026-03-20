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
    println!("Hello World"); // ! indicates println! is a macro
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates Rust printing to standard output.
//
// -- It shows positional {} placeholders for value formatting.
// -- It shows positional and indexed placeholders for value formatting.
// -- It shows named placeholders for value formatting.
// -- It shows formatting traits like binary, hex, and float precision.
// -- It shows formatting traits like Debug and pretty Debug

#[cfg(prog020)]
mod prog {
pub fn main() {

    // Positional {} placeholders
    {
        let name = "Neeraj";
        let age = 30;

        println!("Name: {}", name);
        println!("Age: {}", age);
        println!("{} is {} years old.", name, age);
    }

    // Positional and indexed placeholders
    {
        println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    }

    // Named placeholders
    {
        println!("{subject} {verb} {object}",
                object="the lazy dog",
                subject="the quick brown fox",
                verb="jumps over");
    }

    // traits like binary, hex, and float precision
    {
        println!("Binary: {:b}", 10);
        println!("Hex: {:x}", 255);
        println!("Float: {:.2}", 3.14159);
    }

    // traits like Debug and pretty Debug
    {
        let v = vec![10, 20, 30];
        println!("Debug vector: {:?}", v);
        println!("Pretty debug vector: {:#?}", v);
    }
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates let binding in Rust. It binds a value to a name using the
// `let` keyword.
//

#[cfg(prog030)]
mod prog {
pub fn main() {

    // Explicit data type
    let a: i32 = 1000;

    // Data type inference
    let b = 42;              // inferred as i32

    println!("a={}, b={}", a, b);

    // One more example
    let c;
    c = 24; // Compiler gives error if the variable type cannot be inferred.
    println!("c={}", c); // Compiler gives warning if a variable is declared but never used.
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It shows some basic Rust data types and type inference.
//
// -- It shows how the compiler deduces types.
//
#[cfg(prog040)]
mod prog {
pub fn main() {
    // Explicit data types
    let a: i8 = -10;
    let b: i32 = -1000;

    let c: u16 = 200;
    let d: u64 = 5000;

    let e: f32 = 3.14;
    let f: f64 = 2.71828;

    let g: bool = true;
    let h: char = 'R';
    let i: &str = "Hello";

    // Data type inference
    let j = 42;             // inferred as i32
    let k = 2.5;            // inferred as f64
    let l = false;          // inferred as bool
    let m = 'A';            // inferred as char
    let n = "Rust";         // inferred as &str

    // Pointers
    let p: *const i32 = &j;   // pointer to integer - const indicates read-only pointer
    let q: *const char = &m;  // pointer to character

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
    println!("{}", f);
    println!("{}", g);
    println!("{}", h);
    println!("{}", i);
    println!("{}", j);
    println!("{}", k);
    println!("{}", l);
    println!("{}", m);
    println!("{}", n);

    // Pointer dereference is allowed only inside an unsafe block
    unsafe {
        println!("p addr={:p}, value={}", p, *p);
        println!("q addr={:p}, value={}", q, *q);
    }
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates Rust fixed-size arrays.
//
// -- It shows arrays of integers, characters, strings, and booleans.
//
#[cfg(prog050)]
mod prog {

pub fn main() {

    // Array of size 1024 initialized with 0
    {
        let _arr: [i32; 1024] = [0; 1024]; // Rust requires arrays to be initialized at declaration

        // Note: _ indicates that a value may be intentionally unused.
    }

    // Array of size 1024 initialized with 5
    {
        let _arr: [i32; 1024] = [5; 1024]; // all elements are set to 5
    }

    // Array iteration and modification
    {
        let mut arr: [i32; 3] = [10, 20, 30];

        println!("arr elements:");
        for v in arr.iter() {
            println!("{}", v);
        }

        println!("arr elements (debug format) {:?}:", arr);

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
    }

    // Array with inferred type and size
    {
        let arr = [1, 2, 3, 4, 5];

        println!("arr elements:");
        for v in arr.iter() {
            println!("{}", v);
        }
    }

    // Array of characters
    {
        let arr = ['R', 'u', 's', 't'];

        println!("arr characters:");
        for v in arr.iter() {
            println!("{}", v);
        }
    }

    // Array of string slices
    {
        let arr = ["Hello", "Rust", "World"];

        println!("arr strings:");
        for v in arr.iter() {
            println!("{}", v);
        }
    }

    // Array of booleans
    {
        let arr = [true, false, true, false];

        println!("arr booleans:");
        for v in arr.iter() {
            println!("{}", v);
        }
    }
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates heap allocation.
//
// -- It shows allocating a fixed-size integer buffer on the heap using Box.
// -- It shows allocating a growable-size integer buffer on the heap using Vec.
// -- The heap memory is automatically freed when the variables go out of scope.
// -- Box<T> and Vec<T> are generic types that take a type argument.
//

#[cfg(prog060)]
mod prog {

pub fn main() {

    // Allocate buffer of integers on heap using Box
    {
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
    }

    // Allocate growable buffer using Vec
    {
        let mut vec: Vec<i32> = vec![0; 5];

        // Creates a vector with 5 elements that can grow dynamically 
        // when new elements are pushed.

        for i in 0..vec.len() {
            vec[i] = (i as i32) * 10;
        }

        println!("Vec buffer contents:");
        for v in vec.iter() {
            println!("{}", v);
        }
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
    {
        let x = 10;
        println!("x = {}", x);
        // x = 20; // Not allowed (immutable)
    }

    // Mutable variable
    {
        let mut y = 10;
        println!("y = {}", y);

        y = 20; // Allowed (mutable)
        println!("y updated = {}", y);
    }
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

    let mut value = 10; // Owner

    value += 1;
    println!("value = {}", value);

    // Immutable borrows (multiple allowed - read-only access) /////////////////
    let r1 = &value;
    let r2 = &value;

    // *r1 += 1;                   // Cannot modify through immutable reference
    // let r3 = &mut value;        // Cannot have mutable borrow 
    // value += 1;                 // Cannot modify owner while immutably borrowed

    println!("value = {}", value); // Can read owner

    println!("r1 = {}, r2 = {}", r1, r2); // immutable borrows end here as it is a last access

    // Mutable borrow (exclusive access) ///////////////////////////////////////

    let r4 = &mut value;

    // let r5 = &mut value;        // Cannot have second mutable borrow
    // let r6 = &value;            // Cannot have immutable borrow
    // value += 1;                 // Cannot modify owner while mutably borrowed

    // println!("value = {}", value); // Cannot read owner

    *r4 += 5;
    println!("r4 = {}", r4);       // Mutable borrow ends here as it is a last access

    // Note: To modify a value, explicit dereferencing using * is required; 
    // for reading it is not needed because Rust automatically dereferences.
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates how Rust enforces memory safety at compile time.
//
// -- Immutable borrows (&T) allow multiple readers simultaneously but they
//    do not allow data modification. This prevents accidental modification 
//    while data is being shared.
// -- Mutable borrows (&mut T) provide exclusive access. This prevents data
//    races and ensures safe modification of the data.
// -- Ownership ensures only one owner of data at a time, preventing 
//    double free and use-after-free.
// -- Values are automatically dropped when they go out of scope.
// -- Bounds-check prevents out-of-bounds memory access.
// -- Option<T> prevents null pointer usage by returning Some or None.
// -- Lifetime check ensures references never outlive the data they point to.
//

#[cfg(prog090)]
mod prog {

pub fn main() {

    // Immutable / Mutable borrows
    {
        let mut value = String::from("Hello"); // Owner

        // Immutable borrows (multiple allowed - read-only access)
        let r1 = &value;
        let r2 = &value;

        println!("r1 = {}, r2 = {}", r1, r2);

        // Mutable borrow (exclusive access)
        let r3 = &mut value;

        r3.push_str(" World");
        println!("r3 = {}", r3);
    }

    // Ownership transfer (move)
    {
        let s1 = String::from("hello");
        let s2 = s1;                     // ownership moved
        println!("s2 = {}", s2);
        // println!("{}", s1);           // compile error: use-after-move
    }

    // Automatic cleanup
    {
        {
            let v = vec![1, 2, 3];
            println!("vector = {:?}", v);
        
        } // v is dropped here

        {
            let mut b = Box::new(10);
            println!("box value = {}", b);

            // Reassignment drops the previous heap allocation automatically
            b = Box::new(20);
            println!("box reassigned = {}", b);
        
        } // b is dropped here
    }

    // Bounds-check
    {
        let arr = [10, 20, 30];
        println!("arr[1] = {}", arr[1]);
        // println!("{}", arr[5]); // Compilation fails - index out of bounds

        let arr = Box::new([10, 20, 30]);
        println!("arr[1] = {}", arr[1]);
        let _i = 5;
        // println!("{}", arr[_i]); // Compilation fails - index out of bounds
    }

    // Return Option<T> instead of nullptr or -1
    {
        let data = vec![10, 20, 30, 40];

        match find_value(&data, 20) {
            Some(v) => println!("value found = {}", v),
            None => println!("value not found"),
        }

        match find_value(&data, 99) {
            Some(v) => println!("value found = {}", v),
            None => println!("value not found"),
        }
    }

    // Lifetime check
    {
        let r;
        // {
            let x = 10;
            r = &x;     // Compilation fails -  borrowed value does not live long enough
        // }
        println!("{}", r); 
    }                  
}

fn find_value(vec: &Vec<i32>, key: i32) -> Option<i32> {
    for v in vec {
        println!("{}", v); // auto-dereferenced by println! for formatting
        if *v == key {     // explicit dereference required for comparison
            return Some(*v);
        }
    }
    None
}

}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates common String operations.
//
// -- It shows string empty check, length, modification.
// -- It shows string character traversal.
// -- It shows string comparison.
// -- It shows replace and tokenization.
// -- It shows creating string slices from String object and literals.
// -- It shows an array of strings.
//

#[cfg(prog100)]
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

    // Iterate characters
    for ch in s.chars() {
        println!("char = {}", ch);
    }

    // Compare strings using if-else
    let s2 = String::from("Hello, Rust!");
    if s == s2 {
        println!("Strings are equal");
    } else {
        println!("Strings are not equal");
    }

    // Replace
    let replaced = s.replace("Rust", "World");
    println!("Replaced: {}", replaced);

    // Tokenize using delimiter and print
    let text = "apple,banana,grape,orange";
    for token in text.split(',') {
        println!("token = {}", token);
    }

    // Tokenize using delimiter and collect in vector
    let text = "apple,banana,grape,orange";
    let tokens: Vec<&str> = text.split(',').collect(); // slices are immutable borrows of the original string

    for token in &tokens {
        println!("token = {}", token);
    }    

    // Note: A string slice (&str) provides a read-only view into a String 
    // or a string literal without owning the data.

    // String slice
    let string_slice: &str = &String::from("Hello, Rust!");
    let string_slice2: &str = "Hello Slice";
    println!("String slice = {} {}", string_slice, string_slice2);

    // Array of strings
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    for f in fruits.iter() {
        println!("fruit = {}", f);
    }

    // Difference between &String and &str
    let ref_string: &String = &s; // immutable borrow of the String object
    let ref_str: &str = &s;       // string slice - immutable borrow of the string data
    println!("&String = {}", ref_string);
    println!("&str = {}", ref_str);
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates usage of tuples in Rust. A tuple is a compound type that 
// groups multiple values of possibly different types into one unit.
// 
// -- It shows simple tuple and accessing elements using indexing.
// -- It shows tuples with mixed data types.
// -- It shows nested tuple.
// -- It shows returning multiple values from a function using tuple.
//

#[cfg(prog110)]
mod prog {

pub fn main() {

    // Simple tuple
    let t = (1, 2, 3); // It deduces the type as (i32, i32, i32)
    println!("t = ({}, {}, {})", t.0, t.1, t.2);

    // Tuple with mixed data types
    let person = ("Alice", 30, true); // It deduces the type as (&str, i32, bool).
    println!("Name={}, Age={}, Active={}", person.0, person.1, person.2);

    // Tuple destructuring
    let (x, y, z) = (5, 10, 15);
    println!("x={}, y={}, z={}", x, y, z);

    // Nested tuple
    let nested = ((1, 2), (3, 4));
    println!("nested first={}, second={}", nested.0.0, nested.1.1);

    // Tuple returned from function
    let (name, age, active) = get_person();
    println!("Person = {}, {}, {}", name, age, active);

    let (p, active) = get_person2();
    println!("Name = {}, Age = {}, Active = {}", p.name, p.age, active);
}

fn get_person() -> (&'static str, i32, bool) {
    ("Alice", 30, true)
}

struct Person {
    name: &'static str,
    age: i32,
}

fn get_person2() -> (Person, bool) {
    (Person { name: "Alice", age: 30 }, true)
}

}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates usage of ranges in Rust.
//
// -- It shows inclusive and exclusive ranges.
// -- It shows collecting a range into a vector.
// -- It shows ranges with arrays and vectors.
//

#[cfg(prog120)]
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

    // Using range to iterate through array & vector
    let arr = [10, 20, 30, 40, 50];

    for i in 0..arr.len() {
        println!("{}", arr[i]);
    }

    let vec = vec![10, 20, 30, 40, 50];

    for i in 0..vec.len() {
        println!("{}", vec[i]);
    }
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates usage of struct in Rust. A struct in Rust is a custom data
// type that groups related data fields.
//
// -- It shows creating struct instances.
// -- It shows accessing and updating struct fields.
// -- It shows storing struct instances in a vector.
// -- It shows storing struct instances in an array.
// -- It shows storing struct instances in a hash map.
//

#[cfg(prog130)]
mod prog {

use std::collections::HashMap;

struct Person {
    name: String,
    age: u32,
}

pub fn main() {

    // Create struct instances
    {
        let mut p1 = Person {             // Mutable binding
            name: String::from("Alice"),
            age: 30,
        };

        println!("Name={}, Age={}", p1.name, p1.age);

        // Update struct field
        p1.age = 31;
        println!("Updated Age={}", p1.age);

        // Another struct instance
        let p2 = Person {                 // Immutable binding
            name: String::from("Bob"),
            age: 25,
        };

        println!("Person2 Name={}, Age={}", p2.name, p2.age);
    }

    // Store struct instances in a vector
    {
        let mut people: Vec<Person> = Vec::new();

        // Add new instances
        people.push(Person { name: String::from("David"), age: 40 });
        people.push(Person { name: String::from("Emma"), age: 35 });

        for p in &people { // &people is an immutable reference to the vector
            println!("Vector Person Name={}, Age={}", p.name, p.age);
        }
    }

    // Store struct instances in an array
    {
        let people_arr = [
            Person { name: String::from("Alice"), age: 31 },
            Person { name: String::from("Bob"), age: 25 },
            Person { name: String::from("David"), age: 40 },
            Person { name: String::from("Emma"), age: 35 },
        ];

        for p in &people_arr {
            println!("Array Person Name={}, Age={}", p.name, p.age);
        }
    }

    // Store struct instances in a hash map
    {
        let mut people_map: HashMap<String, Person> = HashMap::new();

        people_map.insert(String::from("p1"), Person { name: String::from("Alice"), age: 31 });
        people_map.insert(String::from("p2"), Person { name: String::from("Bob"), age: 25 });
        people_map.insert(String::from("p3"), Person { name: String::from("David"), age: 40 });
        people_map.insert(String::from("p4"), Person { name: String::from("Emma"), age: 35 });

        for (k, p) in &people_map {
            println!("Map Key={}, Name={}, Age={}", k, p.name, p.age);
        }

        // Note: (k, p) is tuple destructuring of a (key, value) pair from the 
        // HashMap; since people_map is iterated by reference (&people_map), 
        // k and p are references (&String, &Person) to the entries inside the map.
    }
}

}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates Rust functions.
//
// -- It shows defining and calling functions.
// -- It shows functions with multiple return values using tuple.
// -- It shows passing immutable borrow, mutable borrow, and moving ownership
// -- It shows passing struct and vector by borrow
//
#[cfg(prog140)]
mod prog {

// Struct definition
struct Person {
    name: String,
    age: i32,
}

// Function returning integer 
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Function returning tuple
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b { (a, b) } else { (b, a) }
}

// Function taking arg as immutable borrow (read-only access)
fn display(s: &String) {
    println!("display = {}", s);
}

// Function taking arg as mutable borrow (write access)
fn update(s: &mut String) {
    s.push_str(" World");
}

// Function taking ownership of argument
fn consume(s: String) {
    println!("consume = {}", s);
}

// Function taking struct by borrow
fn show_person(p: &Person) {
    println!("Person: {} {}", p.name, p.age);
}

// Function taking vector by borrow
fn show_vec(v: &Vec<i32>) {
    for val in v {
        println!("{}", val);
    }
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

    // Struct usage
    let person = Person {
        name: String::from("Neeraj"),
        age: 30,
    };
    show_person(&person);

    // Vector usage
    let vec = vec![1, 2, 3, 4];
    show_vec(&vec);
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates Rust closures and different captures. A closure is an 
// anonymous function that can capture variables from its surrounding scope.
//
// -- It shows defining and calling closure
// -- It shows capture by immutable reference and mutable reference.
// -- It shows capture by move transferring ownership.
// -- It shows closures with iterators: for_each, map, filter.
// -- It shows using closure in thread creation.
//
#[cfg(prog150)]
mod prog {

use std::thread;

pub fn main() {

    // Defining and calling closure
    {
        let multiply = |x: i32, y: i32| -> i32 {
            x * y
        };
        println!("multiply result = {}", multiply(4, 5));
    }

    // Capture by immutable reference
    {
        let factor = 10;
        let scale = |value: i32| value * factor;
        println!("scaled result = {}", scale(3));

        let x = 10;
        let callable = || println!("x = {}", x);
        callable();
    }

    // Capture by mutable reference
    {
        let mut z = 5;
        let mut callable = || { // Closure must be declared as mutable if it modifies captured variables
            z += 1;
            println!("z = {}", z);
        };
        callable();
    }

    // Note: Closure capture (immutable, mutable, or move) is automatically 
    // decided by the compiler based on how the captured variable is used.

    // Capture by move (ownership transfer)
    {
        let s = String::from("Rust");
        println!("s is {}", s);
        let callable = move || println!("moved value = {}", s);
        // println!("s is {}", s); // Not allowed: `s` is moved into the closure at creation time
        callable();
    }

    // Passing closure as function argument
    {
        let square = |v: i32| v * v;
        let result = apply(square, 6);
        println!("closure result = {}", result);
    }

    // Closure used with for_each method of an iterator.
    {
        let vec = vec![1, 2, 3, 4, 5];

        println!("for_each:");
        vec.iter().for_each(|v| println!("{}", v));
    }

    // Closures used with map and filter methods of an iterator.
    {
        let vec = vec![1, 2, 3, 4, 5];

        let squared: Vec<i32> = vec.iter().map(|v| v * v).collect();
        println!("squared: {:?}", squared);

        let even: Vec<i32> = vec.into_iter().filter(|v| v % 2 == 0).collect();
        println!("even: {:?}", even);
    }    

    // Closure used in thread creation (move required for ownership transfer)
    {
        let msg = String::from("Hello from thread");

        let handle = thread::spawn(move || {
            println!("{}", msg);
        });

        handle.join().unwrap();
    }
}

// Generic function accepting a closure as arg
fn apply<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(value)
}

// Note: It defines a generic function that takes a type parameter F, where the 
// constraint specifies that F must be a closure implementing Fn(i32) -> i32.

}

////////////////////////////////////////////////////////////////////////////////
//
// It shows Rust control flow constructs.
//
// -- It shows conditional branching with if / else.
// -- It shows infinite looping with loop and break.
// -- It shows loop as an expression with break returning a value.
// -- It shows labeled loops with continue.
// -- It shows conditional looping with while.
// -- It shows pattern matching with match.
//

#[cfg(prog160)]
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
    println!("count = {}", count);

    // loop - break with value
    let mut count = 0;
    let loop_result = loop {
        count += 1;
        if count == 3 {
            break count * 10;   // Return variant of count
        }
    };
    println!("loop_result = {}", loop_result);

    // labeled loop + continue
    let mut outer = 0;
    'outer: loop {
        outer += 1;
        if outer == 3 {
            break 'outer;
        }

        let mut inner = 0;
        loop {
            inner += 1;
            if inner == 2 {
                continue 'outer;
            }
        }
    }
    println!("outer = {}", outer);

    // while
    let mut n = 0;
    while n < 3 {
        n += 1;
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

////////////////////////////////////////////////////////////////////////////////
//
// It shows Rust expressions that do not require semicolons.
//
// -- It shows returning values from blocks.
// -- It shows if expressions returning values.
// -- It shows functions returning values.
//

#[cfg(prog170)]
mod prog {

pub fn main() {

    let x = {
        let a = 5;
        let b = 10;
        a + b // Do not require semicolon
    };
    println!("{}", x);

    let n = 10;
    let result = if n > 5 {
        "greater" // Do not require semicolons
    } else {
        "smaller"
    };
    println!("{}", result);

    let y = add(3, 4);
    println!("{}", y);
}

fn add(a: i32, b: i32) -> i32 {
    a + b // Do not require semicolons
}

}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates Rust enums and pattern matching. Pattern matching checks 
// a value against patterns and, when matched, destructures and extracts 
// its contents.
//
// -- It shows fieldless enums (similar to named constants in C++).
// -- It shows enums with multiple variants carrying associated data.
// -- It shows pattern matching using match, if let, and while let.
// -- It shows pattern matching with Option<T> and Result<T, E>.
//

#[cfg(prog180)]
mod prog {

use std::time::{SystemTime, UNIX_EPOCH};

enum State {          // fieldless enums (similar to named constants in C++)
    Success,
    Pending,
    Warning,
    Failure,
}

enum Status {          // enum with multiple variants carrying associated data
    Success(i32),
    Pending(i32),
    Warning(String),
    Failure(String),
}

fn get_state() -> State {  // Returns State enum with no associated data
    let v = random_0_3();

    match v {
        0 => State::Success,
        1 => State::Pending,
        2 => State::Warning,
        _ => State::Failure,
    }
}

fn get_status() -> Status { // Returns Status enum with associated data
    let v = random_0_3();

    match v {
        0 => Status::Success(100),
        1 => Status::Pending(2),
        2 => Status::Warning("Low memory".to_string()),
        _ => Status::Failure("Operation failed".to_string()),
    }
}

pub fn main() {

    // Fieldless enum
    {
        let state = get_state();
        match state {
            State::Success => println!("StatusCode: Success"),
            State::Pending => println!("StatusCode: Pending"),
            State::Warning => println!("StatusCode: Warning"),
            State::Failure => println!("StatusCode: Failure"),
        }
    }

    // Pattern matching in match
    {
        let result = get_status();

        match result {
            Status::Success(code) => println!("Success with code {}", code),
            Status::Pending(v) => println!("Pending {}", v),
            Status::Warning(msg) => println!("Warning: {}", msg),
            Status::Failure(msg) => println!("Failure: {}", msg),
        }
    }

    // Pattern matching in match returning a value
    {
        let result = get_status();
        let x = match result {
            Status::Success(code) => code,
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
    }

    // Pattern matching using if let
    {
        let status = get_status();
        if let Status::Success(v) = status {
            println!("if let pattern: {}", v);
        }
    }

    // Pattern matching using while let
    {
        while let Status::Success(v) = get_status() {
            println!("while let pattern: {}", v);
        }
    }

    // Pattern matching with built-in enum Option<T>
    let opt = get_number();
    match opt {
        Some(v) => println!("Option got {}", v),
        None => println!("Option: no value"),
    }

    // Pattern matching with built-in enum Result<T, E>
    let res = fetch_number();
    match res {
        Ok(v) => println!("Result got {}", v),
        Err(e) => println!("Result error: {}", e),
    }
}

/*
Built-in enum Option<T> is a generic enum type argument as T)
enum Option<T> {
    None,
    Some(T),
}
*/

fn get_number() -> Option<u32> {
    let v = random_0_3();
    if v == 0 {
        None
    } else {
        Some(v)
    }
}

// Note: Traditionally a function might return -1 or nullptr to indicate 
// failure. Option provides alternative by returning valid Some value or None.

/* 
Built-in enum Result<T, E> is a generic enum type argument as T and E)
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

fn fetch_number() -> Result<u32, String> {
    let v = random_0_3();
    if v == 0 {
        Err("No value found".to_string())
    } else {
        Ok(v)
    }
}

// Function to generate random number from 0 to 3
fn random_0_3() -> u32 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    nanos % 4
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates common Rust error handling mechanisms.
//
// -- It shows Option enum that may contain a value (Some) or no value (None).
// -- It shows Result enum that contains either a success value (Ok) or an 
//    error value (Err).
// -- It shows unwrap() that extracts the value and panics if it is None or Err.
// -- It shows expect() that behaves like unwrap but allows a custom panic
//    error message on failure.
// -- Rust does not support traditional exception handling.
//

#[cfg(prog190)]
mod prog {

pub fn main() {

    // Option enum
    match check_even(4) {
        Some(v) => println!("Even value = {}", v),
        None => println!("No even value"),
    }

    // Result enum
    match do_divide(10, 2) {
        Ok(v) => println!("Division result = {}", v),
        Err(e) => println!("Error = {}", e),
    }

    // unwrap
    let x = do_divide(20, 2).unwrap();
    // let x = do_divide(20, 0).unwrap(); // Panics
    println!("unwrap result = {}", x);

    // expect
    let y = do_divide(30, 3).expect("Division failed");
    println!("expect result = {}", y);
}

fn check_even(v: i32) -> Option<i32> {
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
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates Rust error handling in file operations.
//
// -- It shows unwrap() that extracts the value and panics if it is Err.
// -- It shows expect() that behaves like unwrap but allows a custom panic
//    error message on failure.
// -- It shows explicit pattern matching against the Result enum and exiting 
//    if Err.
//

#[cfg(prog200)]
mod prog {

use std::fs::File;
use std::io::{Write, Read};
use std::process;

pub fn main() {

    let filename = "test.txt";

    // Open and write using unwrap
    {
        let mut file = File::create(filename).unwrap();
        file.write_all(b"Hello Rust").unwrap();
        drop(file);
    }

    // Open and read using expect
    {
        let mut file = File::open(filename).expect("Failed to open file");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Failed to read file");
        drop(file);
        println!("Read content  {}", content);
    }

    // Open and read using explicit pattern matching
    {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(e) => {
                println!("Error opening file: {}", e);
                process::exit(1);
            }
        };
        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Ok(_) => {}
            Err(e) => {
                println!("Error reading file: {}", e);
                process::exit(1);
            }
        }
        drop(file);
        println!("Read content  {}", content);
    }
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates common Rust containers.
//
// -- It shows Vec (dynamic array).
// -- It shows LinkedList.
// -- It shows BTreeMap (ordered map).
// -- It shows HashMap (unordered map).
//
// -- Vec: Dynamic array, contiguous memory, fast index access (O(1)), 
//    fast push/pop at end, costly middle insert/remove (O(n)) time complexity.
// -- LinkedList: Doubly linked list, no indexing, fast insert/remove 
//    anywhere (O(1) with pointer), higher memory overhead
// -- BTreeMap: Ordered map, values stored in sorted key order using 
//    balanced B-tree nodes, time complexity O(log n)
// -- HashMap: Unordered map, values stored using hash-based buckets.  
//    average time complexity O(1)
//

#[cfg(prog210)]
mod prog {

use std::collections::{LinkedList, BTreeMap, HashMap};

struct Person {
    name: String,
    age: i32,
}

pub fn main() {

    // Vector (dynamic array)
    {
        let mut vec: Vec<Person> = Vec::new();
        vec.push(Person { name: "Alice".to_string(), age: 30 });
        vec.push(Person { name: "Bob".to_string(), age: 25 });

        println!("Read vec[1] age={}", vec[1].age);

        vec[1].age = 26;

        println!("Vector contents:");
        for p in vec.iter() {
            println!("name={}, age={}", p.name, p.age);
        }
    }

    // LinkedList
    {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(5);
        list.push_back(10);

        // Read front
        println!("LinkedList front={:?}", list.front());

        // Update front
        if let Some(v) = list.front_mut() {
            *v = 20;
        }

        println!("LinkedList contents:");
        for v in list.iter() {
            println!("{}", v);
        }
    }

    // BTreeMap (ordered map)
    {
        let mut bmap: BTreeMap<&str, i32> = BTreeMap::new();
        bmap.insert("Alice", 30);
        bmap.insert("Bob", 25);

        // Read by key
        if let Some(v) = bmap.get("X") {
            println!("Read X={}", v);
        }

        // Update by key
        if let Some(v) = bmap.get_mut("Alice") {
            *v = 31;
        }

        println!("BTreeMap contents:");
        for (k, v) in bmap.iter() {
            println!("{} -> {}", k, v);
        }
    }

    // HashMap (unordered map)
    {
        let mut hmap: HashMap<&str, i32> = HashMap::new();
        hmap.insert("X", 1);
        hmap.insert("Y", 2);

        // Read by key
        if let Some(v) = hmap.get("X") {
            println!("Read X={}", v);
        }

        // Update by key
        if let Some(v) = hmap.get_mut("X") {
            *v = 100;
        }

        println!("HashMap contents:");
        for (k, v) in hmap.iter() {
            println!("{} -> {}", k, v);
        }
    }
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates some features from Rust standard library (std). 
// 
// The standard library (std) is the built-in Rust library that provides 
// commonly used functionality such as:
//
// -- File handling
// -- Memory utilities
// -- Collections
// -- Strings handling
// -- Threading
// -- Time handling
// -- Networking
// -- Error handling
// -- Environment and OS interaction
//

#[cfg(prog220)]
mod prog {

use std::fs::File;
use std::io::{Write, Read};
use std::mem;
use std::collections::{HashMap};
use std::thread;
use std::time::{SystemTime, Duration};
use std::env;
use std::net::TcpStream;

pub fn main() {

    // File handling & error handling
    {
        let mut file = File::create("data.txt").unwrap();
        file.write_all(b"Hello Rust").unwrap();
        drop(file);

        let mut file = File::open("data.txt").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        println!("File content = {}", content);
    }

    // Memory utilities
    {
        println!("size of i32 = {}", mem::size_of::<i32>());
        println!("size of u64 = {}", mem::size_of::<u64>());
    }

    // Collections
    {
        let mut map = HashMap::new();
        map.insert("one", 1);
        map.insert("two", 2);

        for (k, v) in map.iter() {
            println!("{} -> {}", k, v);
        }
    }

    // String handling
    {
        let mut s = String::from("Hello");
        s.push_str(" Rust");
        println!("String = {}", s);
    }

    // Threading
    {
        let handle = thread::spawn(|| {
            println!("Hello from thread");
        });
        handle.join().unwrap();
    }

    // Time handling
    {
        let now = SystemTime::now();
        println!("Current time = {:?}", now);

        thread::sleep(Duration::from_millis(100));
    }

    // Networking
    {
        match TcpStream::connect("google.com:80") {
            Ok(_) => println!("Connected to google.com"),
            Err(e) => println!("Connection failed: {}", e),
        }
    }

    // Environment and OS interaction
    {
        let current_dir = env::current_dir().unwrap();
        println!("Current directory = {:?}", current_dir);
    }
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates Rust traits.
//
// -- A struct defines a custom data type.
// -- A trait defines an interface (a set of methods) that can be implemented
//    for the struct.
// -- impl provides the implementation of the trait for the struct.
//

#[cfg(prog230)]
mod prog {

struct Car {
    model: String,
}

trait CarOps {
    fn get_model(&self) -> &str;            // &self is an immutable reference to the current object (instance of the struct)
    fn set_model(&mut self, model: String); // &mut self is a mutable reference to the current object (instance of the struct)
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
