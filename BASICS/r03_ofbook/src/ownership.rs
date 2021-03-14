// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// Rust uses RAII

// C++ destructor equivalent: drop() function
// Calling by reference is named 'borrowing'
// You can have only one mutable reference to a particular piece of data in a particular scope


fn string_copy_demo() {

    // 1. Shallow copy
    let s1 = String::from("s1");
    let mut s2 = s1;  // This is what in C++ called: move;
    // Now s1 is not accessible
    // println!("  * s1: {}", s1);
    println!("  * s2: {}", s2);
    s2 = String::from("s2");
    println!("  * s2: {}", s2);

    // 2. Deep copy
    let mut s3 = s2.clone();
    println!("  * s2: {}", s2);
    println!("  * s3: {}", s3);
    s3 = String::from("s3 modified");
    println!("  * s2: {}", s2);
    println!("  * s3: {}", s3);
}

fn function_params() {

    fn makes_copy(num: i32) {
        println!("  * num squared is: {}", num * num);
    }

    fn takes_ownership(msg: String) {
        println!("  * msg is: {}", msg);
    }

    fn takes_ref(msg: &mut String) {
        msg.push_str(" was modified by 'takes_ref()'");
    }

    // msg is moved to this function then moved back to caller
    fn takes_and_returns_ownership(mut msg: String) -> String {
        msg.push_str(" was modified by 'takes_and_returns_ownership()'");
        msg
    }

    let msg = String::from("Hello, Rust!");
    let num = 5;

    // 1. Call by value and loosing ownership on complex objects
    makes_copy(num);
    takes_ownership(msg);
    println!("  * num after calling makes_copy(): {}", num);  // num still can be used
    // msg can't be used because it was moved to takes_ownership() and destroyed there
    // println!("  * msg after calling takes_ownership(): {}", msg);

    // 2. Calling and returning values using moving semantic (== by value)
    let mut msg2 = String::from("Another message");
    msg2 = takes_and_returns_ownership(msg2);
    println!("  * msg after calling 'takes_and_returns_ownership()': {}", msg2);

    // 3. Calling and modifying objects using mutable references
    let mut msg3 = String::from("message 3");
    takes_ref(&mut msg3);
    println!("  * msg3 after calling 'takes_ref()': {}", msg3);

}

fn slice_demo() {

    // 1. String slices
    // Note: str is a String slice type
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        let mut i = 0;
        for &b in bytes {
            if b == b' ' || b == b',' { break; }
            i += 1;
        }
        &s[..i]
    }

    let s = String::from("Hello, Rust!");
    let w1 = first_word(&s);
    //let mut first_word = &s[..4];  // same as [0..4]
    let w2 = &s[7..s.len()-1];
    println!("  * First word: {}", w1);
    println!("  * Last word: {}", w2);

    // 2. Array slices
    let a = [1, 2, 3, 4, 5];
    let a_slice = &a[1..3];  // type of a_slice is '&[i32]', remember: right range edge not included
    println!("  * slice of int array: {:?}", a_slice);

}


pub fn demo() {

    println!("== ownership demo begin ==");

    string_copy_demo();
    function_params();
    slice_demo();

    println!("== ownership demo end ==");
    println!();
}
