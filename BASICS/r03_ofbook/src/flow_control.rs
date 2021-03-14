


fn basic_if() {

    fn is_odd(x: i32) -> bool { x % 2 != 0 }

    let num = -30;

    if num < 5 {
        println!("  * {} < 5: true", num);
    } else {
        println!("  * {} < 5: false", num);
    }

    // If in assignment (ternary operator analog)
    let num_is_odd = if is_odd(num) { "odd" } else { "even" };
    println!("  * number {} is {}", num, num_is_odd);
}


fn basic_loops() {

    let mut counter = 0;

    // 1. Infinite loop
    // Note that loop returns a value
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // Loop will return 20
        }
    };
    println!("  * loop result is {}", result);

    // 2. While - loop
    counter = 3;
    while counter > 0 {
        println!("   {}", counter);
        counter -= 1;
    }
    println!("   GO!");

    // 3. Looping thru a collection
    let a = [10, 20, 30];
    println!("  * Integer array values: ");
    for elem in &a {
        println!("     {}", elem);
    }

    // 4. Range-based:
    for n in (1..4).rev() {  // Note that right range edge '4' is not included
        println!("   {}", n);
    }
    println!("   GO!");
}


fn if_let_demo() {
    println!();
    println!("-- if-let demo --");

    let v1 = Some(42);

    // Check if 'v1' is Some, if so, execute code in curly braces;
    // 'val' is an arbitrary name for the local variable to be passed (bound) 
    // to the code in curly braces.
    // !In other words: 'val' does not participate in 'if' comparison!
    // Human-readable: if Some that takes 'val' as a parameter, happens to be the v1 type:
    if let Some(val) = v1 {  
        if val == 5 { println!("  * Value is 5 (if-let variant)! "); }
        else { println!("  * Some({}) and v1 match (if-let variant)! ", val); }
    }

    // Normal 'if' statement can only check the concrete value
    if Some(5) == v1 {
        println!("  * Some(5) and v1 match (regular if variant)! ");
    }

    // Same:
    match v1 {
        // val does not participate in comparison. 
        // It is just an arbitrary name, see enums demo for more explanation.
        // What is important here, is type 'Some'.
        Some(val) => {
                if val == 5 { println!("  * Value is 5 (match variant)! "); }
                else { println!("  * Some({}) and v1 match (match variant)! ", val); }
                }
        _ => (),
    }
}

fn while_loop_with_pattern_matching() {

    let mut v = vec!["orange", "green", "blue"];

    // Simplest way to iterate a vector
    for val in &v {
        println!("  * color: {}", val);
    }

    // Iterating a vector including index
    for (i, val) in v.iter().enumerate() {
        println!("  * color: '{}' at index: {}", val, i);
    }

    // Iterate and remove elements
    while let Some(val) = v.pop() {
        println!("  * color: {}", val);
    }
}



pub fn demo() {

    println!("== flow control demo begin ==");

    basic_if();
    basic_loops();
    if_let_demo();
    while_loop_with_pattern_matching();

    println!("== flow control demo end ==");
    println!();
}
