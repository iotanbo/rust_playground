
// This is the main executable of crate 'mylib'

use mylib::A_mod;
use mylib::B_mod;


pub fn main() {

    println!("== MAIN APP START ==");

    A_mod::demo();
    B_mod::demo();
    B_mod::b_one::demo();  // b_one submodule is visible
    // B_mod::b_two::demo();  // b_two submodule is not declared as pub and is not visible
    B_mod::inner::demo();  // complex submodule of B_mod
    B_mod::inner::sm1::demo();  // submodule of B_mod's inner submodule

    println!("== MAIN APP END ==");
}
