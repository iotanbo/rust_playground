
// Constants:
//   * can be global scope
//   * must explicitly declare type
//   * may be set only to a constant expression (computed at compile time)
//   * should be upper case
const DEMO_CONST: u32 = 42;

pub fn demo() {

    println!("== variables and constants demo begin ==");

    // 1. Constant variable
    println!("  * DEMO_CONST: {}", DEMO_CONST);

    // 2. Immutable variable
    //let a = 5;  // An immutable variable
    //a = 6;  // Error: can't assign twice to immutable variable

    // 3. Mutable variable
    let mut b = 5;
    b += 7;
    println!("  * Mutable variable b: {}", b);

    // 4. Shadowed variable
    // Variable can be shadowed with 'let' keyword:
    let b = String::from("new 'b' value is string");
    println!("  * Shadowed variable b: {}", b.to_uppercase());

    println!("== variables and constants demo end ==");
    println!();
}