// This is a simplified binary crate consisting of one file
// and using `mylib` lib

// To run this file: cargo run --bin single_file_exe


use mylib::A_mod;


pub fn main() {
    println!("== SINGLE-FILE EXE START ==");

    A_mod::demo();

    println!("== SINGLE-FILE EXE END ==");
}

#[allow(dead_code)]
fn some_private_func() -> &'static str {
    "some_private_func"
}


// TESTS
// Run tests for this crate only: `cargo test --bin single_file_exe`
#[cfg(test)]
mod test {

use super::*;

#[test]
fn test_some_private_func() {
    assert_eq!("some_private_func", some_private_func());
}

}  // mod test
