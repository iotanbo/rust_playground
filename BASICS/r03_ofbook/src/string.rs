// https://doc.rust-lang.org/book/ch08-02-strings.html
// 

// * 'str' slice type belongs to rust lang itself, while String is from the standard library

fn string_basics() {

    // 1. Create from literal
    // data type showed explicitly, it's equivalent to const char* in C
    let data: &str = "initial contents";  
    let _s = data.to_string();

    // 2. Create new empty string
    let mut s = String::new();

    // 3. Append string slice
    s.push_str("bar");
    println!("  * {}", s);

    // 4. Concatenate string and slice
    s += " updated";
    println!("  * {}", s);

    // 5. Concatenate two strings
    let s2 = " even more".to_string();
    s += &s2;
    println!("  * {}", s);

    // 6. Formatting
    let tack: String = "tack".to_string();
    let s3 = format!("{}-{}", "tick".to_string(), tack);
    println!("  * {}", s3);

    // 7. Iterating characters
    let namaste = "नमस्ते";
    for c in namaste.chars() {
        println!("{}", c);
    }

    // 8. Iterating bytes
    
    // manual string join for nicer format
    // let namaste_len = namaste.len();
    // let mut namaste_bytes_as_nums = String::from("[");
    // for (i, b) in namaste.bytes().enumerate() {
    //     if i < namaste_len-1 {
    //         namaste_bytes_as_nums.push_str(&format!("{}, ", b));
    //     } else {
    //         namaste_bytes_as_nums.push_str(&format!("{}]", b));
    //     }
    // }
    // println!("{}", namaste_bytes_as_nums);

    let namaste_bytes = namaste.bytes();
    println!("  * Namaste bytes ({}): {:?}",namaste_bytes.len(), namaste_bytes);
}


pub fn demo() {

    println!("== string demo begin ==");

    string_basics();

    println!("== string demo end ==");
    println!();
}
