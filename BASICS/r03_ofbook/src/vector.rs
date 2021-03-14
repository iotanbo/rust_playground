// https://doc.rust-lang.org/book/ch08-01-vectors.html


// In Rust, it's more common to pass slices as arguments rather than vectors 
//   when you just want to provide read access. 
//   The same goes for String and &str.

fn vector_basics() {

    // 1. Create
    let mut v1: Vec<i32> = Vec::new();  // new() is an analog of C++ constructor

    // 2. Add value to the end
    v1.push(100);

    // 3. Debug print
    println!("{:?}", v1);

    // 4. Create with a macro
    let mut v2 = vec![1, 2, 3];
    println!("{:?}", v2);

    // 5. Access element by index with making a copy of element
    if v1[0] == 100 { println!("  * First element of v1 is 100.") }
    println!("{:?}", v1);
    // Because copy is made, we can safely modify vector
    let v1_copy_el0 = v1[0];
    v1.push(200);
    println!("The first v1 element is: {}", v1_copy_el0);

    // 5.1 Access element by index and return immutable reference
    let v1_ref_el0 = &v1[0];
    // v1.push(300);  <- WILL PANIC because v1_ref_el0 may implicitly change if reallocation occurs
    println!("The first v1 element is: {}", v1_ref_el0);

    // 5.2 Access element by index and return mutable reference
    let v1_ref_el0 = &mut v1[0];
    // Modify element in place, much like C
    *v1_ref_el0 = -128;
    println!("The first v1 element is: {}", v1_ref_el0);

    // 6. Create vector<u8> of size 5 filled with zeroes
    let vec: Vec<u8> = vec![0; 5];
    assert_eq!(vec, [0, 0, 0, 0, 0]);   

    // 7. Remove and process all values from the vector
    while let Some(val) = v2.pop() {  // this cycle will automatically brake when top is None
        println!("  * processing value: {}", val);
    }
    assert_eq!(v2.len(), 0);

    // 8. Safely get vector's element
    match v2.get(2) {
        Some(third) => println!("The third element of v2 is {}", third),
        _ => println!("v2 does not have third element."),
    }

    // 9. Create vector of elements of different types using enum as a wrapper
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Cmd {
        Go(i32, i32),
        Write(String),
        Quit,
    }
    use Cmd::*;

    let mut cmds: Vec<Cmd> = vec![Write("Hello".to_string()), Go(2, 2)];
    cmds.push(Quit);
    println!("  * cmds: {:?}", cmds);

    // 10. Access by slice
    // 10.1. Define function that takes immutable vector slice as a parameter
    fn print_elements(slice_name: &str, slice: &[i32]) {
        println!("  * Printing elements of '{}': ", slice_name);
        for i in 0..slice.len() {
            println!("    * {}[{}]: {}", slice_name, i, slice[i]);
        }
    }
    print_elements("v1", &v1[..]);

}



pub fn demo() {

    println!("== vector demo begin ==");

    vector_basics();



    println!("== vector demo end ==");
    println!();
}

