fn main() {
    prog::main();
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates Rust supertraits and struct composition.
//
// -- Traits can be inherited by other traits. A supertrait is the trait
//    that is inherited by another trait.
// -- The inheriting trait must be implemented in a separate impl block.
// -- Struct composition is used to build complex types. It means building a 
//    struct by including other structs.
//

#[cfg(prog010)]
mod prog {

trait CarOps {                      // supertrait of DriveOps
    fn get_model(&self) -> &str;
    fn set_model(&mut self, model: String);
}

trait DriveOps: CarOps {            // inheriting trait (has supertrait CarOps)
    fn drive(&self);
}

struct Engine {
    horsepower: u32,
}

// struct composition - embedding one struct inside another
struct Car {
    model: String,
    engine: Engine,
}

impl CarOps for Car {

    fn get_model(&self) -> &str {
        &self.model
    }

    fn set_model(&mut self, model: String) {
        self.model = model;
    }
}

impl DriveOps for Car { // inheriting trait is implemented separately

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

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates the trait derive attribute. It tells the compiler to 
// automatically generate implementations of traits for the given struct.
//
// -- Debug: allows a value to be printed using {:?} for debugging.
// -- Default: provides a default value.
// -- Clone: allows duplication using deep copy (explicit clone()).
// -- Copy: allows duplication using implicit bitwise copy (no move).
// -- PartialEq: enables equality comparison using == and !=.
// -- PartialOrd: enables ordering comparison using <, >, <=, >=.
//

#[cfg(prog020)]
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

    // Copy and Clone
    let p3 = p;           // Bitwise copy (no move happens, i.e., ownership is not moved; without Copy, a move would occur)
    let p4 = p.clone();   // Explicit duplication using Clone
    println!("copy = {:?}, clone = {:?}", p3, p4);

    // PartialEq and PartialOrd
    println!("p == p3 = {}", p == p3);
    println!("p < Point{{x:3,y:4}} = {}", p < Point { x: 3, y: 4 });
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates Rust formatting traits from std::fmt.
//
// Rust std::fmt provides formatting traits like Display, Debug, Binary,
// and Octal that define how values are printed using format specifiers
// like {}, {:?}, {:b}, {:o}.
//
// -- {} uses Display trait (user-friendly format)
// -- {:?} uses Debug trait (debug format)
// -- {:#?} pretty Debug format
// -- {:b} uses Binary trait
// -- {:o} uses Octal trait
// -- {:x} uses LowerHex trait
// -- {:X} uses UpperHex trait
//

#[cfg(prog030)]
mod prog {

pub fn main() {

    // Integer (implements Display, Debug, Binary, Octal, Hex)
    {
        let x = 42;

        println!("Display: {}", x);
        println!("Debug: {:?}", x);
        println!("Binary: {:b}", x);
        println!("Octal: {:o}", x);
        println!("Lower hex: {:x}", x);
        println!("Upper hex: {:X}", x);
    }

    // Vector (implements Debug but not Display)
    {
        let v = vec![10, 20, 30];

        println!("Debug vector: {:?}", v);
        println!("Pretty debug vector: {:#?}", v);
    }

    // String (supports Display and Debug)
    {
        let s = String::from("Hello Rust");

        println!("Display string: {}", s);
        println!("Debug string: {:?}", s);
    }

    // Struct (traits must be explicitly derived or implemented)
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    // Note: For structs, some traits like Debug, Clone, Copy, and Default can 
    // be derived automatically using #[derive(...)], but Display must be 
    // implemented manually using impl std::fmt::Display.

    let pt = Point { x: 10, y: 20 };

    println!("Debug struct: {:?}", pt);
    println!("Pretty struct: {:#?}", pt);
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates Rust macros. A macro is a construct that generates 
// Rust code at compile time, helping reduce repetitive code.
//
// -- It shows println! macro.
// -- It shows vec! macro.
// -- It shows assert! macro.
// -- It shows a custom macro.
//

#[cfg(prog040)]
mod prog {

// Custom macro
macro_rules! print_twice {
    ($x:expr) => {
        println!("{}", $x);
        println!("{}", $x);
    };
}

// Note: It defines a macro using macro_rules! that matches any 
// expression ($x:expr) and expands it into two println! statements 
// printing that expression.

pub fn main() {

    // println! macro
    {
        let x = 10;
        println!("x = {}", x);
    }

    // vec! macro
    {
        let v = vec![1, 2, 3];
        println!("Vector length = {}", v.len());

        // Conceptual expansion:
        // let mut temp = Vec::new();
        // temp.push(1);
        // temp.push(2);
        // temp.push(3);
        // let v = temp;
    }

    // assert! macro
    {
        let x = 10;
        assert!(x > 0);

        // Conceptual expansion:
        // if !(x > 0) {
        //     panic!("assertion failed: x > 0");
        // }
    }

    // Custom macro
    {
        print_twice!(5);

        // Conceptual expansion:
        // println!("{}", 5);
        // println!("{}", 5);
    }
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates generics in Rust.
//
// -- It shows a generic function.
// -- It shows a generic struct.
// -- It shows a generic enum.
// -- It shows a generic function returning a generic enum.
// -- It shows a generic multiply function.
//

#[cfg(prog050)]
mod prog {

// generic function
fn identity<T>(value: T) -> T {
    value
}

// generic struct
struct BoxValue<T> {
    value: T,
}

// generic enum
enum SimpleResult<T> {
    Ok(T),
    Err,
}

// Generic function returning generic enum
fn check_greater<T: PartialOrd>(x: T, y: T) -> SimpleResult<T> {
    if x > y {
        SimpleResult::Ok(x)
    } else {
        SimpleResult::Err
    }
}

// Note: PartialOrd is a constraint that the generic type T must support 
// comparison operations like >, <, etc.

// Generic multiply function
fn multiply<T>(x: T, y: T) -> T
where
    T: std::ops::Mul<Output = T>
{
    x * y
}

// Note: std::ops::Mul<Output = T> is a constraint that the generic type T must 
// support multiplication (*) and return the same type T.

pub fn main() {

    // Generic function
    {
        let a = identity(10);
        let b = identity(3.14);

        println!("a={}, b={}", a, b);
    }

    // Generic struct
    {
        let v1 = BoxValue { value: 100 };
        let v2 = BoxValue { value: "Rust" };

        println!("BoxValue1={}, BoxValue2={}", v1.value, v2.value);
    }

    // Generic function returning generic enum
    {
        let r1 = check_greater(10, 5);
        match r1 {
            SimpleResult::Ok(v) => println!("Ok {}", v),
            SimpleResult::Err => println!("Err"),
        }

        let r2 = check_greater(3.5, 7.2);
        match r2 {
            SimpleResult::Ok(v) => println!("Ok {}", v),
            SimpleResult::Err => println!("Err"),
        }
    }

    // Generic multiply function
    {
        let m1 = multiply(6, 7);
        println!("multiply = {}", m1);

        let m2 = multiply(2.5, 4.0);
        println!("multiply = {}", m2);
    }
}
}

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates lifetimes in Rust.
// 
// A lifetime specifies how long a reference must remain valid (i.e., the 
// referenced data must live at least as long as the reference).
//
// -- It shows a global static reference.
// -- It shows a static reference defined inside a function.
// -- It shows lifetime annotations in functions.
// -- It shows lifetime annotations in structs.
// -- It shows lifetime annotations in trait methods.
// -- It shows lifetime annotations in impl blocks.
//

#[cfg(prog060)]
mod prog {

// Here it shows a global static reference. It lives for the entire duration 
// of the program.
static GLOBAL_CAR: &str = "Global Car: Tesla";

// Here it shows a static reference defined inside the function. It lives for 
// the entire duration of the program (but its scope is limited to this function).
fn get_static_car() {

    static LOCAL_CAR: &str = "Static Car: BMW";
    println!("Inside function: {}", LOCAL_CAR);
}

// Here the function uses lifetime annotation 'a. It matches the one used in 
// the input and return references. It means the returned reference
// lives at least as long as the input references.
fn choose_model<'a>(m1: &'a str, m2: &'a str) -> &'a str {
    if m1.len() > m2.len() { m1 } else { m2 }
}

// Here the struct uses lifetime annotation 'a. It means references stored in 
// the struct follow lifetime 'a.
struct Car<'a> {
    model: &'a str,
}

// Here the trait uses lifetime annotation 'a. It means references 
// returned/accepted by its methods follow lifetime 'a.
trait CarOps<'a> {
    fn get_model(&self) -> &'a str;
    fn set_model(&mut self, model: &'a str);
}

// Here the implementation uses lifetime annotation 'a. It matches the one 
// used in the struct and trait. It means references stored in the struct and 
// returned/accepted by the trait methods follow lifetime 'a.
impl<'a> CarOps<'a> for Car<'a> {

    fn get_model(&self) -> &'a str {
        self.model
    }

    fn set_model(&mut self, model: &'a str) {
        self.model = model;
    }
}

// Here the function uses lifetime annotation 'a. It matches the one used in 
// the struct argument (Car<'a>). It means references inside the struct follow 
// lifetime 'a.
fn print_car<'a>(car: &Car<'a>) {
    println!("Car model = {}", car.get_model());
}

pub fn main() {

    println!("{}", GLOBAL_CAR);

    get_static_car();

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

////////////////////////////////////////////////////////////////////////////////
//
// It demonstrates Rust visibility modifiers. The visibility applies to items
// such as modules, functions, structs, etc.
//
// -- pub makes an item visible everywhere, including other crates that
//    depend on this crate.
// -- pub(crate) makes an item visible within the current crate.
// -- pub(super) makes an item visible to the parent module.
// -- pub(self) makes an item visible only within the same module.
// -- Items without pub are private by default.
//
// -- A module in Rust is a namespace that groups related items (functions, 
//    structs, enums, traits, etc.). Modules can contain inner modules, 
//    creating a parent–child hierarchy.

#[cfg(prog070)]
mod prog {

mod outer {

    pub mod inner {

        pub fn i_public_visible() {
            i_self_visible();
            super::o_private_visible(); // inner can call parent's private functions
        }

        pub(super) fn i_parent_visible() {
            println!("i_parent_visible");
        }

        pub(self) fn i_self_visible() { // like private only
            println!("i_self_visible");
        }
    }

    pub fn o_public_visible() {
        println!("o_public_visible");

        o_private_visible();
        inner::i_parent_visible();
    }

    pub(crate) fn o_crate_visible() {
        println!("o_crate_visible");
    }

    fn o_private_visible() {
        println!("o_private_visible");
    }
}

pub fn main() {

    outer::o_public_visible();
    outer::o_crate_visible();

    outer::inner::i_public_visible();
}

}

