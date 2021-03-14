// This is an example of complex module with submodules

// `mod.rs` is a standard file name and should not be changed!

// make visible b_one submodule:
pub mod b_one;

// inner submodule with even more complex structure
pub mod inner;

// private modules
mod b_two;

pub fn demo() {
    println!("  * This is a B-module demo.");
    b_two::demo();
}
