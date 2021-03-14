// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
// 


/* Declaration looks as follows:

enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

use std::fs::File;
use std::io::ErrorKind;
use std::io::prelude::*;


// 1. Nested match example (too verbose, not ideomatic)
fn result_type_nested_match() {

    const FILENAME: &str = "hello.txt";
    let f = File::open(FILENAME);
    // Check the result
    let mut f = match f {
        Ok(file) => file,
        // if error, check error kind
        Err(error) => match error.kind() {
            // if file not exists, try to create one
            ErrorKind::NotFound => match File::create(FILENAME) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file '{}': {:?}", FILENAME, e),
            },
            other_error => {
                panic!("Problem opening the file '{}': {:?}", FILENAME, other_error)
            }
        },
    };

    // At this point, it's guaranteed that file is successfully opened
    let mut contents = String::new();
    if let Ok(_) = f.read_to_string(&mut contents) {
        println!(" Contents of '{}': {}", FILENAME, contents);
    }

}

// 2. Still verbose, many parenthesis and braces
fn error_handling_with_unwrap_or_else() {

    const FILENAME: &str = "hello.txt";
    // unwrap_or_else: return value if success or execute lambda if error
    let mut f = File::open(FILENAME).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(FILENAME).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // At this point, it's guaranteed that file is successfully opened
    let mut contents = String::new();
    if let Ok(_) = f.read_to_string(&mut contents) {
        println!(" Contents of '{}' (var.2): {}", FILENAME, contents);
    }

}

// 3. unwrap: return result or panic with error message 
//    expect: return result or panic with specified message + error message;
fn unwrap_and_expect_demo() {
    const FILENAME: &str = "hello.txt";
    // let mut f = File::open(FILENAME).unwrap();
    // or
    let mut f = File::open(FILENAME).expect(&format!("File {} expected to exist.", FILENAME));

    // At this point, file is either successfully opened or app panicked.
    let mut contents = String::new();
    if let Ok(_) = f.read_to_string(&mut contents) {
        println!(" Contents of '{}' (var.3): {}", FILENAME, contents);
    }

}

// 4. Explicitly propagate errors (still verbose)
fn print_file_contents(file: &str) -> Result<(), std::io::Error> {
    let f = File::open(file);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => { println!(" Contents of '{}' (var.4): {}", file, s); Ok(())},
        Err(e) => Err(e),
    }
}

// 5. Propagate errors using '?' operator (ideomatic)
fn print_file_contents_ideomatic(file: &str) -> Result<(), std::io::Error> {
    let mut f = File::open(file)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    println!(" Contents of '{}' (var.5 ideomatic): {}", file, s);
    Ok(())
}

// 6. Shorter notation with method chaining (ideomatic)
fn print_file_contents_method_chaining(file: &str) -> Result<(), std::io::Error> {
    let mut s = String::new();
    File::open(file)?.read_to_string(&mut s)?;
    println!(" Contents of '{}' (var.6 method chaining): {}", file, s);
    Ok(())
}

// 7. The shortest version that uses single read_to_string() function
fn print_file_shortest_version(file: &str) -> Result<(), std::io::Error> {
    let s = std::fs::read_to_string(file)?;
    println!(" Contents of '{}' (var.7 the shortest): {}", file, s);
    Ok(())
}


pub fn demo() {

    println!("== result_type demo begin ==");

    result_type_nested_match();
    error_handling_with_unwrap_or_else();
    unwrap_and_expect_demo();

    // Print contents of following files:
    let files = ["dummy.txt", "hello.txt"];
    // errors that may occur in 'print_file_contents()' and 'print_file_contents_ideomatic()'
    // will be propagated and processed here
    for i in 0..files.len() {
        let f = files[i];

        print_file_contents(f).unwrap_or_else(
            // Why do we take 'error' (name is arbitrary) as parameter?
            // Because print_file_contents() returns Result<(), std::io::Error>,
            // so in case of error the returned value will be of type 'std::io::Error'
            |error: std::io::Error| {  println!("  * File '{}' can't be opened: {}", f, error)}
        );

        print_file_contents_ideomatic(f).unwrap_or_else(
            |error| {  println!("  * File '{}' can't be opened: {}", f, error)}
        );

        print_file_contents_method_chaining(f).unwrap_or_else(
            |error| {  println!("  * File '{}' can't be opened: {}", f, error)}
        );

        print_file_shortest_version(f).unwrap_or_else(
            |error| {  println!("  * File '{}' can't be opened: {}", f, error)}
        );

        

    }
    
    
    println!("== result_type demo end ==");
    println!();
}
