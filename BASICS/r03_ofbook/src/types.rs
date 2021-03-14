// https://doc.rust-lang.org/book/ch03-02-data-types.html


fn scalar_types_demo() {

    // Primary scalar types: integers, floats, bools, chars
    // Signed: i8 ... i128, unsigned: u8 ... u128
    // Size (arch dependent): isize, usize

    // Visual separators are supported:
    let mut c: u32 = 32_000;  // same as 32000
    c += 100u32;  // explicit type in number literal
    println!("  * c is: {}", c);

    // Binary value:
    let my_byte:i8 = -0b10000000;  // minus because it represents -128 value
    println!("  * myByte is: {}", my_byte);

    // Hex and octal value examples: 0xFF, 0o77

    // Characters are 4 bytes long
    let heart_eyed_cat = '😻';
    println!("  * heart_eyed_cat: {}", heart_eyed_cat);

}


fn compound_types_demo() {

    // Tuples
    let tup = (500, 6.4, 1);  // i32, f64, i32
    let (_, y, _) = tup;
    println!("  * The value of y is: {}", y);

    // Accessing to tuple element by index:
    let first_tuple_elem = tup.0;
    println!("  * first_tuple_elem: {}", first_tuple_elem);

    // Arrays: much like C arrays
    let my_array = [1, 2, 3, 4, 5];
    println!("  * third element of my_array is: {}", my_array[2]);

    // Type and size of array can be specified:

    let week_days: [&str; 7] = ["Sunday", "Monday", "Tuesday", "Wednesday", 
        "Thursday", "Friday", "Saturday"];

    print!("  * Week days: "); 
    println!("{:?}", week_days);
}

// Newtype pattern demo
// https://doc.rust-lang.org/book/ch19-04-advanced-types.html

// A simple wrapper object: a tuple that has a single element of type 'Vec<String>'
struct StrVecWrapper(Vec<String>);

use std::fmt;

// Now that we have a wrapper object, it is possible to implement custom display trait
// for a vector of strings
impl fmt::Display for StrVecWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " * Custom formatted vector of strings: [{}]", self.0.join(", "))
    }
}

fn newtype_pattern_demo() {
    
    // * can be used to Implement External Traits on External Types
    // * no runtime performance penalty for using this pattern
    let v = StrVecWrapper(vec!["one".to_owned(), "two".to_owned(), "three".to_owned()]);
    // Now we can easily print vector of strings using our custom format
    println!("{}", &v);

    // * newtype pattern is a lightweight way to achieve encapsulation 
    //   to hide implementation details: we can e.g. implement only those methods of Vec
    //   in StrVecWrapper that are desired for our public API 
    // * it is also possible to give access to all Vec methods by implementing Deref trait.


}


pub fn demo() {

    println!("== types demo begin ==");

    // 1. Support of inferred types
    // let a = 5;  // Inferred type is i32 (default for integers)

    // 2. Type can be explicitly specified
    // let guess: u32 = "42".parse().expect("Not a number!");

    scalar_types_demo();
    compound_types_demo();
    newtype_pattern_demo();

    println!("== types demo end ==");
    println!();
    
}
