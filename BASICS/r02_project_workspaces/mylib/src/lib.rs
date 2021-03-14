
// Make module public so that executable can us it directly
pub mod layout_description;


pub fn mylib_version() {
    println!("  * mylib v.0.1.0");
    println!();
}

pub fn print_layout_description() {
    layout_description::describe();
}
