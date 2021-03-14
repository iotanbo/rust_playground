// This is an example of a single-file module



pub fn demo() -> &'static str {
    let msg = "  * This is A_module demo";
    println!("{}", msg);
    msg
}

#[allow(dead_code)]
fn private_func() -> &'static str {
    "private_func"
}



/** UNIT TESTS.
 * It seems like rust encourages you to write unit tests directly in the src file.
 * If you put them in a separate file `test_...rs` and that file is not included into the library,
 * the tests will be ignored.
*/
#[cfg(test)]  // indicates that section must be built only for the test config
mod test {

    // makes parent context visible
    use super::*;

/// Even though this function is not annotated with # [test],
/// it will be build only for test config as it is located inside 'test' module
fn some_aux_test_func() -> &'static str {
    "private_func"
}

/// Simple test, can be executed with `cargo test`
/// or `cargo test --lib`
#[test]
fn test_demo() {
    assert_eq!(demo(), "  * This is A_module demo");
}

#[test]
fn test_private_func() {
    assert_eq!(some_aux_test_func(), private_func());
}

}  // mod test
