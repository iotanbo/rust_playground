// https://doc.rust-lang.org/std/option/enum.Option.html

// Option<T> enum is one of the most useful and ideomatic enums in Rust

// * represents a value or its absence
// * same purpose as a null pointer has in C, but with compiler-checked type safety
// 
// For much more examples see the official documentation ^


fn basic_option_demo() {

    // Create an option of type i32 that contains value 42
    let o1 = Some(42);
    // Get o1 value and compare it to 42
    assert_eq!(o1.unwrap(), 42);

    // Create an option of type i32 that does not contain value
    let o2: Option<i32> = None;

    // Check that o2 does not contain valid value
    assert_eq!(o2.is_none(), true);
    // Get o2 value and compare it to 42: this WILL PANIC because o2 is None
    // assert_eq!(o2.unwrap(), 42);

    // Get a value or assign default '33'
    let v2 = o2.unwrap_or(33);
    assert_eq!(v2, 33);
    
}


fn advanced_option_demo() {

    let o1 = Some(42);
    let o2: Option<i32> = None;

    fn divisible_by_3(val: i32) -> bool { val%3 == 0 }

    // Apply a function to the contained value, return result as Optional as well
    // let mut maybe_divisible_by_3 = o1.map(|v| v%3 == 0);  // using a closure
    let mut maybe_divisible_by_3 = o1.map(divisible_by_3);
    assert_eq!(maybe_divisible_by_3.unwrap(), true);

    // Because o2 is None, is_div_by_3() is not called
    maybe_divisible_by_3 = o2.map(divisible_by_3);
    assert_eq!(maybe_divisible_by_3.is_none(), true);
}


pub fn demo() {

    println!("== Option<T> enum demo begin ==");

    basic_option_demo();
    advanced_option_demo();

    println!("== Option<T> enum demo end ==");
    println!();
}
