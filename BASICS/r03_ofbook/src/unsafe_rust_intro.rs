// https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html


fn raw_pointers_demo() {

    let mut a = 5;

    // It's allowed to have multiple raw pointers, either mutable or not
    let pa = &mut a as *mut i32;
    let cpa = &a as *const i32;

    unsafe {
        *pa += 7;
        println!(" * a modified from dereferenced raw pointer: {}", *cpa);
    }
    
    println!(" * a memory address: 0x{:X}", cpa as usize);
}



pub fn demo() {

    println!("== unsafe rust demo begin==");

    raw_pointers_demo();


    println!("== unsafe rust demo end==");
    println!();
}