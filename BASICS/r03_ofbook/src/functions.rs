//https://doc.rust-lang.org/book/ch03-03-how-functions-work.html

// * Statements do not return values
// * Expressions evaluate to something and return result as a value

// Example of function that returns a value
fn fourty_two() -> i32 {
    // 42 is an expression, there is no semicolon after it.
    // it is same as return 42;
    42  // NO SEMICOLON!
}


pub fn demo() {

    println!("== functions demo begin ==");

    println!("  * fourty_two(): {}", fourty_two());
    

    println!("== functions demo end ==");
    println!();
}
