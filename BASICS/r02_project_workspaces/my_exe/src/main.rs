
// My lib is an external package now
extern crate mylib;


use mylib::layout_description::advantages;
// use mylib::layout_description::disadvantages;

fn main() {
    println!("== MAIN EXE APP START ==");

    mylib::mylib_version();
    mylib::layout_description::describe();

    // Demonstrates access to one of mylib's submodules
    let _advantages = advantages::enumerate();

    // let _disadvantages = disadvantages::enumerate();

    mylib::layout_description::summarize();

    println!("== MAIN EXE APP START ==");
}
