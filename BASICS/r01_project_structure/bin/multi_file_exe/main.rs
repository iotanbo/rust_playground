

// To run this file: cargo run --bin multi_file_exe


use mylib::A_mod;


pub fn main() {
    println!("== MULTI-FILE EXE START ==");

    A_mod::demo();

    println!("== MULTI-FILE END ==");
}
