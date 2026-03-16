fn main() {
    prog::main();
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates raw pointers and unsafe operations in Rust.
//
// -- It shows raw pointer creation and dereferencing.
// -- It shows heap allocation using Box and accessing it via raw pointer.
//

#[cfg(prog010)]
mod prog {

pub fn main() {

    // Create raw pointer from reference
    let x = 10;
    let raw_ptr: *const i32 = &x;

    // Unsafe block required for raw pointer dereference
    unsafe {
        println!("Raw pointer value = {}", *raw_ptr);
    }

    // Allocate buffer of integers on heap
    let mut buf: Box<[i32; 5]> = Box::new([0; 5]);

    // Get raw pointer to the buffer
    let ptr = buf.as_mut_ptr();

    unsafe {
        // Initialize using pointer arithmetic
        for i in 0..5 {
            *ptr.add(i) = (i as i32) * 10; // add(n) returns a pointer n elements ahead of the current pointer.
        }

        println!("Buffer contents:");
        for i in 0..5 {
            println!("{}", *ptr.add(i));
        }
    }
}
}

//////////////////////////////////////////////////////////////////////////////
//
// It demonstrates Rust macros. A macro is a construct that generates 
// Rust code at compile time, helping reduce repetitive code.
//
// -- It shows println! macro.
// -- It shows vec! macro.
// -- It shows assert! macro.
// -- It shows a custom macro example.
//
#[cfg(prog020)]
mod prog {

// Custom macro
macro_rules! print_twice {
    ($x:expr) => {
        println!("{}", $x);
        println!("{}", $x);
    };
}

pub fn main() {

    // println! macro example
    let x = 10;
    println!("x = {}", x);

    // vec! macro example
    let v = vec![1, 2, 3];
    println!("Vector length = {}", v.len());

    // Conceptual expansion:
    // let mut temp = Vec::new();
    // temp.push(1);
    // temp.push(2);
    // temp.push(3);
    // let v = temp;

    // assert! macro example
    assert!(x > 0);

    // Conceptual expansion:
    // if !(x > 0) {
    //     panic!("assertion failed: x > 0");
    // }

    // Custom macro usage examples
    print_twice!(5);
    print_twice!(x + 2);
    print_twice!("Hello Rust");

    // Conceptual expansion examples:
    // println!("{}", 5);
    // println!("{}", 5);

    // println!("{}", x + 2);
    // println!("{}", x + 2);

    // println!("{}", "Hello Rust");
    // println!("{}", "Hello Rust");
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates Rust constructor and destructor.
//
// -- Rust does not inherently support constructors, but they can be simulated.
// -- A destructor can be implemented using the Drop trait.
// -- Any additional method that is not part of a trait can be implemented
//    in separate impl blocks.
//

#[cfg(prog030)]
mod prog {

struct Car {
    model: String,
}

// Constructor simulation
impl Car {

    fn new(model: &str) -> Car {
        println!("Creating Car {}", model);
        Car {
            model: String::from(model),
        }
    }
}

// Destructor using Drop trait
impl Drop for Car {
    fn drop(&mut self) {
        println!("Car {} is being destroyed", self.model);
    }
}

// Additional functions
impl Car {

    fn print_model(&self) {
        println!("Car model = {}", self.model);
    }
}

pub fn main() {

    let c = Car::new("Tesla");
    c.print_model();
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates Rust supertraits and struct composition.
//
// -- Rust does not support classical inheritance like C++/Java.
// -- Traits can inherit from other traits (supertraits).
// -- Struct composition can be used to build complex types.
// -- A struct can implement multiple traits in separate impl blocks.
//
// -- One struct inheriting from another struct is not supported.
// -- One impl implementing multiple traits is not supported.
// -- One impl inheriting another impl is not supported.

#[cfg(prog040)]
mod prog {

trait CarOps {
    fn get_model(&self) -> &str;
    fn set_model(&mut self, model: String);
}

// Trait inheriting another trait
trait DriveOps: CarOps {
    fn drive(&self);
}

// Composed struct
struct Engine {
    horsepower: u32,
}

struct Car {
    model: String,
    engine: Engine,   // Composition: Car has an Engine
}

impl CarOps for Car {

    fn get_model(&self) -> &str {
        &self.model
    }

    fn set_model(&mut self, model: String) {
        self.model = model;
    }
}

impl DriveOps for Car {

    fn drive(&self) {
        println!(
            "Driving {} with engine {} HP",
            self.model, self.engine.horsepower
        );
    }
}

pub fn main() {

    let mut c = Car {
        model: String::from("Tesla"),
        engine: Engine { horsepower: 450 },
    };

    println!("model = {}", c.get_model());

    c.set_model(String::from("BMW"));
    println!("updated model = {}", c.get_model());

    c.drive();
}
}

//////////////////////////////////////////////////////////////////////////////
//
// It demonstrates the trait derive attribute. It tells the compiler to
// automatically generate implementations of traits for the given struct.
//
// Debug: allows a value to be printed using {:?} for debugging.
// Default: provides a default value.
// Clone: allows duplication using deep copy.
// Copy: allows duplication using bitwise copy.
// PartialEq: enables equality comparison using == and !=.
// PartialOrd: enables ordering comparison using <, >, <=, >=.
//

#[cfg(prog050)]
mod prog {

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

pub fn main() {

    // Debug
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p); // {:?} is a format specifier used to print a value using the Debug trait.

    // Default
    let p2 = Point::default();
    println!("default = {:?}", p2);

    // Clone / Copy
    let p3 = p;
    let p4 = p.clone();
    println!("copy = {:?}, clone = {:?}", p3, p4);

    // PartialEq / PartialOrd
    println!("p == p3 = {}", p == p3);
    println!("p < Point{{x:3,y:4}} = {}", p < Point { x: 3, y: 4 });
}
}

//////////////////////////////////////////////////////////////////////////////
//
// It demonstrates generics in Rust.
//
// -- It shows a simple generic function.
// -- It shows a simple generic struct.
// -- It shows a generic function returning a generic enum.
// -- It shows a generic multiply function.
//
#[cfg(prog060)]
mod prog {

// Simple generic function
fn identity<T>(value: T) -> T {
    value
}

// Simple generic struct
struct BoxValue<T> {
    value: T,
}

// Simple generic enum
enum SimpleResult<T> {
    Ok(T),
    Err,
}

// Generic function returning SimpleResult
fn check_greater<T: PartialOrd>(x: T, y: T) -> SimpleResult<T> {
    // PartialOrd Indicates a constraint that the generic type T must 
    // support comparison operations
    if x > y {
        SimpleResult::Ok(x)
    } else {
        SimpleResult::Err
    }
}

// Generic multiply function
fn multiply<T>(x: T, y: T) -> T
where
    T: std::ops::Mul<Output = T>, // Indicates a constraint that the generic 
                                  // type T must support multiplication (*) 
                                  // and the result must also be of type T
{
    x * y
}

pub fn main() {

    // Generic function
    let a = identity(10);
    let b = identity(3.14);
    println!("a={}, b={}", a, b);

    // Generic struct
    let v1 = BoxValue { value: 100 };
    let v2 = BoxValue { value: "Rust" };
    println!("BoxValue1={}, BoxValue2={}", v1.value, v2.value);

    // Generic comparison with i32
    let r1 = check_greater(10, 5);

    match r1 {
        SimpleResult::Ok(v) => println!("i32 Ok {}", v),
        SimpleResult::Err => println!("i32 Err"),
    }

    // Generic comparison with f64
    let r2 = check_greater(3.5, 7.2);

    match r2 {
        SimpleResult::Ok(v) => println!("f64 Ok {}", v),
        SimpleResult::Err => println!("f64 Err"),
    }

    // Generic multiply with i32
    let m1 = multiply(6, 7);
    println!("i32 multiply = {}", m1);

    // Generic multiply with f64
    let m2 = multiply(2.5, 4.0);
    println!("f64 multiply = {}", m2);
}
}

//////////////////////////////////////////////////////////////////////////////
//
// It demonstrates different lifetime usages in Rust.
//
// -- It shows 'static lifetime with car-related data.
// -- It shows 'static lifetime defined inside a function and returned.
// -- It shows lifetime in functions.
// -- It shows lifetime in struct definitions.
// -- It shows lifetime in methods (impl blocks).
// -- It shows lifetime in traits.
//
#[cfg(prog070)]
mod prog {

// 'static means this reference lives for the entire duration of the program.
static GLOBAL_CAR: &str = "Global Car: Tesla";

// Function returning a reference with 'static lifetime.
fn get_static_car() -> &'static str {
    // Static defined inside a function also lives for the entire program.
    static LOCAL_CAR: &str = "Static Car: BMW";
    LOCAL_CAR
}

// 'a means the returned reference will live at least as long as both input references.
fn choose_model<'a>(m1: &'a str, m2: &'a str) -> &'a str {
    if m1.len() > m2.len() { m1 } else { m2 }
}

// Struct holds a reference whose lifetime must outlive the struct.
struct Car<'a> {
    model: &'a str,
}

// Trait methods operate on references tied to lifetime 'a.
trait CarOps<'a> {
    fn get_model(&self) -> &'a str;
    fn set_model(&mut self, model: &'a str);
}

// Implementation ensures returned references follow lifetime 'a.
impl<'a> CarOps<'a> for Car<'a> {

    fn get_model(&self) -> &'a str {
        self.model
    }

    fn set_model(&mut self, model: &'a str) {
        self.model = model;
    }
}

// Function borrowing Car while respecting the same lifetime.
fn print_car<'a>(car: &Car<'a>) {
    println!("Car model = {}", car.get_model());
}

pub fn main() {

    println!("{}", GLOBAL_CAR);

    let s = get_static_car();
    println!("{}", s);

    let m1 = "Tesla";
    let m2 = "Mercedes";
    let chosen = choose_model(m1, m2);
    println!("Chosen model = {}", chosen);

    let mut c = Car { model: m1 };
    print_car(&c);

    c.set_model("Audi");
    println!("updated model = {}", c.get_model());
}
}
