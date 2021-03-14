
mod vars_and_consts;
mod types;
mod functions;
mod flow_control;
mod ownership;
mod structs;
mod enums;
mod option_enum;
mod vector;
mod slice;
mod string;
mod hash_map;
mod result_type;
mod custom_error_type;
mod generics;
mod traits;
mod lifetime_spec;
mod testing;
mod error_handling;
mod iterators;
mod smart_pointers;
mod concurrency;
mod polymorphism;
mod unsafe_rust_intro;
mod function_pointers;



fn main() {
    vars_and_consts::demo();
    
    functions::demo();
    
    ownership::demo();
    structs::demo();
    enums::demo();
    option_enum::demo();
    vector::demo();
    slice::demo();
    string::demo();
    hash_map::demo();
    result_type::demo();
    custom_error_type::demo();
    generics::demo();
    traits::demo();
    lifetime_spec::demo();
    testing::demo();
    error_handling::demo();
    iterators::demo();
    smart_pointers::demo();
    concurrency::demo();
    polymorphism::demo();
    flow_control::demo();
    unsafe_rust_intro::demo();
    types::demo();
    function_pointers::demo();
}
